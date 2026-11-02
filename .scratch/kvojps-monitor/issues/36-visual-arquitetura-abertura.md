# 36: Aplicar Arquitetura à abertura do Painel

**What to build:** implementar a direção visual C aprovada dentro da própria janela, consumindo o ciclo real de abertura. Reescrever o visual para produção a partir da referência descartável.

**Blocked by:** 35 — Implementar prontidão e ciclo de abertura do Painel.

**Status:** ready-for-agent

- [ ] Compor fundo escuro, malha de linhas luminosas em perspectiva e identidade inferior esquerda, conforme Arquitetura (C). Exibir exatamente “kvojps monitor”.
- [ ] Usar como fonte visual a branch local `prototype/panel-opening`, commit `91d27bf`, entrada `opening-prototype.html`, variante C. Não incorporar controles de simulação, variantes alternativas ou portas falsas ao app.
- [ ] Vincular “Preparando o Painel…” e “Carregando Módulos…” às atividades reais correspondentes. Se tudo estiver pronto antes do mínimo, manter apenas a composição, sem mensagem alegando atividade.
- [ ] Fazer a passagem suave para os Módulos dentro da duração contratada, sem reiniciar o mínimo após preparo demorado. Respeitar prontidão/recuperação antes de revelar conteúdo.
- [ ] Manter abertura escura nos temas claro, escuro e do sistema, revelando o tema restaurado ao entrar no Painel sem flashes intermediários de tema incorreto.
- [ ] Respeitar redução manual e do sistema: mesma composição estática, mensagens e duração mínima, sem linhas em movimento ou transição animada. Preferência do sistema prevalece.
- [ ] Garantir legibilidade, responsividade, texto ampliado, contraste e operação por teclado; conteúdo coberto não recebe foco/interação. Mensagens não roubam foco nem provocam anúncios repetidos a cada frame.
- [ ] Preservar a decoração existente dos Módulos, sem executá-la indevidamente sob a abertura ou após fechar. Não criar preferências de produto para duração ou intensidade.
- [ ] Verificar com fixtures de snapshot os estados do ciclo, temas, movimento reduzido e falhas; executar check, testes pertinentes e build. Evidência nativa integrada pertence ao ticket Validar a abertura no app nativo.

## Fontes aprovadas

- [Definir prontidão e ciclo de abertura do Painel](../../panel-opening/issues/01-opening-readiness.md).
- [Validar sequência visual cinematográfica de abertura](../../panel-opening/issues/02-opening-visual.md).
- [Abertura cinematográfica do Painel](../../panel-opening/map.md).
