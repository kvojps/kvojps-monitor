# 20: Detecção instantânea (hooks)

**What to build:** nas Configurações → Agentes ligo "Detecção instantânea"; o app instala hooks no Claude Code e no Codex de forma aditiva, sem tocar nos hooks de outros apps, e me mostra o passo de confiança em `/hooks` do Codex. Com hooks, os estados chegam na hora e "Aguardando" do Codex passa a funcionar. Desligar remove só o que o app adicionou. Ver spec, "Módulo Agentes" e ticket 02 do mapa.

**Blocked by:** 18 (Sessões do Codex), 19 (Notificações de Agentes)

**Status:** done

- [x] Modo `hook <fonte> <evento>` do próprio binário: lê o JSON do stdin, extrai só metadados (id da sessão, estado, ferramenta, cwd) e envia ao socket `$XDG_RUNTIME_DIR/kvojps-monitor.sock`
- [x] Sempre sai com código 0 em < 200 ms, inclusive com o app fora do ar (teste explícito)
- [x] Claude: `UserPromptSubmit`, `PermissionRequest`, `Notification`, `Stop`, `StopFailure`, `SessionEnd` em `~/.claude/settings.json`
- [x] Codex: `SessionStart`, `SessionEnd`, `UserPromptSubmit`, `PermissionRequest`, `PostToolUse`, `Stop`, `Interrupt` em `~/.codex/hooks.json`
- [x] Instalação e remoção aditivas preservando hooks de terceiros (teste explícito); arquivos só mudam após o clique
- [x] Polling do Claude continua como garantia para crash/interrupção
- [x] UI mostra o passo de confiança em `/hooks` do Codex

## Comments

- 31/10/2026 — Usuário aprovou acrescentar `PostToolUse` ao Codex para reconciliar
  a retomada após aprovação; mudança registrada no ticket 02 de origem e na spec.
- 31/10/2026 — Usuário aprovou a exceção de testes: fixtures de stdin de hooks
  baseadas nos contratos oficiais sanitizados, complementadas pelas capturas reais
  de polling existentes e por testes do binário/socket. Não instalar hooks nem
  iniciar sessões de IA somente para capturar dados.

## Implementação

Modo `hook` antes da inicialização do Tauri, stdin limitado pela porta injetável
(100 ms / 64 KiB) e socket Unix não bloqueante. O núcleo recebe somente metadados;
conteúdo sentinela é descartado nos testes. Configurações instalam/removem somente
entradas registradas no manifesto do app, com validação prévia e reversão de falha.
A coleta de polling roda separada do recebimento de hooks; amostras iniciadas antes
de um hook não sobrescrevem o estado mais recente. Claude mantém polling de 2 s.
O passo manual de confiança do Codex aparece nas Configurações.
