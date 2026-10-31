//! Desenha o ícone do Indicador em ARGB32, sem arquivos de imagem: o mesmo binário serve ao
//! `.deb` e ao AppImage, e os estados do ícone são só parâmetros.

/// Marcas que o Indicador mostra além do glifo.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EstadoIndicador {
    /// Alguma Sessão de agente está Aguardando você.
    pub ponto_de_acento: bool,
    /// A Segurança da rede está em Risco.
    pub badge_de_risco: bool,
}

/// Uma imagem quadrada ARGB32 em ordem de rede, como o StatusNotifierItem espera.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pixmap {
    pub lado: i32,
    pub argb: Vec<u8>,
}

const GLIFO: [u8; 3] = [0xee, 0xee, 0xee];
const ACENTO: [u8; 3] = [0x5e, 0xea, 0xd4];
const RISCO: [u8; 3] = [0xf8, 0x71, 0x71];

/// Raio das marcas e da folga que as separa do glifo, em unidades da grade 16×16.
const RAIO_PONTO: f32 = 2.4;
const RAIO_BADGE: f32 = 2.8;
const FOLGA: f32 = 1.0;
const CENTRO_PONTO: (f32, f32) = (13.2, 2.8);
const CENTRO_BADGE: (f32, f32) = (13.0, 13.0);

/// Amostras por eixo em cada pixel (antisserrilhado por supersampling).
const AMOSTRAS: usize = 4;

pub fn desenhar(estado: EstadoIndicador, lado: i32) -> Pixmap {
    let escala = 16.0 / lado as f32;
    let mut argb = Vec::with_capacity((lado * lado * 4) as usize);
    for py in 0..lado {
        for px in 0..lado {
            let mut soma = [0u32; 3];
            let mut cobertas = 0u32;
            for sy in 0..AMOSTRAS {
                for sx in 0..AMOSTRAS {
                    let x = (px as f32 + (sx as f32 + 0.5) / AMOSTRAS as f32) * escala;
                    let y = (py as f32 + (sy as f32 + 0.5) / AMOSTRAS as f32) * escala;
                    if let Some(cor) = cor_em(estado, x, y) {
                        cobertas += 1;
                        for (acc, c) in soma.iter_mut().zip(cor) {
                            *acc += c as u32;
                        }
                    }
                }
            }
            let total = (AMOSTRAS * AMOSTRAS) as u32;
            let alfa = (cobertas * 255 / total) as u8;
            argb.push(alfa);
            for acc in soma {
                argb.push(acc.checked_div(cobertas).unwrap_or(0) as u8);
            }
        }
    }
    Pixmap { lado, argb }
}

/// Cor do ponto (x, y) na grade 16×16, ou None se transparente. As marcas ficam por cima do glifo.
fn cor_em(estado: EstadoIndicador, x: f32, y: f32) -> Option<[u8; 3]> {
    let marcas = [
        (estado.ponto_de_acento, CENTRO_PONTO, RAIO_PONTO, ACENTO),
        (estado.badge_de_risco, CENTRO_BADGE, RAIO_BADGE, RISCO),
    ];
    for (ligada, centro, raio, cor) in marcas {
        if !ligada {
            continue;
        }
        let d = distancia((x, y), centro);
        if d <= raio {
            return Some(cor);
        }
        if d <= raio + FOLGA {
            return None;
        }
    }
    no_glifo(x, y).then_some(GLIFO)
}

/// O glifo do protótipo: quadro de cantos arredondados com três barras.
fn no_glifo(x: f32, y: f32) -> bool {
    const TRACO: f32 = 0.75;
    let quadro = caixa_arredondada(x, y, (8.0, 8.0), 6.0, 3.0).abs() <= TRACO;
    let barras = [
        ((5.0, 10.0), (5.0, 8.0)),
        ((8.0, 10.0), (8.0, 5.0)),
        ((11.0, 10.0), (11.0, 7.0)),
    ];
    quadro || barras.iter().any(|&(a, b)| segmento((x, y), a, b) <= TRACO)
}

fn distancia(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

/// Distância com sinal até a borda de um quadrado de meio-lado `meio` e cantos de raio `raio`.
fn caixa_arredondada(x: f32, y: f32, centro: (f32, f32), meio: f32, raio: f32) -> f32 {
    let qx = (x - centro.0).abs() - meio + raio;
    let qy = (y - centro.1).abs() - meio + raio;
    let fora = (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt();
    fora + qx.max(qy).min(0.0) - raio
}

fn segmento(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let (abx, aby) = (b.0 - a.0, b.1 - a.1);
    let t = (((p.0 - a.0) * abx + (p.1 - a.1) * aby) / (abx * abx + aby * aby)).clamp(0.0, 1.0);
    distancia(p, (a.0 + abx * t, a.1 + aby * t))
}

#[cfg(test)]
mod testes {
    use super::*;

    // Cores da spec, independentes das constantes do desenho.
    const ACENTO: [u8; 3] = [0x5e, 0xea, 0xd4];
    const RISCO: [u8; 3] = [0xf8, 0x71, 0x71];

    fn tem_cor(p: &Pixmap, rgb: [u8; 3]) -> bool {
        p.argb.chunks(4).any(|px| px[0] == 0xff && px[1..] == rgb)
    }

    fn opacos(p: &Pixmap) -> usize {
        p.argb.chunks(4).filter(|px| px[0] > 0).count()
    }

    #[test]
    fn icone_normal_e_um_glifo_claro_sem_marcas() {
        let p = desenhar(EstadoIndicador::default(), 22);

        assert_eq!(p.argb.len(), 22 * 22 * 4);
        assert!(opacos(&p) > 0);
        assert!(!tem_cor(&p, ACENTO));
        assert!(!tem_cor(&p, RISCO));
    }

    #[test]
    fn sessao_aguardando_ganha_o_ponto_de_acento() {
        let estado = EstadoIndicador {
            ponto_de_acento: true,
            ..Default::default()
        };

        let p = desenhar(estado, 22);

        assert!(tem_cor(&p, ACENTO));
        assert!(!tem_cor(&p, RISCO));
    }

    #[test]
    fn rede_em_risco_ganha_o_badge() {
        let estado = EstadoIndicador {
            badge_de_risco: true,
            ..Default::default()
        };

        let p = desenhar(estado, 22);

        assert!(tem_cor(&p, RISCO));
        assert!(!tem_cor(&p, ACENTO));
    }

    #[test]
    fn as_duas_marcas_convivem() {
        let p = desenhar(
            EstadoIndicador {
                ponto_de_acento: true,
                badge_de_risco: true,
            },
            44,
        );

        assert!(tem_cor(&p, ACENTO));
        assert!(tem_cor(&p, RISCO));
    }
}
