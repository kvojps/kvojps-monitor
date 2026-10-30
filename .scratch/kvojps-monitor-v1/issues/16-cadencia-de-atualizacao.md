# Cadência de atualização dos Módulos com o Painel fechado

Type: grilling
Status: resolved
Blocked by: 12, 13, 15
Map: [kvojps-monitor v1](../map.md)

## Question

Com que intervalo o núcleo atualiza os Módulos IA (Codex via `codex app-server`, cache da status line do Claude), GitHub (calendário e Repo em destaque) e Trello, e o que muda quando o Painel está fechado, ao abrir o Painel (atualizar na hora?), sem rede, ou após suspensão/retomada? Há botão de atualizar manual? Rede e Agentes já estão definidos em [Definir rede saudável ou em risco e cadência do teste de velocidade](08-saude-da-rede.md), [Modo do tráfego por processo](14-trafego-por-processo.md) e [Quais eventos de Sessão de agente geram notificação](09-notificacoes-agentes.md); ponto de partida da pesquisa: calendário TTL ~1h, destaque 10–15 min, Trello 30–60 s.

## Answer

Ritmos por fonte (Rede e Agentes seguem o já decidido):

| Fonte | Painel fechado (ou Módulo não visível) | Módulo visível |
|---|---|---|
| Codex | `codex app-server` de longa duração ouvindo `account/rateLimits/updated` + consulta de garantia a cada 5 min | + consulta a cada 60 s |
| Claude | inotify no cache da status line (sem custo); opt-in `oauth/usage` a cada 5 min | igual |
| GitHub · calendário | 1 h | 1 h |
| GitHub · Repo em destaque | 15 min | 5 min |
| Trello | 5 min | 30 s |

- **Abrir o Painel / trocar de Módulo**: mostra o cache na hora; se o dado passou do intervalo "visível" do Módulo, atualiza em segundo plano com indicador discreto de "atualizando", sem tela de carregamento.
- **Atualizar manual**: botão ↻ no cabeçalho de IA, GitHub e Trello, atualiza só aquele Módulo, espera de 10 s entre cliques. Agentes e Rede sem ↻ (teste de velocidade mantém botão próprio).
- **Sem rede** (conectividade do NetworkManager): pausa GitHub, Trello, `oauth/usage` e consulta do Codex; Módulos mostram o último dado com a idade, sem faixa de erro. Ao voltar, atualiza tudo uma vez, escalonado.
- **Suspensão/retomada** (sinal do logind): ao retomar, espera conectividade, atualiza tudo uma vez escalonado e zera os relógios de cada fonte; reinicia subprocessos mortos (ex.: `codex app-server`).
- **Inicialização (login)**: ~10 s após iniciar, aguarda conectividade e atualiza uma fonte a cada ~3 s; depois cada uma segue seu ritmo.
- **Erros transitórios** (timeout, 5xx, 429): backoff 30 s → 1 → 2 → 4 → 8 → 15 min, respeitando `Retry-After`; mostra o dado antigo com a idade e, após 3 falhas seguidas, linha discreta "Falha ao atualizar há X". Sem notificação. 401/403 seguem [Fluxo de credenciais nas Configurações](15-fluxo-de-credenciais.md) (para de tentar).
- **Cache em disco**: último snapshot de cada Módulo (IA, GitHub, Trello, último teste de velocidade) em `~/.cache/kvojps-monitor/`, exibido com a idade até a primeira atualização após o login. Nunca contém credenciais.
- **Mover card no Trello**: otimista — o card muda de lista na hora; em falha volta à lista original com "Não foi possível mover"; em sucesso relê o quadro logo em seguida e zera o relógio de 30 s.
