# Sessões do Claude Code

`recorded.json`: saída de `claude agents --json --all` capturada nesta máquina em
31/10/2026. Contém somente os metadados emitidos pelo comando, sem conversas.
Os demais estados são variantes explícitas desse esquema nos testes, pois não
estavam disponíveis durante a captura. `startedAt` usa milissegundos Unix.

Os testes de hooks usam variantes sintéticas explícitas do contrato de stdin do
[Claude Code](https://code.claude.com/docs/en/hooks) e do
[Codex](https://learn.chatgpt.com/docs/hooks), consultados em 31/10/2026.
As variantes incluem conteúdo sentinela para verificar que somente metadados
atravessam o socket; nenhum hook foi instalado nas configurações reais durante
os testes. A reconciliação com polling usa as capturas desta pasta.

Caminho, projeto, nome de sessão e identificadores foram substituídos por exemplos. Os snapshots de UI e as expectativas dos testes usam os mesmos exemplos.
