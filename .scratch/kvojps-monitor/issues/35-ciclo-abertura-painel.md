# 35: Implementar prontidão e ciclo de abertura do Painel

**What to build:** implementar o ciclo real de preparo local e recuperação, integrado à criação e destruição da janela e à restauração do Painel. A camada visual final é entregue no ticket Aplicar Arquitetura à abertura do Painel.

**Blocked by:** 34 — Incorporar a abertura aprovada à spec.

**Status:** ready-for-human

- [x] Executar a sequência em toda abertura, inclusive a primeira após iniciar o app. Login automático continua iniciando apenas o Indicador.
- [x] Considerar pronto quando estrutura, Âncora e dados locais disponíveis estiverem apresentados; não aguardar HTTP, keyring desbloqueado ou atualização remota. Sem cache, exibir o estado explícito apropriado por Módulo.
- [x] Contar o mínimo total de 1,2 segundo a partir da primeira apresentação visível da sequência, incluindo a transição. Preparo mais longo não ganha outro intervalo mínimo ao final; transição não revela conteúdo antes da prontidão ou da recuperação.
- [x] Limitar a espera local a 5 segundos: falha ou demora de dados/preferências usa valores padrão e sinalização na área afetada. Respostas tardias ou de abertura cancelada não sobrescrevem uma abertura posterior.
- [x] Se a própria interface não conseguir abrir, substituir a sequência por mensagem em pt-BR com “Tentar novamente” e “Fechar”. Não deixar espera infinita quando a inicialização rejeita.
- [x] Fechamento cancela aquela abertura e libera recursos. Nova solicitação com janela aberta apenas a traz à frente; reabertura após fechar começa outra sequência.
- [x] Expor estados reais de preparo para as mensagens visuais; não fabricar etapas para ocupar o mínimo. Atualizações posteriores preservam estrutura e posição de leitura.
- [x] Preservar Âncora, destino de notificação, boas-vindas e Configurações nos seus contratos existentes.
- [x] Cobrir no seam existente, com relógio injetável e fixtures, preparo rápido/lento, ausência de cache, rede pendente, rejeição, timeout, recuperação parcial, falha fatal, repetição, fechamento e resposta tardia. Toda nova E/S do núcleo passa pelas portas injetáveis.
- [x] Preservar guardrails com testes pertinentes: nenhum conteúdo sensível em logs/cache/notificações, keyring obrigatório para salvar credenciais, hooks apenas por clique explícito/aditivos/reversíveis e token Claude apenas em memória sem renovação.

## Fontes aprovadas

- [Definir prontidão e ciclo de abertura do Painel](../../panel-opening/issues/01-opening-readiness.md).
- [Validar sequência visual cinematográfica de abertura](../../panel-opening/issues/02-opening-visual.md).
- [Abertura cinematográfica do Painel](../../panel-opening/map.md).


## Implementação — 02/11/2026

O ponto de entrada monta `PanelOpening`, que mantém a estrutura do Painel durante o preparo e só a revela após restauração e mínimo de 1,2 segundo contado depois da primeira pintura. O relógio de apresentação e os temporizadores são injetáveis. Estados de preparo correspondem à restauração local e à apresentação dos Módulos; concluído o trabalho, a mensagem não simula atividade durante o mínimo restante.

O comando `initial_state` existente só consulta memória e não aguarda rede ou keyring. Rejeição ou timeout de 5 segundos usa padrões, preserva snapshots recebidos e sinaliza preferências e Módulos sem dados. Respostas tardias são descartadas. Falhas da interface exibem “Tentar novamente” e “Fechar”; fechamento e nova tentativa liberam inscrições, observadores e temporizadores. O Painel preserva Âncora, destino de notificação, boas-vindas e Configurações. Os contratos nativos de destruir/recriar e trazer uma janela existente à frente permanecem cobertos pelos testes existentes.

A apresentação é funcional e estática; a composição Arquitetura e a transição visual final pertencem ao próximo ticket. Não houve nova E/S no núcleo nem mudança em credenciais, hooks, cache ou notificações.

### Revisão de padrões

Sem violações documentadas. O apontamento de clareza de nomes foi corrigido para `minimumElapsed` e `openingActive`.

### Revisão da spec

Sem bloqueios restantes após corrigir a contagem desde a apresentação e restringir avisos aos Módulos sem snapshots. Testes adicionais cobrem recuperação parcial e relógio de apresentação injetado.

### Validação

- 11 testes novos da abertura: preparo rápido/lento, mínimo desde apresentação, timeout, rejeição, recuperação parcial, falha fatal e nova tentativa, fechamento, reabertura, resposta tardia, dados ausentes e atualização pendente, boas-vindas e ausência de comandos sensíveis automáticos.
- Suíte Rust: 305 testes unitários e 3 de integração aprovados, incluindo guardrails de keyring, hooks, token Claude e conteúdo sensível. Testes de sockets executados fora do sandbox.
- Suíte UI completa: 364 testes aprovados em 16 arquivos.
- Checagem Svelte/TypeScript sem erros ou avisos; build de produção aprovado.
- Interação real no GNOME/Wayland e desempenho nativo não foram verificados nesta execução; permanecem para validação humana conforme a spec.
