# Intensificar o movimento visual do Painel

Type: prototype
Status: resolved
Map: [Painel cinematográfico para trabalho](../map.md)
Blocked by: 01, 02

## Question

Que movimentos adicionais tornam o Painel aprovado mais vivo e cinematográfico, mantendo sua utilidade no trabalho?

Explorar no protótipo H alternativas de movimento além dos anéis e do brilho atuais, para avaliação ao vivo pelo usuário. Comparar onde o movimento acontece, sua intensidade, ritmo e continuidade; preservar a composição e a revisão de Rede aprovadas. Possibilidades de exploração incluem fluxos nas faixas, varreduras decorativas e transições de interação, sem pressupor qual alternativa será escolhida.

Distinguir decoração de atividade real: não inventar medições, variar valores artificialmente ou sugerir coleta/atividade inexistente. Preservar leitura, foco, estados e ações acessíveis; não piscar texto ou alertas. Manter redução manual e do sistema, ausência de trabalho de animação com Painel oculto e os critérios de desempenho acordados em [Consolidar a especificação visual validada do Painel](03-visual-specification.md). Se a proposta exigir rever esses critérios, discutir explicitamente antes de considerar aprovada.

Usar a skill prototype; manter exploração descartável, sem implementar o redesign em produção. Registrar a alternativa validada e sua referência como asset; a consolidação final incorporará a decisão, explicitando quais regras de movimento anteriores forem substituídas.

## Comments

### Origem

Pedido explícito do usuário após aprovação da revisão visual de Rede: “Quero abrir um ticket aq pra adicionarmos mais movimentação ao painel”. Este ticket abre nova exploração; não representa aprovação prévia de efeitos específicos.

### Primeira exploração de movimento — aguardando avaliação

Skill prototype aplicada sobre a composição H, preservando Rede e os dados. Em `cinematic-prototype.html?variant=H&motion=flow`, o seletor Movimento compara Referência aprovada (`baseline`), Fluxo nas faixas (`flow`), Varredura nas bordas (`scan`) e Coreografia combinada (`combined`). O último combina fluxos, bordas, deriva decorativa nos títulos de estação e resposta dos botões ao apontador. Os efeitos não representam atividade real.

Fonte descartável: [Prototype.svelte](../../../src/lib/cinematic-prototype/Prototype.svelte). Redução manual/do sistema e pausa com documento oculto abrangem também os pseudoelementos novos; mudanças da preferência do sistema são acompanhadas. Sem alterações na UI de produção. `npm run check`: zero erros e avisos. Avaliação visual e desempenho na máquina do usuário pendentes; nenhuma alternativa aprovada. Captura final em branch local de protótipo após a escolha.

## Answer

O usuário escolheu explicitamente **Coreografia combinada** após comparar as alternativas no protótipo. A referência aprovada é a composição H em `cinematic-prototype.html?variant=H&motion=combined`, que estava na branch local `prototype/cinematic-panel-composition` (apagada em 2026-11-01), em `src/lib/cinematic-prototype/Prototype.svelte`.

Adotar, além dos anéis e brilho existentes: fluxo luminoso nas faixas superiores (ciclo de 3,5 s, com fases deslocadas), varredura decorativa nas bordas dos Módulos (7 s, defasada entre colunas), deriva de fundo nos títulos das estações (9 s) e deslocamento de 2 px dos botões de navegação ao apontador (transição de 150 ms). Faixas e bordas são decoração, sem significado de coleta ou atividade. Dados, gráficos, texto e alertas permanecem estáveis. A composição e a revisão de Rede aprovadas permanecem.

Esta escolha amplia o repertório de movimento aprovado em [Validar linguagem visual, movimento e leitura no trabalho](02-motion-and-readability.md); preserva redução manual e do sistema, pausa quando oculto, legibilidade e ausência de medições inventadas. Os critérios de desempenho registrados em [Consolidar a especificação visual validada do Painel](03-visual-specification.md) continuam vigentes e ainda precisam ser medidos; aprovação visual não comprova o orçamento de CPU nem a fluidez.

A consolidação final deve incorporar esta decisão e explicitar a ampliação das regras anteriores. Não há nova questão identificada nem implementação em produção nesta resolução.
