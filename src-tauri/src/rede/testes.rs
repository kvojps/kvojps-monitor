use super::*;
use crate::portas::falsos::{RaizFalsa, RelogioFalso};

use super::network_manager::{LeituraNetworkManager, PropriedadesNetworkManager};
struct DbusFalso(LeituraNetworkManager);
impl PropriedadesNetworkManager for DbusFalso {
    fn ler(&self) -> std::io::Result<LeituraNetworkManager> {
        Ok(self.0.clone())
    }
}
fn leitura_nm() -> LeituraNetworkManager {
    serde_json::from_str(include_str!("../../fixtures/rede/networkmanager.json")).unwrap()
}
struct Nm;
impl NetworkManager for Nm {
    fn conexao(&self) -> std::io::Result<Conexao> {
        NetworkManagerDbus(DbusFalso(leitura_nm())).conexao()
    }
}
struct Ping(f64);
impl Sonda for Ping {
    fn medir(&self, _: std::net::SocketAddr) -> Option<f64> {
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
        fn medir(&self, _: std::net::SocketAddr) -> Option<f64> {
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
    fn medir(&self, ip: std::net::SocketAddr) -> Option<f64> {
        let v = *self.0.lock().unwrap();
        if ip.ip() == "1.1.1.1".parse::<std::net::IpAddr>().unwrap() {
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
    for nome in ["boa", "ruim", "sem-conexao", "vpn", "atencao", "risco"] {
        let mut c = cenario();
        if nome == "atencao" {
            c.nm.0.lock().unwrap().conectividade = 2;
        }
        if nome == "risco" {
            c.nm.0.lock().unwrap().wifi = Some(Wifi::default());
        }
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

#[test]
fn mudanca_de_conectividade_preserva_janela_na_mesma_rede() {
    let mut c = cenario();
    medir(&mut c, None, None);
    c.nm.0.lock().unwrap().conectividade = 3;
    let dados = medir(&mut c, Some(2.0), Some(20.0));
    assert_eq!(dados.internet.perda, Some(50.0));
    assert_eq!(dados.qualidade, Some(Qualidade::Ruim));
}
#[test]
fn sonda_lenta_nao_distorce_taxa_de_trafego() {
    use crate::portas::{Arquivos, Espaco, Relogio};
    struct ArquivosComTrafego(RelogioFalso);
    impl Arquivos for ArquivosComTrafego {
        fn ler(&self, _: &str) -> std::io::Result<String> {
            Ok(format!(
                "wlo1: {} 0 0 0 0 0 0 0 {}\n",
                self.0.agora(),
                self.0.agora() / 2
            ))
        }
        fn listar(&self, _: &str) -> std::io::Result<Vec<String>> {
            unreachable!()
        }
        fn espaco(&self, _: &str) -> std::io::Result<Espaco> {
            unreachable!()
        }
    }
    struct SondaLenta(RelogioFalso);
    impl Sonda for SondaLenta {
        fn medir(&self, ip: std::net::SocketAddr) -> Option<f64> {
            if ip.ip() == "1.1.1.1".parse::<std::net::IpAddr>().unwrap() {
                self.0.avancar(700);
            }
            Some(20.0)
        }
    }
    let relogio = RelogioFalso::em(1000);
    let mut rede = Rede::new(
        ArquivosComTrafego(relogio.clone()),
        Nm,
        SondaLenta(relogio.clone()),
        relogio.clone(),
    );
    rede.passo();
    relogio.avancar(300);
    assert_eq!(
        rede.passo().unwrap().1.trafego,
        Some(Trafego {
            descida: 1000.0,
            subida: 500.0
        })
    );
}

#[test]
fn gateway_ipv6_link_local_preserva_interface_e_mede_internet_ipv6() {
    struct SondaIpv6;
    impl Sonda for SondaIpv6 {
        fn medir(&self, destino: std::net::SocketAddr) -> Option<f64> {
            match destino {
                std::net::SocketAddr::V6(v6) if v6.ip().is_unicast_link_local() => {
                    (v6.scope_id() == 7).then_some(2.0)
                }
                std::net::SocketAddr::V6(_) => Some(20.0),
                _ => None,
            }
        }
    }
    let c = cenario();
    c.raiz.escrever("/sys/class/net/wlo1/ifindex", "7\n");
    c.nm.0.lock().unwrap().gateway = Some("fe80::1".parse().unwrap());
    let mut rede = Rede::new(c.raiz, c.nm, SondaIpv6, c.relogio);
    let dados = rede.passo().unwrap().1;
    assert_eq!(dados.gateway.latencia, Some(2.0));
    assert_eq!(dados.internet.latencia, Some(20.0));
    assert_eq!(dados.qualidade, Some(Qualidade::Boa));
}

#[test]
fn dbus_seleciona_link_fisico_com_vpn_primaria_e_ignora_loopback() {
    let mut leitura = leitura_nm();
    let mut vpn = leitura.ativas[0].clone();
    vpn.caminho = "/org/freedesktop/NetworkManager/ActiveConnection/3".into();
    vpn.tipo = "vpn".into();
    vpn.vpn = true;
    vpn.interface = "tun0".into();
    leitura.primaria = vpn.caminho.clone();
    leitura.ativas.insert(0, vpn);
    let relogio = RelogioFalso::em(1000);
    let mut rede = Rede::new(
        RaizFalsa::de("rede"),
        NetworkManagerDbus(DbusFalso(leitura)),
        Ping(20.0),
        relogio,
    );
    let dados = rede.passo().unwrap().1;
    assert!(dados.vpn);
    assert!(dados.conectada);
    assert_eq!(dados.gateway.latencia, Some(20.0));
    assert_eq!(dados.qualidade, Some(Qualidade::Boa));
}
#[test]
fn apenas_loopback_nao_e_conexao_externa() {
    let mut leitura = leitura_nm();
    leitura.ativas.remove(0);
    leitura.primaria = "/".into();
    let mut rede = Rede::new(
        RaizFalsa::de("rede"),
        NetworkManagerDbus(DbusFalso(leitura)),
        Ping(20.0),
        RelogioFalso::em(1000),
    );
    let dados = rede.passo().unwrap().1;
    assert!(!dados.conectada);
    assert_eq!(dados.qualidade, None);
}
#[test]
fn conectividade_limitada_com_gateway_vivo_e_ruim_mesmo_com_mediana_antiga_boa() {
    let mut c = cenario();
    for _ in 0..23 {
        medir(&mut c, Some(2.0), Some(20.0));
    }
    c.nm.0.lock().unwrap().conectividade = 3;
    let dados = medir(&mut c, Some(2.0), None);
    assert_eq!(dados.internet.latencia, Some(20.0));
    assert_eq!(dados.qualidade, Some(Qualidade::Ruim));
}

#[test]
fn risco_notifica_uma_vez_por_conexao_e_badge_some_ao_desconectar() {
    let c = cenario();
    c.nm.0.lock().unwrap().wifi = Some(Wifi {
        flags: 0,
        wpa: 0,
        rsn: 0,
        key_mgmt: String::new(),
        pairwise: vec![],
    });
    let avisos = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    struct NotificadorFalso(std::sync::Arc<std::sync::Mutex<Vec<crate::portas::Notificacao>>>);
    impl crate::portas::Notificador for NotificadorFalso {
        fn notificar(&self, aviso: crate::portas::Notificacao) {
            self.0.lock().unwrap().push(aviso);
        }
    }
    let mut rede = c
        .rede
        .com_seguranca(VerificacaoFalsa, NotificadorFalso(avisos.clone()));
    assert!(rede.passo().unwrap().1.badge_de_risco());
    c.relogio.avancar(1000);
    c.nm.0.lock().unwrap().wifi = None;
    assert!(!rede.passo().unwrap().1.badge_de_risco());
    c.relogio.avancar(1000);
    c.nm.0.lock().unwrap().wifi = Some(Wifi::default());
    assert!(rede.passo().unwrap().1.badge_de_risco());
    assert_eq!(avisos.lock().unwrap().len(), 1);
    *c.nm.0.lock().unwrap() = Conexao::default();
    c.relogio.avancar(1000);
    let dados = rede.passo().unwrap().1;
    assert_eq!(dados.seguranca, None);
    assert!(!dados.badge_de_risco());
    *c.nm.0.lock().unwrap() = Nm.conexao().unwrap();
    c.nm.0.lock().unwrap().wifi = Some(Wifi::default());
    c.relogio.avancar(1000);
    rede.passo();
    assert_eq!(
        *avisos.lock().unwrap(),
        vec![crate::portas::Notificacao::RedeEmRisco; 2]
    );
}
struct VerificacaoFalsa;
impl Verificacao for VerificacaoFalsa {
    fn dns(&self) -> Option<bool> {
        Some(false)
    }
    fn mac(&self, _: &Conexao) -> Option<String> {
        None
    }
}

#[test]
fn criptografia_fisica_com_vpn_e_cabo_respeita_flags_e_perfil() {
    for (flags, wpa, rsn, key, pairwise, esperado, motivo) in [
        (0, 0, 0, "", "", Seguranca::Risco, "Rede aberta sem OWE"),
        (1, 0, 0, "none", "", Seguranca::Risco, "Criptografia WEP"),
        (1, 0x144, 0, "wpa-psk", "", Seguranca::Risco, "WPA com TKIP"),
        (
            1,
            0,
            0x188,
            "wpa-psk",
            "tkip",
            Seguranca::Risco,
            "WPA com TKIP",
        ),
        (0, 0, 0x800, "owe", "", Seguranca::SemAlertas, ""),
        (1, 0, 0x188, "wpa-psk", "ccmp", Seguranca::SemAlertas, ""),
        (1, 0, 0x400, "sae", "", Seguranca::SemAlertas, ""),
    ] {
        let mut c = cenario();
        {
            let mut nm = c.nm.0.lock().unwrap();
            nm.vpn = true;
            nm.wifi = Some(Wifi {
                flags,
                wpa,
                rsn,
                key_mgmt: key.into(),
                pairwise: vec![pairwise.into()],
            });
        }
        let dados = c.rede.passo().unwrap().1;
        assert_eq!(dados.seguranca, Some(esperado));
        if !motivo.is_empty() {
            assert!(dados.motivos.contains(&motivo.to_string()));
        }
        c.nm.0.lock().unwrap().tipo = "802-3-ethernet".into();
        c.relogio.avancar(1000);
        assert_eq!(
            c.rede.passo().unwrap().1.seguranca,
            Some(Seguranca::SemAlertas)
        );
    }
}

#[derive(Clone)]
struct VerificacaoControlada {
    dns: std::sync::Arc<std::sync::Mutex<(Option<bool>, usize)>>,
    mac: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}
impl Verificacao for VerificacaoControlada {
    fn dns(&self) -> Option<bool> {
        let mut d = self.dns.lock().unwrap();
        d.1 += 1;
        d.0
    }
    fn mac(&self, _: &Conexao) -> Option<String> {
        self.mac.lock().unwrap().clone()
    }
}
#[test]
fn dns_e_portal_a_cada_dez_minutos_e_mac_preserva_alerta_na_mesma_conexao() {
    let c = cenario();
    let v = VerificacaoControlada {
        dns: std::sync::Arc::new(std::sync::Mutex::new((Some(false), 0))),
        mac: std::sync::Arc::new(std::sync::Mutex::new(Some("aa:bb:cc:dd:ee:ff".into()))),
    };
    let mut rede = c.rede.com_seguranca(v.clone(), SemNotificador);
    assert_eq!(
        rede.passo().unwrap().1.seguranca,
        Some(Seguranca::SemAlertas)
    );
    *v.mac.lock().unwrap() = Some("11:22:33:44:55:66".into());
    c.nm.0.lock().unwrap().revisao += 1;
    c.relogio.avancar(1000);
    assert_eq!(rede.passo().unwrap().1.seguranca, Some(Seguranca::Atencao));
    assert_eq!(v.dns.lock().unwrap().1, 1);
    v.dns.lock().unwrap().0 = Some(true);
    c.relogio.avancar(599_000);
    let dados = rede.passo().unwrap().1;
    assert_eq!(dados.seguranca, Some(Seguranca::Risco));
    assert!(dados
        .motivos
        .contains(&"DNS respondeu a um nome inexistente".into()));
    assert_eq!(v.dns.lock().unwrap().1, 2);
    c.nm.0.lock().unwrap().id.push('2');
    v.dns.lock().unwrap().0 = Some(false);
    c.nm.0.lock().unwrap().conectividade = 2;
    c.relogio.avancar(1000);
    let dados = rede.passo().unwrap().1;
    assert_eq!(dados.seguranca, Some(Seguranca::Atencao));
    assert_eq!(dados.motivos, vec!["Portal cativo detectado"]);
    assert_eq!(v.dns.lock().unwrap().1, 3);
}

#[derive(Clone)]
struct LeiturasGravadas {
    dns: RespostaDns,
    vizinhos: std::sync::Arc<std::sync::Mutex<String>>,
}
impl LeiturasSeguranca for LeiturasGravadas {
    fn resolver(&self) -> Option<RespostaDns> {
        Some(self.dns.clone())
    }
    fn vizinhos(&self, _: &Conexao) -> Option<String> {
        Some(self.vizinhos.lock().unwrap().clone())
    }
}
#[test]
fn respostas_gravadas_passam_pelos_adapters_e_falhas_nao_viram_risco() {
    let original: RespostaDns =
        serde_json::from_str(include_str!("../../fixtures/rede/dns.json")).unwrap();
    for (respondeu, erro, esperado) in [
        (original.respondeu, original.erro, Seguranca::SemAlertas),
        (
            false,
            Some("org.freedesktop.resolve1.DnsError.NXDOMAIN".into()),
            Seguranca::SemAlertas,
        ),
        (true, None, Seguranca::Risco),
        (
            false,
            Some("org.freedesktop.resolve1.Timeout".into()),
            Seguranca::SemAlertas,
        ),
    ] {
        let c = cenario();
        let leituras = LeiturasGravadas {
            dns: RespostaDns { respondeu, erro },
            vizinhos: std::sync::Arc::new(std::sync::Mutex::new(
                include_str!("../../fixtures/rede/vizinhos.json").into(),
            )),
        };
        let vizinhos = leituras.vizinhos.clone();
        let mut rede = c
            .rede
            .com_seguranca(VerificacaoSistema(leituras), SemNotificador);
        assert_eq!(rede.passo().unwrap().1.seguranca, Some(esperado));
        // Variação controlada da saída gravada: MAC alterado em evento do NM.
        let mut texto = vizinhos.lock().unwrap();
        *texto = texto.replace("02:00:00:00:00:01", "11:22:33:44:55:66");
        drop(texto);
        c.nm.0.lock().unwrap().revisao += 1;
        c.relogio.avancar(1000);
        assert!(rede
            .passo()
            .unwrap()
            .1
            .motivos
            .contains(&"MAC do gateway mudou nesta conexão".into()));
    }
}

#[test]
fn portal_recupera_por_evento_e_mac_so_consulta_por_evento() {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    struct VerificacaoContada(Arc<AtomicUsize>);
    impl Verificacao for VerificacaoContada {
        fn dns(&self) -> Option<bool> {
            Some(false)
        }
        fn mac(&self, _: &Conexao) -> Option<String> {
            self.0.fetch_add(1, Ordering::SeqCst);
            None
        }
    }
    let c = cenario();
    c.nm.0.lock().unwrap().conectividade = 2;
    let chamadas = Arc::new(AtomicUsize::new(0));
    let mut rede = c
        .rede
        .com_seguranca(VerificacaoContada(chamadas.clone()), SemNotificador);
    assert_eq!(rede.passo().unwrap().1.seguranca, Some(Seguranca::Atencao));
    for _ in 0..5 {
        c.relogio.avancar(1000);
        rede.passo();
    }
    assert_eq!(chamadas.load(Ordering::SeqCst), 1);
    {
        let mut nm = c.nm.0.lock().unwrap();
        nm.conectividade = 4;
        nm.revisao += 1;
    }
    c.relogio.avancar(1000);
    assert_eq!(
        rede.passo().unwrap().1.seguranca,
        Some(Seguranca::SemAlertas)
    );
    assert_eq!(chamadas.load(Ordering::SeqCst), 2);
}
