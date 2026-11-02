# 26: Adotar a revisão aprovada do visual do Painel

**What to build:** incorporar à fonte da verdade da v1 a revisão visual aprovada, deixando explícitos o comportamento e os limites que orientarão a implementação, sem alterar o app neste ticket.

**Blocked by:** None (can start immediately).

**Status:** done

Origem: proposta “Proposta de revisão do visual do Painel”, consolidada em 01/11/2026 no mapa “Aprimorar a identidade tech do Painel”; adoção e decomposição aprovadas pelo usuário nesta rodada de to-tickets. As decisões resolvidas do mapa preservam as referências dos protótipos locais.

- [x] Incorporar o texto proposto de fonte da verdade, composição, Faixa de bordo e Movimento, mantendo B como referência e explicitando somente as exceções aprovadas.
- [x] Incorporar todos os critérios de aceite propostos: composição integrada, concorrência, operações reais, indisponibilidade, acessibilidade, movimento, desempenho nativo e privacidade.
- [x] Preservar grade, fontes de dados, privilégios, coleta, estados canônicos, notificações e configurações existentes; controles de intensidade e duração continuam exclusivos dos protótipos.
- [x] Vincular as decisões de origem e os tickets de implementação sem modificar ou encerrar os tickets do mapa.
- [x] Conferir consistência com os requisitos existentes, inclusive destruição da webview ao fechar e metas de desempenho; divergências não aprovadas voltam ao usuário antes de qualquer alteração.

## Notas da implementação

- Revisão aprovada incorporada à [spec da v1](../spec.md#visual-do-painel): composição, Faixa de bordo, Órbita contínua e critérios de aceite. Incluídos vínculos para decisões de origem e tickets 27–33, sem alterar o app ou os tickets do mapa.
- Conferida a continuidade da destruição da webview ao fechar, das metas nativas e dos guardrails existentes. Adoção documental não equivale a validação nativa.
- Validação: `npm run check` sem erros ou avisos; 327 testes de UI, 281 testes unitários Rust e 3 testes de integração passaram; `git diff --check` e destinos dos links locais conferidos. A suíte Rust precisou executar fora do sandbox para criar sockets locais nos testes de integração.
- Revisão em dois eixos (`code-review`), contra o commit inicial `852d288a9b191ecee97d66a8929c1462339d6a8b`: Standards sem achados; Spec sem achados.
