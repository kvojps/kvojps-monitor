# 02: Índice + uma seção

**What to build:** as Configurações passam a ter um índice à esquerda, com os grupos "App" (Geral, Aparência) e "Módulos" (IA, Agentes, GitHub, Trello, Rede), e só a seção escolhida à direita. A grade de cartões desaparece.
- Entrar pelo botão do topo abre Geral, e "Configurar" de um Módulo no Painel abre a seção daquele Módulo.
- O item ativo usa `aria-current` e o foco vai para o título da seção.
- "← Painel" volta ao Painel de qualquer seção.
- Até 650 px, o índice vira uma fileira horizontal rolável acima da seção, sem rolagem horizontal da página.
- Sistema não entra no índice.

Ver `../spec.md`, histórias 1–6, 13, 34, 37 e 38.

**Blocked by:** 01 (Peças compartilhadas e Rede como seção própria)

**Status:** done

- [x] Índice com dois grupos e uma seção visível por vez
- [x] Botão do topo abre Geral; "Configurar" de IA, Agentes, GitHub e Trello no Painel abre a seção certa
- [x] `aria-current` no item ativo, foco no título da seção selecionada e navegação por teclado
- [x] Layout estreito com o índice em fileira horizontal
- [x] Sistema ausente do índice; "← Painel" funciona
- [x] Testes via Painel + Core falso + fixtures (seam do teste atual das Configurações)

**Notas da implementação:**

- As Boas-vindas continuam acima da seção selecionada, como passo intermediário até o ticket 03 colocá-las sozinhas no lado direito.
- O foco vai para o `h3` da seção (recebe `tabindex="-1"` ao ser selecionada). Setas, Home e End movem o foco entre os itens do índice; Enter ou Espaço seleciona.
- `focus("system")` cai em Geral, já que Sistema não entra no índice.
