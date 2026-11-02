# Quais eventos reais alimentam a Faixa de bordo?

Type: grilling
Status: resolved
Labels: wayfinder:grilling
Blocked by: 02
Parent: [Aprimorar a identidade tech do Painel](../map.md)

## Question

Quais eventos existentes ou faltantes de cada Módulo permitem comunicar início, resultado, falha, pausa e retomada reais na Faixa de bordo? Definir o que conta como mudança relevante em Rede, Sistema e Agentes, como agrupar operações concorrentes por fonte e quais mensagens recebem anúncio não interruptivo, sem fabricar etapas nem confundir amostras contínuas com operações pontuais. Levantar o contrato atual no código e propor apenas as lacunas necessárias à comunicação aprovada, preservando privacidade e os estados canônicos.

Depende de [Como mostrar operações reais com mensagens de bordo?](02-mensagens-de-bordo.md). A resposta alimenta [Como consolidar as melhorias aprovadas na spec da v1?](04-consolidacao.md).

## Comments

- 01/11/2026: o usuário confirmou com “Concordo com tudo” as duas primeiras escolhas: mensagens de início apenas para atualizações solicitadas pelo usuário e retomadas após pausa; atualizações automáticas bem-sucedidas silenciosas, com falhas e recuperações na faixa. Em Sessões de agente, mostrar transições para Aguardando você, Turno concluído, Com falha e Encerrada, agrupadas quando simultâneas e sem conteúdo da conversa; Em andamento permanece no Módulo, sem anúncio a cada retomada. Restam os critérios de Rede e Sistema, concorrência e anúncios. Ticket segue claimed; sem alteração da spec ou do app.

## Answer

Em 01/11/2026, o usuário confirmou as duas rodadas com “Concordo com tudo”, aprovando os critérios abaixo. Esta resolução planeja o contrato; não altera a spec nem implementa eventos no app.

### Eventos aprovados

- **IA, GitHub e Trello:** mostrar início de atualizações solicitadas pelo usuário e de retomadas após pausa. Atualizações automáticas bem-sucedidas ficam silenciosas; falhas e recuperações reais aparecem na faixa. Em IA, identificar o Provedor de IA realmente consultado antes de agrupar no Módulo: não atribuir atividade a Claude com base no sinal de consulta Codex.
- **Sessões de agente:** comunicar transições para Aguardando você, Turno concluído, Com falha e Encerrada, agrupando mudanças simultâneas. Em andamento permanece no Módulo, sem anúncio a cada retomada. Usar o estado canônico Turno concluído, sem afirmar sucesso: ele também pode representar interrupção pelo usuário. Não incluir conversa, nome de sessão, projeto, branch ou motivo textual de espera.
- **Rede:** comunicar mudanças de conexão e VPN, e mudanças de categoria da Qualidade da rede e Segurança da rede. Não transformar latência, tráfego ou cada sondagem em mensagens. Não inferir troca de rede apenas por snapshots que continuam conectados.
- **Sistema:** comunicar somente entrada e saída dos alertas existentes de temperatura, disco e bateria. Não criar novos limites nem mensagens por amostra de CPU, RAM ou tráfego. Ausência normal de sensor ou bateria não é falha.
- **Operações pontuais:** incluir teste de velocidade e movimentação de cards do Trello, com início e resultado reais. Exemplo sanitizado: “Trello: movimentação concluída”, sem identificar card ou lista. Cancelamento tem mensagem própria e nunca equivale a sucesso.
- **Reabertura:** carregar o estado atual como referência; não reapresentar estados existentes como eventos novos ao reabrir o Painel. Não fabricar conclusões, etapas ou duração de operação com temporizadores.

### Concorrência, permanência e anúncios

- Um resumo por Módulo, agrupando operações simultâneas por fonte, sem carrossel. Priorizar falha, pausa e atenção antes de atividade e sucesso; preservar detalhes e ações nos Módulos.
- Manter a regra já aprovada em [Como mostrar operações reais com mensagens de bordo?](02-mensagens-de-bordo.md): operação em andamento enquanto real; resultado por 4 segundos; sem histórico. O temporizador só controla permanência visual.
- Anúncios agrupados e não interruptivos para resultados solicitados pelo usuário, falhas, recuperações e mudanças relevantes aprovadas acima. Inícios automáticos permanecem silenciosos para leitores de tela, inclusive quando a retomada tiver indicação visual. Sem anúncio a cada amostra ou repetição de estado.
- Não alterar as notificações do sistema previstas na spec. Os anúncios acessíveis da faixa são distintos dessas notificações; alertas de Sistema continuam restritos ao Painel.
- Mensagens contêm apenas fonte, evento e estado operacional, nunca credenciais ou conteúdo de prompts, respostas, commits ou cards.

### Contrato atual e lacunas necessárias

Levantamento local realizado pelo subagente de apoio; referências relativas à raiz do repositório:

- `src-tauri/src/snapshot.rs` e `src/lib/core.ts`: transporte por snapshots de Módulo, sem fluxo dedicado de operações. `updatedAt` representa atualização de dados, não identidade de operação. A implementação precisa distinguir uma nova transição de um snapshot repetido ou inicial.
- `src-tauri/src/github/mod.rs` e `src-tauri/src/trello/mod.rs`: há `refreshing`, `paused`, estado canônico e marcas de sucesso/falha. `refreshing=false` isolado não comprova sucesso; pausa também pode incluir espera após reconexão. Movimentação de card é comando separado e precisa de sinais próprios sanitizados de início e resultado.
- `src-tauri/src/ai/mod.rs` e `src-tauri/src/ai/claude.rs`: `refreshing` corresponde à consulta Codex, sem início equivalente para toda atividade Claude. A comunicação deve respeitar estados canônicos e a política existente de falhas; acrescentar sinais por Provedor de IA apenas onde necessários.
- `src-tauri/src/network/mod.rs` e `src-tauri/src/network/speed.rs`: conexão, VPN, qualidade, segurança e estados de teste de velocidade já existem. Faltam motivo explícito de cancelamento e correlação de operação no contrato publicado; resultado descartado por mudança de conexão ou suspensão não é sucesso.
- `src-tauri/src/system/mod.rs`: amostras contínuas e alertas existentes permitem detectar entrada e saída de atenção. Não há contrato geral de início, falha e recuperação da coleta; não acrescentá-lo para sustentar mensagens que ficaram fora da seleção aprovada.
- `src-tauri/src/agents/mod.rs`: identidade e estados de sessão permitem comparar transições. Snapshots novos não significam turnos novos; respeitar limitações de detecção por polling e hooks, sem inferir espera ou sucesso.

Essas lacunas entram como requisitos da proposta de consolidação, sem ampliar fontes de dados, privilégios ou coleta em segundo plano. A revisão integrada e seus critérios de aceite ficam em [Como consolidar as melhorias aprovadas na spec da v1?](04-consolidacao.md).
