# 09: Conferência nativa do topo e cartão IA e Agentes

Type: task
Status: resolved
Blocked by: None (can start immediately).

## Question

O topo, a grade e o cartão IA e Agentes implementados no ticket 02 estão iguais ao B no app nativo?

- [x] `npm run dev:fixtures`, Painel aberto, lado a lado com `.scratch/compact-panel/assets/compact-B.png`, temas escuro e claro.
- [x] Sem rolagem em 1100 × 700 e 1000 × 700 com as fixtures do B.
- [x] Estados condicionais conferidos com `KVOJPS_FIXTURES` (por exemplo `ai=claude-failure`, `ai=offline`, `agents=empty`, `agents=mixed`).
- [x] Detalhe Sessões de agente, foco, Escape e movimento (orbitais, barra do topo, brilho) com redução e Painel oculto.
- [x] **Aprovação do usuário** registrada aqui; ajustes viram correções neste ticket.

## Comments

Origem: ticket 02 (2026-11-01); o agente não conseguiu capturar a tela nativa.

## Answer

Aprovado pelo usuário em 2026-11-01, depois das correções abaixo. Conferido no app nativo (`npm run dev:fixtures`), com capturas de `scripts/capture-panel.sh`.

- **Sem rolagem:** fixtures do B em 1100 × 700 e 1000 × 700, temas escuro e claro: `overflowing` vazio, `scrollHeight` = `clientHeight` = 641. O mesmo vale para os estados condicionais `ai=claude-failure,agents=empty`, `ai=offline,agents=mixed` e `ai=stale,agents=waiting`, todos como linhas `small` no vocabulário do B.
- **Fonte:** textos `small` com horas e datas estavam em monoespaçada, e o B usa a fonte normal. Corrigido na IA (“5 h · reinicia”, “Reinício”, “Leitura”) e, a pedido do usuário, em GitHub, Rede, Trello e Sistema. Números grandes, contagens e taxas continuam monoespaçados, como no B.
- **IA e Agentes ocupa o espaço do cartão:** o resumo de Agentes toma a altura livre acima e abaixo dele, e o aviso “aguardando você” fica no fim do cartão, sem vão.
- **Trello** (pedido do usuário, divergência do B registrada em [Trello fiel ao B](05-trello.md#divergência-aprovada-2026-11-01)): nomes das listas em uma linha; prévia do primeiro card removida.
- **Pendente:** “⚠ Uso alto na janela de 5 h” no estado B continua em [Uso alto no estado B](10-high-usage-in-state-b.md).
