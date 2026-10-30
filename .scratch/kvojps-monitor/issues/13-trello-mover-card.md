# 13: Trello: mover card otimista

**What to build:** clico num card e depois na lista de destino; o card muda na hora. Se a API falhar, ele volta ao lugar com "Não foi possível mover"; se der certo, o quadro é relido e o relógio de 30 s zera. Ver spec, "Módulo Trello".

**Blocked by:** 12 (Trello: conectar e ver o quadro)

**Status:** ready-for-agent

- [ ] Seleção de card + clique na lista de destino na UI
- [ ] `PUT /1/cards/{id}` com `idList`
- [ ] Atualização otimista com reversão e mensagem em falha (teste explícito)
- [ ] Em sucesso, relê o quadro e zera o relógio de 30 s
- [ ] Testes de UI do fluxo de seleção e do comando emitido
