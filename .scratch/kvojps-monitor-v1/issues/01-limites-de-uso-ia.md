# Como ler os Limites de uso de Claude e Codex localmente

Type: research
Status: resolved
Blocked by: 
Map: [kvojps-monitor v1](../map.md)

## Question

Como obter, sem custo e a partir desta máquina, o consumo e o horário de reinício da janela de sessão (~5h) e do limite semanal (incl. semanais por modelo, se existirem) dos planos Claude (Pro/Max) e ChatGPT/Codex (Plus/Pro)? Fontes candidatas: arquivos locais (~/.claude, ~/.codex), comandos dos CLIs (/usage, /status), endpoints usados pelos próprios apps com o token OAuth local, ferramentas open-source existentes (ex.: ccusage). Dá para detectar o plano? Quais os riscos (endpoints não documentados, ToS)?

## Answer

**Codex**: fonte principal `codex app-server` (JSON-RPC via stdio, interface oficial para clientes de terceiros, marcada "experimental"): `account/rateLimits/read` devolve % usado e reset da janela de 5h e da semanal, por bucket; o CLI cuida do token. Testado aqui (Plus: 0% 5h, 29% semanal). Fallback offline: snapshots `rate_limits` (primary 300 min, secondary 10080 min) em `~/.codex/sessions/**/rollout-*.jsonl` — ficam velhos sem uso.

**Claude (documentado)**: o JSON entregue ao script de status line do Claude Code traz `rate_limits.five_hour` e `rate_limits.seven_day` (% e reset) — só Pro/Max/Ultra, só após a 1ª resposta numa sessão aberta, sem semanais por modelo.
**Claude (não documentado)**: `GET api.anthropic.com/api/oauth/usage` (o que `/usage` usa) com token OAuth local + header `anthropic-beta: oauth-2025-04-20` → `five_hour`, `seven_day`, semanais por modelo, `extra_usage`. Testado: 200 OK. Risco: Consumer Terms §3 e página Legal and compliance restringem uso de OAuth fora dos apps nativos e acesso automatizado → zona cinzenta; formatos internos instáveis.

**Plano**: Claude `subscriptionType` em `~/.claude/.credentials.json` (aqui `pro`); Codex `account/read.planType` ou claim `chatgpt_plan_type` (aqui `plus`). ccusage só estima por logs — não serve para limites reais. Termos da OpenAI não verificados (403).

**Recomendação da pesquisa**: Codex via app-server + fallback rollouts; Claude via cache do status line, `oauth/usage` só como opt-in desligado por padrão com aviso.

Achados completos: branch local `research/limites-de-uso-ia` (commit `c2e9616`), arquivo `research/limites-de-uso-ia.md`.
