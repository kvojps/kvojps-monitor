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
- Relógio da webview fixado em `1793401200000` (30/10/2026 20:00, publicação da fixture `claude-usage`, com as duas janelas de IA abertas como no B),
  como nas validações anteriores.
- Preferências e entrada de autostart ficam em `$TMPDIR/kvojps-monitor-fixtures`;
  as boas-vindas são puladas.
- Só em desenvolvimento: feature Cargo `fixtures`, desligada por padrão; build
  `--release` com a feature falha por `compile_error!`; as fixtures são lidas do
  repositório em tempo de execução, nunca embutidas no binário. Testes em
  `src-tauri/src/fixtures.rs` e `build_tests` em `src-tauri/src/lib.rs`.

## Captura por agentes

O GNOME recusa `org.gnome.Shell.Screenshot` a clientes comuns (“Screenshot is not
allowed”). No modo de fixtures, a própria instância renderiza a webview com o snapshot
do WebKitGTK e mede a rolagem; nada disso existe no build de produção (feature
`fixtures`, `src-tauri/src/capture.rs`).

Com `npm run dev:fixtures` rodando:

```sh
scripts/capture-panel.sh /caminho/absoluto/b-1100-escuro 1100x700 dark
scripts/capture-panel.sh /caminho/absoluto/b-1000-claro 1000x700 light
```

O script repassa `--capture <prefixo> --size LxA --theme dark|light|system` à
instância viva (instância única), espera e imprime `<prefixo>.json`; a imagem fica em
`<prefixo>.png`. Sai com erro se a captura falhou. O tema é salvo nas preferências
temporárias do modo de fixtures; sem `--theme`, vale o já salvo.

A instância reabre o Painel, pede o tamanho lógico, corrige pela diferença medida (no
Wayland o tao entrega 52 px a menos em cada eixo), espera o layout e 1,5 s de
transição, mede e só então captura. O JSON traz:

- `width`/`height`: área visível obtida; `opened`: tamanho com que o Painel abriu
  sozinho, antes do ajuste.
- `scroller`: `scrollHeight`/`clientHeight` (e largura) de `.panel .scroller`.
- `overflowing`: todo contêiner rolável cujo conteúdo excede a caixa. **Vazio = sem
  rolagem.**
- `theme`, `devicePixelRatio`, `styles` (folhas de estilo injetadas pelo Vite).

**Vite frio:** na primeira partida, o `vite-plugin-svelte` pode falhar ao carregar o CSS
de alguns componentes (“failed to load virtual css module” no log); o WebKit mantém a
falha nas reaberturas e o Painel sai sem o CSS desses Módulos (cabeçalhos quebrados,
falsa rolagem). A captura recusa esse estado e pede para reiniciar
`npm run dev:fixtures`.
