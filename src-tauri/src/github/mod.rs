use crate::modulo::Modulo;
use crate::portas::{Http, Processos, Relogio};
use crate::snapshot::Snapshot;
use chrono::{DateTime, Datelike, Utc};
use serde_json::{json, Value};

pub struct Github<P, H, R> {
    processos: P,
    http: H,
    relogio: R,
    calendario_em: Option<u64>,
    repo_em: Option<u64>,
    manual_em: Option<u64>,
    tentativa_em: Option<u64>,
    dados: Value,
}
impl<P: Processos, H: Http, R: Relogio> Github<P, H, R> {
    pub fn new(processos: P, http: H, relogio: R) -> Self {
        Self {
            processos,
            http,
            relogio,
            calendario_em: None,
            repo_em: None,
            manual_em: None,
            tentativa_em: None,
            dados: json!({"estado":"semGh", "calendario":null, "destaque":null, "atualizando":false}),
        }
    }
    pub fn snapshot(&self, atualizando: bool) -> Snapshot {
        let mut dados = self.dados.clone();
        dados["atualizando"] = json!(atualizando);
        Snapshot {
            modulo: Modulo::Github,
            atualizado_em: self.repo_em,
            dados,
        }
    }
    pub fn devido(&self, visivel: bool) -> bool {
        let agora = self.relogio.agora();
        self.calendario_em
            .is_none_or(|t| agora.saturating_sub(t) >= 3_600_000 || ano(t) != ano(agora))
            || self
                .repo_em
                .is_none_or(|t| agora.saturating_sub(t) >= if visivel { 300_000 } else { 900_000 })
    }
    pub fn manual(&mut self) -> bool {
        let agora = self.relogio.agora();
        if self
            .manual_em
            .is_some_and(|t| agora.saturating_sub(t) < 10_000)
        {
            return false;
        }
        self.manual_em = Some(agora);
        true
    }
    pub fn deve_coletar(&self, visivel: bool, manual: bool) -> bool {
        let agora = self.relogio.agora();
        let manual_liberado = manual
            && self
                .manual_em
                .is_none_or(|t| agora.saturating_sub(t) >= 10_000);
        manual_liberado
            || (self.devido(visivel)
                && (self.dados["estado"] == "pronto"
                    || self
                        .tentativa_em
                        .is_none_or(|t| agora.saturating_sub(t) >= 30_000)))
    }
    /// Decide a coleta com o mesmo relógio usado pelas janelas e pelo calendário.
    pub fn passo(&mut self, visivel: bool, manual: bool) -> Option<Snapshot> {
        if !self.deve_coletar(visivel, manual) {
            return None;
        }
        let forcar = manual && self.manual();
        self.atualizar(forcar)
    }
    pub fn atualizar(&mut self, forcar: bool) -> Option<Snapshot> {
        let agora = self.relogio.agora();
        self.tentativa_em = Some(agora);
        let calendario = forcar
            || self
                .calendario_em
                .is_none_or(|t| agora.saturating_sub(t) >= 3_600_000 || ano(t) != ano(agora));
        // Não guarda o token na estrutura nem no snapshot.
        let Ok(token) = self.processos.executar("gh", &["auth", "token"]) else {
            self.dados["estado"] = json!("semGh");
            return Some(self.snapshot(false));
        };
        let resultado = self.consultar(token.trim(), agora, calendario);
        match resultado {
            Ok(()) => {
                self.repo_em = Some(agora);
                self.dados["estado"] = json!("pronto");
            }
            Err(()) => self.dados["estado"] = json!("erro"),
        }
        Some(self.snapshot(false))
    }
    fn consultar(&mut self, token: &str, agora: u64, calendario: bool) -> Result<(), ()> {
        let ano = ano(agora);
        let campo = if calendario {
            "contributionsCollection(from:$from,to:$to) { restrictedContributionsCount contributionCalendar { totalContributions weeks { contributionDays { date contributionCount color } } } }"
        } else {
            ""
        };
        let declaracao = if calendario {
            "($from:DateTime!,$to:DateTime!)"
        } else {
            ""
        };
        let query = format!("query{declaracao} {{ viewer {{ id login {campo} repositories(first:20, affiliations:[OWNER,COLLABORATOR,ORGANIZATION_MEMBER], orderBy:{{field:PUSHED_AT,direction:DESC}}) {{ nodes {{ nameWithOwner }} }} }} }}");
        let resposta = self.http.graphql(token, &json!({"query":query,"variables":{"from":format!("{ano}-01-01T00:00:00Z"),"to":format!("{ano}-12-31T23:59:59Z")}}))?;
        if resposta.get("errors").is_some() {
            return Err(());
        }
        let viewer = &resposta["data"]["viewer"];
        let id = viewer["id"].as_str().ok_or(())?;
        let mut destaque: Option<Value> = None;
        let desde = instante(agora.saturating_sub(7 * 24 * 3_600_000));
        let ate = instante(agora);
        for repo in viewer["repositories"]["nodes"].as_array().ok_or(())? {
            let nome = repo["nameWithOwner"].as_str().ok_or(())?;
            let (owner, name) = nome.split_once('/').ok_or(())?;
            let resposta = self.http.graphql(token, &json!({"query": "query($owner:String!,$name:String!,$id:ID!,$since:GitTimestamp!,$until:GitTimestamp!) { repository(owner:$owner,name:$name) { defaultBranchRef { target { ... on Commit { janela:history(first:1,author:{id:$id},since:$since,until:$until) { totalCount nodes { committedDate messageHeadline } } total:history(author:{id:$id}) { totalCount } } } } } }", "variables":{"owner":owner,"name":name,"id":id,"since":desde,"until":ate}}))?;
            if resposta.get("errors").is_some() {
                return Err(());
            }
            let commit = &resposta["data"]["repository"]["defaultBranchRef"]["target"];
            if commit.is_null() {
                continue;
            }
            let n = commit["janela"]["totalCount"].as_u64().ok_or(())?;
            if n == 0 {
                continue;
            }
            let ultimo = &commit["janela"]["nodes"][0];
            let data = ultimo["committedDate"].as_str().ok_or(())?;
            let ms = DateTime::parse_from_rfc3339(data)
                .map_err(|_| ())?
                .timestamp_millis();
            let candidato = json!({"nome":nome,"commits":n,"total":commit["total"]["totalCount"],"titulo":ultimo["messageHeadline"],"quando":ms});
            if destaque.as_ref().is_none_or(|d| {
                (n, ms)
                    > (
                        d["commits"].as_u64().unwrap_or(0),
                        d["quando"].as_i64().unwrap_or(0),
                    )
            }) {
                destaque = Some(candidato);
            }
        }
        if calendario {
            let colecao = &viewer["contributionsCollection"];
            if !colecao["contributionCalendar"].is_object() {
                return Err(());
            }
            // totalContributions já inclui privadas; restricted é a parcela anônima, não se soma de novo.
            self.dados["calendario"] = colecao.clone();
            self.calendario_em = Some(agora);
        }
        self.dados["login"] = viewer["login"].clone();
        self.dados["destaque"] = json!(destaque);
        Ok(())
    }
}
fn ano(ms: u64) -> i32 {
    DateTime::<Utc>::from_timestamp_millis(ms as i64)
        .unwrap()
        .year()
}
fn instante(ms: u64) -> String {
    DateTime::<Utc>::from_timestamp_millis(ms as i64)
        .unwrap()
        .to_rfc3339()
}

pub struct Coletor(pub std::sync::mpsc::Sender<bool>);
pub fn iniciar(app: tauri::AppHandle) {
    use tauri::Manager;
    let (tx, rx) = std::sync::mpsc::channel();
    app.manage(Coletor(tx));
    std::thread::spawn(move || {
        let mut github = Github::new(
            crate::portas::ProcessosSistema,
            crate::portas::HttpGithub,
            crate::portas::RelogioDoSistema,
        );
        loop {
            let manual = rx
                .recv_timeout(std::time::Duration::from_secs(1))
                .unwrap_or(false);
            let visivel = app
                .state::<std::sync::Mutex<crate::estado::Estado>>()
                .lock()
                .unwrap()
                .modulo_visivel()
                == Some(Modulo::Github);
            // Publica o dado anterior antes de bloquear em HTTP.
            if github.deve_coletar(visivel, manual) {
                crate::snapshot::publicar(&app, github.snapshot(true));
            }
            if let Some(snapshot) = github.passo(visivel, manual) {
                crate::snapshot::publicar(&app, snapshot);
            }
        }
    });
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::portas::falsos::RelogioFalso;
    pub(super) struct Gh;
    impl crate::portas::Processos for Gh {
        fn executar(&self, _: &str, _: &[&str]) -> Result<String, ()> {
            Err(())
        }
    }
    pub(super) struct Http;
    impl crate::portas::Http for Http {
        fn graphql(&self, _: &str, _: &serde_json::Value) -> Result<serde_json::Value, ()> {
            panic!("sem credencial não consulta HTTP")
        }
    }
    #[test]
    fn sem_gh_publica_configuracao_sem_credencial() {
        let mut github = Github::new(Gh, Http, RelogioFalso::em(1793448000000));
        let s = github.atualizar(true).unwrap();
        assert_eq!(s.dados["estado"], "semGh");
        assert!(!serde_json::to_string(&s).unwrap().contains("token"));
    }
}

#[cfg(test)]
mod cenarios {
    use super::*;
    use crate::portas::falsos::RelogioFalso;
    struct Gh;
    impl Processos for Gh {
        fn executar(&self, _: &str, _: &[&str]) -> Result<String, ()> {
            Ok("segredo-de-teste".into())
        }
    }
    struct Api {
        vazio: bool,
    }
    impl Http for Api {
        fn graphql(&self, _: &str, corpo: &Value) -> Result<Value, ()> {
            if corpo["query"].as_str().unwrap().contains("viewer {") {
                Ok(
                    serde_json::from_str(include_str!("../../fixtures/github/viewer.json"))
                        .unwrap(),
                )
            } else {
                let mut v: Value =
                    serde_json::from_str(include_str!("../../fixtures/github/history.json"))
                        .unwrap();
                let c = &mut v["data"]["repository"]["defaultBranchRef"]["target"];
                if self.vazio {
                    c["janela"]["totalCount"] = json!(0);
                }
                // Variante da captura: empate com outro repositório e commit mais recente.
                if corpo["variables"]["name"] == "repo-1" {
                    c["janela"]["nodes"][0]["committedDate"] = json!("2026-10-31T12:00:00Z");
                }
                Ok(v)
            }
        }
    }
    #[test]
    fn empate_escolhe_commit_mais_recente_sem_expor_token() {
        let mut g = Github::new(Gh, Api { vazio: false }, RelogioFalso::em(1793448000000));
        let s = g.atualizar(true).unwrap();
        assert_eq!(s.dados["destaque"]["nome"], "exemplo/repo-1");
        assert_eq!(s.dados["destaque"]["commits"], 27);
        assert!(!serde_json::to_string(&s)
            .unwrap()
            .contains("segredo-de-teste"));
    }
    #[test]
    fn janela_vazia_nao_substitui_destaque() {
        let mut g = Github::new(Gh, Api { vazio: true }, RelogioFalso::em(1793448000000));
        assert!(g.atualizar(true).unwrap().dados["destaque"].is_null());
    }
    #[test]
    fn cadencia_visivel_e_manual_usam_relogio_injetado() {
        let r = RelogioFalso::em(1793448000000);
        let mut g = Github::new(Gh, Api { vazio: false }, r.clone());
        g.atualizar(true);
        assert!(!g.devido(true));
        r.avancar(300_000);
        assert!(g.devido(true));
        assert!(!g.devido(false));
        assert!(g.manual());
        assert!(!g.manual());
        r.avancar(10_000);
        assert!(g.manual());
    }
    struct CalendarioAno(RelogioFalso);
    impl Http for CalendarioAno {
        fn graphql(&self, token: &str, corpo: &Value) -> Result<Value, ()> {
            if corpo["query"]
                .as_str()
                .unwrap()
                .contains("contributionsCollection")
            {
                assert_eq!(
                    corpo["variables"]["from"],
                    format!("{}-01-01T00:00:00Z", ano(self.0.agora()))
                );
                assert_eq!(
                    corpo["variables"]["to"],
                    format!("{}-12-31T23:59:59Z", ano(self.0.agora()))
                );
            }
            Api { vazio: false }.graphql(token, corpo)
        }
    }
    #[test]
    fn virada_de_ano_renova_calendario_antes_de_uma_hora() {
        let r = RelogioFalso::em(1798761599000); // 31 dez 2026, 23:59:59 UTC
        let mut g = Github::new(Gh, CalendarioAno(r.clone()), r.clone());
        g.atualizar(true);
        r.avancar(2000);
        assert!(g.devido(false));
        assert_eq!(g.passo(false, false).unwrap().dados["estado"], "pronto");
    }
    struct ApiErro;
    impl Http for ApiErro {
        fn graphql(&self, _: &str, _: &Value) -> Result<Value, ()> {
            Err(())
        }
    }
    #[test]
    fn snapshots_da_ui_correspondem_ao_nucleo() {
        let relogio = RelogioFalso::em(1793448000000);
        let mut cheio = Github::new(Gh, Api { vazio: false }, relogio.clone());
        let mut vazio = Github::new(Gh, Api { vazio: true }, relogio.clone());
        let mut erro = Github::new(Gh, ApiErro, relogio.clone());
        let mut sem = Github::new(super::testes::Gh, super::testes::Http, relogio);
        for (nome, snapshot) in [
            ("cheio", cheio.atualizar(true).unwrap()),
            ("vazio", vazio.atualizar(true).unwrap()),
            ("sem-gh", sem.atualizar(true).unwrap()),
            ("erro", erro.atualizar(true).unwrap()),
        ] {
            let caminho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(format!("../src/lib/github/fixtures/{nome}.json"));
            if std::env::var_os("ATUALIZAR_FIXTURES").is_some() {
                std::fs::write(
                    &caminho,
                    serde_json::to_string_pretty(&snapshot.dados).unwrap() + "\n",
                )
                .unwrap();
            }
            let fixture: Value = serde_json::from_slice(&std::fs::read(caminho).unwrap()).unwrap();
            assert_eq!(snapshot.dados, fixture);
        }
    }

    #[test]
    fn sem_gh_espera_antes_de_repetir_e_manual_respeita_dez_segundos() {
        let r = RelogioFalso::em(1793448000000);
        let mut g = Github::new(super::testes::Gh, super::testes::Http, r.clone());
        assert!(g.passo(false, false).is_some());
        assert!(g.passo(false, false).is_none());
        assert!(g.passo(false, true).is_some());
        r.avancar(9999);
        assert!(g.passo(false, true).is_none());
        r.avancar(1);
        assert!(g.passo(false, true).is_some());
        r.avancar(30000);
        assert!(g.passo(false, false).is_some());
    }
    struct CadenciaCalendario {
        relogio: RelogioFalso,
    }
    impl Http for CadenciaCalendario {
        fn graphql(&self, token: &str, corpo: &Value) -> Result<Value, ()> {
            if corpo["query"].as_str().unwrap().contains("viewer {") {
                let mut resposta = Api { vazio: false }.graphql(token, corpo)?;
                let calendario = corpo["query"]
                    .as_str()
                    .unwrap()
                    .contains("contributionsCollection");
                let mut total = 1;
                if self.relogio.agora() >= 1793451600000 {
                    total = 99;
                }
                if calendario {
                    resposta["data"]["viewer"]["contributionsCollection"]["contributionCalendar"]
                        ["totalContributions"] = json!(total);
                } else {
                    resposta["data"]["viewer"]
                        .as_object_mut()
                        .unwrap()
                        .remove("contributionsCollection");
                }
                Ok(resposta)
            } else {
                Api { vazio: false }.graphql(token, corpo)
            }
        }
    }
    #[test]
    fn calendario_permanece_uma_hora_enquanto_destaque_atualiza_a_cada_cinco_minutos() {
        let r = RelogioFalso::em(1793448000000);
        let mut g = Github::new(Gh, CadenciaCalendario { relogio: r.clone() }, r.clone());
        let inicial = g.passo(true, false).unwrap();
        assert_eq!(
            inicial.dados["calendario"]["contributionCalendar"]["totalContributions"],
            1
        );
        r.avancar(300000);
        let seguinte = g.passo(true, false).unwrap();
        assert_eq!(seguinte.atualizado_em, Some(1793448300000));
        assert_eq!(seguinte.dados["calendario"], inicial.dados["calendario"]);
        r.avancar(3300000);
        let hora = g.passo(false, false).unwrap();
        assert_eq!(
            hora.dados["calendario"]["contributionCalendar"]["totalContributions"],
            99
        );
    }
}
