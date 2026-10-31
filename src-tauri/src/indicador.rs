//! O Indicador: StatusNotifierItem via `ksni`, sem extensão do GNOME Shell.
//! Na extensão AppIndicator, clique simples abre o menu; duplo clique chega como `activate` e
//! clique do meio como `secondary_activate` (ADR 0001).

use ksni::menu::StandardItem;
use ksni::{Icon, MenuItem, ToolTip, Tray};
use tauri::AppHandle;

use crate::icone::{self, EstadoIndicador};
use crate::painel;
use crate::strings::STRINGS;

/// Tamanhos entregues ao host; ele escolhe o mais próximo da barra.
const LADOS: [i32; 3] = [22, 32, 44];

pub struct Indicador {
    pub app: AppHandle,
    pub estado: EstadoIndicador,
}

impl Tray for Indicador {
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }

    fn title(&self) -> String {
        STRINGS.indicador.titulo.clone()
    }

    fn tool_tip(&self) -> ToolTip {
        ToolTip {
            title: STRINGS.indicador.titulo.clone(),
            ..Default::default()
        }
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        LADOS
            .iter()
            .map(|&lado| {
                let p = icone::desenhar(self.estado, lado);
                Icon {
                    width: p.lado,
                    height: p.lado,
                    data: p.argb,
                }
            })
            .collect()
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        painel::abrir(&self.app);
    }

    fn secondary_activate(&mut self, _x: i32, _y: i32) {
        painel::abrir(&self.app);
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        vec![StandardItem {
            label: STRINGS.indicador.abrir_painel.clone(),
            activate: Box::new(|this: &mut Self| painel::abrir(&this.app)),
            ..Default::default()
        }
        .into()]
    }
}
