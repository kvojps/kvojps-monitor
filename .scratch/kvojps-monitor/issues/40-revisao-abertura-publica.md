# 40: Revisão antes da abertura pública

**What to build:** uma revisão documentada dos arquivos e do histórico Git que serão expostos ao tornar o repositório público, sem registrar os próprios dados sensíveis encontrados.

**Status:** needs-info

- [x] Delimitar quais branches, tags e histórico ficarão públicos; preservar a regra de nunca enviar branches `research/*` e `prototype/*`
- [ ] Revisar arquivos atuais e histórico alcançável pelas referências que serão publicadas para credenciais, dados pessoais e conteúdo de prompts, respostas, commits e cards
- [x] Incluir fixtures em `src-tauri/fixtures/`, pesquisas em `docs/research/`, tickets e validações em `.scratch/`, imagens em `docs/assets/` e skills versionadas na revisão
- [ ] Distinguir dados sintéticos/sanitizados de capturas reais; revisar visualmente as imagens destinadas ao README
- [x] Registrar escopo, método, limitações e pendências em `.scratch/kvojps-monitor/validation/40/README.md`, usando apenas caminhos e categorias, nunca reproduzindo segredos ou conteúdo privado
- [x] Se houver exposição, registrar a correção necessária e bloquear a abertura até a resolução; não reescrever histórico nem revogar credenciais sem autorização específica
- [x] Conferir a presença de `LICENSE` MIT (ticket 22), README (23), documentação de segurança (41) e guia para compilar/fazer fork (42) antes da abertura
- [ ] Registrar a revisão final depois que documentos e capturas estiverem prontos; não alterar a visibilidade do repositório como parte deste ticket

## Comments

- 2026-11-02 — Aprovado pelo mantenedor na preparação da abertura pública e lançamento da v1. Este ticket não afirma que exista vazamento; formaliza a revisão do material que será publicado. A implementação dos documentos pode ocorrer em paralelo à revisão inicial.

- 2026-11-02 — Revisão inicial registrada em [validation/40/README.md](../validation/40/README.md), com base na `main` em `447c220`. Abertura bloqueada por dados reais em fixtures/histórico, cobertura semântica/visual histórica restante e dependências 22/23/41/42. Escopo previsto: somente `main`, nenhuma tag; inventário remoto ainda pendente. Sem reescrita de histórico, revogação ou alteração de visibilidade. Revisão final permanece aberta; decisões sobre exposição/expurgo exigem mantenedor.

- 2026-11-02 — Mantenedor autorizou sanitizar os dados identificados e removê-los também do histórico Git. Consulta ao GitHub revelou repositório já público, com somente `main` publicada. Execução e limitações registradas em [validation/40/sanitizacao.md](../validation/40/sanitizacao.md). Esta autorização substitui a restrição anterior apenas para o expurgo solicitado; não autoriza publicar branches de protótipo nem mudar visibilidade. Revisão final da v1 continua pendente.
