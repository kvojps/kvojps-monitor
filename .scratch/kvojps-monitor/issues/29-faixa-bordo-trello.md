# 29: Comunicar atualizações e movimentações do Trello

**What to build:** acompanhar na Faixa de bordo atualizações do Trello e movimentações de cards com início e resultado reais, preservando estados, ações e reversão de movimentações na falha.

**Blocked by:** 27 — Integrar assinatura e Faixa de bordo às atualizações do GitHub.

**Status:** ready-for-agent

- [ ] Distinguir identidade, origem e resultado de atualização e movimentação; fim de refreshing não comprova sucesso e movimentação tem sinais próprios sanitizados.
- [ ] Mostrar início de atualização solicitada e retomada; sucesso automático fica silencioso, falhas e recuperações aparecem. Movimentação mostra início e resultado reais, incluindo falha e cancelamento quando aplicável, sem inferir sucesso de operação descartada.
- [ ] Aplicar agrupamento por Módulo, prioridade, resultado por 4 segundos e anúncios não interruptivos da faixa, sem perder ações e explicações persistentes no Módulo.
- [ ] Preservar seleção, sincronização do resumo, reversão na falha e alternativas existentes de interação, sem ampliar escopo da integração Trello.
- [ ] Mensagens de movimentação não identificam card ou lista. Testar com sentinelas que conteúdo de cards e credenciais não alcançam mensagens, logs, cache ou notificações; não persistir histórico operacional.
- [ ] Cobrir atualização e movimento concorrentes, sucesso rápido, falha, pausa, recuperação, cancelamento aplicável, snapshots repetidos e reabertura usando portas existentes, fixtures reais, relógio falso e fixtures de snapshots/transições na UI; preservar demais testes de privacidade.
