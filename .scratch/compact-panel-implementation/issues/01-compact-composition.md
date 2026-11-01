# 01: Composição compacta e janela adaptada à área útil

**What to build:** abrir o Painel na composição B — Núcleo lateral, com janela de aproximadamente 1100 × 700 em dimensões lógicas, respeitando área útil e escala. Estabelecer a estrutura compacta para os resumos dos tickets seguintes, preservando Configurações e o contexto do usuário.

**Blocked by:** None (can start immediately).

**Status:** ready-for-agent

- [ ] Substituir a janela de 95% da área útil pelo alvo aprovado, limitado à área disponível, sem maximizar nem entrar em tela cheia.
- [ ] Agrupar visualmente IA e Agentes à esquerda; posicionar GitHub/Rede acima e Trello/Sistema abaixo nas outras duas colunas, conforme as proporções de B, sem fundir Módulos ou snapshots.
- [ ] Remover título/subtítulo geral, navegação redundante por Módulos e faixas de Estação; manter cabeçalhos compactos e linguagem LCARS. Não incorporar controles, mensagens ou dimensões de depuração do laboratório.
- [ ] Manter Configurações acessível no topo como área separada, com formulários e ações reais, primeira execução, retorno e restauração do foco/contexto. Preservar Âncora na reabertura sem depender do trilho removido.
- [ ] Oferecer fallback responsivo com reorganização e rolagem quando necessário; usar como referência duas colunas até 950 px e uma até 650 px. Não cortar conteúdo, esconder alertas ou reduzir automaticamente fontes.
- [ ] Preservar temas escuro/claro/sistema, redução manual/sistema e movimento aprovado nos elementos remanescentes; pausar decoração com Painel oculto.
- [ ] Testar composição, retorno de Configurações e Âncora, além do conjunto de Módulos efetivamente visíveis: Configurações/Painel oculto não tornam Módulos visíveis; mudanças de interseção não multiplicam coleta nem alteram cadências.
- [ ] Manter o seam único de E/S, fontes, consentimentos e guardrails. Esta etapa permite rolagem enquanto os resumos forem compactados nos tickets seguintes; a ausência de rolagem da composição completa será verificada na integração.

## Comments

Origem: especificação aprovada do Painel compacto, complementada pela especificação visual cinematográfica e pela spec da v1. Divisão de implementação aprovada pelo usuário em 2026-10-31. A referência visual é B — Núcleo lateral; aprovação do protótipo não comprova comportamento nativo.
