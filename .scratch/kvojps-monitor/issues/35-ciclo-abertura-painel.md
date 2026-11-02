# 35: Implementar prontidão e ciclo de abertura do Painel

**What to build:** implementar o ciclo real de preparo local e recuperação, integrado à criação e destruição da janela e à restauração do Painel. A camada visual final é entregue no ticket Aplicar Arquitetura à abertura do Painel.

**Blocked by:** 34 — Incorporar a abertura aprovada à spec.

**Status:** ready-for-agent

- [ ] Executar a sequência em toda abertura, inclusive a primeira após iniciar o app. Login automático continua iniciando apenas o Indicador.
- [ ] Considerar pronto quando estrutura, Âncora e dados locais disponíveis estiverem apresentados; não aguardar HTTP, keyring desbloqueado ou atualização remota. Sem cache, exibir o estado explícito apropriado por Módulo.
- [ ] Contar o mínimo total de 1,2 segundo a partir da primeira apresentação visível da sequência, incluindo a transição. Preparo mais longo não ganha outro intervalo mínimo ao final; transição não revela conteúdo antes da prontidão ou da recuperação.
- [ ] Limitar a espera local a 5 segundos: falha ou demora de dados/preferências usa valores padrão e sinalização na área afetada. Respostas tardias ou de abertura cancelada não sobrescrevem uma abertura posterior.
- [ ] Se a própria interface não conseguir abrir, substituir a sequência por mensagem em pt-BR com “Tentar novamente” e “Fechar”. Não deixar espera infinita quando a inicialização rejeita.
- [ ] Fechamento cancela aquela abertura e libera recursos. Nova solicitação com janela aberta apenas a traz à frente; reabertura após fechar começa outra sequência.
- [ ] Expor estados reais de preparo para as mensagens visuais; não fabricar etapas para ocupar o mínimo. Atualizações posteriores preservam estrutura e posição de leitura.
- [ ] Preservar Âncora, destino de notificação, boas-vindas e Configurações nos seus contratos existentes.
- [ ] Cobrir no seam existente, com relógio injetável e fixtures, preparo rápido/lento, ausência de cache, rede pendente, rejeição, timeout, recuperação parcial, falha fatal, repetição, fechamento e resposta tardia. Toda nova E/S do núcleo passa pelas portas injetáveis.
- [ ] Preservar guardrails com testes pertinentes: nenhum conteúdo sensível em logs/cache/notificações, keyring obrigatório para salvar credenciais, hooks apenas por clique explícito/aditivos/reversíveis e token Claude apenas em memória sem renovação.

## Fontes aprovadas

- [Definir prontidão e ciclo de abertura do Painel](../../panel-opening/issues/01-opening-readiness.md).
- [Validar sequência visual cinematográfica de abertura](../../panel-opening/issues/02-opening-visual.md).
- [Abertura cinematográfica do Painel](../../panel-opening/map.md).
