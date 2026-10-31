use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::modulo::Modulo;

const ARQUIVO_PREFERENCIAS: &str = "preferencias.json";

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Preferencias {
    ultimo_modulo: Modulo,
    /// A primeira execução já ligou o início com o login.
    inicializado: bool,
}

/// Estado do núcleo sobre o Painel: o que está visível e o que persiste entre execuções.
pub struct Estado {
    dir_config: PathBuf,
    preferencias: Preferencias,
    painel_aberto: bool,
    /// O que a UI mostra agora; None = Configurações.
    vista: Option<Modulo>,
}

impl Estado {
    /// Lê as preferências de `dir_config`; arquivo ausente ou ilegível vale como primeira execução.
    pub fn carregar(dir_config: PathBuf) -> Self {
        let preferencias: Preferencias = fs::read(dir_config.join(ARQUIVO_PREFERENCIAS))
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default();
        let vista = Some(preferencias.ultimo_modulo);
        Estado {
            dir_config,
            preferencias,
            painel_aberto: false,
            vista,
        }
    }

    pub fn ultimo_modulo(&self) -> Modulo {
        self.preferencias.ultimo_modulo
    }

    /// A UI passou a mostrar `modulo` (None = Configurações).
    pub fn mostrar(&mut self, modulo: Option<Modulo>) {
        self.vista = modulo;
        if let Some(m) = modulo {
            self.preferencias.ultimo_modulo = m;
            self.salvar();
        }
    }

    pub fn primeira_execucao(&self) -> bool {
        !self.preferencias.inicializado
    }

    pub fn concluir_primeira_execucao(&mut self) {
        self.preferencias.inicializado = true;
        self.salvar();
    }

    /// O Painel nasce de novo a cada abertura, no último Módulo visitado.
    pub fn abrir_painel(&mut self) {
        self.painel_aberto = true;
        self.vista = Some(self.preferencias.ultimo_modulo);
    }

    pub fn fechar_painel(&mut self) {
        self.painel_aberto = false;
    }

    /// O Módulo que o usuário está vendo; a coleta de alguns Módulos depende disso.
    pub fn modulo_visivel(&self) -> Option<Modulo> {
        self.vista.filter(|_| self.painel_aberto)
    }

    fn salvar(&self) {
        // Falha ao gravar só custa lembrar o Módulo; o app segue funcionando.
        let _ = fs::create_dir_all(&self.dir_config).and_then(|_| {
            let json =
                serde_json::to_vec_pretty(&self.preferencias).map_err(std::io::Error::other)?;
            fs::write(self.dir_config.join(ARQUIVO_PREFERENCIAS), json)
        });
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn dir() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn primeira_execucao_abre_no_modulo_ia() {
        let d = dir();
        assert_eq!(
            Estado::carregar(d.path().into()).ultimo_modulo(),
            Modulo::Ia
        );
    }

    #[test]
    fn ultimo_modulo_visitado_persiste_entre_execucoes() {
        let d = dir();
        let mut estado = Estado::carregar(d.path().into());
        estado.mostrar(Some(Modulo::Rede));

        assert_eq!(
            Estado::carregar(d.path().into()).ultimo_modulo(),
            Modulo::Rede
        );
    }

    #[test]
    fn configuracoes_nao_mudam_o_ultimo_modulo() {
        let d = dir();
        let mut estado = Estado::carregar(d.path().into());
        estado.mostrar(Some(Modulo::Github));
        estado.mostrar(None);

        assert_eq!(
            Estado::carregar(d.path().into()).ultimo_modulo(),
            Modulo::Github
        );
    }

    #[test]
    fn preferencias_ilegiveis_valem_como_primeira_execucao() {
        let d = dir();
        fs::write(d.path().join("preferencias.json"), "{ não é json").unwrap();

        assert_eq!(
            Estado::carregar(d.path().into()).ultimo_modulo(),
            Modulo::Ia
        );
    }

    #[test]
    fn com_o_painel_fechado_nenhum_modulo_esta_visivel() {
        let d = dir();
        let mut estado = Estado::carregar(d.path().into());
        estado.mostrar(Some(Modulo::Sistema));

        assert_eq!(estado.modulo_visivel(), None);
    }

    #[test]
    fn ao_abrir_o_painel_o_ultimo_modulo_fica_visivel() {
        let d = dir();
        let mut estado = Estado::carregar(d.path().into());
        estado.mostrar(Some(Modulo::Sistema));
        estado.abrir_painel();
        assert_eq!(estado.modulo_visivel(), Some(Modulo::Sistema));

        estado.mostrar(Some(Modulo::Trello));
        assert_eq!(estado.modulo_visivel(), Some(Modulo::Trello));

        estado.fechar_painel();
        assert_eq!(estado.modulo_visivel(), None);
    }

    #[test]
    fn nas_configuracoes_nenhum_modulo_esta_visivel() {
        let d = dir();
        let mut estado = Estado::carregar(d.path().into());
        estado.abrir_painel();
        estado.mostrar(None);
        assert_eq!(estado.modulo_visivel(), None);

        // A webview é destruída ao fechar: reabrir recarrega a UI no último Módulo.
        estado.fechar_painel();
        estado.abrir_painel();
        assert_eq!(estado.modulo_visivel(), Some(Modulo::Ia));
    }

    #[test]
    fn primeira_execucao_so_vale_ate_ser_concluida() {
        let d = dir();
        let mut estado = Estado::carregar(d.path().into());
        assert!(estado.primeira_execucao());

        estado.concluir_primeira_execucao();

        assert!(!Estado::carregar(d.path().into()).primeira_execucao());
    }

    #[test]
    fn concluir_a_primeira_execucao_preserva_o_ultimo_modulo() {
        let d = dir();
        let mut estado = Estado::carregar(d.path().into());
        estado.mostrar(Some(Modulo::Agentes));
        estado.concluir_primeira_execucao();

        assert_eq!(
            Estado::carregar(d.path().into()).ultimo_modulo(),
            Modulo::Agentes
        );
    }
}
