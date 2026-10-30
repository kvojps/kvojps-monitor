# Pipeline de release e repo público

Type: grilling
Status: resolved
Assignee: Kvojps
Blocked by: 07, 15
Map: [kvojps-monitor v1](../map.md)

## Question

Como a v1 é publicada no GitHub: o que dispara um release (tag `v*`, manual), versionamento, o que o workflow do GitHub Actions faz (`tauri-action` em `ubuntu-24.04`, `.deb` + AppImage, checksums, assinatura?), se há atualização automática ou só aviso de nova versão, licença do projeto, e o que o README precisa conter (instalação, extensão AppIndicator, guia de key + token do Trello, `gh auth login`, aviso do opt-in `oauth/usage`)? Stack e empacotamento já decididos em [Escolher a stack](07-escolher-stack.md); credenciais em [Fluxo de credenciais nas Configurações](15-fluxo-de-credenciais.md).

## Answer

- **Repo**: `Kvojps/kvojps-monitor`, público. Publica código, README, `GLOSSARY.md` e `docs/adr/`; `.scratch/` sai do repo após a spec ser consolidada; branches `research/*` nunca sobem.
- **Licença**: MIT.
- **Disparo**: push de tag `v*` roda o workflow de release, que cria um release em **rascunho** com os artefatos; o usuário revisa e publica à mão. Sem release-please.
- **Versionamento**: SemVer; `tauri.conf.json` é a fonte da verdade e o CI falha se a tag não bater com ela (e com `Cargo.toml`/`package.json`). Começa em `0.1.0`; vira `1.0.0` quando o usuário declarar estável.
- **Workflow de release**: reaproveita os checks do CI, depois `tauri-action` em `ubuntu-24.04` gera `.deb` + AppImage, publica `SHA256SUMS` e GitHub artifact attestations (sem GPG). Notas geradas automaticamente pelo GitHub, editadas no rascunho; sem `CHANGELOG.md`.
- **CI**: workflow `ci` em push na `main` e em PRs: `cargo fmt --check`, `clippy`, `cargo test`, `svelte-check` e testes da UI; build sem empacotar.
- **Atualização**: só aviso. Checagem diária da API de releases do GitHub, ligada por padrão e desligável; mostra uma linha discreta nas Configurações com link para o release, sem notificação. Sem `tauri-plugin-updater` na v1.
- **Idioma**: UI e README em pt-BR, sem i18n na v1; textos da UI num único arquivo de strings.
- **README** (enxuto, guias longos em `docs/` com link): o que é + captura do Painel; requisitos (Ubuntu 24.04+, GNOME, extensão AppIndicator e como ativar); instalação `.deb` e AppImage com verificação (`sha256sum -c`, `gh attestation verify`); primeiros passos por Módulo (`gh auth login`, guia de key + token do Trello, conectar a status line do Claude, Codex detectado sozinho); aviso do opt-in `oauth/usage` (não documentado, risco de ToS); privacidade (tudo local, lista de hosts acessados); desinstalação (desfazer status line e autostart); licença.
