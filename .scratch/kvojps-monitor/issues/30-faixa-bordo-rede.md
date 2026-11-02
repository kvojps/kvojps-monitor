# 30: Comunicar mudanças de Rede e teste de velocidade

**What to build:** comunicar na Faixa de bordo mudanças relevantes da Rede e o andamento real do teste de velocidade, sem transformar amostras contínuas em mensagens ou resultados descartados em sucesso.

**Blocked by:** 27 — Integrar assinatura e Faixa de bordo às atualizações do GitHub.

**Status:** ready-for-agent

- [ ] Comunicar mudanças de conexão, VPN e categoria de Qualidade da rede e Segurança da rede; não gerar mensagens por sondagem, latência, tráfego ou snapshot repetido. Permanecer conectado não comprova troca de rede.
- [ ] Correlacionar início e resultado real de teste de velocidade com identidade de operação e motivo explícito de cancelamento, suprindo apenas as lacunas necessárias do contrato.
- [ ] Diferenciar sucesso, falha e cancelamento; descartar corretamente resultados após mudança de conexão ou suspensão, sem conclusão inferida por tempo.
- [ ] Aplicar agrupamento, prioridade, permanência e anúncios não interruptivos da faixa. Reabertura estabelece referência sem reapresentar mudanças antigas; inícios automáticos são silenciosos para leitores de tela.
- [ ] Preservar medições, coleta, cadências, limites de privilégio, notificações e ações existentes; indisponibilidade nunca vira valor zero ou sucesso e explicações permanecem no Módulo.
- [ ] Testar transições de categoria, VPN e conexão, repetição, reabertura, teste rápido/longo, falha, cancelamento, suspensão e resultado obsoleto com portas existentes, fixtures reais, relógio falso e fixtures de snapshots/transições na UI.
- [ ] Testar sanitização com sentinelas: apenas fonte, evento e estado operacional nas mensagens; sem conteúdo sensível em logs, cache ou notificações e sem histórico operacional persistido. Preservar demais guardrails de privacidade.
