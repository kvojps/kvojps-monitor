//! Leituras de `/proc` e `/sys` pela porta de arquivos.

use std::io;

use serde::Serialize;

use crate::portas::Arquivos;

/// Contadores acumulados da linha `cpu` de `/proc/stat`.
#[derive(Debug, Clone, Copy)]
pub struct Jiffies {
    ocioso: u64,
    total: u64,
}

impl Jiffies {
    /// Uso em % entre duas amostras; None se os contadores não avançaram.
    pub fn uso_desde(self, anterior: Jiffies) -> Option<f64> {
        let total = self.total.checked_sub(anterior.total).filter(|&t| t > 0)?;
        let ocioso = self.ocioso.checked_sub(anterior.ocioso)?.min(total);
        Some((total - ocioso) as f64 * 100.0 / total as f64)
    }
}

pub fn jiffies(arquivos: &impl Arquivos) -> io::Result<Jiffies> {
    let stat = arquivos.ler("/proc/stat")?;
    let campos: Vec<u64> = stat
        .lines()
        .find_map(|l| l.strip_prefix("cpu "))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "/proc/stat sem linha cpu"))?
        .split_whitespace()
        .map_while(|c| c.parse().ok())
        .collect();
    // user nice system idle iowait irq softirq steal; guest e guest_nice já estão em user/nice.
    if campos.len() < 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "/proc/stat curto",
        ));
    }
    Ok(Jiffies {
        ocioso: campos[3] + campos.get(4).unwrap_or(&0),
        total: campos.iter().take(8).sum(),
    })
}

/// Em bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Memoria {
    pub usada: u64,
    pub total: u64,
}

/// RAM pelo critério do `free` (usada = total − `MemAvailable`) e swap, se configurado.
pub fn memoria(arquivos: &impl Arquivos) -> io::Result<(Memoria, Option<Memoria>)> {
    let meminfo = arquivos.ler("/proc/meminfo")?;
    let campo = |nome: &str| -> io::Result<u64> {
        meminfo
            .lines()
            .find_map(|l| l.strip_prefix(nome)?.strip_prefix(':'))
            .and_then(|v| v.trim().trim_end_matches("kB").trim().parse::<u64>().ok())
            .map(|kb| kb * 1024)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, nome.to_owned()))
    };
    let total = campo("MemTotal")?;
    let ram = Memoria {
        usada: total.saturating_sub(campo("MemAvailable")?),
        total,
    };
    let swap_total = campo("SwapTotal").unwrap_or(0);
    let swap = (swap_total > 0).then(|| Memoria {
        usada: swap_total.saturating_sub(campo("SwapFree").unwrap_or(swap_total)),
        total: swap_total,
    });
    Ok((ram, swap))
}

const SENSORES_DE_CPU: [&str; 2] = ["k10temp", "coretemp"];
/// Rótulos da leitura que representa a CPU inteira, em ordem de preferência.
const ROTULOS_DA_CPU: [&str; 3] = ["Tctl", "Tdie", "Package id 0"];
/// Atenção a partir daqui mesmo que o `temp*_crit` do sensor seja mais alto, em m°C.
const LIMITE_DE_ATENCAO: i64 = 90_000;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Temperatura {
    /// Com uma casa.
    pub celsius: f64,
    /// Chegou ao `temp*_crit` do sensor ou a 90 °C.
    pub atencao: bool,
}

/// Temperatura da CPU pelo hwmon (`k10temp` ou `coretemp`); None sem sensor.
pub fn temperatura(arquivos: &impl Arquivos) -> Option<Temperatura> {
    let mut hwmons = arquivos.listar("/sys/class/hwmon").ok()?;
    hwmons.sort();
    let dir = hwmons
        .into_iter()
        .map(|h| format!("/sys/class/hwmon/{h}"))
        .find(|d| {
            arquivos
                .ler(&format!("{d}/name"))
                .is_ok_and(|n| SENSORES_DE_CPU.contains(&n.trim()))
        })?;
    let ler = |arquivo: String| -> Option<String> {
        Some(
            arquivos
                .ler(&format!("{dir}/{arquivo}"))
                .ok()?
                .trim()
                .to_owned(),
        )
    };

    let mut indices: Vec<u32> = arquivos
        .listar(&dir)
        .ok()?
        .iter()
        .filter_map(|a| a.strip_prefix("temp")?.strip_suffix("_input")?.parse().ok())
        .collect();
    indices.sort();
    let rotulo = |i: &u32| ler(format!("temp{i}_label")).unwrap_or_default();
    let indice = ROTULOS_DA_CPU
        .iter()
        .find_map(|r| indices.iter().find(|i| rotulo(i) == *r))
        .or(indices.first())?;

    let mili: i64 = ler(format!("temp{indice}_input"))?.parse().ok()?;
    let limite = ler(format!("temp{indice}_crit"))
        .and_then(|c| c.parse().ok())
        .map_or(LIMITE_DE_ATENCAO, |crit: i64| crit.min(LIMITE_DE_ATENCAO));
    Some(Temperatura {
        celsius: (mili as f64 / 100.0).round() / 10.0,
        atencao: mili >= limite,
    })
}

/// Sistemas de arquivos que nunca são um disco do usuário, mesmo sobre um bloco.
const TIPOS_EXCLUIDOS: [&str; 4] = ["tmpfs", "squashfs", "overlay", "efivarfs"];

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Disco {
    pub ponto: String,
    /// Bytes, como no `df`: livre é o disponível ao usuário.
    pub usado: u64,
    pub total: u64,
    pub livre: u64,
    /// Mais de 90% em uso.
    pub atencao: bool,
}

/// Uma barra por disco real montado: sistema de arquivos sobre `/dev/*`, fora de snaps e `/boot*`,
/// uma barra por dispositivo de bloco (o primeiro ponto, em geral `/`): subvolumes btrfs e bind
/// mounts repetem a origem com outro `major:minor` anônimo.
pub fn discos(arquivos: &impl Arquivos) -> Vec<Disco> {
    let Ok(mountinfo) = arquivos.ler("/proc/self/mountinfo") else {
        return Vec::new();
    };
    let mut vistos = Vec::new();
    mountinfo
        .lines()
        .filter_map(|linha| {
            let (antes, depois) = linha.split_once(" - ")?;
            let antes: Vec<&str> = antes.split(' ').collect();
            let ponto = desescapar(antes.get(4)?);
            let mut depois = depois.split(' ');
            let (tipo, origem) = (depois.next()?, depois.next()?);
            let real = origem.starts_with("/dev/")
                && !TIPOS_EXCLUIDOS.contains(&tipo)
                && !ponto.starts_with("/boot")
                && !ponto.starts_with("/snap/");
            if !real || vistos.contains(&origem) {
                return None;
            }
            vistos.push(origem);
            let espaco = arquivos.espaco(&ponto).ok().filter(|e| e.total > 0)?;
            let usado = espaco.total.saturating_sub(espaco.livre);
            Some(Disco {
                atencao: usado * 10 > (usado + espaco.disponivel) * 9,
                ponto,
                usado,
                total: espaco.total,
                livre: espaco.disponivel,
            })
        })
        .collect()
}

/// O kernel escapa espaço, tab, quebra de linha e `\` em octal (`\040`).
fn desescapar(campo: &str) -> String {
    let mut saida = String::with_capacity(campo.len());
    let mut resto = campo;
    while let Some(i) = resto.find('\\') {
        saida.push_str(&resto[..i]);
        let codigo = resto.get(i + 1..i + 4);
        match codigo.and_then(|c| u8::from_str_radix(c, 8).ok()) {
            Some(byte) => {
                saida.push(byte as char);
                resto = &resto[i + 4..];
            }
            None => {
                saida.push('\\');
                resto = &resto[i + 1..];
            }
        }
    }
    saida.push_str(resto);
    saida
}
