# 20: Detecção instantânea (hooks)

**What to build:** nas Configurações → Agentes ligo "Detecção instantânea"; o app instala hooks no Claude Code e no Codex de forma aditiva, sem tocar nos hooks de outros apps, e me mostra o passo de confiança em `/hooks` do Codex. Com hooks, os estados chegam na hora e "Aguardando" do Codex passa a funcionar. Desligar remove só o que o app adicionou. Ver spec, "Módulo Agentes" e ticket 02 do mapa.

**Blocked by:** 18 (Sessões do Codex), 19 (Notificações de Agentes)

**Status:** ready-for-agent

- [ ] Modo `hook <fonte> <evento>` do próprio binário: lê o JSON do stdin, extrai só metadados (id da sessão, estado, ferramenta, cwd) e envia ao socket `$XDG_RUNTIME_DIR/kvojps-monitor.sock`
- [ ] Sempre sai com código 0 em < 200 ms, inclusive com o app fora do ar (teste explícito)
- [ ] Claude: `UserPromptSubmit`, `PermissionRequest`, `Notification`, `Stop`, `StopFailure`, `SessionEnd` em `~/.claude/settings.json`
- [ ] Codex: `SessionStart`, `SessionEnd`, `UserPromptSubmit`, `PermissionRequest`, `Stop`, `Interrupt` em `~/.codex/hooks.json`
- [ ] Instalação e remoção aditivas preservando hooks de terceiros (teste explícito); arquivos só mudam após o clique
- [ ] Polling do Claude continua como garantia para crash/interrupção
- [ ] UI mostra o passo de confiança em `/hooks` do Codex
