Fixtures capturadas do SQLite e rollouts locais em 31/10/2026. IDs, caminhos e título anonimizados; somente metadados dos eventos foram preservados. O SQLite reproduz as colunas consultadas do schema local; as demais, inclusive conteúdo e credenciais, foram omitidas.

`rollout.jsonl`: início de turno. `task_complete.jsonl`: conclusão. `turn_aborted.jsonl`: interrupção. Falha (`event_msg/error`) é uma variante explícita do schema nos testes, pois não havia evento desse tipo nos rollouts locais.

A associação por descritores abertos foi verificada em `/proc` no host: um executável `codex` mantinha três rollouts abertos. O teste recria essa topologia com PID, descritor e caminhos anonimizados. Processos Codex sem rollout aberto não tornam sessões históricas ativas.
