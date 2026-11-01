# kvojps-monitor

App pessoal de bandeja para Ubuntu/GNOME/Wayland (Tauri v2: núcleo Rust + UI Svelte 5) que reúne Limites de uso de IA, Sessões de agente, GitHub, Trello, rede e sistema num Indicador com Painel. Tudo local, custo zero.

## Fonte da verdade

- **Implementando a v1**: siga a spec em `.scratch/kvojps-monitor/spec.md` (tickets em `.scratch/kvojps-monitor/issues/`). O visual segue o protótipo B (seção "Visual do Painel"); os porquês ficam na seção "Decisões e porquês" e em `docs/adr/`.
- **Divergindo da spec**: pare e pergunte antes de seguir; registre a mudança no ticket de origem.

## Idioma

Código em inglês: arquivos, identificadores, comentários, nomes de testes, chaves de dados e comandos internos. Textos exibidos na interface em pt-BR. README, `docs/` e `GLOSSARY.md` em pt-BR. Mensagens de commit em inglês, no formato Conventional Commits (`feat:`, `fix:`, `docs:`).

## Privacidade (guardrails)

O app lê dados sensíveis do usuário; trate estas regras como invioláveis e cubra cada uma com teste:

- Credenciais vivem só no GNOME Keyring; sem keyring, o app recusa salvar.
- Logs, cache em disco e notificações carregam só metadados (fonte, evento, status, duração), nunca credenciais nem conteúdo de prompts, respostas, commits ou cards.
- `~/.claude/settings.json` e `~/.codex/hooks.json` só mudam após clique explícito do usuário, de forma aditiva e reversível, preservando o que outros apps instalaram.
- O token do Claude é lido só em memória e nunca renovado pelo app.

## Testes

Um único seam no núcleo: toda E/S (processos, HTTP, sistema de arquivos, D-Bus, keyring, notificador, relógio) passa por portas injetáveis, alimentadas nos testes com saídas reais gravadas como fixtures. A UI é testada a partir de fixtures de snapshot. Detalhes em "Testing Decisions" da spec.

## Git

Branches `research/*` e `prototype/*` são locais: nunca faça push delas. `.scratch/` é versionada até a v1 ficar pronta.

## Agent skills

### Issue tracker

Markdown local em `.scratch/` (spec, tickets e mapas do `/wayfinder`). See `docs/agents/issue-tracker.md`.

### Triage labels

Os cinco papéis canônicos com os nomes padrão (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`). See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: `GLOSSARY.md` e `docs/adr/` na raiz. See `docs/agents/domain.md`.
