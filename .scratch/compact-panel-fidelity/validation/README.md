# Validação nativa — modo de fixtures

O modo de fixtures abre o app nativo (Tauri/WebKitGTK) em desenvolvimento com os
mesmos snapshots do protótipo B, para comparar o Painel de produção lado a lado com
`compact-B.png` na janela real.

## Executar

Feche qualquer instância do app antes (instância única: a nova entregaria o Painel à
antiga e sairia). Na raiz:

```sh
npm run dev:fixtures
```

Abra o Painel pelo Indicador. Para outra fixture em um ou mais Módulos, use
`módulo=fixture` separados por vírgula, com os nomes de `src/lib/<módulo>/fixtures/`:

```sh
KVOJPS_FIXTURES="ai=claude-failure,network=poor" npm run dev:fixtures
```

Módulos: `ai`, `agents`, `github`, `trello`, `network`, `system`. Padrão (B): IA
`claude-usage`, Agentes `states`, GitHub/Trello/Rede `full`, Sistema `notebook`.

## O que o modo garante

- Os snapshots seguem o caminho normal (`snapshot:<módulo>` e `initial_state`); a UI
  não tem caminho paralelo. Nenhum coletor inicia: sem leitura do sistema, rede,
  keyring, `~/.claude/settings.json` ou `~/.codex/hooks.json`, sem notificações e
  sem verificação de versão.
- Mover card, testar velocidade, velocidade automática, conectar/desconectar Claude,
  `oauth/usage`, token do GitHub, conexão do Trello e hooks respondem em memória.
  Abrir card, quadro, release ou logs continua abrindo o navegador/arquivos reais.
- Relógio da webview fixado em `1793448000000` (instante da fixture `claude-usage`),
  como nas validações anteriores.
- Preferências e entrada de autostart ficam em `$TMPDIR/kvojps-monitor-fixtures`;
  as boas-vindas são puladas.
- Só em desenvolvimento: feature Cargo `fixtures`, desligada por padrão; build
  `--release` com a feature falha por `compile_error!`; as fixtures são lidas do
  repositório em tempo de execução, nunca embutidas no binário. Testes em
  `src-tauri/src/fixtures.rs` e `build_tests` em `src-tauri/src/lib.rs`.
