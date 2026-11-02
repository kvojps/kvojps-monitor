# Definir prontidão e ciclo de abertura do Painel

Labels: wayfinder:grilling
Type: grilling
Status: resolved
Blocked by: none

## Question

Qual é o contrato de abertura do Painel: o que precisa estar pronto para encerrar a sequência, como os 1,2 segundo mínimos se combinam com esse preparo e como agir diante de falha, demora, fechamento antecipado ou nova solicitação de abertura?

Partir das preferências confirmadas nas Notes de [Abertura cinematográfica do Painel](../map.md). Esclarecer explicitamente se consultas à rede podem bloquear a abertura, o comportamento com cache ausente e como evitar que aparência ou conteúdo mudem abruptamente após a transição. Definir também a experiência com movimento reduzido.

Registrar na resolução a revisão proposta da história 12 da spec, que hoje exige abertura sem tela de carregamento, distinguindo preparo do Painel de atualização dos Módulos. A spec só deve ser alterada quando o contrato concreto estiver acordado com o usuário.

## Answer

Contrato acordado com o usuário em duas rodadas de perguntas, ambas confirmadas com “ok”.

- A sequência acontece dentro do Painel em toda abertura, inclusive na primeira após iniciar o app. O login automático continua silencioso.
- Prontidão significa estrutura do Painel, Âncora restaurada e dados locais disponíveis apresentados. Consultas à rede não bloqueiam a abertura: a atualização dos Módulos acontece em segundo plano.
- Sem cache, cada Módulo apresenta um estado explícito de carregamento, indisponibilidade ou configuração pendente, conforme o caso.
- A duração mínima é de 1,2 segundo desde a primeira apresentação visível da sequência, incluindo a transição. Se o preparo levar mais tempo, não se acrescenta outro intervalo mínimo ao final.
- “Preparando o Painel…” acompanha a restauração local; “Carregando Módulos…” aparece somente durante o preparo real dos Módulos. Se o preparo terminar antes do mínimo, o visual permanece sem mensagens que aleguem atividade em andamento.
- A espera pelo preparo local fica limitada a 5 segundos. Falha ou demora em dados ou preferências leva à abertura com valores padrão e indicação do problema na área afetada. Se a própria interface não conseguir abrir, a sequência dá lugar a uma mensagem com “Tentar novamente” e “Fechar”.
- Fechar durante a sequência cancela aquela abertura. Outra solicitação enquanto a janela está aberta apenas a traz à frente, sem reiniciar a sequência. Reabrir depois de fechar inicia uma nova sequência.
- Após a transição, os Módulos recebem atualizações normalmente, preservando a estrutura e a posição de leitura, sem reconstruir o Painel inteiro.
- Com movimento reduzido, permanecem fundo escuro, mensagens e duração mínima, em apresentação estática, sem linhas em movimento nem transições animadas.

### Revisão proposta da história 12

“Como usuário, quero que o Painel apresente uma sequência de abertura durante o preparo local, com duração mínima de 1,2 segundo, e depois mostre os dados locais disponíveis, atualizando os Módulos em segundo plano com um indicador discreto, sem depender da rede para abrir.”

Essa proposta substitui a exigência atual de cache imediato e abertura “sem tela de carregamento”. Os critérios detalhados são os deste contrato. A spec não foi alterada nesta etapa; sua revisão e a implementação ficam para o handoff após a validação visual.

### Continuação

O contrato desbloqueia [Validar sequência visual cinematográfica de abertura](02-opening-visual.md). A composição, a cadência visual e a reação do usuário permanecem nesse ticket; nenhuma nova pergunta de comportamento ficou pendente nesta rodada.
