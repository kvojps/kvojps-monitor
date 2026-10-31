# Respostas da API do Trello

`capture.py` grava as três respostas usadas pelo adapter: `GET /1/members/me`, `GET /1/members/me/boards?filter=open` e `GET /1/boards/{id}/lists?cards=open` do quadro escolhido. A key e o token são pedidos no terminal sem eco e vão só no header `Authorization`; nada disso é gravado. Nomes de quadros, listas e cards, nomes de etiquetas, IDs e URLs são substituídos por valores fictícios (`Quadro 1`, `Lista 1`, `Card 1.1`); datas de entrega, cores de etiquetas, ordem e estrutura vêm da API.

Para capturar de novo: `python3 src-tauri/fixtures/trello/capture.py`.

**Estado atual:** `me.json`, `boards.json` e `lists.json` são provisórios, montados a partir do formato documentado da API (developer.atlassian.com) no mesmo formato que o script grava, até a primeira captura real substituí-los.

Os testes derivam vencido, concluído, etiquetas sem cor e cards futuros como variações controladas de `lists.json`, com relógio falso. 401/403, 5xx, 429 com `Retry-After` e falhas de transporte são simulados na porta HTTP, sem corpo de resposta.

As fixtures de UI em `src/lib/trello/fixtures/` são publicadas pelo núcleo e conferidas nos testes. Para regravá-las: `UPDATE_FIXTURES=1 cargo test --manifest-path src-tauri/Cargo.toml trello`.

O cache em disco guarda quadro, listas, URLs curtas, cores de etiquetas e datas; nunca títulos de cards, nomes de etiquetas, key ou token.
