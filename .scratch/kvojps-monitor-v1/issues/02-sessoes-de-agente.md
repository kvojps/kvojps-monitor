# Como detectar Sessões de agente do Claude Code e do Codex e seu estado

Type: research
Status: resolved
Blocked by: 
Map: [kvojps-monitor v1](../map.md)

## Question

Como listar as Sessões de agente locais do Claude Code e do Codex e classificar cada uma como em andamento, aguardando o usuário ou finalizada? Fontes candidatas: processos em execução, transcripts/JSONL em ~/.claude/projects e ~/.codex/sessions, hooks do Claude Code, notify do Codex. Que metadados dá para mostrar (projeto/diretório, título, início, última atividade)? Como detectar o término/espera de forma confiável para notificar?

## Answer

**Claude Code**: fonte principal oficial `claude agents --json --all` (~0,16s) — por sessão viva traz `pid`, `cwd`, `startedAt`, `sessionId`, `name`, `status` = `busy` (em andamento) / `waiting` (aguardando usuário, com `waitingFor`) / `idle` (turno concluído); sessões em background usam `state` working/blocked/done/failed/stopped. Hooks para tempo real: `UserPromptSubmit` (início), `PermissionRequest`/`Notification` (aguardando), `Stop`/`StopFailure` (fim de turno; não dispara em interrupção), `SessionEnd` (timeout 1,5s). Polling continua necessário para crash/interrupção.

**Codex**: sem listagem JSON (`codex agents` é interativo). Ler `~/.codex/state_5.sqlite` (read-only; tabela `threads`: cwd, título, criação, última atualização, git_branch — sem status) + último evento do rollout `.jsonl` (`task_started`/`task_complete`/`turn_aborted`) + processo vivo. Aguardando aprovação só via hook `PermissionRequest`. Hooks Codex: SessionStart/End, UserPromptSubmit, PermissionRequest, Stop, Interrupt (sem Notification); exigem confiança manual em `/hooks`. `notify` só cobre fim de turno.

**Restrições**: já existem hooks de outro app (Orca) em `~/.claude/settings.json` e `~/.codex/hooks.json` — instalar hooks de forma aditiva, sem sobrescrever; nunca registrar conteúdo de mensagens. Cada leitor atrás de um adapter (formatos do Codex e `~/.claude/sessions/*.json` não são interfaces estáveis).

Achados completos: [`docs/research/sessoes-de-agente.md`](../../../docs/research/sessoes-de-agente.md) (trazido da branch `research/sessoes-de-agente`, commit `24d392e`, apagada em 2026-11-01).

## Comments

- 31/10/2026 — Na implementação do ticket 20, o usuário aprovou acrescentar
  `PostToolUse` aos hooks do Codex e atualizar a spec: depois de `PermissionRequest`,
  os seis eventos originais não sinalizavam a retomada após a aprovação.
  `PostToolUse` retorna a sessão a Em andamento quando a ferramenta termina.
