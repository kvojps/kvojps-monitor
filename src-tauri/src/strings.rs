//! Textos do núcleo, lidos do mesmo arquivo de strings da UI.

use std::sync::LazyLock;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Strings {
    pub indicador: Indicador,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Indicador {
    pub titulo: String,
    pub abrir_painel: String,
}

pub static STRINGS: LazyLock<Strings> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../../src/lib/strings.json")).expect("strings.json inválido")
});

#[cfg(test)]
mod testes {
    #[test]
    fn arquivo_de_strings_tem_os_textos_do_indicador() {
        assert_eq!(super::STRINGS.indicador.abrir_painel, "Abrir painel");
    }
}
