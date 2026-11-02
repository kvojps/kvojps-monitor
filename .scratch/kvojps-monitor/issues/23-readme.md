# 23: README em pt-BR

**What to build:** um README enxuto em pt-BR que leva um novo usuário da instalação ao uso, com guias longos em `docs/`. Ver spec, "Release e repositório".

**Blocked by:** 06, 07, 10, 13, 16, 20, 21, 22

**Status:** ready-for-human

- [x] O que é + imagem real do app em execução, incluindo o Painel; armazenada em `docs/assets/`, com texto alternativo e dados sanitizados, sem apresentar o protótipo B como captura da versão distribuída
- [x] Requisitos (Ubuntu 24.04+, GNOME, extensão AppIndicator) e como ativar a extensão
- [x] Instalação `.deb`/AppImage com verificação (`sha256sum -c`, `gh attestation verify`)
- [x] Primeiros passos por Módulo
- [x] Aviso do opt-in `oauth/usage`
- [x] Privacidade: tudo local, lista de hosts acessados (api.github.com, api.trello.com, api.anthropic.com só no opt-in, speed.cloudflare.com, conectividade/ping)
- [x] Desinstalação: desfazer status line, hooks e autostart
- [x] Licença
- [x] Explicar que é um projeto pessoal que não recebe contribuições externas; forks e adaptações seguem a licença MIT, sem convite para issues de funcionalidades ou PRs
- [x] Link para o guia de desenvolvimento destinado a quem quiser compilar ou manter um fork (ticket 42)
- [x] Link para `SECURITY.md` e o canal privado de relatos de segurança (ticket 41)

## Comments

- 2026-11-02 — Escopo confirmado pelo mantenedor: preparar a documentação para a abertura pública junto do lançamento da v1. Manter este ticket como README da versão distribuída, sem criar uma etapa de README pré-v1. Usar imagens reais do app com dados sanitizados. Não receber contribuições externas; permitir forks conforme MIT. Complementos registrados nos tickets 40–42.

- 2026-11-02 — Ticket 41 implementou a [política de segurança](../../../SECURITY.md) e confirmou o canal privado habilitado pela API. Ao criar o README da v1, incluir links para a política e para https://github.com/kvojps/kvojps-monitor/security/advisories/new. Integração permanece pendente neste ticket.

- 2026-11-02 — Ticket 42 preparou o [guia de desenvolvimento e forks](../../../docs/desenvolvimento.md). Ao criar o README da v1, incluir o link e a política de não receber contribuições externas. Integração permanece pendente neste ticket.

- 2026-11-02 — README implementado em `README.md`. A imagem `docs/assets/painel-fixtures.png` foi capturada da janela Tauri nativa em execução com variantes sintéticas `readme` dos seis Módulos. A captura mede 1100 × 700, tema escuro, sem rolagem e com todas as folhas de estilo do Painel. Inspeção visual e `file`: PNG RGB, sem metadados EXIF. Legenda e texto alternativo identificam exemplos e deixam claro que não é captura da versão distribuída. JSON de métricas permaneceu em `/tmp`; origem e campos sintéticos constam em `src/lib/fixtures/README.md` e `docs/assets/README.md`.

- 2026-11-02 — A integração dos links de segurança (ticket 41) e desenvolvimento/forks (ticket 42), pendente nos comentários anteriores, foi concluída no README acima.
