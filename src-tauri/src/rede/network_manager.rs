use super::{Conexao, Wifi};
use serde::Deserialize;
use std::io;
use zbus::blocking::{Connection, Proxy};
use zbus::zvariant::OwnedObjectPath;

pub trait NetworkManager: Send {
    fn conexao(&self) -> io::Result<Conexao>;
    fn verificar_portal(&self) -> Option<bool> {
        None
    }
}

/// Propriedades lidas do D-Bus, antes de escolher o link físico.
#[derive(Clone, Deserialize)]
pub struct LeituraNetworkManager {
    pub primaria: String,
    pub conectividade: u32,
    pub ativas: Vec<ConexaoAtiva>,
    #[serde(default)]
    pub revisao: u64,
}
#[derive(Clone, Deserialize)]
pub struct ConexaoAtiva {
    pub caminho: String,
    pub estado: u32,
    pub tipo: String,
    pub vpn: bool,
    pub interface: String,
    pub gateway4: String,
    pub gateway6: String,
    #[serde(default)]
    pub wifi: Option<Wifi>,
}

/// Porta das propriedades do NetworkManager: os testes injetam a saída D-Bus gravada.
pub trait PropriedadesNetworkManager: Send {
    fn ler(&self) -> io::Result<LeituraNetworkManager>;
    fn portal(&self) -> Option<bool> {
        None
    }
}
pub struct NetworkManagerDbus<P = DbusSistema>(pub P);
pub struct DbusSistema(std::sync::Arc<std::sync::Mutex<Option<LeituraNetworkManager>>>);
const DESTINO: &str = "org.freedesktop.NetworkManager";
impl NetworkManagerDbus {
    pub fn conectar() -> Self {
        let cache = std::sync::Arc::new(std::sync::Mutex::new(None));
        let leitura = cache.clone();
        std::thread::spawn(move || loop {
            // Assina antes da leitura inicial, evitando perder uma troca de AP/perfil.
            let resultado = (|| -> zbus::Result<()> {
                let bus = Connection::system()?;
                let regra = zbus::MatchRule::builder()
                    .msg_type(zbus::message::Type::Signal)
                    .sender(DESTINO)?
                    .path_namespace("/org/freedesktop/NetworkManager")?
                    .build();
                let mensagens =
                    zbus::blocking::MessageIterator::for_match_rule(regra, &bus, Some(256))?;
                *leitura.lock().unwrap() = Some(ler_dbus()?);
                let mut revisao = 0;
                for mensagem in mensagens {
                    mensagem?;
                    revisao += 1;
                    let mut atual = ler_dbus()?;
                    atual.revisao = revisao;
                    *leitura.lock().unwrap() = Some(atual);
                }
                Ok(())
            })();
            let _ = resultado;
            *leitura.lock().unwrap() = None;
            std::thread::sleep(std::time::Duration::from_secs(5));
        });
        Self(DbusSistema(cache))
    }
}
impl<P: PropriedadesNetworkManager> NetworkManager for NetworkManagerDbus<P> {
    fn verificar_portal(&self) -> Option<bool> {
        self.0.portal()
    }
    fn conexao(&self) -> io::Result<Conexao> {
        let leitura = self.0.ler()?;
        let mut fisica: Option<Conexao> = None;
        let mut vpn = false;
        for ativa in leitura.ativas.into_iter().filter(|a| a.estado == 2) {
            let eh_vpn = ativa.vpn || ativa.tipo == "wireguard";
            vpn |= eh_vpn;
            if eh_vpn || ativa.tipo == "loopback" {
                continue;
            }
            // A conexão primária pode ser a VPN; nesse caso conserva o link físico ativo.
            if fisica.is_some() && ativa.caminho != leitura.primaria {
                continue;
            }
            fisica = Some(Conexao {
                id: ativa.caminho,
                tipo: ativa.tipo,
                conectividade: leitura.conectividade,
                vpn: false,
                gateway: ativa
                    .gateway4
                    .parse()
                    .ok()
                    .or_else(|| ativa.gateway6.parse().ok()),
                interface: ativa.interface,
                wifi: ativa.wifi,
                revisao: leitura.revisao,
            });
        }
        let mut conexao = fisica.unwrap_or_default();
        conexao.vpn = vpn;
        Ok(conexao)
    }
}
impl PropriedadesNetworkManager for DbusSistema {
    fn portal(&self) -> Option<bool> {
        let bus = Connection::system().ok()?;
        let nm = Proxy::new(&bus, DESTINO, "/org/freedesktop/NetworkManager", DESTINO).ok()?;
        let estado: u32 = nm.call("CheckConnectivity", &()).ok()?;
        Some(estado == 2)
    }
    fn ler(&self) -> io::Result<LeituraNetworkManager> {
        self.0
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| io::Error::other("NetworkManager indisponível"))
    }
}
fn ler_dbus() -> zbus::Result<LeituraNetworkManager> {
    let bus = Connection::system()?;
    let nm = Proxy::new(&bus, DESTINO, "/org/freedesktop/NetworkManager", DESTINO)?;
    let primaria: OwnedObjectPath = nm.get_property("PrimaryConnection")?;
    let caminhos: Vec<OwnedObjectPath> = nm.get_property("ActiveConnections")?;
    let mut ativas = Vec::new();
    for caminho in caminhos {
        let ativa = Proxy::new(
            &bus,
            DESTINO,
            caminho.clone(),
            "org.freedesktop.NetworkManager.Connection.Active",
        )?;
        let estado: u32 = ativa.get_property("State")?;
        let tipo: String = ativa.get_property("Type")?;
        let vpn: bool = ativa.get_property("Vpn")?;
        let mut leitura = ConexaoAtiva {
            caminho: caminho.to_string(),
            estado,
            tipo,
            vpn,
            interface: String::new(),
            gateway4: String::new(),
            gateway6: String::new(),
            wifi: None,
        };
        if estado == 2 && !vpn && !matches!(leitura.tipo.as_str(), "loopback" | "wireguard") {
            let dispositivos: Vec<OwnedObjectPath> = ativa.get_property("Devices")?;
            if let Some(d) = dispositivos.first() {
                leitura.interface = Proxy::new(
                    &bus,
                    DESTINO,
                    d.clone(),
                    "org.freedesktop.NetworkManager.Device",
                )?
                .get_property("Interface")?;
            }
            if leitura.tipo == "802-11-wireless" {
                if let Some(d) = dispositivos.first() {
                    let wifi = Proxy::new(
                        &bus,
                        DESTINO,
                        d.clone(),
                        "org.freedesktop.NetworkManager.Device.Wireless",
                    )?;
                    let ap: OwnedObjectPath = wifi.get_property("ActiveAccessPoint")?;
                    if ap.as_str() != "/" {
                        let ap = Proxy::new(
                            &bus,
                            DESTINO,
                            ap,
                            "org.freedesktop.NetworkManager.AccessPoint",
                        )?;
                        let perfil: OwnedObjectPath = ativa.get_property("Connection")?;
                        let perfil = Proxy::new(
                            &bus,
                            DESTINO,
                            perfil,
                            "org.freedesktop.NetworkManager.Settings.Connection",
                        )?;
                        // GetSettings omite segredos; nunca chama GetSecrets.
                        let settings: std::collections::HashMap<
                            String,
                            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
                        > = perfil.call("GetSettings", &())?;
                        let seguranca = settings.get("802-11-wireless-security");
                        let key_mgmt = seguranca
                            .and_then(|s| s.get("key-mgmt"))
                            .and_then(|v| <&str>::try_from(v).ok())
                            .unwrap_or("")
                            .to_string();
                        let pairwise = seguranca
                            .and_then(|s| s.get("pairwise"))
                            .and_then(|v| v.try_clone().ok())
                            .and_then(|v| Vec::<String>::try_from(v).ok())
                            .unwrap_or_default();
                        leitura.wifi = Some(Wifi {
                            flags: ap.get_property("Flags")?,
                            wpa: ap.get_property("WpaFlags")?,
                            rsn: ap.get_property("RsnFlags")?,
                            key_mgmt,
                            pairwise,
                        });
                    }
                }
            }
            for (campo, contrato, gateway) in [
                (
                    "Ip4Config",
                    "org.freedesktop.NetworkManager.IP4Config",
                    &mut leitura.gateway4,
                ),
                (
                    "Ip6Config",
                    "org.freedesktop.NetworkManager.IP6Config",
                    &mut leitura.gateway6,
                ),
            ] {
                let config: OwnedObjectPath = ativa.get_property(campo)?;
                if config.as_str() != "/" {
                    *gateway =
                        Proxy::new(&bus, DESTINO, config, contrato)?.get_property("Gateway")?;
                }
            }
        }
        ativas.push(leitura);
    }
    Ok(LeituraNetworkManager {
        revisao: 0,
        primaria: primaria.to_string(),
        conectividade: nm.get_property("Connectivity")?,
        ativas,
    })
}
