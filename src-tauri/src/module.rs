use serde::{Deserialize, Serialize};

/// A panel area dedicated to a data source.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(rename_all = "lowercase")]
pub enum Module {
    #[default]
    #[serde(alias = "ia")]
    Ai,
    #[serde(alias = "agentes")]
    Agents,
    Github,
    Trello,
    #[serde(alias = "rede")]
    Network,
    #[serde(alias = "sistema")]
    System,
}

impl Module {
    /// Stable identifier shared by the UI and event names.
    pub fn id(self) -> &'static str {
        match self {
            Module::Ai => "ai",
            Module::Agents => "agents",
            Module::Github => "github",
            Module::Trello => "trello",
            Module::Network => "network",
            Module::System => "system",
        }
    }
}
