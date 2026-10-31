use serde::{Deserialize, Serialize};

/// Uma área do Painel dedicada a uma fonte de dados.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Modulo {
    #[default]
    Ia,
    Agentes,
    Github,
    Trello,
    Rede,
    Sistema,
}

impl Modulo {
    /// Identificador estável, igual ao usado pela UI e nos nomes de evento.
    pub fn id(self) -> &'static str {
        match self {
            Modulo::Ia => "ia",
            Modulo::Agentes => "agentes",
            Modulo::Github => "github",
            Modulo::Trello => "trello",
            Modulo::Rede => "rede",
            Modulo::Sistema => "sistema",
        }
    }
}
