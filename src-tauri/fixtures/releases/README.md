# Releases do GitHub

Capturados nesta máquina em 2026-10-31.

- `none.json`: `gh api repos/Kvojps/kvojps-monitor/releases/latest`. O repositório
  ainda não tem release publicada; a API responde 404 com este corpo.
- `recorded.json`: `gh api repos/tauri-apps/tauri/releases/latest`, reduzido aos
  campos de metadados do release (sem `body`, `assets` nem `author`). É o formato
  real da resposta de `releases/latest`.
- `newer.json`: o mesmo registro com `tag_name`, `name`, `html_url` e `url`
  reescritos para `Kvojps/kvojps-monitor` `v0.2.0`, já que o repositório do app
  ainda não tem releases reais.
