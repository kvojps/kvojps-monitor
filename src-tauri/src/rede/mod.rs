//! Qualidade e tráfego: coleta contínua, sem cache em disco nem notificações.
mod network_manager;
mod sonda;
#[cfg(test)]
mod testes;

use crate::portas::{Arquivos, Relogio};
pub use network_manager::{NetworkManager, NetworkManagerDbus};
use serde::{Deserialize, Serialize};
pub use sonda::{PingSocket, Sonda};
use std::collections::{HashMap, VecDeque};
use std::net::{IpAddr, SocketAddr, SocketAddrV6};

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct Conexao {
    pub id: String,
    pub tipo: String,
    pub conectividade: u32,
    pub vpn: bool,
    pub gateway: Option<IpAddr>,
    pub interface: String,
}
impl Conexao {
    fn conectada(&self) -> bool {
        !self.id.is_empty()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Qualidade {
    Boa,
    Regular,
    Ruim,
}
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Medidas {
    pub latencia: Option<f64>,
    pub jitter: Option<f64>,
    pub perda: Option<f64>,
}
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Trafego {
    pub descida: f64,
    pub subida: f64,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DadosRede {
    pub conectada: bool,
    pub cabeada: bool,
    pub vpn: bool,
    pub qualidade: Option<Qualidade>,
    pub gateway: Medidas,
    pub internet: Medidas,
    pub trafego: Option<Trafego>,
    pub historico: Vec<Trafego>,
}

type Contadores = HashMap<String, (u64, u64)>;

pub struct Rede<A, N, S, R> {
    arquivos: A,
    nm: N,
    sonda: S,
    relogio: R,
    proximo: u64,
    proximo_ping: u64,
    conexao: Conexao,
    amostras: VecDeque<(u64, Option<f64>, Option<f64>)>,
    anterior: Option<(u64, Contadores)>,
    historico: VecDeque<Trafego>,
}
impl<A: Arquivos, N: NetworkManager, S: Sonda, R: Relogio> Rede<A, N, S, R> {
    pub fn new(arquivos: A, nm: N, sonda: S, relogio: R) -> Self {
        Self {
            arquivos,
            nm,
            sonda,
            relogio,
            proximo: 0,
            proximo_ping: 0,
            conexao: Conexao::default(),
            amostras: VecDeque::new(),
            anterior: None,
            historico: VecDeque::new(),
        }
    }
    pub fn passo(&mut self) -> Option<(u64, DadosRede)> {
        let agora = self.relogio.agora();
        if agora < self.proximo {
            return None;
        }
        self.proximo = agora + 1000;
        // O NM é consultado mesmo com o Painel fechado. Erro de D-Bus não reutiliza um link antigo.
        let conexao = self.nm.conexao().unwrap_or_default();
        if self.conexao.id != conexao.id
            || self.conexao.interface != conexao.interface
            || self.conexao.gateway != conexao.gateway
        {
            self.amostras.clear();
            self.anterior = None;
            self.historico.clear();
            self.proximo_ping = 0;
        }
        self.conexao = conexao;
        let conectada = self.conexao.conectada();
        let trafego = self.trafego(self.relogio.agora());
        if conectada && agora >= self.proximo_ping {
            self.proximo_ping = agora + 5000;
            let gateway = self.conexao.gateway.map(|ip| match ip {
                IpAddr::V6(ip) if ip.is_unicast_link_local() => {
                    let indice = self
                        .arquivos
                        .ler(&format!(
                            "/sys/class/net/{}/ifindex",
                            self.conexao.interface
                        ))
                        .ok()
                        .and_then(|s| s.trim().parse().ok())
                        .unwrap_or(0);
                    SocketAddr::V6(SocketAddrV6::new(ip, 0, 0, indice))
                }
                _ => SocketAddr::new(ip, 0),
            });
            let internet: SocketAddr = if gateway.is_some_and(|g| g.is_ipv6()) {
                "[2606:4700:4700::1111]:0".parse().unwrap()
            } else {
                "1.1.1.1:0".parse().unwrap()
            };
            let (gateway, internet) = std::thread::scope(|scope| {
                let gateway = scope.spawn(|| gateway.and_then(|ip| self.sonda.medir(ip)));
                let internet = self.sonda.medir(internet);
                (gateway.join().unwrap_or(None), internet)
            });
            self.amostras.push_back((agora, gateway, internet));
        }
        while self
            .amostras
            .front()
            .is_some_and(|a| agora.saturating_sub(a.0) >= 120_000)
        {
            self.amostras.pop_front();
        }
        let gateway = if self.conexao.gateway.is_some() {
            medidas(self.amostras.iter().map(|a| a.1))
        } else {
            Medidas::default()
        };
        let internet = medidas(self.amostras.iter().map(|a| a.2));
        let qualidade =
            conectada.then(|| classificar(&gateway, &internet, self.conexao.conectividade));
        if let Some(t) = &trafego {
            self.historico.push_back(t.clone());
            while self.historico.len() > 60 {
                self.historico.pop_front();
            }
        }
        Some((
            agora,
            DadosRede {
                conectada,
                cabeada: self.conexao.tipo == "802-3-ethernet",
                vpn: self.conexao.vpn,
                qualidade,
                gateway,
                internet,
                trafego,
                historico: self.historico.iter().cloned().collect(),
            },
        ))
    }
    fn trafego(&mut self, agora: u64) -> Option<Trafego> {
        let texto = match self.arquivos.ler("/proc/net/dev") {
            Ok(t) => t,
            Err(_) => {
                self.anterior = None;
                return None;
            }
        };
        let atuais: HashMap<_, _> = texto
            .lines()
            .filter_map(|linha| {
                let (nome, numeros) = linha.split_once(':')?;
                let valores: Vec<_> = numeros.split_whitespace().collect();
                Some((
                    nome.trim().to_string(),
                    (
                        valores.first()?.parse::<u64>().ok()?,
                        valores.get(8)?.parse::<u64>().ok()?,
                    ),
                ))
            })
            .collect();
        let anterior = self.anterior.replace((agora, atuais.clone()));
        let (quando, anteriores) = anterior?;
        if !self.conexao.conectada() {
            return None;
        }
        // Conta o link físico uma vez: evita duplicar tráfego de túneis/VPN e loopback.
        let nome = &self.conexao.interface;
        let (rx, tx) = atuais.get(nome)?;
        let (rx_antes, tx_antes) = anteriores.get(nome)?;
        let segundos = agora.checked_sub(quando)? as f64 / 1000.0;
        if segundos <= 0.0 || rx < rx_antes || tx < tx_antes {
            return None;
        }
        Some(Trafego {
            descida: (rx - rx_antes) as f64 / segundos,
            subida: (tx - tx_antes) as f64 / segundos,
        })
    }
}
fn medidas(amostras: impl Iterator<Item = Option<f64>>) -> Medidas {
    let amostras: Vec<_> = amostras.collect();
    if amostras.is_empty() {
        return Medidas::default();
    }
    let mut valores: Vec<f64> = amostras.iter().flatten().copied().collect();
    valores.sort_by(f64::total_cmp);
    let n = valores.len();
    let latencia = if n == 0 {
        None
    } else if n.is_multiple_of(2) {
        Some((valores[n / 2 - 1] + valores[n / 2]) / 2.0)
    } else {
        Some(valores[n / 2])
    };
    let variacoes: Vec<_> = amostras
        .windows(2)
        .filter_map(|p| Some((p[1]? - p[0]?).abs()))
        .collect();
    Medidas {
        latencia,
        jitter: (!variacoes.is_empty())
            .then(|| variacoes.iter().sum::<f64>() / variacoes.len() as f64),
        perda: Some((amostras.len() - n) as f64 * 100.0 / amostras.len() as f64),
    }
}
fn classificar(g: &Medidas, i: &Medidas, conectividade: u32) -> Qualidade {
    let acima = |v: Option<f64>, limite| v.is_some_and(|v| v > limite);
    if acima(g.perda, 5.0)
        || acima(i.perda, 5.0)
        || acima(i.latencia, 300.0)
        || (g.latencia.is_some() && (i.latencia.is_none() || matches!(conectividade, 1 | 3)))
    {
        Qualidade::Ruim
    } else if acima(g.perda, 1.0)
        || acima(i.perda, 1.0)
        || acima(i.latencia, 100.0)
        || acima(g.latencia, 30.0)
        || acima(g.jitter, 30.0)
        || acima(i.jitter, 30.0)
    {
        Qualidade::Regular
    } else {
        Qualidade::Boa
    }
}
pub fn iniciar(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let mut rede = Rede::new(
            crate::portas::Raiz::sistema(),
            NetworkManagerDbus::conectar(),
            PingSocket,
            crate::portas::RelogioDoSistema,
        );
        loop {
            if let Some((quando, dados)) = rede.passo() {
                crate::snapshot::publicar(&app, snapshot(quando, &dados));
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}
fn snapshot(quando: u64, dados: &DadosRede) -> crate::snapshot::Snapshot {
    crate::snapshot::Snapshot {
        modulo: crate::modulo::Modulo::Rede,
        atualizado_em: Some(quando),
        dados: serde_json::to_value(dados).unwrap(),
    }
}
