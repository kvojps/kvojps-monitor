# Spec: kvojps-monitor v1

Nota de sanitização (ticket 40): contagens e horários do exemplo de repositório privado são sintéticos; não representam atividade real do mantenedor.

Status: ready-for-agent
Origem: mapa [kvojps-monitor v1](../kvojps-monitor-v1/map.md) — cada decisão abaixo vem de um ticket resolvido lá; em caso de dúvida, o ticket é a fonte.
Vocabulário: [GLOSSARY.md](../../GLOSSARY.md). Decisão arquitetural: [ADR 0001](../../docs/adr/0001-indicador-sem-extensao-gnome.md).

## Problem Statement

Para saber como está o meu dia de trabalho eu abro vários apps e abas: `/usage` no Claude Code e no Codex para ver quanto resta dos Limites de uso, os terminais para ver se alguma Sessão de agente parou esperando por mim, o perfil do GitHub para ver minhas contribuições, o Trello para ver e mover cards, e ferramentas soltas para saber se a rede em que estou é confiável e rápida e se a máquina está sobrecarregada. Isso me tira do foco, e eu frequentemente descubro tarde que um agente estava parado aguardando aprovação ou que estou numa rede aberta.

## Solution

Um app pessoal de bandeja para Ubuntu/GNOME/Wayland, gratuito e totalmente local. Um **Indicador** na barra superior, que inicia com o login, abre um **Painel** compacto com seis **Módulos** — IA, Agentes, GitHub, Trello, Rede, Sistema — e as **Configurações**. O app coleta tudo em segundo plano, me notifica só do que exige ação (Sessão de agente Aguardando você ou Com falha, turno longo concluído, rede em Risco) e marca o Indicador quando algo precisa de mim. Nenhum backend hospedado: só APIs gratuitas com as minhas próprias credenciais, guardadas no GNOME Keyring. Distribuído publicamente via GitHub Releases como `.deb` e AppImage.

## User Stories

### Indicador e Painel

1. Como usuário, quero um Indicador na barra superior do GNOME que inicie com o login, para ter o app sempre à mão sem abri-lo manualmente.
2. Como usuário, quero que o clique simples no Indicador abra um menu com "Abrir painel", e que o duplo clique ou o clique do meio abram o Painel direto, para chegar aos dados em um gesto.
3. Como usuário, quero um Painel compacto (~440×600), sem decoração, de cantos arredondados, para consultar tudo sem uma janela pesada.
4. Como usuário, quero que o Painel feche com Esc e ao perder o foco, para que ele se comporte como um popup de bandeja.
5. Como usuário, quero que o Painel abra no último Módulo que visitei, para retomar de onde parei.
6. Como usuário, quero um trilho vertical de ícones (IA, Agentes, GitHub, Trello, Rede, Sistema) com Configurações no rodapé, e um Módulo em detalhe por vez, para navegar sem rolar por tudo.
7. Como usuário, quero no cabeçalho de cada Módulo o nome dele e a hora da última atualização, para saber se o dado é recente.
8. Como usuário, quero que o Indicador ganhe um ponto de acento enquanto alguma Sessão de agente estiver Aguardando você, e que o ícone de Agentes no trilho espelhe esse ponto, para perceber de relance que algo me espera.
9. Como usuário, quero um badge discreto no Indicador quando a Segurança da rede estiver em Risco, para não ficar numa rede perigosa sem saber.
10. Como usuário, quero que abrir o app de novo enquanto ele já roda só abra o Painel, para nunca ter duas instâncias.
11. Como usuário, quero visual escuro por padrão seguindo o tema do sistema (claro também suportado), acento verde-água, números em monoespaçada tabular e cores semânticas só para estados, para uma leitura limpa e técnica.
12. Como usuário, quero que o Painel mostre o dado em cache imediatamente ao abrir ou trocar de Módulo e atualize em segundo plano com um indicador discreto, sem tela de carregamento.

### Módulo IA

13. Como usuário de Claude Pro/Max, quero ver o % usado e o horário de reinício da janela de 5 h e da semanal do Claude, para dosar meu uso.
14. Como usuário de ChatGPT Plus/Pro, quero ver o % usado e o reinício da janela de 5 h e da semanal do Codex, para dosar meu uso.
15. Como usuário, quero ver o plano detectado de cada Provedor de IA, para saber a que limites estou sujeito.
16. Como usuário, quero ver de que fonte e com que idade veio cada leitura, para confiar no número.
17. Como usuário, quero conectar o app ao Claude Code por um botão "Conectar ao Claude Code" que instala um wrapper de status line encadeando a minha status line existente, para alimentar os limites sem perder o que já uso.
18. Como usuário, quero "Desconectar" restaure exatamente a status line original (ou remova a chave se não havia), para desfazer sem resíduo.
19. Como usuário, quero ser avisado com "Status line desconectada · Reconectar" se outra ferramenta trocar a minha status line, sem que o app a reescreva sozinho.
20. Como usuário, quero opcionalmente ligar a leitura via endpoint não documentado `oauth/usage`, com aviso claro de risco nos termos, para ter limites sempre atualizados e semanais por modelo se eu aceitar o risco.
21. Como usuário, quero que um token do Claude expirado no opt-in mostre "Token do Claude expirado · abra o Claude Code" e volte à status line, sem que o app renove o token.
22. Como usuário, quero que uma leitura antiga com janela ainda válida mostre o % com "atualizado há X", que uma janela já reiniciada mostre "Reiniciado às HH:MM · sem leitura nova" sem barra, e que o app nunca assuma 0% sem leitura.
23. Como usuário, quero um estado vazio explicativo quando não houver leitura ("Abra uma sessão do Claude Code para ler os limites") ou quando um CLI não estiver instalado ("Codex não encontrado · instale e faça login no CLI").
24. Como usuário, quero um botão ↻ para atualizar o Módulo IA na hora.

### Módulo Agentes

25. Como usuário, quero ver minhas Sessões de agente locais do Claude Code e do Codex, cada uma com Estado da sessão (cor), projeto (pasta do cwd), nome, detalhe e tempo, para saber o que está rodando.
26. Como usuário, quero que a sessão Aguardando você fique destacada no topo, para agir nela primeiro.
27. Como usuário, quero que as sessões Encerradas fiquem numa seção recolhida por até 2 h (no máximo 5), para lembrar o que terminou sem poluir a lista.
28. Como usuário, quero uma notificação nativa quando uma sessão entrar em Aguardando você ("Aguardando aprovação: <ferramenta>" ou "Aguardando sua resposta").
29. Como usuário, quero uma notificação quando um turno terminar Com falha.
30. Como usuário, quero uma notificação de Turno concluído só quando o turno durou ≥ 30 s ("Turno concluído em 4 min 12 s"), para não ser incomodado por respostas rápidas.
31. Como usuário, quero que interrupção pelo usuário e sessões Encerradas nunca notifiquem.
32. Como usuário, quero no máximo uma notificação viva por sessão (a nova substitui a anterior) e que 3+ eventos em 10 s virem uma notificação agrupada ("3 sessões aguardando você").
33. Como usuário, quero que as notificações nunca contenham trechos de prompt ou resposta, para não vazar conteúdo na tela.
34. Como usuário, quero que clicar numa notificação abra o Painel no Módulo Agentes com a sessão destacada.
35. Como usuário, quero ligar "Detecção instantânea" nas Configurações, que instala hooks no Claude Code e no Codex de forma aditiva, sem tocar nos hooks de outros apps, e me mostra o passo de confiança em `/hooks` do Codex.
36. Como usuário, quero que, sem hooks, o app ainda detecte todos os estados do Claude por polling e Turno concluído/Com falha do Codex, indicando que "Aguardando" do Codex está indisponível.
37. Como usuário, quero três interruptores de notificação (Aguardando, Turno concluído, Falha), todos ligados por padrão.

### Módulo GitHub

38. Como usuário, quero ver o calendário de contribuições do ano corrente igual ao do meu perfil, com o total, incluindo privadas como contagem anônima.
39. Como usuário, quero ver o Repo em destaque — o repositório com mais commits meus na branch padrão nos últimos 7 dias — com "42 commits em 7 dias · 240 no total".
40. Como usuário, quero ver a última atividade no Repo em destaque como meu último commit, em tempo relativo, com o título truncado em uma linha.
41. Como usuário, quero "Sem commits nos últimos 7 dias" quando não houver destaque, sem um substituto.
42. Como usuário, quero que o app use meu `gh` já autenticado sem copiar o token, e que, sem `gh`, me ofereça "Rodar `gh auth login`" ou "Colar token clássico".
43. Como usuário, quero um botão ↻ para atualizar o Módulo GitHub.

### Módulo Trello

44. Como usuário, quero ver as listas do meu quadro escolhido em colunas, com cada card mostrando título (até 2 linhas), etiquetas como pontos coloridos e data de entrega (cor de atenção se vencida).
45. Como usuário, quero mover um card clicando nele e depois na lista de destino, vendo a mudança na hora, e voltando ao lugar com "Não foi possível mover" se falhar.
46. Como usuário, quero abrir um card no navegador com duplo clique.
47. Como usuário, quero um passo a passo numa tela só para criar key, gerar token e escolher o quadro.
48. Como usuário, quero um botão ↻ para atualizar o Módulo Trello.

### Módulo Rede

49. Como usuário, quero ver a Segurança da rede como Sem alertas / Atenção / Risco (nunca "Segura"), com o motivo.
50. Como usuário, quero Risco para rede aberta (sem OWE), WEP, WPA com TKIP e DNS respondendo a nomes inexistentes; Atenção para portal cativo e MAC do gateway mudando na mesma conexão.
51. Como usuário, quero uma notificação só quando a Segurança entrar em Risco, uma vez por conexão.
52. Como usuário, quero ver a Qualidade da rede como Boa / Regular / Ruim com latência, jitter e perda até o gateway e a internet, e a última velocidade com idade.
53. Como usuário, quero rodar um teste de velocidade completo sob demanda, e um teste leve automático ao conectar e a cada 6 h, desligável e nunca em conexão limitada (metered).
54. Como usuário, quero um gráfico de tráfego total em tempo real.
55. Como usuário, quero os 5 apps que mais usam rede agora (↓/↑, agrupados por nome) e uma linha "Outros" explicando o que não é atribuível.
56. Como usuário, quero etiquetas "Cabeada" e "VPN ativa" e "Sem conexão" quando aplicável.

### Módulo Sistema

57. Como usuário, quero ver o uso total de CPU com um mini‑gráfico dos últimos ~60 s e a temperatura quando houver sensor.
58. Como usuário, quero ver RAM usada/total (pelo critério do `free`) e swap como linha secundária se houver.
59. Como usuário, quero uma barra por disco real montado com usado/total e livre.
60. Como usuário de notebook, quero ver bateria (%, estado, tempo restante); num desktop sem bateria, o bloco some.
61. Como usuário, quero cor de atenção para disco > 90%, bateria < 15% e temperatura alta, sem notificações.

### Configurações e primeira execução

62. Como novo usuário, quero uma tela de boas-vindas única na primeira abertura listando os Módulos com status (detectado / precisa configurar / não precisa) e atalhos para configurar cada um.
63. Como usuário, quero que um Módulo sem credencial siga visível com estado vazio e botão "Configurar".
64. Como usuário, quero que credenciais sejam validadas antes de salvar, com erro junto do campo ("Token inválido", "Falta o escopo repo", "Sem conexão, tente de novo") e "Conectado como <login>" no sucesso.
65. Como usuário, quero que credenciais fiquem só no GNOME Keyring, e que sem keyring o app recuse salvar e explique o porquê.
66. Como usuário, quero que uma credencial inválida em uso deixe o último dado esmaecido com a idade e uma faixa "Credencial inválida · Reconectar", sem notificação, e que o app pare de tentar até eu corrigi-la.
67. Como usuário, quero desligar o início com o login.
68. Como usuário, quero desligar o teste de velocidade automático e a checagem de nova versão.
69. Como usuário, quero ver uma linha discreta nas Configurações quando houver nova versão, com link para o release.
70. Como usuário, quero um botão "Abrir pasta de logs" para diagnosticar problemas.
71. Como usuário, quero desconectar Trello, Claude Code (status line) e hooks, desfazendo o que o app instalou.

### Comportamento em segundo plano

72. Como usuário, quero que o app pause chamadas de rede quando estiver sem conexão e mostre o último dado com idade, sem faixa de erro, atualizando tudo de forma escalonada ao voltar.
73. Como usuário, quero que, após suspender/retomar, o app atualize tudo uma vez e reinicie subprocessos mortos.
74. Como usuário, quero que falhas transitórias usem backoff até 15 min e só mostrem "Falha ao atualizar há X" após 3 falhas seguidas, sem notificação.
75. Como usuário, quero ver o último dado conhecido logo após o login, vindo de um cache em disco que nunca contém credenciais.

### Instalação e release

76. Como usuário, quero instalar por `.deb` ou AppImage baixados do GitHub Releases e verificar com `sha256sum -c` e `gh attestation verify`.
77. Como usuário, quero um README em pt-BR com requisitos (Ubuntu 24.04+, GNOME, extensão AppIndicator), instalação, primeiros passos por Módulo, aviso do opt-in `oauth/usage`, privacidade (hosts acessados) e desinstalação.
78. Como mantenedor, quero que um push de tag `v*` gere um release em rascunho com `.deb`, AppImage, `SHA256SUMS`, attestations e notas automáticas, e que o CI falhe se a tag não bater com a versão do projeto.
79. Como mantenedor, quero CI em push na `main` e em PRs com formatação, lint, testes do núcleo e da UI e build sem empacotar.

## Implementation Decisions

### Stack e processos
- **Tauri v2**: núcleo em Rust (coleta, estado, keyring, notificações, autostart) + UI em **Svelte 5 + TypeScript + Vite**, CSS próprio com tokens de design (sem Tailwind nem kit de componentes).
- **Um único processo de usuário, sem privilégio**. O núcleo detém todo o estado e chama `codex app-server` (longa duração) e `claude agents --json` como subprocessos. Sem serviço auxiliar privilegiado, sem `postinst` com capabilities.
- **Instância única**: um segundo lançamento sinaliza a instância viva para abrir o Painel e sai.
- **Núcleo → UI**: o núcleo empurra **um snapshot por Módulo** via eventos Tauri quando algo muda; a UI só renderiza e envia **comandos** pontuais (mover card, rodar teste de velocidade, atualizar Módulo, salvar/validar credencial, conectar/desconectar status line, ligar/desligar hooks, mudar preferências). A UI também informa ao núcleo qual Módulo está visível (a coleta de alguns depende disso).
- **Indicador** (ADR 0001): só bandeja via crate `ksni`. Clique simples → menu com "Abrir painel"; duplo clique / clique do meio → Painel. Estados do ícone: normal, ponto de acento (sessão Aguardando você), badge de Risco de rede. Sem extensão do GNOME Shell.
- **Painel**: janela sem decoração posicionada pelo Mutter; fecha com Esc e ao perder o foco; abre no último Módulo visitado (lembrado entre execuções), exceto via clique em notificação (Módulo Agentes com a sessão destacada). Ao fechar, a webview é ocultada; **medir a memória no primeiro esqueleto** e, se passar de ~150 MB em repouso, destruir a webview ao fechar.
- **Autostart**: ligado na primeira execução, desligável; entrada `.desktop` em `~/.config/autostart/`; no AppImage, reescrita a cada execução com o caminho atual.
- **Segredos**: GNOME Keyring via Secret Service (crate `keyring`). Sem keyring: recusa salvar e explica. Nunca texto puro.
- **Cada fonte atrás de um adapter** próprio: formatos de `~/.claude`, `~/.codex`, SQLite do Codex e rollouts não são interfaces estáveis.
- **Textos da UI** num único arquivo de strings, em pt-BR, sem i18n.

### Ritmo de coleta (núcleo)

| Fonte | Painel fechado / Módulo não visível | Módulo visível |
|---|---|---|
| Codex (limites) | app-server ouvindo `account/rateLimits/updated` + consulta de garantia a cada 5 min | + consulta a cada 60 s |
| Claude (limites) | inotify no cache da status line; opt-in `oauth/usage` a cada 5 min | igual |
| Claude (sessões) | hooks se ligados; senão `claude agents --json` a cada ~2 s | igual |
| Codex (sessões) | hooks se ligados + SQLite/rollouts | igual |
| GitHub · calendário | 1 h | 1 h |
| GitHub · Repo em destaque | 15 min | 5 min |
| Trello | 5 min | 30 s |
| Rede · ping gateway e internet | 5 s, sempre | 5 s |
| Rede · DNS e portal | ao conectar e a cada 10 min | igual |
| Rede · Segurança Wi‑Fi, MAC do gateway | eventos do NetworkManager | igual |
| Rede · velocidade leve | ao conectar e a cada 6 h (se ligado, não metered) | igual |
| Rede · tráfego total | 1 s, sempre | 1 s |
| Rede · por processo | não coleta | 2 s |
| Sistema · CPU, RAM, temperatura | não coleta | 2 s |
| Sistema · disco | não coleta | 30 s |
| Sistema · bateria | não coleta | sinal do UPower |

- **Abrir Painel / trocar Módulo**: mostra cache; se o dado passou do intervalo "visível", atualiza em segundo plano com indicador discreto.
- **↻ manual** em IA, GitHub e Trello, por Módulo, com espera de 10 s entre cliques. Agentes, Rede e Sistema sem ↻.
- **Sem rede** (conectividade do NetworkManager): pausa GitHub, Trello, `oauth/usage` e consulta do Codex; mostra último dado com idade, sem faixa de erro. Ao voltar, atualiza tudo uma vez, escalonado.
- **Suspensão/retomada** (sinal do logind): espera conectividade, atualiza tudo escalonado, zera relógios, reinicia subprocessos mortos.
- **Login**: ~10 s após iniciar, aguarda conectividade e atualiza uma fonte a cada ~3 s.
- **Erros transitórios** (timeout, 5xx, 429): backoff 30 s → 1 → 2 → 4 → 8 → 15 min, respeitando `Retry-After`; após 3 falhas seguidas, linha "Falha ao atualizar há X". Sem notificação.
- **401/403**: para de tentar até a credencial mudar; faixa "Credencial inválida · Reconectar"; dado antigo esmaecido com idade.
- **Cache em disco** em `~/.cache/kvojps-monitor/`: último snapshot de IA, GitHub, Trello e último teste de velocidade; nunca credenciais. Sistema não tem cache (sempre ao vivo).

### Módulo IA
- **Codex**: `codex app-server` (JSON-RPC via stdio) — `account/read` (detecção e `planType`) e `account/rateLimits/read` (janelas de 5 h e semanal, por bucket). Fallback: snapshots `rate_limits` nos rollouts `~/.codex/sessions/**/rollout-*.jsonl`.
- **Claude, fonte padrão — status line**: botão "Conectar ao Claude Code" (Configurações → IA e boas-vindas). O app grava um wrapper em `~/.local/share/kvojps-monitor/claude-statusline` e aponta `statusLine.command` do `~/.claude/settings.json` do usuário para ele; se havia comando, o wrapper o encadeia (repassa stdin, devolve a saída dele) e só copia `rate_limits` (`five_hour`, `seven_day`) para o cache. O comando original fica guardado na configuração do app; "Desconectar" o restaura (ou remove a chave). Se o `statusLine` for trocado por terceiros: "Status line desconectada" + reconectar, nunca reescreve sozinho. `statusLine` de projetos não alimenta o cache (aceito). Nunca altera `settings.json` sem o clique.
- **Claude, opt-in `oauth/usage`** (desligado por padrão, aviso de API não documentada e zona cinzenta nos termos): `GET api.anthropic.com/api/oauth/usage` com o token OAuth de `~/.claude/.credentials.json` lido só em memória e header `anthropic-beta: oauth-2025-04-20`; a cada 5 min com backoff; **nunca faz refresh do token**; 401 → volta à status line e mostra "Token do Claude expirado · abra o Claude Code". Semanais por modelo só quando não-nulos; campos desconhecidos ignorados. Com as duas fontes, vale a leitura mais recente de cada janela.
- **Plano**: Claude `subscriptionType` em `~/.claude/.credentials.json`; Codex `planType`. Suportar Claude Pro/Max e ChatGPT Plus/Pro.
- **Detecção**: Codex = `codex` no PATH + `account/read` responde. Claude = `claude` no PATH + credenciais locais.
- **Dado desatualizado** (Claude e fallback do Codex): (1) janela ainda válida → % normal + "atualizado há X"; (2) janela já reiniciou → "Reiniciado às HH:MM · sem leitura nova", sem barra, **nunca assume 0%**; (3) nenhuma leitura → estado vazio explicativo.
- Sem notificação nem marca no Indicador para limites na v1.

### Módulo Agentes
- **Estados da sessão**: Em andamento, Aguardando você, Turno concluído (inclui interrupção), Com falha, Encerrada.
- **Claude Code**: `claude agents --json --all` — `status` `busy`/`waiting` (com `waitingFor`)/`idle`; background `state` working/blocked/done/failed/stopped. Polling a cada ~2 s sem hooks; com hooks, continua como garantia para crash/interrupção.
- **Codex**: SQLite `~/.codex/state_5.sqlite` (read-only; tabela `threads`: cwd, título, criação, última atualização, branch) + último evento do rollout (`task_started`/`task_complete`/`turn_aborted`) + processo vivo. Sem hooks, "Aguardando" do Codex aparece como indisponível no Módulo.
- **Hooks (opt-in "Detecção instantânea")**: instalados de forma **aditiva** em `~/.claude/settings.json` e `~/.codex/hooks.json`, sem sobrescrever hooks de terceiros (ex.: Orca); remoção desfaz só o que o app adicionou; a UI mostra o passo de confiança em `/hooks` do Codex. Claude: `UserPromptSubmit`, `PermissionRequest`, `Notification`, `Stop`, `StopFailure`, `SessionEnd`. Codex: `SessionStart`, `SessionEnd`, `UserPromptSubmit`, `PermissionRequest`, `PostToolUse`, `Stop`, `Interrupt`. `PostToolUse` devolve a sessão a Em andamento após a ferramenta aprovada executar.
- **Canal hook → núcleo**: o hook executa o próprio binário em modo `hook <fonte> <evento>`; ele lê o JSON do stdin, extrai só metadados (id da sessão, estado, nome da ferramenta, cwd — nunca conteúdo) e envia a um socket Unix em `$XDG_RUNTIME_DIR/kvojps-monitor.sock`. App fora do ar → descarta em silêncio. Sempre sai com código 0 em < 200 ms.
- **Lista**: sessões vivas, a Aguardando você destacada no topo; Encerradas numa seção recolhida por 2 h após encerrar, no máximo 5, só as vistas vivas pelo app, não persistidas entre execuções.
- **Notificações**: Aguardando você e Com falha sempre; Turno concluído só se o turno durou ≥ 30 s; Encerrada e interrupção nunca. Título `<Claude Code | Codex> · <projeto>` [`· <nome>`]; corpo "Aguardando aprovação: <ferramenta>" / "Aguardando sua resposta" / "Turno concluído em 4 min 12 s" / "Turno terminou com erro". Uma notificação viva por sessão (substitui); ≥ 3 eventos em 10 s → agrupada. Não perturbe do GNOME silencia sem lógica própria. Clique → Painel no Módulo Agentes com a sessão destacada. Três interruptores por tipo, ligados por padrão, sem separação por provedor.

### Módulo GitHub
- **Credencial**: `gh auth token` a cada atualização (nada copiado). Fallback: token clássico com escopo `repo` colado e guardado no keyring. Configurações mostram "via gh · <login>" ou "token próprio".
- **Calendário**: GraphQL `viewer.contributionsCollection(from: 1º jan, to: 31 dez).contributionCalendar` do ano corrente; privadas como contagem anônima (`restrictedContributionsCount`).
- **Repo em destaque**: `viewer.repositories` (afiliação dono/colaborador/membro de org, os 20 com push mais recente, forks incluídos) + `history(author: {id}, since:)` na branch padrão; janela móvel de 7×24 h; empate → commit mais recente. Exibe commits na janela e total ("42 commits em 7 dias · 240 no total"), último commit (tempo relativo + título truncado). Não usa `pushedAt`. Vazio: "Sem commits nos últimos 7 dias". Sem `read:user`.
- **Validação ao salvar token**: `viewer { login }` + escopo `repo`.

### Módulo Trello
- **Credenciais**: key do usuário (criada em trello.com/apps/admin) + token (`scope=read,write`, `expiration=never`) colado manualmente; ambos no keyring; sem captura via `return_url`. Fluxo numa tela: key → "Gerar token" (abre a autorização com a key preenchida) → seletor de quadro com os quadros abertos. Validação: `GET /1/members/me` + listagem de quadros. "Desconectar" apaga key e token.
- **Leitura**: `GET /1/boards/{id}/lists?cards=open`. Polling (sem webhooks). Limites da API (300 req/10 s por key, 100/10 s por token) folgados.
- **Card**: título (até 2 linhas), etiquetas como pontos coloridos, data de entrega (atenção se vencida). Duplo clique abre o card no navegador. Sem membros, checklists, capas.
- **Mover**: clicar no card e depois na lista de destino; `PUT /1/cards/{id}` com `idList`; otimista — muda na hora, em falha volta com "Não foi possível mover", em sucesso relê o quadro e zera o relógio de 30 s.
- Auth isolada no adapter para migrar a OAuth 2.0 depois.

### Módulo Rede
- **Dois eixos independentes**: Segurança da rede (Sem alertas / Atenção / Risco, nunca "Segura") e Qualidade da rede (Boa / Regular / Ruim + números).
- **Segurança**: flags do AP e perfil ativo via NetworkManager (D-Bus). **Risco**: aberta sem OWE, WEP, WPA com TKIP, DNS respondendo a nome aleatório inexistente (NXDOMAIN esperado). **Atenção**: portal cativo (connectivity check do NM), MAC do gateway mudou na mesma conexão. Cabeada: mostra "Cabeada", valem DNS, portal e MAC. VPN: Segurança avalia o link físico; etiqueta "VPN ativa".
- **Qualidade**: ICMP sem root via ping socket (fallback: tempo de conexão TCP 443) ao gateway e à internet a cada 5 s, sempre; janela móvel de 2 min. **Ruim**: perda > 5%, latência mediana até a internet > 300 ms, sem internet com gateway respondendo, ou descida < 2 Mbps. **Regular**: perda > 1%, internet > 100 ms, gateway > 30 ms, jitter > 30 ms, ou descida < 10 Mbps. **Boa**: o resto. Velocidade só pesa se medida há < 6 h na rede atual; exibida sempre com idade. Com VPN, Qualidade mede como sai de fato.
- **Velocidade**: Cloudflare (speed.cloudflare.com, HTTPS). Completo sob demanda (botão); leve automático (~25 MB ↓, ~10 MB ↑) ao conectar e a cada 6 h, desligável, nunca em conexão metered. Último resultado em cache.
- **Tráfego total**: delta de `/proc/net/dev` por segundo, sempre.
- **Por processo**: contadores TCP por conexão do kernel (`sock_diag`/`tcp_info`), só processos do próprio usuário; top 5 agrupados por nome do app, taxa ↓/↑ média dos últimos ~5 s; linha "Outros" = total da interface − atribuído, com dica "UDP/QUIC, processos do sistema e conexões curtas". Coleta só com o Módulo Rede visível. Não pesa na Qualidade.
- **Notificação**: só ao entrar em Risco, uma vez por conexão. Badge no Indicador só em Risco. Qualidade não notifica.
- **Sem conexão**: os dois eixos "Sem conexão"; último teste visível com idade; sem badge.

### Módulo Sistema
- **CPU**: uso total de `/proc/stat` + mini‑gráfico de ~60 s (começa vazio ao abrir); temperatura via hwmon (`k10temp`/`coretemp`), linha some sem sensor. Sem uso por núcleo.
- **RAM**: usada = total − `MemAvailable`; swap como linha secundária só se configurado.
- **Discos**: sistemas de arquivos reais de `/proc/self/mountinfo` + `statvfs`, excluindo tmpfs, squashfs/snap, overlay, efivarfs e `/boot*`; uma barra por disco.
- **Bateria**: UPower `DisplayDevice` (%, estado, tempo restante quando informado); bloco oculto sem bateria.
- **Atenção** (só cor no Painel): disco > 90%, bateria < 15%, temperatura ≥ `temp*_crit` do sensor ou 90 °C. Sem notificação nem marca no Indicador.

### Configurações e primeira execução
- **Seções**: Geral (início com o login, checagem de nova versão, pasta de logs), IA (Conectar/Desconectar Claude Code, opt-in `oauth/usage` com aviso, status do Codex), Agentes (Detecção instantânea, três interruptores de notificação), GitHub (fonte da credencial), Trello (passo a passo, quadro, desconectar), Rede (teste de velocidade automático).
- **Boas-vindas**: tela única só na primeira abertura, com status por Módulo e atalhos de configuração (inclui "Conectar ao Claude Code").
- **Módulo sem credencial**: visível no trilho, estado vazio com "Configurar".
- **Validação antes de gravar**; falha não salva nada e mostra o erro junto do campo.

### Logs
- Arquivo em `~/.local/state/kvojps-monitor/`, rotação de 3 arquivos de 1 MB, nível `info`. Registra só fonte, evento, status HTTP e duração — **nunca** credenciais, prompts, respostas nem títulos de commits/cards/sessões.

### Atualização do app
- Checagem diária da API de releases do GitHub (ligada por padrão, desligável); linha discreta nas Configurações com link; sem notificação, sem auto‑update.

### Release e repositório
- Repo público `Kvojps/kvojps-monitor`, licença MIT. `.scratch/` é versionado como acompanhamento do planejamento e apagado pelo usuário quando a v1 estiver finalizada; branches `research/*` e `prototype/*` nunca sobem.
- **SemVer** a partir de `0.1.0`; `tauri.conf.json` é a fonte da verdade e o CI confere tag ↔ `tauri.conf.json` ↔ `Cargo.toml` ↔ `package.json`.
- **CI** (push na `main` e PRs): `cargo fmt --check`, `clippy`, `cargo test`, `svelte-check`, testes da UI, build sem empacotar.
- **Release** (tag `v*`): reaproveita o CI; `tauri-action` em `ubuntu-24.04` gera `.deb` + AppImage; publica `SHA256SUMS` e GitHub artifact attestations (sem GPG); release em **rascunho** com notas automáticas, publicado à mão. Sem `CHANGELOG.md`.
- **Suporte declarado**: Ubuntu 24.04+ com GNOME e extensão AppIndicator; outras distros "pode funcionar". Sem Flatpak/Snap.
- **README** (pt-BR, enxuto, guias longos em `docs/`): o que é + captura do Painel; requisitos e como ativar a extensão AppIndicator; instalação `.deb`/AppImage com verificação; primeiros passos por Módulo; aviso do opt-in `oauth/usage`; privacidade (tudo local, lista de hosts acessados: api.github.com, api.trello.com, api.anthropic.com só no opt-in, speed.cloudflare.com, os de conectividade/ping); desinstalação (desfazer status line, hooks e autostart); licença.

## Testing Decisions

- **Exceção aprovada no ticket 20 (31/10/2026)**: payloads de stdin dos hooks podem usar fixtures sanitizadas dos contratos oficiais, complementadas por capturas reais de polling e testes de integração do binário/socket; não é necessário instalar hooks ou iniciar sessões de IA somente para a captura.

- **Um bom teste** exercita comportamento externo: dado o que as fontes devolvem e o passar do tempo, o que o núcleo publica (snapshots, notificações, estado do Indicador, efeitos de comandos). Nunca testa estruturas internas, nomes de funções privadas ou ordem de chamadas.
- **Seam do núcleo (principal)**: toda E/S passa por portas injetáveis — executor de processos (`claude agents`, `codex app-server`, `gh`), cliente HTTP, sistema de arquivos (raiz falsa para `/proc`, hwmon, `~/.claude`, `~/.codex`, SQLite), D-Bus (NetworkManager, UPower, logind), keyring, notificador, socket de hooks e **relógio**. Os testes alimentam as portas com **saídas reais gravadas** (fixtures capturadas nesta máquina: JSON do `claude agents`, respostas do app-server, rollouts, respostas GraphQL/Trello/`oauth/usage`, amostras de `/proc`) e afirmam sobre o que sai do núcleo. Assim parsers de formatos instáveis, regras de estado (Estados da sessão, Segurança/Qualidade da rede, dado desatualizado da IA), anti-ruído de notificações, cadências, backoff, pausa sem rede e retomada ficam cobertos por um único seam, com relógio falso em vez de esperas reais.
- Casos que precisam de testes explícitos: encadeamento e restauração da status line; instalação/remoção aditiva de hooks preservando hooks de terceiros; nunca assumir 0% após reinício de janela; agrupamento ≥ 3 eventos em 10 s; Turno concluído < 30 s não notifica; Risco notifica uma vez por conexão; mover card otimista com reversão; 401 para de tentar; cache em disco sem credenciais; logs sem conteúdo; modo `hook` sai com 0 com o app fora do ar.
- **Seam da UI**: componentes Svelte testados com Vitest + Testing Library a partir de **fixtures de snapshot** por Módulo (cheio, vazio, desatualizado, credencial inválida, erro), e comandos emitidos verificados.
- **Sem E2E** da janela Tauri nem do Indicador; o comportamento do Indicador/Painel no GNOME é verificado manualmente.
- **Prior art**: nenhum — repo sem código. Os primeiros testes de cada adapter definem o padrão de fixtures.

## Out of Scope

- Descoberta de dispositivos na rede local (varredura ativa).
- Criar/editar cards, comentários e múltiplos quadros no Trello.
- Sessões de agente na nuvem (claude.ai/code, Codex cloud).
- Windows, macOS, Flatpak, Snap.
- Qualquer backend hospedado ou serviço pago.
- Alerta de Limite de uso a 80% e atalho global de teclado.
- Extensão do GNOME Shell (clique simples, Painel ancorado ao ícone) — [ADR 0001](../../docs/adr/0001-indicador-sem-extensao-gnome.md).
- Serviço eBPF privilegiado para tráfego por processo exato (UDP/QUIC, processos do sistema).
- PRs aguardando revisão e status de CI no Módulo GitHub.
- "SSID parecido" como sinal de Segurança da rede; uso de CPU por núcleo; escolha de discos nas Configurações.
- Atualização automática (`tauri-plugin-updater`), i18n, `CHANGELOG.md`, assinatura GPG.

## Further Notes

- **Ordem sugerida de fatias verticais** (para `to-tickets`): (1) esqueleto Tauri — Indicador, Painel, trilho, instância única, autostart, medição de memória; (2) Sistema; (3) Rede; (4) IA; (5) Agentes + notificações + hooks; (6) GitHub; (7) Trello; (8) primeira execução e Configurações completas; (9) CI e release (o CI pode nascer já na fatia 1).
- **Riscos conhecidos**: `codex app-server` é marcado "experimental"; `oauth/usage` é não documentado e pode mudar ou violar os termos (por isso opt-in); formatos de `~/.claude/sessions`, SQLite e rollouts do Codex podem mudar a qualquer versão (por isso adapters e fixtures); termos da OpenAI não foram verificados (403 na pesquisa).
- **Não verificado**: memória real do Tauri em repouso (medir na fatia 1).
- **Achados completos** das pesquisas ficam nas branches locais `research/*` e o protótipo visual em `prototype/painel-visual` (arquivo `prototype/painel-visual/index.html`) — referência visual para a UI.
