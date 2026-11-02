# 28: Comunicar consultas por Provedor de IA

**What to build:** mostrar na Faixa de bordo as operações reais de Claude e Codex, identificando o Provedor de IA consultado antes de agrupar no Módulo IA e mantendo independência entre fontes.

**Blocked by:** 27 — Integrar assinatura e Faixa de bordo às atualizações do GitHub.

**Status:** ready-for-agent

- [ ] Suprir somente as lacunas necessárias de origem, identidade e resultado por Provedor de IA; não atribuir consultas do Codex ao Claude nem inventar atividade para a status line.
- [ ] Mostrar início de atualizações solicitadas e retomadas após pausa; manter sucesso automático silencioso e comunicar falhas e recuperações reais conforme a política existente de falhas.
- [ ] Preservar fontes, cadências, estados canônicos, opt-in de oauth/usage e fallback para status line. Token Claude é lido só em memória e nunca renovado; integração não altera configurações de hooks ou status line sem clique explícito.
- [ ] Manter indisponibilidade total ou parcial explícita e explicações e ações persistentes, sem apresentar ausência como zero ou sucesso e sem apagar a condição de um provedor pelo sucesso do outro.
- [ ] Aplicar permanência, prioridade, anúncios e referência inicial da faixa; snapshots repetidos e operações descartadas não geram resultado fabricado.
- [ ] Cobrir consultas concorrentes, resultados rápidos, falha, pausa, recuperação, repetição e reabertura com portas injetáveis, fixtures reais no núcleo, relógio falso e fixtures de snapshots/transições na UI.
- [ ] Testar com sentinelas que mensagens, logs, cache e notificações não recebem credenciais, prompts ou respostas; não persistir histórico operacional e preservar os testes dos demais guardrails.
