# 05: Trello fiel ao B

Type: task
Status: resolved
Blocked by: 01, 02

**What to build:** reescrever o resumo do Trello e o quadro completo a partir do protótipo (`CompactPrototype.svelte` e `TrelloBoardPrototype.svelte`), com dados reais, até ficarem iguais ao B e ao quadro aprovado no app nativo.

- [x] Cabeçalho “TRELLO” em maiúsculas com ↻/⚙ por **ícone**; remover os botões textuais “Atualizar Trello”/“Configurar Trello”. **Reverte** a decisão do ticket 06 anterior.
- [x] Nome do quadro com “N cards · M listas”, grade de contagens por lista, prévia “Primeiro card · Lista” com título real e etiquetas com as cores reais, “Abrir quadro completo ↗”. Contagens acompanham movimentação e reversão.
- [x] Quadro completo fiel a `TrelloBoardPrototype.svelte` e `trello-board-prototype.png`, mantendo movimentação, reversão, seleção preservada, arraste, caminho por teclado e abertura externa já implementados.
- [x] Falha, offline, credencial e antigo como linha condicional no corpo, no vocabulário do B.
- [x] Testes de UI por fixtures; sem rolagem em 1100 × 700 e 1000 × 700 com as fixtures do B.
- [x] Captura no app nativo lado a lado com `compact-B.png` e o quadro aprovado, e **aprovação do usuário** registrada aqui.

## Comments

Origem: [mapa de fidelidade](../map.md), decisões Q10, Q16, Q19 da entrevista de 2026-10-31.

2026-11-01 (implementação): `TrelloModule.svelte` reescrito com a marcação e o CSS do protótipo. Resumo: “TRELLO” como `h2` com ↻/⚙ por ícone e dica por ponteiro/foco (Escape dispensa a dica), como no GitHub; saíram os botões textuais “Atualizar Trello”/“Configurar Trello” (os nomes acessíveis continuam), o subtítulo “Fonte · atualizado” do Painel e a data de entrega na prévia. Nome do quadro com “N cards · M listas”, grade de contagens em duas colunas, prévia “Primeiro card · Lista” com título e etiquetas nas cores reais (20 × 3 px, como no B), “Abrir quadro completo ↗”. Quadro completo no layout de `TrelloBoardPrototype.svelte`: “Quadro Trello” + “Fechar ×”, nome do quadro e contagens com a dica “Selecione um card e a lista de destino, ou arraste entre colunas.”, barra de movimentação, status, colunas com etiquetas no topo, título, “Sem vencimento” ou a entrega (vencido/concluída, exigência da v1) e “Abrir no Trello ↗” sob o card selecionado (substitui o link da barra). “Nenhum card nesta lista” como no protótipo; sem a linha de simulação de falha. Estados condicionais como linhas `small` no topo do corpo: “⚠ Trello não conectado” (+ token pendente), escolher quadro, sem leitura, “⚠ Credencial inválida · Reconectar”, “⚠ Falha ao atualizar há X”, “Atualizado há X”, “Atualizando…” e a falha de movimentação com o quadro fechado. Movimentação, reversão, seleção preservada, arraste, teclado e abertura externa intactos. Testes atualizados, incluindo “nada além do B no estado normal” e o layout do quadro (244 passam), `svelte-check` sem erros. Pendente: rolagem em 1100 × 700 e 1000 × 700 e captura nativa com aprovação do usuário.

## Answer

Aprovado pelo usuário em 2026-11-01 após conferência no app nativo (`npm run dev:fixtures`) lado a lado com `compact-B.png` e o quadro aprovado.

- Resumo do Trello reescrito a partir do protótipo: “TRELLO” com ↻/⚙ por ícone e dica, nome do quadro com “N cards · M listas”, grade de contagens em duas colunas, prévia “Primeiro card · Lista” com etiquetas nas cores reais, “Abrir quadro completo ↗”.
- Fora do estado normal: botões textuais de atualizar/configurar, subtítulo “Fonte · atualizado” e data de entrega na prévia.
- Quadro completo no layout de `TrelloBoardPrototype.svelte`, com entrega ou “Sem vencimento” por card e “Abrir no Trello ↗” sob o card selecionado; movimentação, reversão, arraste, teclado e abertura externa preservados.
- Estados condicionais como linhas `small` no topo do corpo: não conectado, escolher quadro, sem leitura, credencial inválida, falha, antigo, atualizando e falha de movimentação.
- Sem divergências da spec.

## Divergência aprovada (2026-11-01)

Na [Conferência nativa do topo e cartão IA e Agentes](09-native-review-top-and-ai-agents.md), o usuário pediu, contra o B:

- **Sem prévia “Primeiro card · Lista”** no resumo. As etiquetas nas cores reais ficam só no quadro completo. “Quadro sem cards” aparece só quando o quadro não tem nenhum card.
- **Nomes das listas em uma linha** na grade de contagens, com reticências; o nome inteiro aparece na dica (`title`).
- “N cards · M listas” e “Atualizado há X” na fonte normal, como no B.
