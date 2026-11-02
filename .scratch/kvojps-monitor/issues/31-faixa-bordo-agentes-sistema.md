# 31: Comunicar transições de Agentes e alertas de Sistema

**What to build:** apresentar na Faixa de bordo transições relevantes das Sessões de agente e entrada e saída dos alertas existentes de Sistema, agrupadas sem ruído por amostra.

**Blocked by:** 27 — Integrar assinatura e Faixa de bordo às atualizações do GitHub.

**Status:** ready-for-agent

- [ ] Em Agentes, comunicar transições para Aguardando você, Turno concluído, Com falha e Encerrada, agrupando mudanças simultâneas. Em andamento permanece no Módulo; Turno concluído não afirma sucesso, pois pode incluir interrupção pelo usuário.
- [ ] Comparar identidade e estado de Sessão de agente sem tratar snapshot novo como turno novo; respeitar limitações de polling e hooks, sem inferir espera ou sucesso.
- [ ] Em Sistema, comunicar somente entrada e saída dos alertas existentes de temperatura, disco e bateria, sem novos limites nem contrato geral de operações de coleta. Ausência normal de sensor ou bateria não é falha.
- [ ] Aplicar agrupamento, prioridade, permanência e anúncios relevantes não interruptivos; repetição é silenciosa e reabertura estabelece referência atual, sem eventos antigos reapresentados.
- [ ] Preservar explicações e ações nos Módulos e notificações existentes; alertas de Sistema continuam restritos ao Painel.
- [ ] Excluir das mensagens de Agentes conversa, nome de sessão, projeto, branch e motivo textual de espera. Testes com sentinelas cobrem mensagens, logs, cache e notificações, sem histórico operacional persistido e preservando os demais testes de privacidade.
- [ ] Cobrir transições simultâneas, interrupção, encerramento, sensores ausentes, entrada/saída de alerta, repetição e reabertura com portas existentes, fixtures reais, relógio falso quando necessário e fixtures de snapshots/transições na UI.
