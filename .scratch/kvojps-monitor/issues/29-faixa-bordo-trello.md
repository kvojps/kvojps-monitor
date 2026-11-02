# 29: Comunicar atualizações e movimentações do Trello

**What to build:** acompanhar na Faixa de bordo atualizações do Trello e movimentações de cards com início e resultado reais, preservando estados, ações e reversão de movimentações na falha.

**Blocked by:** 27 — Integrar assinatura e Faixa de bordo às atualizações do GitHub.

**Status:** done

- [x] Distinguir identidade, origem e resultado de atualização e movimentação; fim de refreshing não comprova sucesso e movimentação tem sinais próprios sanitizados.
- [x] Mostrar início de atualização solicitada e retomada; sucesso automático fica silencioso, falhas e recuperações aparecem. Movimentação mostra início e resultado reais, incluindo falha e cancelamento quando aplicável, sem inferir sucesso de operação descartada.
- [x] Aplicar agrupamento por Módulo, prioridade, resultado por 4 segundos e anúncios não interruptivos da faixa, sem perder ações e explicações persistentes no Módulo.
- [x] Preservar seleção, sincronização do resumo, reversão na falha e alternativas existentes de interação, sem ampliar escopo da integração Trello.
- [x] Mensagens de movimentação não identificam card ou lista. Testar com sentinelas que conteúdo de cards e credenciais não alcançam mensagens, logs, cache ou notificações; não persistir histórico operacional.
- [x] Cobrir atualização e movimento concorrentes, sucesso rápido, falha, pausa, recuperação, cancelamento aplicável, snapshots repetidos e reabertura usando portas existentes, fixtures reais, relógio falso e fixtures de snapshots/transições na UI; preservar demais testes de privacidade.

## Notas da implementação

- Snapshots do Trello publicam `operation` para atualização e `movement` para movimentação, com identidade, origem e resultado independentes. Os sinais são transitórios, fora do cache e dos logs.
- Atualização manual e retomada mostram início real; sucessos automáticos ficam silenciosos. Falhas, recuperação, pausa e indisponibilidade usam resultados explícitos. A movimentação publica seu resultado antes da releitura do quadro, que tem resultado próprio.
- A porta existente `CurrentStatus` confere conexão e geração depois do HTTP. Respostas descartadas não atualizam o quadro ou cache nem viram sucesso; suspensão durante movimento produz cancelamento e preserva o último quadro confirmado. Um novo movimento após retomada recebe outra identidade e pode concluir normalmente.
- A faixa agrupa atualização e movimentação em um resumo do Trello, mantendo prioridade, permanência de 4 segundos, anúncios `polite`, deduplicação e referência na reabertura. Mensagens de movimentação usam vocabulário fechado, sem nomes ou identificadores de cards e listas. Seleção, resumo otimista, reversão e alternativas de interação seguem cobertos pelos testes existentes do Módulo.
- Fixtures da UI foram conferidas contra snapshots reais do núcleo; fixture de transições cobre concorrência de sinais, resultado rápido, falha, recuperação, cancelamento, repetição e reabertura. Testes com HTTP gravado, relógio falso e sentinelas cobrem os sinais e a ausência de conteúdo sensível ou histórico operacional em cache/logs. Nenhuma nova notificação do sistema.
- Validação: 341 testes de UI; 300 testes unitários Rust e 3 de integração; `npm run check` sem erros ou avisos; `npm run build`; `git diff --check`. Integrações de hooks executadas fora do sandbox para permitir sockets locais.

## Revisão

### Standards

Sem violações documentadas. Uma observação heurística: o vocabulário e a contabilidade operacional têm semelhanças com GitHub e IA. Mantida a organização atual por fonte nesta fatia; compartilhar esse código é uma possibilidade de manutenção, sem bloqueio funcional.

### Spec

Sem achados restantes. Corrigidos e testados: publicação do resultado do movimento antes da releitura, descarte após mudança de ciclo de vida durante HTTP e nova movimentação após cancelamento e retomada.
