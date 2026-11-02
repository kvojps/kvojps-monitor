# 27: Integrar assinatura e Faixa de bordo às atualizações do GitHub

**What to build:** mostrar a assinatura KVOJPS e uma Faixa de bordo compacta no Painel, com mensagens ligadas às atualizações reais do GitHub, preservando métricas, estados e ações dos seis Módulos. Esta é a primeira fatia completa do contrato operacional até a UI.

**Blocked by:** 26 — Adotar a revisão aprovada do visual do Painel.

**Status:** ready-for-agent

- [ ] Integrar a assinatura estática na faixa superior conforme C — Assinatura na faixa e as mensagens conforme A — Faixa de bordo, preservando grade B e lado esquerdo livre na linha de ações.
- [ ] Publicar e consumir apenas os sinais operacionais necessários, distinguindo fonte, origem (usuário, automática ou retomada), identidade e resultado real; não usar updatedAt como identidade nem refreshing=false como prova de sucesso.
- [ ] Mostrar início de atualização solicitada e retomada após pausa; sucesso automático permanece silencioso, enquanto falha e recuperação reais aparecem. Operação rápida pode mostrar diretamente o resultado; cancelamento, quando existente, é distinto de sucesso.
- [ ] Manter indicação enquanto a operação estiver realmente em andamento e resultado por 4 segundos; relógio injetável controla somente permanência visual. Repetição de snapshot não cria operação e reabertura estabelece referência atual sem reapresentar eventos antigos.
- [ ] Agrupar por fonte em um resumo por Módulo, priorizar falha, pausa e atenção antes de atividade e sucesso, e resumir excedentes como “+3 Módulos”. Sem carrossel, histórico ou expansão da grade; detalhes e ações permanecem acessíveis nos Módulos.
- [ ] Manter indisponibilidade explícita, sem zeros ou sucessos fabricados, e explicação e ação no Módulo enquanto a condição persistir, mesmo após sumir da faixa.
- [ ] Oferecer anúncios agrupados não interruptivos para resultados solicitados, falhas e recuperações, sem roubar foco; inícios automáticos, inclusive retomadas visuais, são silenciosos para leitores de tela. Não alterar notificações do sistema.
- [ ] Validar composição nos temas claro, escuro e do sistema, largura reduzida, 1100 × 700, 1000 × 700 e texto ampliado; verificar teclado, ordem de leitura e contraste WCAG AA.
- [ ] Cobrir com fixtures de snapshots e transições os seis Módulos simultâneos, prioridades, operação rápida/longa, falha, pausa, recuperação, cancelamento, repetição e reabertura. Apenas GitHub precisa de integração real neste ticket; os demais são cenários de contrato para as próximas fatias.
- [ ] Toda E/S do núcleo passa pelas portas existentes, com fixtures reais e relógio falso. Mensagens só contêm fonte, evento e estado operacional; dados sensíveis sentinela comprovam sanitização de mensagens, logs, cache e notificações, sem histórico operacional persistido. Preservar testes dos demais guardrails de privacidade.
