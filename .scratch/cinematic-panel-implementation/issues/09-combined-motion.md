# 09: Coreografia combinada com pausa nativa

**What to build:** exibir a Coreografia combinada na composição final, com movimento somente decorativo e pausa efetiva conforme preferências e visibilidade nativa.

**Blocked by:** 02 — IA com instrumentos de Limite de uso; 03 — Agentes com intervenção e sessão direcionada; 04 — GitHub com atividade e Repo em destaque; 05 — Trello em quadro interativo; 06 — Rede com instrumentos e tráfego real; 07 — Sistema com instrumentos e alertas; 08 — Configurações completas, temas e primeira execução.

**Status:** ready-for-agent

- [ ] Aplicar anéis decorativos de 48 s/volta e brilho de 4 s por trecho alternando ida/volta (8 s/ciclo), mantendo arcos de medição imóveis entre atualizações reais.
- [ ] Aplicar fluxo nas faixas superiores de 3,5 s com fases deslocadas, varredura nas bordas de 7 s defasada entre colunas, deriva nos títulos de 9 s e deslocamento de navegação de 2 px ao apontador com transição de 150 ms.
- [ ] Dados, gráficos, textos e alertas permanecem estáveis e não piscam; decoração não comunica coleta ou atividade. Não adicionar timers de dados nem números artificiais.
- [ ] Redução manual ou do sistema interrompe decoração e remove rolagem animada/transições; preferência do sistema prevalece e mudanças em execução são respeitadas.
- [ ] Sincronizar com visibilidade nativa: Painel oculto não executa animação, callbacks ou timers decorativos; document.hidden isoladamente não comprova esse requisito. Reabertura e recriação da webview restauram o comportamento correto.
- [ ] Testar preferências, mudança do ambiente, ocultação/reabertura e estabilidade dos instrumentos, e preparar instrumentação/trace para as medições do ticket 11 sem expor dados sensíveis.
