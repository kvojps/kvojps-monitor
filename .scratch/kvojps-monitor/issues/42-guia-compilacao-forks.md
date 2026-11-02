# 42: Guia para compilar e manter forks

**What to build:** completar `docs/desenvolvimento.md` em pt-BR para que outra pessoa consiga partir de um clone limpo, executar, testar e compilar o app ou seu próprio fork.

**Status:** resolved

- [x] Documentar ferramentas e dependências de sistema para o ambiente suportado (Ubuntu 24.04+, GNOME), a partir da configuração real do projeto
- [x] Documentar instalação das dependências do projeto, execução do app real e uso do modo de fixtures, distinguindo os efeitos de cada modo
- [x] Documentar os comandos de verificação usados pelo CI e os comandos para gerar os pacotes locais disponíveis
- [x] Verificar os passos a partir de um clone limpo ou ambiente isolado e registrar limitações da validação
- [x] Preservar as instruções existentes de protótipo, fixtures e captura; explicar como gerar imagens sanitizadas do app
- [x] Explicar que o projeto não recebe contribuições externas; quem quiser adaptar ou manter uma versão própria pode fazer fork conforme MIT
- [x] Não criar fluxo de contribuição, templates de PR ou convites para enviar alterações ao projeto original
- [x] Incluir link para o guia no README pelo ticket 23

## Comments

- 2026-11-02 — Aprovado pelo mantenedor: documentação voltada a uso e forks, sem receber contribuições. Não é necessário criar `CONTRIBUTING.md`; a política fica explícita no README e neste guia.

## Validação

- Guia conferido contra `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `.github/workflows/ci.yml`, modo de fixtures e script de captura. Link no README delegado explicitamente ao ticket 23, pois o README da v1 ainda não existe; comentário de integração adicionado lá.
- Cópia isolada dos arquivos versionados via `git archive`, sem `node_modules` ou artefatos de build, em diretório temporário fora do repositório. Node 24.20.0, npm 11.19.0 e Rust 1.99.1. `npm ci --offline --no-audit` passou com o cache local; `cargo fmt --check`, `npm run check` (zero erros/avisos), teste isolado `AiModule.test.ts` (46 testes) e `npm run build` passaram nessa cópia.
- Suíte Rust executada na árvore de trabalho com dependências/artefatos existentes e `--locked --offline`: 305 testes unitários e três testes de integração passaram. Os testes de integração exigiram execução fora do sandbox para sockets locais.
- Testes Rust filtrados com `--features fixtures --lib fixtures`: 15 passaram.
- `npm test` completo na cópia isolada: 376 testes em 16 arquivos passaram. Clippy padrão e com `fixtures` falharam em código preexistente (`src-tauri/src/ai/operation.rs:35`, `wrong_self_convention` e `new_ret_no_self`) com Rust 1.99.1 e `-D warnings`; nenhuma alteração Rust faz parte deste ticket.
- Limitações: não foi uma instalação em Ubuntu recém-provisionado; dependências de sistema já estavam disponíveis, e os registros npm/Cargo não foram baixados do zero. Compilação Rust não foi repetida na cópia isolada. Execução gráfica real/fixtures, captura e empacotamento `.deb`/AppImage não foram executados nesta tarefa; os comandos foram conferidos com a configuração e scripts, sem afirmar validação desses fluxos.
- Sanitização: revisão textual dos três arquivos alterados, links e diff; somente comandos, caminhos relativos, versões de ferramentas e contagens de testes. Nenhuma captura, fixture ou dado real de integrações foi adicionado. Identidades Git verificadas como noreply antes do commit.
- Revisão pela skill `code-review` em agentes independentes: Standards, zero achados; Spec, zero achados acionáveis. A integração futura no README e as limitações de execução foram consideradas explícitas e coerentes com o escopo.
