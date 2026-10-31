# Respostas GraphQL do GitHub

Capturadas em 31/10/2026 nesta máquina com `gh api graphql`, usando a autenticação já existente. Nenhum token foi gravado. Login, ID, nomes de repositórios e títulos foram substituídos por valores fictícios antes do versionamento; datas, contagens e estrutura vieram da API.

- `viewer.json`: calendário e universo dos 20 repositórios com push mais recente. Os dados extras de branch presentes na captura não são necessários à primeira consulta do adapter.
- `history.json`: `history(author:, since:, until:)` da branch padrão e total do autor sem janela.
- Os testes derivam janela vazia e empate modificando somente contagem e data. A virada do ano valida os limites enviados à porta HTTP com relógio falso.

As fixtures de UI em `src/lib/github/fixtures/` são publicadas pelo núcleo e conferidas nos testes. Para regravá-las: `ATUALIZAR_FIXTURES=1 cargo test --manifest-path src-tauri/Cargo.toml github`.

Snapshots ficam somente em memória nesta fatia. A política de cache/backoff pertence ao ticket 09; nenhum título de commit deve ser persistido no cache em disco.
