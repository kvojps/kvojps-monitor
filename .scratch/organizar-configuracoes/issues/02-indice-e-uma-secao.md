# 02: Índice + uma seção

**What to build:** as Configurações passam a ter um índice à esquerda, com os grupos "App" (Geral, Aparência) e "Módulos" (IA, Agentes, GitHub, Trello, Rede), e só a seção escolhida à direita. A grade de cartões desaparece.
- Entrar pelo botão do topo abre Geral, e "Configurar" de um Módulo no Painel abre a seção daquele Módulo.
- O item ativo usa `aria-current` e o foco vai para o título da seção.
- "← Painel" volta ao Painel de qualquer seção.
- Até 650 px, o índice vira uma fileira horizontal rolável acima da seção, sem rolagem horizontal da página.
- Sistema não entra no índice.

Ver `../spec.md`, histórias 1–6, 13, 34, 37 e 38.

**Blocked by:** 01 (Peças compartilhadas e Rede como seção própria)

**Status:** ready-for-agent

- [ ] Índice com dois grupos e uma seção visível por vez
- [ ] Botão do topo abre Geral; "Configurar" de IA, Agentes, GitHub e Trello no Painel abre a seção certa
- [ ] `aria-current` no item ativo, foco no título da seção selecionada e navegação por teclado
- [ ] Layout estreito com o índice em fileira horizontal
- [ ] Sistema ausente do índice; "← Painel" funciona
- [ ] Testes via Painel + Core falso + fixtures (seam do teste atual das Configurações)
