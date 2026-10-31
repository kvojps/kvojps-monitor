use super::Conexao;
use std::io;
use zbus::blocking::{Connection, Proxy};
use zbus::zvariant::OwnedObjectPath;

pub trait NetworkManager: Send {
    fn conexao(&self) -> io::Result<Conexao>;
}
pub struct NetworkManagerDbus;
const DESTINO: &str = "org.freedesktop.NetworkManager";
impl NetworkManager for NetworkManagerDbus {
    fn conexao(&self) -> io::Result<Conexao> {
        ler().map_err(io::Error::other)
    }
}
fn ler() -> zbus::Result<Conexao> {
    let bus = Connection::system()?;
    let nm = Proxy::new(&bus, DESTINO, "/org/freedesktop/NetworkManager", DESTINO)?;
    let primaria: OwnedObjectPath = nm.get_property("PrimaryConnection")?;
    let ativas: Vec<OwnedObjectPath> = nm.get_property("ActiveConnections")?;
    let mut vpn = false;
    let mut fisica = None;
    for caminho in ativas {
        let ativa = Proxy::new(
            &bus,
            DESTINO,
            caminho.clone(),
            "org.freedesktop.NetworkManager.Connection.Active",
        )?;
        if ativa.get_property::<u32>("State")? != 2 {
            continue;
        }
        let tipo: String = ativa.get_property("Type")?;
        let eh_vpn: bool = ativa.get_property("Vpn")?;
        vpn |= eh_vpn || tipo == "wireguard";
        if eh_vpn || tipo == "wireguard" || tipo == "loopback" {
            continue;
        }
        // PrimaryConnection pode ser a VPN; nesse caso seleciona o link físico ativo.
        if fisica.is_some() && caminho != primaria {
            continue;
        }
        let dispositivos: Vec<OwnedObjectPath> = ativa.get_property("Devices")?;
        let interface = match dispositivos.first() {
            Some(d) => Proxy::new(
                &bus,
                DESTINO,
                d.clone(),
                "org.freedesktop.NetworkManager.Device",
            )?
            .get_property::<String>("Interface")?,
            None => String::new(),
        };
        let mut gateway = None;
        for (campo, contrato) in [
            ("Ip4Config", "org.freedesktop.NetworkManager.IP4Config"),
            ("Ip6Config", "org.freedesktop.NetworkManager.IP6Config"),
        ] {
            let config: OwnedObjectPath = ativa.get_property(campo)?;
            if config.as_str() != "/" {
                let ip = Proxy::new(&bus, DESTINO, config, contrato)?;
                let endereco: String = ip.get_property("Gateway")?;
                gateway = endereco.parse().ok();
                if gateway.is_some() {
                    break;
                }
            }
        }
        fisica = Some(Conexao {
            id: caminho.to_string(),
            tipo,
            conectividade: nm.get_property("Connectivity")?,
            vpn: false,
            gateway,
            interface,
        });
    }
    let mut conexao = fisica.unwrap_or_default();
    conexao.vpn = vpn;
    Ok(conexao)
}
