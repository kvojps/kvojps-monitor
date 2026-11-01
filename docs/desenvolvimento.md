# Desenvolvimento

Ferramentas para ver o Painel com dados conhecidos, sem coleta real. Nada disto existe no build de produção.

## Protótipo B

```sh
npm run prototype:compact
```

Abre `compact-prototype.html?variant=B` no navegador (porta 1421). É a referência de design e conteúdo do Painel ([spec, "Visual do Painel"](../.scratch/kvojps-monitor/spec.md#visual-do-painel)); a captura aprovada está em [`assets/painel-b.png`](assets/painel-b.png). Fixtures e ações do protótipo são simuladas.

## Modo de fixtures nativo

Abre o app nativo (Tauri/WebKitGTK) com os snapshots do protótipo B, para comparar a produção lado a lado com o B na janela real. Feche qualquer instância antes (instância única: a nova entregaria o Painel à antiga e sairia).

```sh
npm run dev:fixtures
KVOJPS_FIXTURES="ai=claude-failure,network=poor" npm run dev:fixtures
```

Módulos: `ai`, `agents`, `github`, `trello`, `network`, `system`; nomes das fixtures em `src/lib/<módulo>/fixtures/`. Padrão (B): IA `claude-usage`, Agentes `states`, GitHub/Trello/Rede `full`, Sistema `notebook`.

O que o modo garante:

- Os snapshots seguem o caminho normal (`snapshot:<módulo>` e `initial_state`); a UI não tem caminho paralelo. A injeção é no nível do snapshot, não nas portas brutas, porque as fixtures do B são snapshots da UI. Nenhum coletor inicia: sem leitura do sistema, rede, keyring, `~/.claude/settings.json` ou `~/.codex/hooks.json`, sem notificações e sem verificação de versão.
- Mover card, testar velocidade, conectar/desconectar Claude, `oauth/usage`, token do GitHub, Trello e hooks respondem em memória. Abrir card, quadro, release ou logs continua abrindo o navegador/arquivos reais.
- Relógio da webview fixado em `1793401200000` (30/10/2026 20:00), com as duas janelas de IA abertas como no B.
- Preferências e autostart em `$TMPDIR/kvojps-monitor-fixtures`; boas-vindas puladas.
- Feature Cargo `fixtures`, desligada por padrão; `--release` com a feature falha por `compile_error!`; fixtures lidas do repositório em tempo de execução. Testes em `src-tauri/src/fixtures.rs` e `build_tests` em `src-tauri/src/lib.rs`.

## Captura nativa

O GNOME recusa `org.gnome.Shell.Screenshot` a clientes comuns. No modo de fixtures, a própria instância renderiza a webview e mede a rolagem (`src-tauri/src/capture.rs`). Com `npm run dev:fixtures` rodando:

```sh
scripts/capture-panel.sh /caminho/absoluto/b-1100-escuro 1100x700 dark
scripts/capture-panel.sh /caminho/absoluto/b-1000-claro 1000x700 light
```

Gera `<prefixo>.png` e `<prefixo>.json` e sai com erro se a captura falhar. A instância reabre o Painel, pede o tamanho lógico e corrige pela diferença medida (no Wayland o tao entrega 52 px a menos por eixo). O JSON traz `width`/`height`, `opened` (tamanho antes do ajuste), `scroller`, `overflowing` (**vazio = sem rolagem**), `theme`, `devicePixelRatio` e `styles`.

**Vite frio:** na primeira partida, o `vite-plugin-svelte` pode falhar ao carregar o CSS de alguns componentes ("failed to load virtual css module"); o WebKit mantém a falha nas reaberturas. A captura recusa esse estado; reinicie `npm run dev:fixtures`.
