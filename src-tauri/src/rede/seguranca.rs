use super::{Conexao, DadosRede, NetworkManager};
use crate::portas::{Notificacao, Notificador};
use serde::{Deserialize, Serialize};

const PRIVACY: u32 = 0x1;
const PAIR_TKIP: u32 = 0x4;
const GROUP_TKIP: u32 = 0x40;
const OWE: u32 = 0x800;
const OWE_TRANSITION: u32 = 0x1000;

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct Wifi {
    pub flags: u32,
    pub wpa: u32,
    pub rsn: u32,
    pub key_mgmt: String,
    pub pairwise: Vec<String>,
}
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Seguranca {
    SemAlertas,
    Atencao,
    Risco,
}
impl DadosRede {
    pub fn badge_de_risco(&self) -> bool {
        self.conectada && self.seguranca == Some(Seguranca::Risco)
    }
}
pub trait Verificacao: Send {
    /// Some(true): respondeu ao nome inexistente; None: consulta inconclusiva.
    fn dns(&self) -> Option<bool>;
    fn mac(&self, conexao: &Conexao) -> Option<String>;
}
pub struct SemVerificacao;
impl Verificacao for SemVerificacao {
    fn dns(&self) -> Option<bool> {
        None
    }
    fn mac(&self, _: &Conexao) -> Option<String> {
        None
    }
}
pub struct SemNotificador;
impl Notificador for SemNotificador {
    fn notificar(&self, _: Notificacao) {}
}
#[derive(Default)]
pub struct Avaliacao {
    proxima: u64,
    dns: Option<bool>,
    portal: bool,
    conectividade: Option<u32>,
    revisao_mac: Option<u64>,
    mac: Option<String>,
    mac_mudou: bool,
    notificou: bool,
}
impl Avaliacao {
    pub fn avaliar(
        &mut self,
        c: &Conexao,
        agora: u64,
        v: &impl Verificacao,
        nm: &impl NetworkManager,
        n: &impl Notificador,
    ) -> (Option<Seguranca>, Vec<String>) {
        if c.id.is_empty() {
            return (None, vec![]);
        }
        if agora >= self.proxima {
            self.proxima = agora.saturating_add(600_000);
            self.dns = v.dns();
            self.portal = nm.verificar_portal().unwrap_or(c.conectividade == 2);
        }
        if self
            .conectividade
            .is_some_and(|anterior| anterior != c.conectividade)
        {
            self.portal = c.conectividade == 2;
        }
        self.conectividade = Some(c.conectividade);
        if self.revisao_mac != Some(c.revisao) {
            self.revisao_mac = Some(c.revisao);
            if let Some(mac) = v.mac(c) {
                if self.mac.as_ref().is_some_and(|anterior| anterior != &mac) {
                    self.mac_mudou = true;
                }
                self.mac = Some(mac);
            }
        }
        let mut riscos = vec![];
        if c.tipo == "802-11-wireless" {
            if let Some(w) = &c.wifi {
                let owe = w.rsn & (OWE | OWE_TRANSITION) != 0 || w.key_mgmt == "owe";
                if !owe && w.wpa == 0 && w.rsn == 0 {
                    riscos.push(
                        if w.flags & PRIVACY == 0 {
                            "Rede aberta sem OWE"
                        } else {
                            "Criptografia WEP"
                        }
                        .to_string(),
                    );
                } else if (w.wpa | w.rsn) & (PAIR_TKIP | GROUP_TKIP) != 0
                    || w.pairwise.iter().any(|p| p == "tkip")
                {
                    riscos.push("WPA com TKIP".into());
                }
            }
        }
        if self.dns == Some(true) {
            riscos.push("DNS respondeu a um nome inexistente".into());
        }
        let risco = !riscos.is_empty();
        if self.portal {
            riscos.push("Portal cativo detectado".into());
        }
        if self.mac_mudou {
            riscos.push("MAC do gateway mudou nesta conexão".into());
        }
        let estado = if risco {
            Seguranca::Risco
        } else if !riscos.is_empty() {
            Seguranca::Atencao
        } else {
            Seguranca::SemAlertas
        };
        if risco && !self.notificou {
            self.notificou = true;
            n.notificar(Notificacao::RedeEmRisco);
        }
        (Some(estado), riscos)
    }
}

/// Resposta do D-Bus sem o nome consultado nem conteúdo de outros serviços.
#[derive(Clone, Deserialize)]
pub struct RespostaDns {
    pub respondeu: bool,
    pub erro: Option<String>,
}
pub trait LeiturasSeguranca: Send {
    fn resolver(&self) -> Option<RespostaDns>;
    fn vizinhos(&self, c: &Conexao) -> Option<String>;
}
pub struct LeiturasSistema;
pub struct VerificacaoSistema<P = LeiturasSistema>(pub P);
impl<P: LeiturasSeguranca> Verificacao for VerificacaoSistema<P> {
    fn dns(&self) -> Option<bool> {
        let r = self.0.resolver()?;
        match r.erro.as_deref() {
            None => Some(r.respondeu),
            Some("org.freedesktop.resolve1.DnsError.NXDOMAIN") => Some(false),
            _ => None,
        }
    }
    fn mac(&self, c: &Conexao) -> Option<String> {
        let vizinhos: serde_json::Value = serde_json::from_str(&self.0.vizinhos(c)?).ok()?;
        let gateway = c.gateway?.to_string();
        vizinhos
            .as_array()?
            .iter()
            .filter(|v| v.get("dst").and_then(|v| v.as_str()) == Some(gateway.as_str()))
            .find_map(|v| {
                v.get("lladdr")?
                    .as_str()
                    .filter(|s| !s.is_empty())
                    .map(str::to_owned)
            })
    }
}
type RespostaResolveHostname = (Vec<(i32, i32, Vec<u8>)>, String, u64);

impl LeiturasSeguranca for LeiturasSistema {
    fn resolver(&self) -> Option<RespostaDns> {
        // Nome novo a cada consulta; usa o resolvedor da conexão atual, inclusive VPN.
        let aleatorio = std::fs::read_to_string("/proc/sys/kernel/random/uuid").ok()?;
        let nome = format!("{}.com", aleatorio.trim());
        let bus = zbus::blocking::Connection::system().ok()?;
        let proxy = zbus::blocking::Proxy::new(
            &bus,
            "org.freedesktop.resolve1",
            "/org/freedesktop/resolve1",
            "org.freedesktop.resolve1.Manager",
        )
        .ok()?;
        let resposta: zbus::Result<RespostaResolveHostname> =
            proxy.call("ResolveHostname", &(0i32, nome, 0i32, 0u64));
        match resposta {
            Ok((enderecos, _, _)) => Some(RespostaDns {
                respondeu: !enderecos.is_empty(),
                erro: None,
            }),
            Err(zbus::Error::MethodError(nome, _, _)) => Some(RespostaDns {
                respondeu: false,
                erro: Some(nome.to_string()),
            }),
            _ => None,
        }
    }
    fn vizinhos(&self, c: &Conexao) -> Option<String> {
        let gateway = c.gateway?;
        let saida = std::process::Command::new("ip")
            .args([
                "-j",
                "neigh",
                "show",
                "to",
                &gateway.to_string(),
                "dev",
                &c.interface,
            ])
            .output()
            .ok()?;
        if !saida.status.success() {
            return None;
        }
        String::from_utf8(saida.stdout).ok()
    }
}
