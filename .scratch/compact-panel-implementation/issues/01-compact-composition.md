# 01: Composição compacta e janela adaptada à área útil

**What to build:** abrir o Painel na composição B — Núcleo lateral, com janela de aproximadamente 1100 × 700 em dimensões lógicas, respeitando área útil e escala. Estabelecer a estrutura compacta para os resumos dos tickets seguintes, preservando Configurações e o contexto do usuário.

**Blocked by:** None (can start immediately).

**Status:** resolved

- [x] Substituir a janela de 95% da área útil pelo alvo aprovado, limitado à área disponível, sem maximizar nem entrar em tela cheia.
- [x] Agrupar visualmente IA e Agentes à esquerda; posicionar GitHub/Rede acima e Trello/Sistema abaixo nas outras duas colunas, conforme as proporções de B, sem fundir Módulos ou snapshots.
- [x] Remover título/subtítulo geral, navegação redundante por Módulos e faixas de Estação; manter cabeçalhos compactos e linguagem LCARS. Não incorporar controles, mensagens ou dimensões de depuração do laboratório.
- [x] Manter Configurações acessível no topo como área separada, com formulários e ações reais, primeira execução, retorno e restauração do foco/contexto. Preservar Âncora na reabertura sem depender do trilho removido.
- [x] Oferecer fallback responsivo com reorganização e rolagem quando necessário; usar como referência duas colunas até 950 px e uma até 650 px. Não cortar conteúdo, esconder alertas ou reduzir automaticamente fontes.
- [x] Preservar temas escuro/claro/sistema, redução manual/sistema e movimento aprovado nos elementos remanescentes; pausar decoração com Painel oculto.
- [x] Testar composição, retorno de Configurações e Âncora, além do conjunto de Módulos efetivamente visíveis: Configurações/Painel oculto não tornam Módulos visíveis; mudanças de interseção não multiplicam coleta nem alteram cadências.
- [x] Manter o seam único de E/S, fontes, consentimentos e guardrails. Esta etapa permite rolagem enquanto os resumos forem compactados nos tickets seguintes; a ausência de rolagem da composição completa será verificada na integração.

## Comments

Origem: especificação aprovada do Painel compacto, complementada pela especificação visual cinematográfica e pela spec da v1. Divisão de implementação aprovada pelo usuário em 2026-10-31. A referência visual é B — Núcleo lateral; aprovação do protótipo não comprova comportamento nativo.

## Answer

Implementado em 2026-10-31:

- Janela alvo de 1100 × 700 lógicos, convertida pela escala e limitada à área útil física; configuração inicial alinhada ao alvo.
- Composição B com proporções 1,2/1/1, IA e Agentes na lateral esquerda, cabeçalhos compactos, Configurações no topo e fallback em 950/650 px com rolagem.
- Âncora registrada pelo foco nas ações locais dos Módulos; restauração na abertura e retorno de Configurações mantidos. Snapshots, formulários, primeira execução e alertas dos Módulos permanecem independentes.
- Visibilidade exige interseção, Painel nativo visível, documento visível e área Painel; conjuntos repetidos não são reportados novamente. Cadências e fontes do núcleo não foram alteradas.
- Validação: 209 testes da UI, 271 testes unitários Rust e 3 testes de integração de hooks aprovados; `npm run check` sem erros/avisos; `npm run build` aprovado. Testes de sockets Unix executados fora do sandbox após restrição de permissão.
- Revisão com a skill code-review: Standards 0 achados; Spec 0 achados.

Limites: a comparação visual integrada, escala/área útil no GNOME/Wayland e medições nativas ficam no [ticket 07](07-integrated-native-validation.md). Esta etapa mantém rolagem enquanto os resumos dos tickets 02–06 são implementados; não comprova ausência de rolagem da composição final.

**Reaberto em 2026-10-31 por fidelidade ao protótipo:** no app nativo, o resultado diverge do design e do conteúdo do protótipo B. A correção segue em [compact-panel-fidelity](../../compact-panel-fidelity/map.md).
