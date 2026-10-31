use super::*;
use crate::portas::falsos::{RaizFalsa, RelogioFalso};

struct Nm;
impl NetworkManager for Nm {
    fn conexao(&self) -> std::io::Result<Conexao> {
        Ok(serde_json::from_str(include_str!("../../fixtures/rede/conectada.json")).unwrap())
    }
}
struct Ping(f64);
impl Sonda for Ping {
    fn medir(&self, _: std::net::IpAddr) -> Option<f64> {
        Some(self.0)
    }
}
#[test]
fn publica_qualidade_com_relogio_injetado_sem_depender_do_painel() {
    let relogio = RelogioFalso::em(1000);
    let mut rede = Rede::new(RaizFalsa::de("rede"), Nm, Ping(20.0), relogio.clone());
    let (hora, dados) = rede.passo().unwrap();
    assert_eq!(hora, 1000);
    assert_eq!(dados.qualidade, Some(Qualidade::Boa));
    assert_eq!(dados.internet.latencia, Some(20.0));
    assert!(rede.passo().is_none());
    relogio.avancar(1000);
    assert!(rede.passo().is_some());
}

#[test]
fn sem_resposta_em_ambos_destinos_publica_ruim() {
    struct SemGateway;
    impl NetworkManager for SemGateway {
        fn conexao(&self) -> std::io::Result<Conexao> {
            let mut c = Nm.conexao()?;
            c.gateway = None;
            Ok(c)
        }
    }
    struct SemResposta;
    impl Sonda for SemResposta {
        fn medir(&self, _: std::net::IpAddr) -> Option<f64> {
            None
        }
    }
    let mut rede = Rede::new(
        RaizFalsa::de("rede"),
        SemGateway,
        SemResposta,
        RelogioFalso::em(1000),
    );
    assert_eq!(rede.passo().unwrap().1.qualidade, Some(Qualidade::Ruim));
}
#[test]
fn gateway_nao_informado_nao_conta_como_perda() {
    struct SemGateway;
    impl NetworkManager for SemGateway {
        fn conexao(&self) -> std::io::Result<Conexao> {
            let mut c = Nm.conexao()?;
            c.gateway = None;
            Ok(c)
        }
    }
    let mut rede = Rede::new(
        RaizFalsa::de("rede"),
        SemGateway,
        Ping(20.0),
        RelogioFalso::em(1000),
    );
    let dados = rede.passo().unwrap().1;
    assert_eq!(dados.qualidade, Some(Qualidade::Boa));
    assert_eq!(dados.gateway.perda, None);
}

#[derive(Clone)]
struct NmFalso(std::sync::Arc<std::sync::Mutex<Conexao>>);
impl NetworkManager for NmFalso {
    fn conexao(&self) -> std::io::Result<Conexao> {
        Ok(self.0.lock().unwrap().clone())
    }
}
#[derive(Clone)]
struct SondaFalsa(std::sync::Arc<std::sync::Mutex<(Option<f64>, Option<f64>)>>);
impl Sonda for SondaFalsa {
    fn medir(&self, ip: std::net::IpAddr) -> Option<f64> {
        let v = *self.0.lock().unwrap();
        if ip == "1.1.1.1".parse::<std::net::IpAddr>().unwrap() {
            v.1
        } else {
            v.0
        }
    }
}
struct Cenario {
    rede: Rede<RaizFalsa, NmFalso, SondaFalsa, RelogioFalso>,
    raiz: RaizFalsa,
    nm: NmFalso,
    sonda: SondaFalsa,
    relogio: RelogioFalso,
}
fn cenario() -> Cenario {
    let raiz = RaizFalsa::de("rede");
    let nm = NmFalso(std::sync::Arc::new(std::sync::Mutex::new(
        Nm.conexao().unwrap(),
    )));
    let sonda = SondaFalsa(std::sync::Arc::new(std::sync::Mutex::new((
        Some(2.0),
        Some(20.0),
    ))));
    let relogio = RelogioFalso::em(1_793_457_120_000);
    Cenario {
        rede: Rede::new(raiz.clone(), nm.clone(), sonda.clone(), relogio.clone()),
        raiz,
        nm,
        sonda,
        relogio,
    }
}
fn medir(c: &mut Cenario, g: Option<f64>, i: Option<f64>) -> DadosRede {
    *c.sonda.0.lock().unwrap() = (g, i);
    let dados = c.rede.passo().unwrap().1;
    c.relogio.avancar(5000);
    dados
}
#[test]
fn limites_de_latencia_sao_estritos() {
    for (gateway, internet, esperado) in [
        (30.0, 100.0, Qualidade::Boa),
        (30.1, 100.0, Qualidade::Regular),
        (2.0, 100.1, Qualidade::Regular),
        (2.0, 300.0, Qualidade::Regular),
        (2.0, 300.1, Qualidade::Ruim),
    ] {
        assert_eq!(
            medir(&mut cenario(), Some(gateway), Some(internet)).qualidade,
            Some(esperado)
        );
    }
}
#[test]
fn jitter_maior_que_30_em_qualquer_destino_pede_regular() {
    for no_gateway in [true, false] {
        let mut c = cenario();
        medir(&mut c, Some(0.0), Some(20.0));
        let dados = if no_gateway {
            medir(&mut c, Some(30.0), Some(20.0))
        } else {
            medir(&mut c, Some(0.0), Some(50.0))
        };
        assert_eq!(dados.qualidade, Some(Qualidade::Boa));
        let dados = medir(&mut c, Some(0.0), Some(20.0));
        assert_eq!(dados.qualidade, Some(Qualidade::Boa));
        let dados = if no_gateway {
            medir(&mut c, Some(31.0), Some(20.0))
        } else {
            medir(&mut c, Some(0.0), Some(51.0))
        };
        assert_eq!(dados.qualidade, Some(Qualidade::Regular));
    }
}
#[test]
fn perda_entre_1_e_5_regular_acima_de_5_ruim_em_ambos_destinos() {
    for no_gateway in [true, false] {
        let mut c = cenario();
        for _ in 0..19 {
            medir(&mut c, Some(2.0), Some(20.0));
        }
        let dados = if no_gateway {
            medir(&mut c, None, Some(20.0))
        } else {
            medir(&mut c, Some(2.0), None)
        };
        assert_eq!(
            if no_gateway {
                dados.gateway.perda
            } else {
                dados.internet.perda
            },
            Some(5.0)
        );
        assert_eq!(dados.qualidade, Some(Qualidade::Regular));
        let dados = if no_gateway {
            medir(&mut c, None, Some(20.0))
        } else {
            medir(&mut c, Some(2.0), None)
        };
        assert_eq!(dados.qualidade, Some(Qualidade::Ruim));
    }
}
#[test]
fn janela_movel_expira_perdas_apos_dois_minutos() {
    let mut c = cenario();
    medir(&mut c, None, None);
    for _ in 0..23 {
        medir(&mut c, Some(2.0), Some(20.0));
    }
    let dados = medir(&mut c, Some(2.0), Some(20.0));
    assert_eq!(dados.internet.perda, Some(0.0));
    assert_eq!(dados.qualidade, Some(Qualidade::Boa));
}
#[test]
fn mediana_e_jitter_vem_da_janela_e_ping_so_a_cada_5s() {
    let mut c = cenario();
    medir(&mut c, Some(2.0), Some(10.0));
    medir(&mut c, Some(2.0), Some(50.0));
    let dados = medir(&mut c, Some(2.0), Some(20.0));
    assert_eq!(dados.internet.latencia, Some(20.0));
    assert_eq!(dados.internet.jitter, Some(35.0));
    *c.sonda.0.lock().unwrap() = (None, None);
    // medir já avançou cinco segundos; captura uma vez e verifica os quatro ticks seguintes.
    let anterior = c.rede.passo().unwrap().1.internet;
    for _ in 0..4 {
        c.relogio.avancar(1000);
        assert_eq!(c.rede.passo().unwrap().1.internet, anterior);
    }
}
#[test]
fn sem_internet_com_gateway_respondendo_e_ruim() {
    let mut c = cenario();
    c.nm.0.lock().unwrap().conectividade = 1;
    assert_eq!(
        medir(&mut c, Some(2.0), Some(20.0)).qualidade,
        Some(Qualidade::Ruim)
    );
}
#[test]
fn desconexao_e_troca_de_rede_zeram_medidas_e_trafego() {
    let mut c = cenario();
    medir(&mut c, None, None);
    let conexao = c.nm.0.lock().unwrap().clone();
    *c.nm.0.lock().unwrap() = Conexao::default();
    let dados = medir(&mut c, None, None);
    assert!(!dados.conectada);
    assert_eq!(dados.qualidade, None);
    assert_eq!(dados.internet.perda, None);
    assert!(dados.historico.is_empty());
    *c.nm.0.lock().unwrap() = conexao;
    assert_eq!(
        medir(&mut c, Some(2.0), Some(20.0)).qualidade,
        Some(Qualidade::Boa)
    );
}
#[test]
fn delta_por_segundo_ignora_loopback_e_tunel_e_aceita_reset() {
    let mut c = cenario();
    assert!(c.rede.passo().unwrap().1.trafego.is_none());
    c.raiz.escrever("/proc/net/dev","wlo1: 818860497 0 0 0 0 0 0 0 122353707\nlo: 99999999 0 0 0 0 0 0 0 9999999\ntun0: 99999999 0 0 0 0 0 0 0 9999999\n");
    c.relogio.avancar(1000);
    let dados = c.rede.passo().unwrap().1;
    assert_eq!(
        dados.trafego,
        Some(Trafego {
            descida: 2048.0,
            subida: 1024.0
        })
    );
    c.raiz
        .escrever("/proc/net/dev", "wlo1: 10 0 0 0 0 0 0 0 20\n");
    c.relogio.avancar(1000);
    assert!(c.rede.passo().unwrap().1.trafego.is_none());
}
#[test]
fn fixtures_da_ui_sao_snapshots_publicados_pelo_nucleo() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/lib/rede/fixtures");
    for nome in ["boa", "ruim", "sem-conexao", "vpn"] {
        let mut c = cenario();
        if nome == "sem-conexao" {
            *c.nm.0.lock().unwrap() = Conexao::default();
        }
        if nome == "vpn" {
            let mut nm = c.nm.0.lock().unwrap();
            nm.vpn = true;
            nm.tipo = "802-3-ethernet".into();
        }
        let internet = if nome == "ruim" { 301.0 } else { 20.0 };
        medir(&mut c, Some(2.0), Some(internet));
        c.raiz
            .escrever("/proc/net/dev", "wlo1: 818868689 0 0 0 0 0 0 0 122357803\n");
        c.rede.passo();
        c.raiz
            .escrever("/proc/net/dev", "wlo1: 818869713 0 0 0 0 0 0 0 122358315\n");
        c.relogio.avancar(1000);
        let (quando, dados) = c.rede.passo().unwrap();
        let json = serde_json::to_string_pretty(&snapshot(quando, &dados)).unwrap() + "\n";
        let arquivo = dir.join(format!("{nome}.json"));
        if std::env::var_os("ATUALIZAR_FIXTURES").is_some() {
            std::fs::write(arquivo, json).unwrap();
        } else {
            assert_eq!(std::fs::read_to_string(arquivo).unwrap(), json);
        }
    }
}
