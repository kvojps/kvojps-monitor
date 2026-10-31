//! Draw the indicator in ARGB32 without image files: the same binary supports both
//! the .deb and AppImage; icon states are parameters.

/// Marks displayed alongside the indicator glyph.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IndicatorState {
    /// An agent session is waiting for the user.
    pub accent_dot: bool,
    /// Network security is at risk.
    pub risk_badge: bool,
}

/// Square ARGB32 image in network byte order, as expected by StatusNotifierItem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pixmap {
    pub side: i32,
    pub argb: Vec<u8>,
}

const GLYPH: [u8; 3] = [0xee, 0xee, 0xee];
const ACCENT: [u8; 3] = [0x5e, 0xea, 0xd4];
const RISK: [u8; 3] = [0xf8, 0x71, 0x71];

/// Mark radius and clearance from the glyph, in 16 by 16 grid units.
const DOT_RADIUS: f32 = 2.4;
const BADGE_RADIUS: f32 = 2.8;
const PADDING: f32 = 1.0;
const DOT_CENTER: (f32, f32) = (13.2, 2.8);
const BADGE_CENTER: (f32, f32) = (13.0, 13.0);

/// Samples per axis per pixel, for supersampling antialiasing.
const SAMPLES: usize = 4;

pub fn draw(state: IndicatorState, side: i32) -> Pixmap {
    let scale = 16.0 / side as f32;
    let mut argb = Vec::with_capacity((side * side * 4) as usize);
    for py in 0..side {
        for px in 0..side {
            let mut sum = [0u32; 3];
            let mut covered = 0u32;
            for sy in 0..SAMPLES {
                for sx in 0..SAMPLES {
                    let x = (px as f32 + (sx as f32 + 0.5) / SAMPLES as f32) * scale;
                    let y = (py as f32 + (sy as f32 + 0.5) / SAMPLES as f32) * scale;
                    if let Some(color) = color_at(state, x, y) {
                        covered += 1;
                        for (acc, c) in sum.iter_mut().zip(color) {
                            *acc += c as u32;
                        }
                    }
                }
            }
            let total = (SAMPLES * SAMPLES) as u32;
            let alpha = (covered * 255 / total) as u8;
            argb.push(alpha);
            for acc in sum {
                argb.push(acc.checked_div(covered).unwrap_or(0) as u8);
            }
        }
    }
    Pixmap { side, argb }
}

/// Color at (x, y) in the 16 by 16 grid; None when transparent. Marks overlay the glyph.
fn color_at(state: IndicatorState, x: f32, y: f32) -> Option<[u8; 3]> {
    let marks = [
        (state.accent_dot, DOT_CENTER, DOT_RADIUS, ACCENT),
        (state.risk_badge, BADGE_CENTER, BADGE_RADIUS, RISK),
    ];
    for (enabled, center, radius, color) in marks {
        if !enabled {
            continue;
        }
        let d = distance((x, y), center);
        if d <= radius {
            return Some(color);
        }
        if d <= radius + PADDING {
            return None;
        }
    }
    in_glyph(x, y).then_some(GLYPH)
}

/// Prototype glyph: a rounded frame with three bars.
fn in_glyph(x: f32, y: f32) -> bool {
    const STROKE: f32 = 0.75;
    let frame = rounded_box(x, y, (8.0, 8.0), 6.0, 3.0).abs() <= STROKE;
    let bars = [
        ((5.0, 10.0), (5.0, 8.0)),
        ((8.0, 10.0), (8.0, 5.0)),
        ((11.0, 10.0), (11.0, 7.0)),
    ];
    frame || bars.iter().any(|&(a, b)| segment((x, y), a, b) <= STROKE)
}

fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

/// Signed distance to a square with half-side half and rounded corners of the given radius.
fn rounded_box(x: f32, y: f32, center: (f32, f32), half: f32, radius: f32) -> f32 {
    let qx = (x - center.0).abs() - half + radius;
    let qy = (y - center.1).abs() - half + radius;
    let outside = (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt();
    outside + qx.max(qy).min(0.0) - radius
}

fn segment(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let (abx, aby) = (b.0 - a.0, b.1 - a.1);
    let t = (((p.0 - a.0) * abx + (p.1 - a.1) * aby) / (abx * abx + aby * aby)).clamp(0.0, 1.0);
    distance(p, (a.0 + abx * t, a.1 + aby * t))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Spec colors, independent of drawing constants.
    const ACCENT: [u8; 3] = [0x5e, 0xea, 0xd4];
    const RISK: [u8; 3] = [0xf8, 0x71, 0x71];

    fn has_color(p: &Pixmap, rgb: [u8; 3]) -> bool {
        p.argb.chunks(4).any(|px| px[0] == 0xff && px[1..] == rgb)
    }

    fn opaque(p: &Pixmap) -> usize {
        p.argb.chunks(4).filter(|px| px[0] > 0).count()
    }

    #[test]
    fn normal_icon_is_a_light_glyph_without_marks() {
        let p = draw(IndicatorState::default(), 22);

        assert_eq!(p.argb.len(), 22 * 22 * 4);
        assert!(opaque(&p) > 0);
        assert!(!has_color(&p, ACCENT));
        assert!(!has_color(&p, RISK));
    }

    #[test]
    fn waiting_session_adds_accent_dot() {
        let state = IndicatorState {
            accent_dot: true,
            ..Default::default()
        };

        let p = draw(state, 22);

        assert!(has_color(&p, ACCENT));
        assert!(!has_color(&p, RISK));
    }

    #[test]
    fn network_risk_adds_badge() {
        let state = IndicatorState {
            risk_badge: true,
            ..Default::default()
        };

        let p = draw(state, 22);

        assert!(has_color(&p, RISK));
        assert!(!has_color(&p, ACCENT));
    }

    #[test]
    fn both_marks_can_coexist() {
        let p = draw(
            IndicatorState {
                accent_dot: true,
                risk_badge: true,
            },
            44,
        );

        assert!(has_color(&p, ACCENT));
        assert!(has_color(&p, RISK));
    }
}
