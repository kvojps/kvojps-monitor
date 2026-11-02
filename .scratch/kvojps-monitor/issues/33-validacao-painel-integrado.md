# 33: Validar a composição integrada no app nativo

**What to build:** comprovar no Painel integrado que assinatura, Faixa de bordo e movimento atendem às escolhas aprovadas e aos critérios de acessibilidade, privacidade e desempenho no app nativo, corrigindo desvios dentro desses limites.

**Blocked by:** 28 — Comunicar consultas por Provedor de IA; 29 — Comunicar atualizações e movimentações do Trello; 30 — Comunicar mudanças de Rede e teste de velocidade; 31 — Comunicar transições de Agentes e alertas de Sistema; 32 — Aplicar o movimento Órbita contínua.

**Status:** ready-for-human

- [ ] Validar em GNOME/Wayland e WebKitGTK a composição nos temas claro, escuro e do sistema, largura reduzida, 1100 × 700, 1000 × 700 e texto ampliado; preservar responsividade e acesso a métricas, alertas, detalhes e ações.
- [ ] Exercitar os seis Módulos simultâneos com prioridade e resumo de excedentes, sem carrossel ou expansão da grade, incluindo operações rápidas/longas, falha, pausa, recuperação, cancelamento e indisponibilidade total/parcial.
- [ ] Conferir permanência de 4 segundos, independência entre fontes/provedores, ausência de conclusão fabricada, snapshots repetidos, resultados descartados e referência inicial ao reabrir. Verificar explicações persistentes após desaparecerem da faixa.
- [ ] Registrar evidências de teclado, foco, ordem de leitura, contraste WCAG AA e leitor de tela com anúncios agrupados não interruptivos. Fonte e resultado são claros em pt-BR e mensagens não roubam foco.
- [ ] Verificar redução manual e do sistema, pausa com Painel oculto e em Configurações, destruição da webview ao fechar e retomada sem reapresentar eventos antigos.
- [ ] Seguir o protocolo de desempenho existente: resposta visual ≤ 100 ms, 60 fps em tela de 60 Hz, três pares A/B de CPU de 60 segundos com acréscimo médio da decoração ≤ 2 pontos percentuais de um núcleo, ausência de animação oculta e memória/PSS aberto, fechado e na reabertura. Registrar ambiente, medições e traces; protótipos e navegador não substituem evidência nativa.
- [ ] Executar os testes pertinentes do núcleo e UI, incluindo relógio falso, fixtures e sentinelas de sanitização de mensagens, logs, cache e notificações. Preservar guardrails de keyring obrigatório, alterações de hooks apenas por clique explícito/aditivas/reversíveis e token Claude apenas em memória sem renovação.
- [ ] Reaproveitar as verificações pertinentes do ticket 25, sem exigir suas pendências alheias e sem modificá-lo ou encerrá-lo. Registrar evidências e limitações concretas; ausência de acesso ao ambiente nativo não conta como aprovação.
- [ ] Corrigir desvios comprovados dentro da proposta aprovada; mudanças de escolhas, grade, fontes, privilégios, coleta ou configurações voltam ao usuário antes da execução.

## Comments

02/11/2026 — Validação parcial registrada em [evidências e limitações](../validation/33/README.md),
com nove capturas Tauri/WebKitGTK no compositor Wayland e teste integrado dos seis
Módulos. Sem desvio de produção comprovado nesta rodada. Os critérios compostos
acima permanecem abertos: faltam percurso manual na sessão GNOME pessoal, leitor
de tela, texto ampliado e medições/traces do protocolo com build release e fontes
reais. A sessão D-Bus isolada usada nas capturas não valida Indicador ou portal da
sessão pessoal. `ready-for-human` indica essa necessidade concreta, sem aprovação
do aceite nativo e sem alteração ou encerramento do ticket 25.
