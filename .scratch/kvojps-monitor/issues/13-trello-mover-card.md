# 13: Trello: mover card otimista

**What to build:** clico num card e depois na lista de destino; o card muda na hora. Se a API falhar, ele volta ao lugar com "Não foi possível mover"; se der certo, o quadro é relido e o relógio de 30 s zera. Ver spec, "Módulo Trello".

**Blocked by:** 12 (Trello: conectar e ver o quadro)

**Status:** done

- [x] Seleção de card + clique na lista de destino na UI
- [x] `PUT /1/cards/{id}` com `idList`
- [x] Atualização otimista com reversão e mensagem em falha (teste explícito)
- [x] Em sucesso, relê o quadro e zera o relógio de 30 s
- [x] Testes de UI do fluxo de seleção e do comando emitido

## Notas de implementação

- O PUT leva também `pos=bottom`, para o card ficar no fim da lista de destino, onde a UI otimista o coloca; sem isso o Trello mantém a posição antiga e o card pularia na releitura.
- O núcleo só aceita card e lista que estão no quadro mostrado (os ids vindos do painel não chegam à URL de outra forma) e recusa nas mesmas pausas do polling: sem conexão, em backoff ou com credencial recusada. Um 401/403 no PUT marca a credencial como inválida. A UI não deixa selecionar com o Módulo pausado ou credencial recusada.
- Em sucesso, o núcleo aplica o movimento nos dados antes de reler, para o card não voltar se a releitura falhar.
- A UI mantém o card no destino até chegar o quadro relido, para não piscar de volta se a resposta do comando chegar antes do snapshot.
- Um card selecionado sai da seleção com novo clique nele ou clique na própria lista (Esc já fecha o Painel). Com um card selecionado, clicar em qualquer ponto de outra lista (inclusive sobre um card dela) move para lá.
