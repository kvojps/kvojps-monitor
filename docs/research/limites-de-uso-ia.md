# Como ler os limites de uso de Claude e Codex localmente

Pesquisa feita em 2026-10-30 nesta máquina (Ubuntu 26.04, Claude Code 2.1.314, codex-cli 0.165.0, contas Claude **Pro** e ChatGPT **Plus**).

**Pergunta.** Como obter, sem custo e a partir desta máquina, o consumo e o horário de reinício da janela de sessão (~5h) e do limite semanal (inclusive semanais por modelo, se existirem) dos planos Claude (Pro/Max) e ChatGPT/Codex (Plus/Pro)? Dá para detectar o plano? Quais são os riscos?

**Resposta curta.** Há uma fonte local e oficial para cada lado:

- **Codex:** o `codex app-server` (JSON-RPC por stdio) tem o método `account/rateLimits/read`. Os logs `~/.codex/sessions/**/*.jsonl` também guardam o mesmo snapshot, sem precisar de rede.
- **Claude:** o JSON que o Claude Code passa ao script de **status line** traz `rate_limits.five_hour` e `rate_limits.seven_day`.

Os limites semanais por modelo e os detalhes extras do lado Claude só aparecem no endpoint não documentado `GET https://api.anthropic.com/api/oauth/usage`.

Nesta pesquisa, os números obtidos pelas fontes locais foram conferidos com chamadas reais aos dois serviços. Nenhum token foi impresso nem copiado.

---

## 1. Os limites em si (o que medir)

### Claude (Pro/Max)

- **Sessão de 5 h:** "The five-hour limit controls how much you can use Claude within a five-hour period". A janela começa na primeira mensagem e zera 5 h depois. Fonte: [support.claude.com — How do usage and length limits work](https://support.claude.com/en/articles/11647753-how-do-usage-and-length-limits-work).
- **Semanal:** "Weekly limits apply across all models, and some plans also have a separate weekly limit for [um modelo específico]". O reinício semanal acontece num horário fixo atribuído à conta (mesma fonte).
- **Interface de 1ª parte:** o comando `/usage` do Claude Code "Show session cost, plan usage limits, and activity stats" ([code.claude.com/docs/en/commands](https://code.claude.com/docs/en/commands)). O binário 2.1.314 tem os rótulos `Current session`, `Current week (all models)`, `Current week (Sonnet only)` e `Current week (${model.display_name})`. Ou seja, existem limites semanais por modelo, mas dependem do plano. Na conta Pro desta máquina, todos os buckets por modelo vieram `null` (ver §3.3).

### Codex (ChatGPT Plus/Pro)

- [learn.chatgpt.com/docs/pricing](https://learn.chatgpt.com/docs/pricing) mostra estimativas "per five-hour period for Plus" e diz: "Pro plans currently have no five-hour limit" e "Weekly limits may also apply".
- Para consultar, a página indica `/status` no CLI e o painel https://chatgpt.com/codex/settings/usage.
- Na prática, o backend devolve uma janela `primary` (300 min = 5 h) e uma `secondary` (10080 min = 7 dias) por bucket (`limit_id`). Pode haver vários buckets, como `codex` ou `premium`, e o schema prevê `normalModelSlug` para buckets por modelo (ver §2).

---

## 2. Codex: fontes locais

### 2.1 `codex app-server` → `account/rateLimits/read` (recomendado)

- **O que é:** segundo a doc oficial, "Codex app-server is the interface Codex uses to power rich clients (for example, the Codex VS Code extension)". O transporte padrão é stdio com JSON por linha (JSONL). O cliente manda um `initialize` e depois a notificação `initialized`. Fonte: [learn.chatgpt.com/docs/app-server](https://learn.chatgpt.com/docs/app-server). Ressalva da mesma página: "The app-server command and WebSocket transport are experimental and aren't supported for production workloads."
- **Contrato:** `codex app-server generate-json-schema --out DIR` gera o schema da versão instalada. Na 0.165.0 ele inclui:
  - requisições `account/rateLimits/read`, `account/read` e `account/usage/read`;
  - a notificação push `account/rateLimits/updated`.
- **Formato da resposta** (`GetAccountRateLimitsResponse`):
  - `rateLimits` (visão de bucket único, "backward-compatible") e `rateLimitsByLimitId` ("Multi-bucket view keyed by metered `limit_id`");
  - cada `RateLimitSnapshot` tem `limitId`, `limitName`, `normalModelSlug` ("Normal model whose display name ... describe this quota alias"), `primary`, `secondary`, `credits`, `planType` e `rateLimitReachedType`;
  - cada janela (`RateLimitWindow`) tem `usedPercent`, `windowDurationMins` e `resetsAt` (epoch em segundos).
- **Teste real nesta máquina** (script Python que abre `codex app-server` como subprocesso):

  ```json
  "rateLimitsByLimitId": {"codex": {
     "primary":   {"usedPercent": 0,  "windowDurationMins": 300,   "resetsAt": 1793417093},
     "secondary": {"usedPercent": 29, "windowDurationMins": 10080, "resetsAt": 1793814230},
     "planType": "plus", ...}}
  ```

  `account/read` devolveu `{"account": {"type": "chatgpt", "planType": "plus", ...}}`. A resposta também traz `rateLimitResetCredits`, por exemplo um "Full reset (Weekly + 5 hr)" disponível.
- **Por baixo:** o app-server usa o token OAuth de `~/.codex/auth.json` e consulta o backend do ChatGPT. O binário 0.165.0 tem as strings `/wham/usage` e `/api/codex/usage` ([código do backend-client](https://github.com/openai/codex/tree/main/codex-rs)). O app nunca toca no token: quem faz a requisição é o próprio binário oficial.

### 2.2 Logs de sessão `~/.codex/sessions/AAAA/MM/DD/rollout-*.jsonl` (offline)

- Cada evento `{"type":"event_msg","payload":{"type":"token_count","info":…,"rate_limits":{…}}}` traz o snapshot que o servidor mandou naquele turno, por exemplo:

  ```json
  "rate_limits":{"limit_id":"codex","primary":{"used_percent":96.0,"window_minutes":300,"resets_at":1793246064},
                 "secondary":{"used_percent":29.0,"window_minutes":10080,"resets_at":1793814231},
                 "credits":{...},"plan_type":"plus","rate_limit_reached_type":null}
  ```

- Nesta máquina há 1334 arquivos de rollout. Os `limit_id` encontrados foram `codex` (29.668 ocorrências) e `premium` (17 ocorrências, com `primary` e `secondary` nulos).
- **Limitação:** o snapshot só é tão novo quanto o último turno do Codex. Se ninguém usar o Codex, o valor fica velho (o `resets_at` pode já ter passado). Funciona bem como fallback offline ou cache. Os valores vêm dos headers `x-codex-*` da resposta da API (o binário tem, por exemplo, `x-codex-active-limit`, `x-codex-credits-*` e `x-codex-rate-limit-reached-type`). O formato é interno e não documentado.

### 2.3 `/status` no TUI do Codex

Mostra os mesmos limites de forma interativa ([learn.chatgpt.com/docs/pricing](https://learn.chatgpt.com/docs/pricing)). Não serve para automação.

---

## 3. Claude: fontes locais

### 3.1 JSON da status line (documentado, recomendado)

- [code.claude.com/docs/en/statusline](https://code.claude.com/docs/en/statusline) documenta, no JSON que o Claude Code envia via stdin ao script de status line:
  - `rate_limits.five_hour.used_percentage` e `rate_limits.seven_day.used_percentage` ("from 0 to 100");
  - `rate_limits.five_hour.resets_at` e `rate_limits.seven_day.resets_at` ("Unix epoch seconds").
- **Ressalvas** (mesma página): "appears only for claude.ai Pro, Max, and Ultra subscribers … only after the first API response in the session. Each window … may be independently absent, and Claude Code drops a window once its `resets_at` time passes."
- **Não traz** os semanais por modelo, só `five_hour` e `seven_day`.
- **Como usar no tray:** configurar `statusLine.command` em `~/.claude/settings.json` com um script que repassa o que precisa e grava num arquivo de cache, por exemplo `~/.cache/kvojps-monitor/claude.json`. O tray lê esse arquivo.
  - Custo zero e sem rede extra.
  - Só atualiza enquanto houver uma sessão interativa do Claude Code aberta.
  - Mexe na status line do usuário: se já existir uma, o script novo precisa encadeá-la.

### 3.2 `/usage` (TUI)

É a tela oficial com sessão, semana (todos os modelos) e semana por modelo ([commands](https://code.claude.com/docs/en/commands)). É interativa e não tem saída para máquina.

### 3.3 Endpoint `GET https://api.anthropic.com/api/oauth/usage` (não documentado)

- É o endpoint que o próprio Claude Code chama. No binário 2.1.314 aparecem a string `api/oauth/usage` (com variantes de query como `?at_wall=1&skip_spend=1`) e o header beta `oauth-2025-04-20`.
- **Autenticação:** `Authorization: Bearer <claudeAiOauth.accessToken de ~/.claude/.credentials.json>` mais `anthropic-beta: oauth-2025-04-20`.
- **Teste real nesta máquina:** HTTP 200 com:
  - `five_hour {utilization: 15.0, resets_at: "2026-10-31T02:30:00+00:00"}`;
  - `seven_day {utilization: 6.0, resets_at: "2026-11-05T14:00:00+00:00"}`;
  - os buckets por modelo e produto `seven_day_opus`, `seven_day_sonnet`, `seven_day_oauth_apps` e `seven_day_cowork`, todos `null` neste plano Pro;
  - `extra_usage`;
  - uma lista normalizada `limits[]` (`kind: session | weekly_all`, `percent`, `resets_at`, `severity`);
  - `seven_day_breakdown`, a fatia por superfície (Claude Code, chat, Cowork).
- **É a única fonte que traz os semanais por modelo.** Também consegue ler o estado sem precisar de uma sessão do Claude Code aberta.
- **Riscos:**
  - **Não documentado:** campos com codinomes (`tangelo`, `cedar_ember` …) mudam sem aviso.
  - **Token:** o access token expira (`expiresAt`). Renovar exige usar o `refreshToken` fora do binário oficial, o que é ainda mais frágil e pode invalidar a sessão do próprio CLI se houver rotação de refresh token. Uma alternativa segura é só ler o token atual e, se ele estiver expirado, cair para a fonte §3.1 ou §3.4.
  - **ToS:** ver §5.

### 3.4 Headers `anthropic-ratelimit-unified-*` e transcripts

- O binário lê headers de resposta como `anthropic-ratelimit-unified-5h-utilization`, `-5h-reset`, `-7d-utilization`, `-7d-reset`, `-status` e `-representative-claim`. Esses headers só chegam ao cliente que faz a requisição: um app externo só os veria fazendo inferência com o token OAuth, o que **não** é recomendado.
- Os transcripts `~/.claude/projects/**/*.jsonl` **não** guardam percentuais de limite. Só registram mensagens de erro com `"error":"rate_limit"` quando o limite estoura.
- `~/.claude/stats-cache.json` tem só contagem de tokens e mensagens por dia e modelo.

### 3.5 Ferramentas open-source

- [ccusage](https://github.com/ryoppippi/ccusage) "reads local usage data from coding agent CLIs" e gera relatórios diários, por sessão e por "blocks" (as janelas de 5 h). Também suporta Codex (`ccusage codex daily`).
- **Não lê o limite real.** Os blocos são inferidos dos logs locais, e `--token-limit max` usa "highest previous block as limit" ([ccusage.com/guide/blocks-reports](https://ccusage.com/guide/blocks-reports)). Serve para estimar custo e tokens, não para percentual oficial nem horário de reinício.

---

## 4. Detecção do plano

| Serviço | Fonte local | Valor nesta máquina |
|---|---|---|
| Claude | `~/.claude/.credentials.json` → `claudeAiOauth.subscriptionType` (e `rateLimitTier`) | `pro` / `default_claude_ai` |
| Claude | `GET https://api.anthropic.com/api/oauth/profile` (existe no binário; não testado) | — |
| Codex | `codex app-server` → `account/read` → `account.planType` (enum do schema: `free, go, plus, pro, prolite, promax, team, business, enterprise, edu, …`) | `plus` |
| Codex | `~/.codex/auth.json` → claim `https://api.openai.com/auth.chatgpt_plan_type` do `id_token` (JWT, só decodificar o payload); também `auth_mode: "chatgpt"` | `plus` |
| Codex | `plan_type` em cada `rate_limits` dos rollouts | `plus` |

Ler só esses campos não expõe segredos. Para isso, basta nunca registrar em log o arquivo inteiro nem os campos `*_token`.

---

## 5. Riscos (endpoints não documentados e ToS)

1. **Anthropic, uso do token OAuth fora do Claude Code.**
   - A página [Legal and compliance](https://code.claude.com/docs/en/legal-and-compliance) diz que o OAuth "is designed to support ordinary use of Claude Code and other native Anthropic applications" e que "developers may not collect, store, or intermediate Claude.ai credentials or session tokens", com "Anthropic reserves the right to take measures to enforce these restrictions … without prior notice".
   - Os [Consumer Terms](https://www.anthropic.com/legal/consumer-terms), Seção 3, proíbem "access the Services through automated or non-human means, whether through a bot, script, or otherwise", exceto via API key "or where we otherwise explicitly permit it".
   - Um tray pessoal que lê o próprio token para consultar `/api/oauth/usage` é uma área cinzenta. Não é inferência e não intermedeia ninguém, mas é acesso automatizado a um endpoint não documentado. O risco é baixo, mas não zero (bloqueio ou revogação de token).
   - O caminho da status line (§3.1) é explicitamente suportado e não tem esse risco.
2. **OpenAI.** O `codex app-server` é uma interface publicada pela própria OpenAI para clientes de terceiros. Usar o binário oficial (que faz a autenticação sozinho) é o caminho mais limpo. Os riscos são:
   - o rótulo "experimental": o schema pode mudar, por isso convém gerar e conferir o schema a cada versão;
   - chamar `/backend-api/wham/usage` direto com o token seria não documentado e evitável. Não consegui ler os [Terms of Use da OpenAI](https://openai.com/policies/row-terms-of-use/) (HTTP 403 nesta pesquisa), então a cláusula exata sobre acesso automatizado não foi verificada.
3. **Estabilidade de formato.**
   - Os logs `rollout-*.jsonl` e os headers `x-codex-*` são internos.
   - Os buckets do Claude têm nomes-código.
   - Por isso o parser deve ser tolerante (campos opcionais, `null`) e mostrar "desconhecido" em vez de quebrar.
4. **Frequência.** Consultar endpoints de rede a cada poucos segundos é desnecessário e chama atenção. Algo entre 1 e 5 min, com cache e backoff, basta. As fontes offline (status line e rollouts) podem ser lidas por inotify.
5. **Segredos.** Nunca logar nem persistir tokens. Ler `~/.claude/.credentials.json` e `~/.codex/auth.json` só em memória, e só os campos necessários.

---

## Recomendação

- **Codex:** usar `codex app-server` como subprocesso de longa duração.
  - Na partida, fazer `initialize`/`initialized`, depois `account/read` (plano) e `account/rateLimits/read`.
  - Depois, ouvir a notificação `account/rateLimits/updated`, com um poll lento como garantia.
  - Mostrar `primary` (5 h) e `secondary` (semana) para cada `rateLimitsByLimitId`.
  - Fallback offline: o último `token_count.rate_limits` em `~/.codex/sessions`.
  - É oficial, custo zero, e o app nunca toca no token.
- **Claude:** a fonte primária deve ser o **JSON da status line** (`rate_limits.five_hour` / `seven_day`), gravado em cache por um script `statusLine` instalado pelo app, que encadeia a status line existente se houver.
  - Como opção explícita, desligada por padrão e com aviso de ToS e de API não documentada, oferecer a consulta a `GET /api/oauth/usage` com o token local, só leitura e sem refresh. É o único jeito de obter os semanais por modelo e de atualizar sem uma sessão aberta.
- **Plano:** ler `subscriptionType` (Claude) e `account/read.planType` (Codex).
- **Não usar o ccusage** para limites, só para estimativas de tokens e custo, se isso for desejado.
