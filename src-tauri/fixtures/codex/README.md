# Respostas do `codex app-server` e rollouts do Codex

## `app-server.jsonl`

Gravado em 31/10/2026 nesta máquina com codex-cli 0.166.0 (conta ChatGPT Plus) por `capture.py`: abre `codex app-server`, envia `initialize`, `initialized`, `account/read` e `account/rateLimits/read` e grava cada mensagem recebida em 15 s. E-mail, IDs de conta, de instalação e de créditos, nome do servidor e `codexHome` são substituídos por valores fictícios antes de gravar; percentuais, janelas, horários de reinício, `planType` e estrutura vieram do servidor. Para regravar: `python3 -I src-tauri/fixtures/codex/capture.py`.

Os testes derivam como variações controladas desta captura: conta sem login (`account: null`), erro JSON-RPC e ausência de resposta em `account/rateLimits/read`, processo morto e `codex` ausente. A notificação `account/rateLimits/updated` não chegou durante a captura (só aparece com um turno em andamento); os testes a montam com o `RateLimitSnapshot` gravado, que é o formato definido no schema (`codex app-server generate-json-schema`).

## `home/sessions/`

Raiz falsa de `~/.codex`. Contém só eventos `token_count` reais (contagens de tokens e `rate_limits`) lidos dos rollouts desta máquina em 31/10/2026; as demais linhas mantêm apenas `timestamp` e tipos, sem prompts, respostas, caminhos ou chamadas de ferramenta. A cópia automática de `~/.codex/sessions` para o repositório não foi feita: as linhas foram selecionadas e transcritas à mão.

- `2026/10/31/…`: último `token_count` da sessão mais recente (91% em 5 h, 48% semanal), com o horário real.
- `2026/10/30/…`: snapshot real de outra sessão (0% / 29%); o `timestamp` do evento foi derivado do nome do arquivo.
- `2026/10/20/…`: bucket `premium` real, com `primary` e `secondary` nulos; o `timestamp` também foi derivado do nome do arquivo.

As fixtures de UI em `src/lib/ai/fixtures/` são publicadas pelo núcleo e conferidas nos testes. Para regravá-las: `UPDATE_FIXTURES=1 cargo test --manifest-path src-tauri/Cargo.toml ai`.

O cache em disco guarda plano, fonte, idade e janelas; nunca e-mail, IDs de conta ou conteúdo de rollouts.

Os UUIDs presentes nos nomes dos rollouts foram substituídos por identificadores sintéticos na revisão do ticket 40; os horários e contadores não identificam mais uma sessão pelo seu UUID original.
