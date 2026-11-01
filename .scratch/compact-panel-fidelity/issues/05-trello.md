# 05: Trello fiel ao B

Type: task
Status: ready-for-agent
Blocked by: 01, 02

**What to build:** reescrever o resumo do Trello e o quadro completo a partir do protótipo (`CompactPrototype.svelte` e `TrelloBoardPrototype.svelte`), com dados reais, até ficarem iguais ao B e ao quadro aprovado no app nativo.

- [ ] Cabeçalho “TRELLO” em maiúsculas com ↻/⚙ por **ícone**; remover os botões textuais “Atualizar Trello”/“Configurar Trello”. **Reverte** a decisão do ticket 06 anterior.
- [ ] Nome do quadro com “N cards · M listas”, grade de contagens por lista, prévia “Primeiro card · Lista” com título real e etiquetas com as cores reais, “Abrir quadro completo ↗”. Contagens acompanham movimentação e reversão.
- [ ] Quadro completo fiel a `TrelloBoardPrototype.svelte` e `trello-board-prototype.png`, mantendo movimentação, reversão, seleção preservada, arraste, caminho por teclado e abertura externa já implementados.
- [ ] Falha, offline, credencial e antigo como linha condicional no corpo, no vocabulário do B.
- [ ] Testes de UI por fixtures; sem rolagem em 1100 × 700 e 1000 × 700 com as fixtures do B.
- [ ] Captura no app nativo lado a lado com `compact-B.png` e o quadro aprovado, e **aprovação do usuário** registrada aqui.

## Comments

Origem: [mapa de fidelidade](../map.md), decisões Q10, Q16, Q19 da entrevista de 2026-10-31.
