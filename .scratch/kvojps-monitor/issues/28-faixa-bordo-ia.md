# 28: Comunicar consultas por Provedor de IA

**What to build:** mostrar na Faixa de bordo as operações reais de Claude e Codex, identificando o Provedor de IA consultado antes de agrupar no Módulo IA e mantendo independência entre fontes.

**Blocked by:** 27 — Integrar assinatura e Faixa de bordo às atualizações do GitHub.

**Status:** done

- [x] Suprir somente as lacunas necessárias de origem, identidade e resultado por Provedor de IA; não atribuir consultas do Codex ao Claude nem inventar atividade para a status line.
- [x] Mostrar início de atualizações solicitadas e retomadas após pausa; manter sucesso automático silencioso e comunicar falhas e recuperações reais conforme a política existente de falhas.
- [x] Preservar fontes, cadências, estados canônicos, opt-in de oauth/usage e fallback para status line. Token Claude é lido só em memória e nunca renovado; integração não altera configurações de hooks ou status line sem clique explícito.
- [x] Manter indisponibilidade total ou parcial explícita e explicações e ações persistentes, sem apresentar ausência como zero ou sucesso e sem apagar a condição de um provedor pelo sucesso do outro.
- [x] Aplicar permanência, prioridade, anúncios e referência inicial da faixa; snapshots repetidos e operações descartadas não geram resultado fabricado.
- [x] Cobrir consultas concorrentes, resultados rápidos, falha, pausa, recuperação, repetição e reabertura com portas injetáveis, fixtures reais no núcleo, relógio falso e fixtures de snapshots/transições na UI.
- [x] Testar com sentinelas que mensagens, logs, cache e notificações não recebem credenciais, prompts ou respostas; não persistir histórico operacional e preservar os testes dos demais guardrails.

## Notas da implementação

- Claude e Codex publicam `operation` transitória em suas próprias partes do snapshot: identidade sequencial, origem e resultado real. A faixa deduplica e cronometra por Provedor de IA antes de agrupar em um único resumo do Módulo, preservando a prioridade de cada fonte e os anúncios agrupados.
- Consultas solicitadas e retomadas têm início real; consultas rápidas podem chegar diretamente ao resultado. Sucessos automáticos ficam silenciosos. Falhas e recuperações são resultados da consulta; fallback dos rollouts e leituras da status line não fabricam sucesso. A linha persistente de falha continua aparecendo após três falhas consecutivas.
- Pausa descarta consultas pendentes do Codex. Claude publica o início antes do HTTP bloqueante e consulta a porta injetável `CurrentStatus` ao receber a resposta: mudanças de conectividade ou geração descartam o resultado antes de alterar leituras, cache ou logs.
- Token local indisponível mantém `usageUnavailable` e orientação com ação nas Configurações, mesmo depois dos quatro segundos da faixa, com ou sem leitura anterior. Estado e métricas do outro provedor continuam independentes. Opt-in, cadências, estados canônicos, fallback e configurações de hooks/status line foram preservados.
- Testes com fixtures reais e relógio falso cobrem resultados rápidos, concorrência, falha, recuperação, pausa, retomada, descarte, repetição, reabertura e orientação persistente. Sentinelas verificam ausência de credenciais, prompts e respostas nos snapshots operacionais, cache e logs; não há histórico operacional persistido nem nova notificação do sistema.
- Validação: 339 testes de UI e 292 testes unitários Rust passaram; `npm run check` sem erros ou avisos, `npm run build`, formatação Rust e `git diff --check` passaram. Na execução completa de Rust, um teste de integração de hooks passou e dois ficaram bloqueados pelo sandbox ao criar sockets Unix (`Operation not permitted`); a tentativa de executá-los com permissão ampliada foi abortada antes do despacho.
- Revisão `code-review` contra o HEAD inicial `639407f12a8f104e4e17ec1a27cbc8a58628c420`: Standards e Spec sem achados restantes após corrigir descarte de resposta Claude durante mudança de conexão, orientação persistente de token indisponível e composição compartilhada de snapshots intermediários/finais.
