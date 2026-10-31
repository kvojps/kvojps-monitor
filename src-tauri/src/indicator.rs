//! StatusNotifierItem indicator through ksni, without a GNOME Shell extension.
//! AppIndicator opens the menu on a single click; double click invokes activate and
//! middle click invokes secondary_activate (ADR 0001).

use ksni::menu::StandardItem;
use ksni::{Icon, MenuItem, ToolTip, Tray};
use tauri::AppHandle;

use crate::icon::{self, IndicatorState};
use crate::panel;
use crate::strings::STRINGS;

/// Sizes offered to the host, which chooses the closest match for its bar.
const ICON_SIZES: [i32; 3] = [22, 32, 44];

pub struct Indicator {
    pub app: AppHandle,
    pub state: IndicatorState,
}

impl Tray for Indicator {
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }

    fn title(&self) -> String {
        STRINGS.indicator.title.clone()
    }

    fn tool_tip(&self) -> ToolTip {
        ToolTip {
            title: STRINGS.indicator.title.clone(),
            ..Default::default()
        }
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        ICON_SIZES
            .iter()
            .map(|&side| {
                let p = icon::draw(self.state, side);
                Icon {
                    width: p.side,
                    height: p.side,
                    data: p.argb,
                }
            })
            .collect()
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        panel::open(&self.app);
    }

    fn secondary_activate(&mut self, _x: i32, _y: i32) {
        panel::open(&self.app);
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        vec![StandardItem {
            label: STRINGS.indicator.open_panel.clone(),
            activate: Box::new(|this: &mut Self| panel::open(&this.app)),
            ..Default::default()
        }
        .into()]
    }
}
