use super::Conexao;
use serde::Deserialize;
use std::io;
use zbus::blocking::{Connection, Proxy};
use zbus::zvariant::OwnedObjectPath;

pub trait NetworkManager: Send {
    fn conexao(&self) -> io::Result<Conexao>;
}

/// Propriedades lidas do D-Bus, antes de escolher o link físico.
#[derive(Clone, Deserialize)]
pub struct LeituraNetworkManager {
    pub primaria: String,
    pub conectividade: u32,
    pub ativas: Vec<ConexaoAtiva>,
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
}

/// Porta das propriedades do NetworkManager: os testes injetam a saída D-Bus gravada.
pub trait PropriedadesNetworkManager: Send {
    fn ler(&self) -> io::Result<LeituraNetworkManager>;
}
pub struct NetworkManagerDbus<P = DbusSistema>(pub P);
pub struct DbusSistema;
const DESTINO: &str = "org.freedesktop.NetworkManager";
impl NetworkManagerDbus {
    pub fn conectar() -> Self {
        Self(DbusSistema)
    }
}
impl<P: PropriedadesNetworkManager> NetworkManager for NetworkManagerDbus<P> {
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
            });
        }
        let mut conexao = fisica.unwrap_or_default();
        conexao.vpn = vpn;
        Ok(conexao)
    }
}
impl PropriedadesNetworkManager for DbusSistema {
    fn ler(&self) -> io::Result<LeituraNetworkManager> {
        ler_dbus().map_err(io::Error::other)
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
        primaria: primaria.to_string(),
        conectividade: nm.get_property("Connectivity")?,
        ativas,
    })
}
