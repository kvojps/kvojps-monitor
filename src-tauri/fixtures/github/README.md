# Respostas GraphQL do GitHub

Capturadas em 31/10/2026 nesta máquina com `gh api graphql`, usando a autenticação já existente. Nenhum token foi gravado. Login, ID, nomes de repositórios e títulos foram substituídos por valores fictícios antes do versionamento; datas, contagens e estrutura vieram da API.

- `viewer.json`: calendário e universo dos repositórios com push mais recente. Os metadados extras de branch foram omitidos por não serem necessários à primeira consulta do adapter e permitirem associação com atividade privada registrada na pesquisa. O calendário mantém as contagens da captura anonimizada; não contém conteúdo nem associação de contribuições a repositórios privados.
- `history.json`: `history(author:, since:, until:)` da branch padrão e total do autor sem janela.
- Os testes derivam janela vazia e empate modificando somente contagem e data. A virada do ano valida os limites enviados à porta HTTP com relógio falso.

As fixtures de UI em `src/lib/github/fixtures/` são publicadas pelo núcleo e conferidas nos testes. Para regravá-las: `UPDATE_FIXTURES=1 cargo test --manifest-path src-tauri/Cargo.toml github`.

Snapshots ficam somente em memória nesta fatia. A política de cache/backoff pertence ao ticket 09; nenhum título de commit deve ser persistido no cache em disco.

- `validation.json`: captura de `gh api --include graphql -f 'query=query { viewer { login } }'` em 31/10/2026. Preserva o valor real de `X-OAuth-Scopes` e o corpo da resposta, com login anonimizado para `exemplo`. Os testes derivam escopos ausentes/parciais e corpos inválidos como variantes controladas desta captura.
