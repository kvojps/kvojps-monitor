# 09: Coreografia combinada com pausa nativa

**What to build:** exibir a Coreografia combinada na composição final, com movimento somente decorativo e pausa efetiva conforme preferências e visibilidade nativa.

**Blocked by:** 02 — IA com instrumentos de Limite de uso; 03 — Agentes com intervenção e sessão direcionada; 04 — GitHub com atividade e Repo em destaque; 05 — Trello em quadro interativo; 06 — Rede com instrumentos e tráfego real; 07 — Sistema com instrumentos e alertas; 08 — Configurações completas, temas e primeira execução.

**Status:** resolved

- [x] Aplicar anéis decorativos de 48 s/volta e brilho de 4 s por trecho alternando ida/volta (8 s/ciclo), mantendo arcos de medição imóveis entre atualizações reais.
- [x] Aplicar fluxo nas faixas superiores de 3,5 s com fases deslocadas, varredura nas bordas de 7 s defasada entre colunas, deriva nos títulos de 9 s e deslocamento de navegação de 2 px ao apontador com transição de 150 ms.
- [x] Dados, gráficos, textos e alertas permanecem estáveis e não piscam; decoração não comunica coleta ou atividade. Não adicionar timers de dados nem números artificiais.
- [x] Redução manual ou do sistema interrompe decoração e remove rolagem animada/transições; preferência do sistema prevalece e mudanças em execução são respeitadas.
- [x] Sincronizar com visibilidade nativa: Painel oculto não executa animação, callbacks ou timers decorativos; document.hidden isoladamente não comprova esse requisito. Reabertura e recriação da webview restauram o comportamento correto.
- [x] Testar preferências, mudança do ambiente, ocultação/reabertura e estabilidade dos instrumentos, e preparar instrumentação/trace para as medições do ticket 11 sem expor dados sensíveis.


## Implementação

Coreografia CSS entregue com os períodos e fases aprovados; somente elementos
decorativos animam. A visibilidade inicial vem da janela Tauri, com evento nativo
para fechamento/reabertura. Fechar continua destruindo a webview. A restauração
aguarda estado nativo e preferências; redução manual/sistema e Módulos cobertos
pelas Configurações interrompem a decoração.

Testes no seam de UI com snapshots cobrem visibilidade, reabertura, recriação,
limpeza, preferências e estabilidade do instrumento. Revisão por padrões e spec
sem apontamentos. Instrumentação limitada a uma marca de estado sem conteúdo
sensível; procedimento em [Trace da Coreografia combinada](../../../docs/motion-trace.md).

A verificação nativa e as medições de ausência de trabalho/CPU/fps/memória ainda
pertencem aos tickets 10 e 11; os testes de DOM não as comprovam.

Validação: `npm run check` sem erros/avisos; `npm run build` aprovado; 209 testes
de UI e 274 testes Rust aprovados. Integração de sockets executada fora do sandbox.
