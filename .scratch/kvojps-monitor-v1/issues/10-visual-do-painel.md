# Layout e visual do Painel

Type: prototype
Status: resolved
Blocked by: 01, 02, 03, 04, 05
Map: [kvojps-monitor v1](../map.md)

## Question

Como o Painel deve se parecer e se comportar: tamanho, navegação entre Módulos (abas, lista rolável, grid), densidade de informação de cada Módulo, Configurações e o estilo minimalista/tech escuro?

## Answer

Escolhida a **variante A: trilho lateral, um Módulo por vez**. Usuário escolheu A sem ressalvas; o resto segue o protótipo como está.

- **Tamanho**: janela compacta de ~440×600, sem decoração, cantos arredondados, posicionada pelo Mutter.
- **Navegação**: trilho vertical de ícones à esquerda (IA, Agentes, GitHub, Trello, Rede, Sistema), um Módulo em detalhe por vez à direita; Configurações no rodapé do trilho. Cabeçalho com nome do Módulo e hora da última atualização. O ícone de Agentes ganha o ponto de acento enquanto alguma sessão está Aguardando você (espelha o Indicador). Esc fecha o Painel.
- **Densidade**: cada Módulo em detalhe, rolável se precisar — IA: barras por Limite de uso com %, reinício, plano e fonte/idade do dado; Agentes: lista com Estado da sessão (cor), projeto, nome, detalhe e tempo, a que aguarda destacada; GitHub: total, calendário do ano e repo destaque; Trello: listas em colunas, mover card clicando no card e depois na lista de destino; Rede: Segurança e Qualidade da rede como pílulas, métricas, gráfico de tráfego, tráfego por processo, botão de teste; Sistema: barras de CPU, RAM, disco e bateria.
- **Estilo**: escuro por padrão seguindo o tema do sistema (claro também suportado), acento verde-água (#5eead4 escuro / #0d9488 claro), números em monoespaçada tabular, rótulos pequenos em maiúsculas, cores semânticas só para estados (ok/atenção/falha).
- **Memória do Painel**: não medida — o protótipo é HTML, não Tauri. Vira requisito da spec: medir no primeiro esqueleto Tauri e destruir a webview ao fechar se passar de ~150 MB em repouso.
- O conteúdo das Configurações no protótipo é provisório até [Fluxo de credenciais nas Configurações](15-fluxo-de-credenciais.md).

Protótipo: branch local `prototype/painel-visual` (commit `8ca6cad`, apagada em 2026-11-01), arquivo `prototype/painel-visual/index.html`.

## Comments

### 2026-10-31 — Reabertura da direção visual

O usuário autorizou planejar uma nova direção para todo o Painel: visual futurista cinematográfico de intensidade máxima, inspirado em interfaces de filmes de super-heróis. Confirmou especificação validada por protótipo como destino, possibilidade de ampliar e reorganizar o Painel, preto profundo com ciano e azul elétrico, geometria técnica, anéis de telemetria e movimento contínuo com opção de redução. Condição explícita: preservar todas as funções e a utilidade atual como ferramenta de trabalho, com dados reais, ações acessíveis, legibilidade e fluidez.

O [mapa Painel cinematográfico para trabalho](../../cinematic-panel/map.md) conduz as decisões de substituição. Este registro autoriza explorar divergências visuais; a spec da v1 continua sendo a referência funcional e não foi alterada nesta abertura.

### 2026-10-31 — Complemento visual aprovado

O usuário aprovou integralmente a [Especificação visual — Painel cinematográfico](../../cinematic-panel/spec.md), concluindo [Consolidar a especificação visual validada do Painel](../../cinematic-panel/issues/03-visual-specification.md). O complemento passa a reger as substituições visuais delimitadas em sua matriz, incluindo composição, dimensões, navegação e seleção de dados; a spec da v1 permanece referência funcional. A implementação não foi executada neste esforço.
