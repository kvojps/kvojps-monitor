//! Módulo Sistema: CPU, temperatura, RAM, discos e bateria, coletados só com o Módulo visível.

mod bateria;
mod leituras;

use std::collections::VecDeque;
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;

use serde::Serialize;
use tauri::AppHandle;

use crate::modulo::Modulo;
use crate::portas::{Arquivos, Raiz, Relogio, RelogioDoSistema};
use crate::snapshot::{self, Snapshot};
#[cfg(test)]
use bateria::EstadoBateria;
pub use bateria::UPower;
use bateria::{Bateria, UPowerDbus};
use leituras::{Disco, Jiffies, Memoria, Temperatura};

const INTERVALO_CPU: u64 = 2_000;
const INTERVALO_DISCOS: u64 = 30_000;
/// ~60 s de mini‑gráfico a uma amostra a cada 2 s.
const PONTOS_NO_HISTORICO: usize = 30;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DadosSistema {
    pub cpu: Cpu,
    pub ram: Memoria,
    pub swap: Option<Memoria>,
    pub discos: Vec<Disco>,
    /// None sem bateria.
    pub bateria: Option<Bateria>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Cpu {
    /// Uso total em %, com uma casa; None até haver duas amostras.
    pub uso: Option<f64>,
    /// Do mais antigo ao mais recente.
    pub historico: Vec<f64>,
    /// None sem sensor de CPU.
    pub temperatura: Option<Temperatura>,
}

/// Coletor do Módulo Sistema. O laço de fundo chama `passo` quando o `prazo` vence.
pub struct Sistema<A, U, R> {
    arquivos: A,
    upower: U,
    relogio: R,
    visivel: bool,
    proxima_cpu: u64,
    proximos_discos: u64,
    discos: Vec<Disco>,
    jiffies: Option<Jiffies>,
    uso: Option<f64>,
    historico: VecDeque<f64>,
    bateria: Option<Bateria>,
    ultimo: Option<DadosSistema>,
}

impl<A: Arquivos, U: UPower, R: Relogio> Sistema<A, U, R> {
    pub fn new(arquivos: A, upower: U, relogio: R) -> Self {
        Sistema {
            arquivos,
            upower,
            relogio,
            visivel: false,
            proxima_cpu: 0,
            proximos_discos: 0,
            discos: Vec::new(),
            jiffies: None,
            uso: None,
            historico: VecDeque::new(),
            bateria: None,
            ultimo: None,
        }
    }

    /// O Módulo ficou visível: a coleta recomeça do zero, com o mini‑gráfico vazio.
    pub fn mostrar(&mut self) {
        if self.visivel {
            return;
        }
        self.visivel = true;
        self.jiffies = None;
        self.uso = None;
        self.historico.clear();
        self.bateria = bateria::ler(&self.upower);
        self.proxima_cpu = self.relogio.agora();
        self.proximos_discos = self.proxima_cpu;
    }

    pub fn ocultar(&mut self) {
        self.visivel = false;
        self.ultimo = None;
    }

    /// O UPower avisou que o `DisplayDevice` mudou.
    pub fn bateria_mudou(&mut self) -> Option<(u64, DadosSistema)> {
        if !self.visivel {
            return None;
        }
        self.bateria = bateria::ler(&self.upower);
        let mut dados = self.ultimo.clone()?;
        dados.bateria = self.bateria.clone();
        self.ultimo = Some(dados.clone());
        Some((self.relogio.agora(), dados))
    }

    /// Quando o próximo `passo` tem algo a coletar; None com o Módulo invisível.
    pub fn prazo(&self) -> Option<u64> {
        self.visivel
            .then_some(self.proxima_cpu.min(self.proximos_discos))
    }

    /// Coleta o que venceu; devolve o snapshot novo, com a hora da coleta em epoch ms.
    pub fn passo(&mut self) -> Option<(u64, DadosSistema)> {
        let agora = self.relogio.agora();
        if !self.visivel || self.prazo().is_some_and(|p| agora < p) {
            return None;
        }
        if agora >= self.proximos_discos {
            self.proximos_discos = agora + INTERVALO_DISCOS;
            self.discos = leituras::discos(&self.arquivos);
        }
        if agora >= self.proxima_cpu {
            self.proxima_cpu = agora + INTERVALO_CPU;
            self.uso = self.medir_cpu();
        }
        let (ram, swap) = leituras::memoria(&self.arquivos).ok()?;
        let dados = DadosSistema {
            cpu: Cpu {
                uso: self.uso,
                historico: self.historico.iter().copied().collect(),
                temperatura: leituras::temperatura(&self.arquivos),
            },
            ram,
            swap,
            discos: self.discos.clone(),
            bateria: self.bateria.clone(),
        };
        self.ultimo = Some(dados.clone());
        Some((agora, dados))
    }

    fn medir_cpu(&mut self) -> Option<f64> {
        let atual = leituras::jiffies(&self.arquivos).ok();
        let anterior = std::mem::replace(&mut self.jiffies, atual);
        let uso = (atual?.uso_desde(anterior?)? * 10.0).round() / 10.0;
        if self.historico.len() == PONTOS_NO_HISTORICO {
            self.historico.pop_front();
        }
        self.historico.push_back(uso);
        Some(uso)
    }
}

enum Evento {
    Visivel(bool),
    BateriaMudou,
}

/// Canal para o laço do coletor, que roda numa thread própria e dorme até o prazo ou um evento.
pub struct Coletor(Sender<Evento>);

impl Coletor {
    pub fn iniciar(app: AppHandle) -> Self {
        let (tx, rx) = mpsc::channel();
        let aviso = tx.clone();
        bateria::ouvir_upower(move || {
            let _ = aviso.send(Evento::BateriaMudou);
        });
        let laco = move || {
            let relogio = RelogioDoSistema;
            let mut sistema =
                Sistema::new(Raiz::sistema(), UPowerDbus::conectar(), RelogioDoSistema);
            loop {
                let evento = match sistema.prazo() {
                    Some(prazo) => rx
                        .recv_timeout(Duration::from_millis(prazo.saturating_sub(relogio.agora()))),
                    None => rx.recv().map_err(|_| RecvTimeoutError::Disconnected),
                };
                let novo = match evento {
                    Ok(Evento::Visivel(true)) => {
                        sistema.mostrar();
                        None
                    }
                    Ok(Evento::Visivel(false)) => {
                        sistema.ocultar();
                        None
                    }
                    Ok(Evento::BateriaMudou) => sistema.bateria_mudou(),
                    Err(RecvTimeoutError::Timeout) => sistema.passo(),
                    Err(RecvTimeoutError::Disconnected) => return,
                };
                if let Some((quando, dados)) = novo {
                    snapshot::publicar(&app, snapshot(quando, &dados));
                }
            }
        };
        if let Err(e) = thread::Builder::new().name("sistema".into()).spawn(laco) {
            eprintln!("sistema: falha ao iniciar a coleta: {e}");
        }
        Coletor(tx)
    }

    /// A coleta só corre com o Módulo Sistema visível.
    pub fn informar(&self, modulo_visivel: Option<Modulo>) {
        let _ = self
            .0
            .send(Evento::Visivel(modulo_visivel == Some(Modulo::Sistema)));
    }
}

fn snapshot(quando: u64, dados: &DadosSistema) -> Snapshot {
    Snapshot {
        modulo: Modulo::Sistema,
        atualizado_em: Some(quando),
        dados: serde_json::to_value(dados).expect("DadosSistema serializa"),
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::portas::falsos::{RaizFalsa, RelogioFalso};
    use crate::portas::Espaco;

    const GIB: u64 = 1024 * 1024 * 1024;
    const KIB: u64 = 1024;

    struct Cenario {
        raiz: RaizFalsa,
        upower: bateria::falso::UPowerFalso,
        relogio: RelogioFalso,
        sistema: Sistema<RaizFalsa, bateria::falso::UPowerFalso, RelogioFalso>,
    }

    fn notebook() -> Cenario {
        let raiz = RaizFalsa::de("sistema/notebook");
        // `stat -f -c "%n %S %b %f %a"` gravado junto da fixture.
        let statvfs = std::fs::read_to_string(
            crate::portas::falsos::fixtures().join("sistema/notebook-statvfs.txt"),
        )
        .unwrap();
        for linha in statvfs.lines() {
            let c: Vec<&str> = linha.split_whitespace().collect();
            let n = |i: usize| c[i].parse::<u64>().unwrap();
            raiz.definir_espaco(
                c[0],
                Espaco {
                    total: n(2) * n(1),
                    livre: n(3) * n(1),
                    disponivel: n(4) * n(1),
                },
            );
        }
        let upower = bateria::falso::UPowerFalso::de("notebook-carregada");
        let relogio = RelogioFalso::em(1_000_000);
        let sistema = Sistema::new(raiz.clone(), upower.clone(), relogio.clone());
        Cenario {
            raiz,
            upower,
            relogio,
            sistema,
        }
    }

    #[test]
    fn ram_usada_e_o_total_menos_o_disponivel_como_no_free() {
        let mut c = notebook();
        c.sistema.mostrar();

        let (quando, dados) = c.sistema.passo().unwrap();

        assert_eq!(quando, 1_000_000);
        // MemTotal 15223128 kB, MemAvailable 9252764 kB.
        assert_eq!(dados.ram.total, 15_223_128 * KIB);
        assert_eq!(dados.ram.usada, (15_223_128 - 9_252_764) * KIB);
        assert!(dados.ram.total > 14 * GIB);
    }

    #[test]
    fn swap_aparece_quando_configurado() {
        let mut c = notebook();
        c.sistema.mostrar();

        let swap = c.sistema.passo().unwrap().1.swap.unwrap();

        // SwapTotal 4194300 kB, SwapFree 3801600 kB.
        assert_eq!(swap.total, 4_194_300 * KIB);
        assert_eq!(swap.usada, (4_194_300 - 3_801_600) * KIB);
    }

    #[test]
    fn sem_swap_configurado_nao_ha_linha_de_swap() {
        let mut c = notebook();
        let meminfo = c.raiz.ler("/proc/meminfo").unwrap();
        c.raiz.escrever(
            "/proc/meminfo",
            &meminfo
                .replace("SwapTotal:       4194300 kB", "SwapTotal:             0 kB")
                .replace("SwapFree:        3801600 kB", "SwapFree:              0 kB"),
        );
        c.sistema.mostrar();

        assert_eq!(c.sistema.passo().unwrap().1.swap, None);
    }

    fn amostra_de_cpu(c: &Cenario, n: usize) {
        let stat = std::fs::read_to_string(
            crate::portas::falsos::fixtures().join(format!("sistema/proc-stat/{n}")),
        )
        .unwrap();
        c.raiz.escrever("/proc/stat", &stat);
    }

    #[test]
    fn cpu_comeca_vazia_ao_abrir() {
        let mut c = notebook();
        c.sistema.mostrar();

        let cpu = c.sistema.passo().unwrap().1.cpu;

        assert_eq!(cpu.uso, None);
        assert!(cpu.historico.is_empty());
    }

    #[test]
    fn cpu_e_medida_a_cada_2_s_pela_diferenca_entre_amostras() {
        let mut c = notebook();
        c.sistema.mostrar();
        amostra_de_cpu(&c, 0);
        c.sistema.passo().unwrap();
        assert_eq!(c.sistema.prazo(), Some(1_002_000));

        amostra_de_cpu(&c, 1);
        c.relogio.avancar(1_000);
        assert_eq!(c.sistema.passo(), None, "nada venceu em 1 s");
        c.relogio.avancar(1_000);
        let (quando, dados) = c.sistema.passo().unwrap();

        // 0 → 1: ocupado +55 jiffies (user 44, system 10, softirq 1) de +2395 no total.
        assert_eq!(quando, 1_002_000);
        assert_eq!(dados.cpu.uso, Some(2.3));
        assert_eq!(dados.cpu.historico, vec![2.3]);

        amostra_de_cpu(&c, 2);
        c.relogio.avancar(2_000);
        // 1 → 2: ocupado +48 (user 37, system 11) de +2392.
        assert_eq!(c.sistema.passo().unwrap().1.cpu.historico, vec![2.3, 2.0]);
    }

    #[test]
    fn historico_da_cpu_guarda_os_ultimos_60_s() {
        let mut c = notebook();
        c.sistema.mostrar();
        let mut dados = None;
        for n in 0..40 {
            c.raiz.escrever(
                "/proc/stat",
                &format!("cpu  {} 0 0 {} 0 0 0 0 0 0\n", n * 10, n * 90),
            );
            dados = c.sistema.passo();
            c.relogio.avancar(2_000);
        }

        assert_eq!(dados.unwrap().1.cpu.historico.len(), 30);
    }

    /// Abre o Módulo (de novo, se já estava aberto) e lê a temperatura.
    fn temperatura(c: &mut Cenario) -> Option<Temperatura> {
        c.sistema.ocultar();
        c.sistema.mostrar();
        c.sistema.passo().unwrap().1.cpu.temperatura
    }

    #[test]
    fn temperatura_vem_do_k10temp_e_ignora_outros_sensores() {
        let mut c = notebook();

        // k10temp Tctl 47375 m°C; o nvme ao lado tem crit própria e fica de fora.
        assert_eq!(
            temperatura(&mut c),
            Some(Temperatura {
                celsius: 47.4,
                atencao: false
            })
        );
    }

    #[test]
    fn sem_sensor_de_cpu_a_linha_de_temperatura_some() {
        let mut c = notebook();
        c.raiz.apagar("/sys/class/hwmon/hwmon4");

        assert_eq!(temperatura(&mut c), None);
    }

    #[test]
    fn sem_crit_no_sensor_a_atencao_comeca_em_90_graus() {
        let mut c = notebook();
        c.raiz
            .escrever("/sys/class/hwmon/hwmon4/temp1_input", "89900\n");
        assert!(!temperatura(&mut c).unwrap().atencao);

        c.raiz
            .escrever("/sys/class/hwmon/hwmon4/temp1_input", "90000\n");
        assert!(temperatura(&mut c).unwrap().atencao);
    }

    /// Layout do `coretemp` (Intel): pacote com crit própria e um sensor por núcleo.
    fn com_coretemp(c: &Cenario, pacote: &str, crit: &str) {
        c.raiz.apagar("/sys/class/hwmon/hwmon4");
        for (arquivo, valor) in [
            ("name", "coretemp"),
            ("temp1_label", "Package id 0"),
            ("temp1_input", pacote),
            ("temp1_crit", crit),
            ("temp2_label", "Core 0"),
            ("temp2_input", "61000"),
            ("temp2_crit", "100000"),
        ] {
            c.raiz.escrever(
                &format!("/sys/class/hwmon/hwmon6/{arquivo}"),
                &format!("{valor}\n"),
            );
        }
    }

    #[test]
    fn coretemp_usa_o_pacote() {
        let mut c = notebook();
        com_coretemp(&c, "85000", "100000");

        assert_eq!(
            temperatura(&mut c),
            Some(Temperatura {
                celsius: 85.0,
                atencao: false
            })
        );
    }

    #[test]
    fn atencao_em_90_graus_mesmo_com_crit_mais_alta() {
        let mut c = notebook();
        com_coretemp(&c, "89900", "100000");
        assert!(!temperatura(&mut c).unwrap().atencao);

        com_coretemp(&c, "90000", "100000");
        assert!(temperatura(&mut c).unwrap().atencao);
    }

    #[test]
    fn atencao_na_crit_do_sensor_quando_abaixo_de_90_graus() {
        let mut c = notebook();
        com_coretemp(&c, "79900", "80000");
        assert!(!temperatura(&mut c).unwrap().atencao);

        com_coretemp(&c, "80000", "80000");
        assert!(temperatura(&mut c).unwrap().atencao);
    }

    fn discos(c: &mut Cenario) -> Vec<Disco> {
        c.sistema.mostrar();
        c.sistema.passo().unwrap().1.discos
    }

    #[test]
    fn so_o_disco_raiz_e_real_nesta_maquina() {
        let mut c = notebook();

        // A fixture tem tmpfs, 30+ squashfs de snap, efivarfs, /boot/efi e pseudo‑sistemas.
        assert_eq!(
            discos(&mut c),
            vec![Disco {
                ponto: "/".into(),
                // statvfs de /: frsize 4096, blocos 38426996, livres 19959216, disponíveis 17989040.
                usado: (38_426_996 - 19_959_216) * 4096,
                total: 38_426_996 * 4096,
                livre: 17_989_040 * 4096,
                atencao: false,
            }]
        );
    }

    fn montar(c: &Cenario, linhas: &str) {
        let mountinfo = c.raiz.ler("/proc/self/mountinfo").unwrap();
        c.raiz
            .escrever("/proc/self/mountinfo", &format!("{mountinfo}{linhas}"));
    }

    fn espaco(total: u64, disponivel: u64) -> Espaco {
        Espaco {
            total,
            livre: disponivel,
            disponivel,
        }
    }

    #[test]
    fn um_disco_por_dispositivo_sem_overlay_nem_boot() {
        let mut c = notebook();
        montar(
            &c,
            "900 29 8:17 / /mnt/dados\\040externos rw,relatime shared:500 - ext4 /dev/sdb1 rw\n\
             901 29 0:45 /@home /home rw,relatime shared:1 - btrfs /dev/sda1 rw\n\
             902 29 8:1 / /boot rw,relatime shared:501 - ext4 /dev/sda1 rw\n\
             903 29 0:90 / /var/lib/docker/overlay2/x/merged rw - overlay overlay rw\n",
        );
        c.raiz
            .definir_espaco("/mnt/dados externos", espaco(1000, 400));
        c.raiz.definir_espaco("/boot", espaco(1000, 400));
        c.raiz.definir_espaco("/home", espaco(1000, 400));

        let pontos: Vec<String> = discos(&mut c).into_iter().map(|d| d.ponto).collect();

        // /home é outro subvolume (outro 0:NN) na mesma partição de / e não ganha outra barra.
        assert_eq!(pontos, vec!["/", "/mnt/dados externos"]);
    }

    #[test]
    fn disco_acima_de_90_por_cento_pede_atencao() {
        let mut c = notebook();
        montar(
            &c,
            "900 29 8:17 / /mnt/a rw - ext4 /dev/sdb1 rw\n\
             901 29 8:33 / /mnt/b rw - ext4 /dev/sdc1 rw\n",
        );
        c.raiz.definir_espaco("/mnt/a", espaco(1000, 100));
        c.raiz.definir_espaco("/mnt/b", espaco(1000, 99));

        let d = discos(&mut c);

        assert!(!d[1].atencao, "90% em uso ainda não é > 90%");
        assert!(d[2].atencao);
    }

    #[test]
    fn discos_sao_relidos_a_cada_30_s() {
        let mut c = notebook();
        c.sistema.mostrar();
        let antes = c.sistema.passo().unwrap().1.discos;
        c.raiz.definir_espaco("/", espaco(1000, 10));

        c.relogio.avancar(28_000);
        assert_eq!(c.sistema.passo().unwrap().1.discos, antes);

        c.relogio.avancar(2_000);
        assert_eq!(c.sistema.passo().unwrap().1.discos[0].livre, 10);
    }

    fn bateria(c: &mut Cenario) -> Option<Bateria> {
        c.sistema.mostrar();
        c.sistema.passo().unwrap().1.bateria
    }

    #[test]
    fn notebook_na_tomada_mostra_a_bateria_cheia() {
        let mut c = notebook();

        assert_eq!(
            bateria(&mut c),
            Some(Bateria {
                percentual: 100.0,
                estado: EstadoBateria::Cheia,
                segundos_restantes: None,
                atencao: false,
            })
        );
    }

    #[test]
    fn bateria_descarregando_abaixo_de_15_por_cento_pede_atencao() {
        let mut c = notebook();
        c.upower.responder("notebook-descarregando");

        assert_eq!(
            bateria(&mut c),
            Some(Bateria {
                percentual: 12.0,
                estado: EstadoBateria::Descarregando,
                segundos_restantes: Some(1800),
                atencao: true,
            })
        );
    }

    #[test]
    fn desktop_sem_bateria_oculta_o_bloco() {
        let mut c = notebook();
        c.upower.responder("desktop");

        assert_eq!(bateria(&mut c), None);
    }

    #[test]
    fn sem_upower_o_bloco_some_e_o_resto_segue() {
        let mut c = notebook();
        c.upower.fora_do_ar();

        assert_eq!(bateria(&mut c), None);
    }

    #[test]
    fn bateria_e_relida_so_quando_o_upower_avisa() {
        let mut c = notebook();
        c.sistema.mostrar();
        amostra_de_cpu(&c, 0);
        c.sistema.passo().unwrap();
        c.upower.responder("notebook-descarregando");

        amostra_de_cpu(&c, 1);
        c.relogio.avancar(2_000);
        let (_, dados) = c.sistema.passo().unwrap();
        assert_eq!(dados.bateria.unwrap().estado, EstadoBateria::Cheia);

        c.relogio.avancar(500);
        let (quando, dados) = c.sistema.bateria_mudou().unwrap();
        assert_eq!(quando, 1_002_500);
        assert_eq!(dados.bateria.unwrap().estado, EstadoBateria::Descarregando);
        assert_eq!(
            dados.cpu.historico.len(),
            1,
            "o resto do snapshot é preservado"
        );
    }

    fn leituras_do_upower(c: &Cenario) -> usize {
        c.upower.leituras.load(std::sync::atomic::Ordering::SeqCst)
    }

    #[test]
    fn com_o_modulo_invisivel_nada_e_coletado() {
        let mut c = notebook();

        assert_eq!(c.sistema.prazo(), None);
        assert_eq!(c.sistema.passo(), None);
        assert_eq!(c.sistema.bateria_mudou(), None);
        assert_eq!(leituras_do_upower(&c), 0);
    }

    #[test]
    fn ao_ocultar_a_coleta_para() {
        let mut c = notebook();
        c.sistema.mostrar();
        c.sistema.passo().unwrap();
        let lidas = leituras_do_upower(&c);

        c.sistema.ocultar();
        c.relogio.avancar(60_000);

        assert_eq!(c.sistema.prazo(), None);
        assert_eq!(c.sistema.passo(), None);
        assert_eq!(c.sistema.bateria_mudou(), None);
        assert_eq!(leituras_do_upower(&c), lidas);
    }

    #[test]
    fn ao_reabrir_o_mini_grafico_recomeca_vazio() {
        let mut c = notebook();
        c.sistema.mostrar();
        for n in 0..3 {
            amostra_de_cpu(&c, n);
            c.sistema.passo().unwrap();
            c.relogio.avancar(2_000);
        }
        c.sistema.ocultar();
        c.relogio.avancar(60_000);

        c.sistema.mostrar();
        let cpu = c.sistema.passo().unwrap().1.cpu;

        assert_eq!(cpu.uso, None);
        assert!(cpu.historico.is_empty());
    }

    #[test]
    fn avisar_de_novo_que_esta_visivel_nao_zera_o_mini_grafico() {
        let mut c = notebook();
        c.sistema.mostrar();
        amostra_de_cpu(&c, 0);
        c.sistema.passo().unwrap();
        amostra_de_cpu(&c, 1);
        c.relogio.avancar(2_000);
        c.sistema.passo().unwrap();

        c.sistema.mostrar();

        assert_eq!(c.sistema.prazo(), Some(1_004_000));
        assert_eq!(
            c.sistema.bateria_mudou().unwrap().1.cpu.historico,
            vec![2.3]
        );
    }

    /// Cenários das fixtures de snapshot da UI, em `src/lib/sistema/fixtures/`.
    fn cenario_da_ui(nome: &str) -> (u64, DadosSistema) {
        let mut c = notebook();
        // 30/10/2026 14:32 UTC.
        c.relogio.avancar(1_793_457_120_000 - 1_000_000);
        match nome {
            "notebook" => {}
            "desktop" => c.upower.responder("desktop"),
            "sem-sensor" => c.raiz.apagar("/sys/class/hwmon/hwmon4"),
            "atencao" => {
                c.raiz
                    .escrever("/sys/class/hwmon/hwmon4/temp1_input", "92125\n");
                c.upower.responder("notebook-descarregando");
                montar(&c, "900 29 8:17 / /mnt/dados rw - ext4 /dev/sdb1 rw\n");
                c.raiz
                    .definir_espaco("/mnt/dados", espaco(1_000_000_000_000, 42_000_000_000));
            }
            _ => unreachable!(),
        }
        c.sistema.mostrar();
        let mut ultimo = None;
        for n in 0..3 {
            amostra_de_cpu(&c, n);
            ultimo = c.sistema.passo();
            c.relogio.avancar(2_000);
        }
        ultimo.unwrap()
    }

    /// A UI é testada com o que o núcleo publica de fato. Para regravar depois de mudar o
    /// formato: `ATUALIZAR_FIXTURES=1 cargo test`.
    #[test]
    fn fixtures_de_snapshot_da_ui_saem_do_nucleo() {
        let dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/lib/sistema/fixtures");
        for nome in ["notebook", "desktop", "sem-sensor", "atencao"] {
            let (quando, dados) = cenario_da_ui(nome);
            let json = serde_json::to_string_pretty(&snapshot(quando, &dados)).unwrap() + "\n";
            let arquivo = dir.join(format!("{nome}.json"));
            if std::env::var_os("ATUALIZAR_FIXTURES").is_some() {
                std::fs::create_dir_all(&dir).unwrap();
                std::fs::write(&arquivo, json).unwrap();
            } else {
                let gravado = std::fs::read_to_string(&arquivo).unwrap_or_default();
                assert_eq!(
                    gravado, json,
                    "{nome}.json desatualizada: rode com ATUALIZAR_FIXTURES=1"
                );
            }
        }
    }
}
