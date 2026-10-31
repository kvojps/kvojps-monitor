# 08: Configurações completas, temas e primeira execução

**What to build:** configurar o app e suas fontes na única área separada do Painel, com formulários legíveis, primeira execução completa e preferências visuais persistentes.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** ready-for-agent

- [x] Aplicar a linguagem visual às Configurações, mantendo “Voltar ao Painel”, contexto anterior e Escape para ocultar. Primeira execução usa essa mesma superfície, com status por Módulo e atalhos de configuração.
- [x] Preservar Claude (status line, consentimento oauth/usage e diagnóstico), hooks com passos de confiança, notificações, fonte/token GitHub, Trello (key/token/autorização/quadro/desconexão) e velocidade automática.
- [x] Completar requisitos da v1 ainda ausentes: autostart ligado inicialmente e desligável, sincronização do caminho no AppImage, checagem diária de versão ligada por padrão/desligável e link discreto sem auto-update, e abertura da pasta de logs.
- [x] Oferecer escuro/claro/sistema com tokens completos de geometria/hierarquia e acompanhar mudanças do ambiente. Persistir tema e redução manual de animações; redução do sistema prevalece.
- [x] Validar antes de salvar; erro junto do campo e sucesso identifica conexão. Campos e ações são acessíveis por teclado, sem explicações de implementação desnecessárias.
- [x] Testar explicitamente: credenciais só no GNOME Keyring e recusa sem keyring; logs/cache/notificações só com metadados permitidos; hooks/status line alterados só após clique, aditiva e reversivelmente preservando terceiros; token Claude apenas em memória e sem renovação. Usar exclusivamente o seam de E/S existente.
- [x] Testar fixtures e comandos de formulários, primeira execução, persistência de preferências, mudança de tema do sistema e falhas de validação/portas; não substituir comportamento funcional por simulação do protótipo.

## Registro da implementação (2026-10-31, não commitado)

Mudanças staged sobre `e71e841`; revisão de código (padrões e spec) em andamento, commit pendente.

- **Configurações:** nova superfície `src/lib/settings/Settings.svelte` (cartões LCARS com faixa por Módulo, formulários com estilo comum, títulos h3/h4). Mantém “Voltar ao Painel”, retorno de foco e Escape. “Configurar” de IA/GitHub/Trello abre direto a seção do Módulo (`#settings-<módulo>`). IA mostra também o status do Codex.
- **Primeira execução:** `welcome` no `initial_state`; o Painel abre nas Configurações com “Boas-vindas”, status por Módulo (Detectado / Precisa configurar / Não precisa configurar / Verificando…) e atalhos, incluindo “Conectar ao Claude Code” (só no clique). Marcada como vista (`welcomed`) ao ser exibida; some ao voltar ao Painel. Notificação de Agentes tem prioridade sobre as boas-vindas.
- **Geral:** autostart desligável (`autostart::set`/`enabled`; desligado não é recriado, AppImage continua sincronizado), checagem diária de versão (`releases.rs`: porta `ReleaseFeed`, ligada por padrão, 404 = sem release, falha tenta em 1 h, persiste `checkedAt`/release, some ao instalar a versão), linha discreta com link só para `github.com/Kvojps/kvojps-monitor/releases/`, “Abrir pasta de logs” (`general.rs`, via `xdg-open` na porta `Processes`).
- **Aparência:** tema escuro/claro/sistema e “Reduzir animações” persistidos em `preferencias.json` (`appearance`). Sistema segue o portal `org.freedesktop.appearance color-scheme` (`desktop_theme.rs`, evento `system-theme`; 0 e 2 = claro, 1 = escuro), com fallback para `prefers-color-scheme`. `data-theme`/`data-motion` no `<html>`; redução do sistema prevalece. Tokens claros passaram para `:root[data-theme="light"]`.
- **Testes:** núcleo 266 + 3 (novos: preferências sem credenciais/conteúdo, autostart, checagem de versão, portal, logs/release); UI 203 (novo `settings/Settings.test.ts`). Guardrails de keyring, logs/cache/notificações, hooks/status line e token Claude seguem cobertos pelos testes existentes. Fixtures novas em `src-tauri/fixtures/releases/` e `appearance/` (ver READMEs: o repositório ainda não tem release real; `newer.json` é o registro real do Tauri reescrito).
- **Pendências:** resultado da revisão de código e commit; verificação nativa (tema do sistema mudando ao vivo no GNOME, autostart no login, abertura de logs/link) fica para o ticket 10; contraste do tema claro nas novas telas não foi medido.

