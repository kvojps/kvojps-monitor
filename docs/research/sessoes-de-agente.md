# Como detectar Sessões de agente do Claude Code e do Codex e seu estado

Pesquisa feita em 2026-10-30 nesta máquina (Ubuntu 26.04, Claude Code 2.1.314, codex-cli 0.165.0).
As fontes primárias são a documentação oficial da Anthropic e da OpenAI. Também inspecionei os arquivos locais,
olhando só a estrutura (chaves e tipos de evento). Nenhum conteúdo de conversa nem segredo foi copiado para cá.
Quando digo **observado**, a afirmação vem dessa inspeção local e não está documentada como interface estável.

## Pergunta

Como listar as Sessões de agente locais do Claude Code e do Codex e classificar cada uma como
**em andamento**, **aguardando o usuário** ou **finalizada**? Que metadados dá para mostrar?
Como detectar o término ou a espera com confiabilidade suficiente para notificar?

---

## 1. Claude Code

### 1.1 Listagem com estado: `claude agents --json` (fonte principal)

O Claude Code já tem um comando oficial que lista as sessões e informa o estado de cada uma.

> `claude agents --json` prints active sessions as a JSON array and exits: every live session, plus background
> sessions that are still working or blocked even when their process has exited. Add `--all` to also include
> completed background sessions, and `--cwd <path>` to limit the list to sessions started under that directory.
> — https://code.claude.com/docs/en/agent-view

Campos documentados (mesma página):

| Campo | Quando aparece | Conteúdo |
|---|---|---|
| `cwd`, `kind`, `startedAt` | sempre | diretório, `interactive` ou `background`, início em ms Unix |
| `pid`, `status` | enquanto o processo vive | `busy`, `waiting` ou `idle` |
| `waitingFor` | quando `status` = `waiting` | p.ex. `permission prompt`, `input needed`, `sandbox request`, `dialog open` |
| `id`, `state` | sessões em background | `working`, `blocked`, `done`, `failed`, `stopped` |
| `sessionId`, `name` | quando definidos | UUID (serve para `claude --resume`) e nome de exibição |

A doc também avisa que os arquivos em `~/.claude/jobs/<id>/` "are not a stable interface", e que o estado deve ser
lido "through `claude agents --json` instead of parsing the file" (https://code.claude.com/docs/en/agent-view).

**Observado:** rodei `claude agents --json --all` aqui. O comando devolveu 2 sessões interativas, uma com
`status: "busy"` e outra com `status: "idle"`, e 1 sessão background com `state: "blocked"`. Levou cerca de 0,16 s,
então dá para fazer polling a cada 2–5 s sem custo relevante.

Mapeamento para os três estados do monitor:

| Monitor | Claude Code |
|---|---|
| em andamento | `status: busy` / `state: working` |
| aguardando o usuário | `status: waiting` (com `waitingFor`) / `state: blocked` |
| finalizada (turno concluído, pronta para o próximo prompt) | `status: idle` / `state: done` |
| encerrada (processo saiu) | some da lista (interativa) / `done`, `failed` ou `stopped` (background, com `--all`) |

Repare que "finalizada" tem dois sentidos aqui. Pode ser o **turno** concluído com a sessão ainda aberta
(`idle`), ou o **processo** encerrado. A doc diz explicitamente: "A session that finished its turn and is waiting
for your next instruction reads `done`, not `blocked`" (https://code.claude.com/docs/en/agent-view).

### 1.2 Arquivos locais

- **`~/.claude/sessions/<pid>.json`**: "holds one small file per running session, used to detect concurrent
  sessions and crashes. [...] Claude Code removes each file when its session exits and clears crash leftovers on
  the next launch." (https://code.claude.com/docs/en/claude-directory). **Observado:** as chaves incluem `pid`,
  `sessionId`, `cwd`, `startedAt`, `kind`, `entrypoint`, `name`, `status` (`busy`/`idle`), `updatedAt` e
  `statusUpdatedAt`. É a mesma informação do `claude agents --json`, mas o formato interno não é documentado.
  Serve como fallback barato via inotify, e é preciso conferir se o `pid` ainda está vivo.
- **Transcripts em `~/.claude/projects/<cwd-com-hífens>/<sessionId>.jsonl`**: "Conversation transcript: messages,
  tool calls, and tool results [...]". Subagentes ficam em `<session>/subagents/`, e a retenção padrão
  (`cleanupPeriodDays`) é de 30 dias (https://code.claude.com/docs/en/claude-directory). Os transcripts guardam
  texto em claro, inclusive possíveis segredos ("Plaintext storage", mesma página). O monitor deve ler só
  metadados. **Observado:** cada linha é um objeto JSON com `type`. Os tipos úteis:
  - `user` / `assistant`: têm `timestamp`, `cwd`, `gitBranch`, `sessionId` e `version`. O `assistant` traz
    `message.stop_reason`.
  - `ai-title` (`aiTitle`): é o título gerado. Pode haver várias linhas, e a última vale.
  - `system` com `subtype: turn_duration` ou `stop_hook_summary`: aparece no fim de cada turno.
  - O `timestamp` da última linha (ou o mtime do arquivo) dá a **última atividade**.
- `~/.claude/history.jsonl` guarda os prompts digitados, com timestamp e caminho do projeto (doc acima). Tem
  conteúdo do usuário, então é melhor evitar.

### 1.3 Hooks (para notificação em tempo real)

Referência: https://code.claude.com/docs/en/hooks. Os hooks entram em `~/.claude/settings.json`, e "Hook entries
merge across levels rather than replacing each other". Todo hook recebe no stdin `session_id`,
`transcript_path`, `cwd`, `permission_mode` e `hook_event_name`.

| Evento | Uso no monitor |
|---|---|
| `SessionStart` (`source`: startup, resume, clear, compact, fork) | registrar a sessão |
| `UserPromptSubmit` | marcar **em andamento** |
| `Stop` | turno concluído: **finalizada/aguardando próximo prompt**. "Does not run if the stoppage occurred due to a user interrupt." Recebe `last_assistant_message` |
| `StopFailure` (`error`: rate_limit, overloaded, ...) | turno terminou com erro de API |
| `PermissionRequest` | **aguardando o usuário** (aprovação) |
| `Notification` (`notification_type`: `permission_prompt`, `idle_prompt`, `elicitation_dialog`, `agent_needs_input`, `agent_completed`, ...) | **aguardando o usuário**. `idle_prompt` dispara "about 60 seconds after Claude finishes responding" |
| `SessionEnd` (`reason`: clear, resume, logout, prompt_input_exit, other) | sessão encerrada. Orçamento de 1,5 s, então o hook precisa ser rápido |

Limitações: interrupção pelo usuário (Esc) não dispara `Stop`, e kill/crash não dispara `SessionEnd`. Por isso os
hooks precisam de um reconciliador por polling (`claude agents --json` ou checagem de PID).

**Observado:** esta máquina já tem hooks do **Orca** (`~/.orca/agent-hooks/claude-hook.sh`) instalados em
`~/.claude/settings.json` para `SessionStart`, `UserPromptSubmit`, `Stop`, `StopFailure`, `PermissionRequest`,
`PreToolUse`, `PostToolUse`, entre outros. Qualquer instalação de hooks pelo monitor tem de **acrescentar**
entradas, nunca sobrescrever as existentes.

---

## 2. Codex

### 2.1 Não há equivalente direto ao `claude agents --json`

`codex agents` ("Browse all agent sessions on the shared local app-server daemon", segundo `codex --help`) é uma
TUI e não tem saída JSON. O caminho programático oficial é o **app-server** (JSON-RPC):

> Returned `thread` objects include runtime `status` (`notLoaded`, `idle`, `systemError`, or `active` with
> `activeFlags`). [...] `thread/status/changed` is emitted whenever a loaded thread's runtime `status` changes.
> — https://developers.openai.com/codex/app-server (redireciona para https://learn.chatgpt.com/docs/app-server)

O exemplo da doc mostra `activeFlags: ["waitingOnApproval"]`. O problema é que a doc **não garante** que uma
sessão TUI comum (`codex` no terminal) apareça para outro cliente do app-server. **Observado:** não havia processo
codex nem daemon rodando durante a inspeção, e o diretório `~/.codex/app-server-daemon/` estava vazio. Por isso não
recomendo o app-server como fonte principal para o MVP.

### 2.2 Arquivos locais

- **Rollouts em `~/.codex/sessions/AAAA/MM/DD/rollout-<data>-<threadId>.jsonl`**. **Observado:** são 1334
  arquivos. Cada linha tem `timestamp`, `type` e `payload`. Os tipos úteis:
  - `session_meta`: `id`, `timestamp` (início), `cwd`, `originator` (p.ex. `codex-tui`), `source` (`cli`/`vscode`),
    `cli_version` e `git`.
  - `event_msg` com `payload.type` igual a:
    - `task_started`: início de turno, **em andamento**.
    - `task_complete`: fim de turno, com `duration_ms` e `last_agent_message`.
    - `turn_aborted`: turno interrompido.
  - Regra derivada: se o último evento de turno é `task_started` sem um `task_complete`/`turn_aborted` depois,
    o turno está em andamento. Se é `task_complete`, a sessão está ociosa. A **espera por aprovação não é
    gravada** no rollout (nos meus dados de outubro só apareceram `item_completed`, `token_count`,
    `task_started`, `task_complete`, `thread_settings_applied` e `turn_aborted`).
- **`~/.codex/state_5.sqlite`, tabela `threads`**. **Observado:** colunas `id`, `rollout_path`, `cwd`, `title`,
  `name`, `created_at(_ms)`, `updated_at(_ms)`, `source`, `originator`, `model`, `git_branch`, `archived`,
  `first_user_message`, `preview`, entre outras. **Não há coluna de status em tempo de execução.** É a melhor
  fonte para **título, projeto, início e última atividade**. Deve ser aberta em modo somente-leitura
  (`file:...?mode=ro`). O formato é interno e versionado pelo nome do arquivo (`_5`), então pode mudar.
- `~/.codex/session_index.jsonl` tem `id`, `thread_name` e `updated_at` (observado). É um índice leve de títulos.

### 2.3 Hooks e `notify`

**Hooks** (https://developers.openai.com/codex/hooks, que redireciona para https://learn.chatgpt.com/docs/hooks):
os eventos são `SessionStart`, `SessionEnd`, `UserPromptSubmit`, `PreToolUse`, `PermissionRequest`,
`PostToolUse`, `PreCompact`, `PostCompact`, `SubagentStart`, `SubagentStop`, `Stop` e `Interrupt`. Eles
"are enabled by default" e são configurados em `~/.codex/hooks.json` ou no `[hooks]` do `config.toml`. Todo hook
recebe `session_id`, `transcript_path`, `cwd`, `hook_event_name` e `model`, e eventos de turno trazem também
`turn_id`. Pontos importantes:

- Hooks não gerenciados **precisam ser revisados e marcados como confiáveis** via `/hooks` na CLI antes de rodar.
  Isso é uma etapa manual na instalação. **Observado:** o `config.toml` guarda isso em `[hooks.state."...hooks.json:<evento>:0:0"]`.
- `Interrupt` "runs when you interrupt an active turn on the main thread". Isso cobre o caso que o `Stop` do
  Claude não cobre.
- `PermissionRequest` "doesn't run for commands that don't need approval". É o sinal de **aguardando aprovação**.
- `SessionEnd` roda quando a sessão é arquivada ou excluída, quando o Codex fecha normalmente, ou "after a
  conversation has been idle and isn't open in any connected client for 30 minutes". Por enquanto `reason` vale
  sempre `other`.
- **Não existe evento `Notification`**. "Aguardando entrada" (fim de turno) vem do `Stop`.

**`notify`** (https://developers.openai.com/codex/config-advanced, que redireciona para
https://learn.chatgpt.com/docs/config-file/config-advanced): `notify = ["prog", "args"]` é chamado "whenever Codex
emits supported events (currently only `agent-turn-complete`)" com **um argumento JSON** que traz `type`,
`thread-id`, `turn-id`, `cwd`, `input-messages` e `last-assistant-message`. Só cobre o fim de turno e aceita um
único programa. Os hooks substituem isso e são mais completos. Para aprovação, a TUI tem `tui.notifications`
(tipos `agent-turn-complete` e `approval-requested`), mas a saída vai para o terminal (OSC 9/BEL), não para um
programa externo.

**Observado:** `~/.codex/hooks.json` também já tem hooks do Orca (`codex-hook.sh`) para `SessionStart`,
`UserPromptSubmit`, `PreToolUse`, `PermissionRequest`, `PostToolUse`, `SubagentStart/Stop` e `Stop`. Também aqui é
preciso acrescentar entradas, sem sobrescrever.

---

## 3. Processos em execução (para os dois CLIs)

`pgrep -a claude` / `pgrep -a codex` e `/proc/<pid>/cwd` mostram que existe uma sessão viva e em qual diretório.
Não dizem o estado. Para o Claude, o `pid` vem pronto em `claude agents --json`. Para o Codex, uma hipótese **não
verificada** (não havia codex rodando) é que `/proc/<pid>/fd` aponte para o rollout `.jsonl` aberto, o que
ligaria o processo à thread. Mesmo sem isso, a regra "processo vivo com cwd X + rollout mais recente com cwd X"
serve como heurística.

---

## 4. Metadados disponíveis

| Metadado | Claude Code | Codex |
|---|---|---|
| Projeto/diretório | `cwd` (`claude agents --json`) | `threads.cwd` / `session_meta.cwd` |
| Título | `name` (agents); `ai-title.aiTitle` (transcript) | `threads.title` / `name`; `session_index.thread_name` |
| Início | `startedAt` (ms) | `threads.created_at_ms` / `session_meta.timestamp` |
| Última atividade | mtime/último `timestamp` do transcript; `updatedAt` em `sessions/<pid>.json` (obs.) | `threads.updated_at_ms`; mtime do rollout |
| Branch git | `gitBranch` nas linhas do transcript | `threads.git_branch` |
| Motivo da espera | `waitingFor` | só via hook `PermissionRequest` |
| Retomar | `sessionId` → `claude --resume <id>` | `id` → `codex resume <id>` |

---

## 5. Detectar término e espera com confiabilidade

1. **Eventos via hooks** (latência zero): um script pequeno e comum aos dois CLIs, registrado em
   `~/.claude/settings.json` e `~/.codex/hooks.json`, que **apenas** grava o JSON do stdin (sem o texto da mensagem)
   num FIFO ou socket Unix do monitor, ou num arquivo de estado em `$XDG_RUNTIME_DIR/kvojps-monitor/`, e sai com
   `{}`/exit 0 imediatamente.
   - Claude: `UserPromptSubmit` → andamento; `PermissionRequest` e `Notification(permission_prompt|elicitation_dialog|agent_needs_input)` → aguardando; `Stop`/`StopFailure` → turno finalizado; `SessionEnd` → encerrada.
   - Codex: `UserPromptSubmit` → andamento; `PermissionRequest` → aguardando; `Stop` → turno finalizado; `Interrupt` → interrompida; `SessionEnd` → encerrada.
2. **Reconciliação por polling** (para cobrir crash, kill, Esc no Claude e hooks não confiáveis no Codex):
   - Claude: `claude agents --json --all` a cada poucos segundos. É a fonte de verdade documentada.
   - Codex: `threads` (SQLite ro) + último evento de turno no rollout + PID vivo. Se o processo morreu, a sessão
     está encerrada, mesmo sem `SessionEnd`.
3. **Notificar** nas transições `andamento → aguardando` e `andamento → turno finalizado`, com debounce de alguns
   segundos para evitar duplicar o hook e o polling. Nenhuma das fontes exige rede ou custo.

---

## Recomendação

- **Claude Code:** use `claude agents --json --all` como fonte principal de lista e estado. É documentado, rápido
  (cerca de 0,16 s), traz `status` busy/waiting/idle com `waitingFor` e mantém metadados, sem depender de formato
  interno. Complemente com hooks `Stop`, `PermissionRequest`, `Notification` e `SessionEnd` para notificar na
  hora. Tire o título de `name` ou do último `ai-title` do transcript, lendo só essa chave.
- **Codex:** leia `~/.codex/state_5.sqlite` (`threads`, somente leitura) para lista e metadados. Derive
  andamento/ocioso do último `task_started`/`task_complete`/`turn_aborted` do rollout, e use PID vivo para saber se
  está encerrada. Para "aguardando aprovação" e notificação imediata, instale hooks (`PermissionRequest`, `Stop`,
  `Interrupt`, `SessionEnd`). Isso pede um passo manual de confiança em `/hooks`. Prefira hooks a `notify`, que só
  cobre o fim de turno.
- **Instalação de hooks:** acrescente entradas, porque já existem hooks do Orca nos dois CLIs. O script deve ser
  idempotente, rápido (o `SessionEnd` do Claude tem 1,5 s de orçamento) e nunca registrar o conteúdo das mensagens.
- **Riscos:** os formatos do SQLite e do rollout do Codex e o `~/.claude/sessions/*.json` não são interfaces
  estáveis. Isole cada leitor atrás de um adaptador e degrade para "estado desconhecido" se um campo sumir.
