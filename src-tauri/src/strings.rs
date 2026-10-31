//! Core text loaded from the same strings file as the UI.

use std::sync::LazyLock;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Strings {
    pub agent_notifications: AgentNotifications,
    pub indicator: Indicator,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Indicator {
    pub title: String,
    pub open_panel: String,
}

pub static STRINGS: LazyLock<Strings> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../../src/lib/strings.json")).expect("invalid strings.json")
});

#[cfg(test)]
mod tests {
    #[test]
    fn shared_strings_include_indicator_text() {
        assert_eq!(super::STRINGS.indicator.open_panel, "Abrir painel");
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentNotifications {
    pub waiting: String,
    pub approval: String,
    pub failed: String,
    pub done: String,
    pub group_waiting: String,
    pub group: String,
    pub title: String,
}
