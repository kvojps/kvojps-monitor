# 40: Revisão antes da abertura pública

**What to build:** uma revisão documentada dos arquivos e do histórico Git que serão expostos ao tornar o repositório público, sem registrar os próprios dados sensíveis encontrados.

**Status:** ready-for-agent

- [ ] Delimitar quais branches, tags e histórico ficarão públicos; preservar a regra de nunca enviar branches `research/*` e `prototype/*`
- [ ] Revisar arquivos atuais e histórico alcançável pelas referências que serão publicadas para credenciais, dados pessoais e conteúdo de prompts, respostas, commits e cards
- [ ] Incluir fixtures em `src-tauri/fixtures/`, pesquisas em `docs/research/`, tickets e validações em `.scratch/`, imagens em `docs/assets/` e skills versionadas na revisão
- [ ] Distinguir dados sintéticos/sanitizados de capturas reais; revisar visualmente as imagens destinadas ao README
- [ ] Registrar escopo, método, limitações e pendências em `.scratch/kvojps-monitor/validation/40/README.md`, usando apenas caminhos e categorias, nunca reproduzindo segredos ou conteúdo privado
- [ ] Se houver exposição, registrar a correção necessária e bloquear a abertura até a resolução; não reescrever histórico nem revogar credenciais sem autorização específica
- [ ] Conferir a presença de `LICENSE` MIT (ticket 22), README (23), documentação de segurança (41) e guia para compilar/fazer fork (42) antes da abertura
- [ ] Registrar a revisão final depois que documentos e capturas estiverem prontos; não alterar a visibilidade do repositório como parte deste ticket

## Comments

- 2026-11-02 — Aprovado pelo mantenedor na preparação da abertura pública e lançamento da v1. Este ticket não afirma que exista vazamento; formaliza a revisão do material que será publicado. A implementação dos documentos pode ocorrer em paralelo à revisão inicial.
