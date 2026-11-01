# 11: Preenchimento vertical do cartão IA e Agentes

Type: prototype
Status: resolved
Blocked by: None (can start immediately).

## Question

Como o cartão IA e Agentes ocupa toda a altura disponível sem vãos entre os blocos (anéis da IA, resumo de Agentes, aviso “aguardando você”) nem vão no fim quando não há aviso?

- A altura do cartão vem da soma das linhas GitHub/Trello e Rede/Sistema; o conteúdo do cartão é mais baixo que isso.
- Hoje (ticket 09) o resumo de Agentes toma a sobra acima e abaixo dele: com aviso, ficam dois vãos; sem aviso, sobra espaço no fim.
- O próprio `compact-B.png` deixa cerca de 100 px vazios no fim do cartão.
- A linha condicional de uso alto ([Uso alto no estado B](10-high-usage-in-state-b.md)) também muda a altura do conteúdo.

## Comments

Origem: [Conferência nativa do topo e cartão IA e Agentes](09-native-review-top-and-ai-agents.md), 2026-11-01. O usuário não gostou dos vãos verticais e do espaço vazio no fim sem aviso.

Opções para prototipar:

1. **Instrumentos que crescem (recomendada):** os anéis da IA e o anel de Agentes aumentam conforme a altura livre (unidades de container query, com tamanho mínimo e máximo), e a legenda ganha mais espaço entre os itens. A sobra vira instrumento, não vão; o conteúdo continua o mesmo do B.
2. **Espaço do aviso sempre reservado:** sem sessão aguardando, o espaço do aviso mostra uma linha neutra (“Nenhuma sessão aguardando você”). Assim o fim do cartão nunca fica vazio, mas aparece um texto que o B não tem.
3. **Lista curta de sessões recentes** abaixo do resumo, preenchendo a sobra. Preenche de verdade, mas acrescenta conteúdo que o B não mostra (fere “nada além do B”).
4. **Grade mais baixa:** reduzir as alturas de GitHub/Trello/Rede/Sistema até o cartão IA e Agentes ditar a altura. Mexe em Módulos já aprovados e deixa vazio o fundo do Painel.

Conferir no app nativo com e sem aviso, com e sem uso alto, em 1100 × 700 e 1000 × 700, sem rolagem.

## Answer

Aprovada pelo usuário em 2026-11-01: **opção 1, instrumentos que crescem** (variante A do protótipo).

- A sobra de altura do cartão vira instrumento: metade para os dois anéis da IA (lado a lado), metade para o anel de Agentes. Diâmetro mínimo 94 px (o do B), máximo 180 px, nunca mais largo que a coluna; o número do centro cresce junto, e a legenda de Agentes ganha espaço entre os itens.
- Os blocos ficam em sequência, sem vão entre eles: o resumo de Agentes não toma mais a sobra acima e abaixo. O conteúdo continua o do B; nada é acrescentado.
- Linhas condicionais (aviso “aguardando você”, uso alto, falhas) entram na conta: o Painel remede o cartão quando elas aparecem ou somem.
- Implementação: `src/lib/instrumentFill.ts` (`dialSize`, testado) e `ResizeObserver` em `src/lib/Panel.svelte` que define `--dial` no cartão; `AiModule` e `AgentsModule` usam `--dial`.
- Protótipo (5 variantes: atual, A, B aviso reservado, C lista de sessões, D fiel ao B) no branch local `prototype/ai-agents-vertical-fill` (commit `bba9a65`), não enviado.
- Conferência sem rolagem nas fixtures do B em 1100/1000 × 700 fica para a [Validação integrada nativa](07-integrated-native-validation.md): o app aberto durante a decisão rodava sem a feature `fixtures`, e a captura não respondeu.
