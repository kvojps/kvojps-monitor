# Respostas da API do Trello

`capture.py` grava as três respostas usadas pelo adapter: `GET /1/members/me`, `GET /1/members/me/boards?filter=open` e `GET /1/boards/{id}/lists?cards=open` do quadro escolhido. A key e o token são pedidos no terminal sem eco e vão só no header `Authorization`; nada disso é gravado. Nomes de quadros, listas e cards, nomes de etiquetas, IDs e URLs são substituídos por valores fictícios (`Quadro 1`, `Lista 1`, `Card 1.1`); datas de entrega, cores de etiquetas, ordem e estrutura vêm da API.

Para capturar de novo, num terminal interativo (não pelo `!` do Claude Code, que não tem terminal e levaria o token para a conversa): `python3 src-tauri/fixtures/trello/capture.py`.

Capturados em 31/10/2026 nesta máquina, com a conta do mantenedor: um quadro aberto, 8 listas (uma vazia) e 19 cards, com etiquetas em cores base e `_dark` e cards com entrega concluída, sem nenhuma data de entrega. `idOrganization` e `nodeId` das etiquetas também foram substituídos (o `nodeId` passou a apontar para o ID fictício da etiqueta).

Os testes derivam datas de entrega (vencida, concluída, futura), etiquetas sem cor e um segundo quadro aberto (para trocar de quadro) como variações controladas da captura, com relógio falso. 401/403, 5xx, 429 com `Retry-After` e falhas de transporte são simulados na porta HTTP, sem corpo de resposta.

As fixtures de UI em `src/lib/trello/fixtures/` são publicadas pelo núcleo e conferidas nos testes. Para regravá-las: `UPDATE_FIXTURES=1 cargo test --manifest-path src-tauri/Cargo.toml trello`.

O cache em disco guarda quadro, listas, URLs curtas, cores de etiquetas e datas; nunca títulos de cards, nomes de etiquetas, key ou token.
