# 31: Comunicar transições de Agentes e alertas de Sistema

**What to build:** apresentar na Faixa de bordo transições relevantes das Sessões de agente e entrada e saída dos alertas existentes de Sistema, agrupadas sem ruído por amostra.

**Blocked by:** 27 — Integrar assinatura e Faixa de bordo às atualizações do GitHub.

**Status:** done

- [x] Em Agentes, comunicar transições para Aguardando você, Turno concluído, Com falha e Encerrada, agrupando mudanças simultâneas. Em andamento permanece no Módulo; Turno concluído não afirma sucesso, pois pode incluir interrupção pelo usuário.
- [x] Comparar identidade e estado de Sessão de agente sem tratar snapshot novo como turno novo; respeitar limitações de polling e hooks, sem inferir espera ou sucesso.
- [x] Em Sistema, comunicar somente entrada e saída dos alertas existentes de temperatura, disco e bateria, sem novos limites nem contrato geral de operações de coleta. Ausência normal de sensor ou bateria não é falha.
- [x] Aplicar agrupamento, prioridade, permanência e anúncios relevantes não interruptivos; repetição é silenciosa e reabertura estabelece referência atual, sem eventos antigos reapresentados.
- [x] Preservar explicações e ações nos Módulos e notificações existentes; alertas de Sistema continuam restritos ao Painel.
- [x] Excluir das mensagens de Agentes conversa, nome de sessão, projeto, branch e motivo textual de espera. Testes com sentinelas cobrem mensagens, logs, cache e notificações, sem histórico operacional persistido e preservando os demais testes de privacidade.
- [x] Cobrir transições simultâneas, interrupção, encerramento, sensores ausentes, entrada/saída de alerta, repetição e reabertura com portas existentes, fixtures reais, relógio falso quando necessário e fixtures de snapshots/transições na UI.


## Notas da implementação

- A faixa compara identidade (provedor, identificador e início) e estado das Sessões de agente. Agrupa as transições por estado, sem inferir turno novo de timestamp, espera de polling ou sucesso de conclusão/interrupção. Encerramento depende do estado observado, nunca de desaparecimento.
- Sistema usa exclusivamente os indicadores `warning` publicados pelo núcleo, agrupando alertas de temperatura, disco e bateria. Sensor ausente não produz falha nem recuperação; reaparecimento em alerta comunica entrada real.
- Referências independentes por Módulo tornam a primeira leitura válida silenciosa, inclusive quando chega depois de abrir o Painel. Reabertura reinicia as referências. Repetições não renovam os quatro segundos de permanência.
- Mensagens e anúncios usam apenas vocabulário fechado e contagens. Nenhuma alteração em coleta, notificações, cache ou logs; nenhum histórico operacional persistido. Fixtures existentes de snapshots, relógio falso e sentinelas cobrem a UI; os testes de privacidade do núcleo permanecem passando.
- Validação: 351 testes de UI, 305 testes unitários Rust e 3 testes de integração de hooks; checagem de tipos sem erros ou avisos. Integrações executadas fora do sandbox para permitir sockets Unix.

## Revisão

### Standards

Sem violações documentadas. Resolvida a observação heurística sobre índices posicionais dos textos de Sistema, substituídos por chaves `warning` e `recovered`.

### Spec

Sem pendências restantes. Corrigidos e testados o reaparecimento de sensor já em alerta e a referência inicial independente por Módulo em leituras tardias.
