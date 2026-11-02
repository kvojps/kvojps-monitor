# Desenvolvimento

Guia para executar, testar e compilar o app a partir do código, inclusive em um fork. O suporte declarado é Ubuntu 24.04+ com GNOME/Wayland e extensão AppIndicator. Outras distribuições podem funcionar, mas não são o ambiente validado.

Este é um projeto pessoal que não recebe contribuições externas. Para adaptar ou manter sua própria versão, faça um fork conforme a licença MIT; preserve o aviso de copyright e a licença nas cópias distribuídas. Este guia não estabelece um fluxo de contribuição ao projeto original.

## Ferramentas e dependências

Use Git, Node.js 24 com npm e Rust estável via rustup, com `rustfmt` e `clippy`. O mínimo declarado em `src-tauri/Cargo.toml` é Rust 1.90; o CI acompanha o estável. Instale Node e rustup antes dos comandos abaixo. A CLI Tauri é uma dependência local do projeto, sem instalação global.

No Ubuntu, instale as dependências nativas usadas pelo [CI](../.github/workflows/ci.yml):

```sh
sudo apt-get update
sudo apt-get install -y git curl build-essential file pkg-config \
  libayatana-appindicator3-dev libdbus-1-dev librsvg2-dev libssl-dev \
  libwebkit2gtk-4.1-dev libxdo-dev
rustup component add rustfmt clippy
```

Para a sessão gráfica, mantenha o GNOME Keyring/Secret Service desbloqueado e a extensão AppIndicator habilitada. No Ubuntu, os pacotes são `gnome-keyring` e `gnome-shell-extension-appindicator`; habilite “Ubuntu AppIndicators” no aplicativo Extensões e, se necessário, encerre e reabra a sessão. Sem keyring, o app recusa salvar credenciais.

As integrações dependem das fontes disponíveis: Claude Code e Codex no `PATH` e autenticados para seus respectivos Módulos; GitHub CLI (`gh`) autenticado pode fornecer o token do GitHub. Elas não são necessárias para compilar ou executar os testes com fixtures. Ausência de uma fonte deve aparecer como indisponibilidade no Painel.

## Clone e instalação

```sh
git clone https://github.com/kvojps/kvojps-monitor.git
cd kvojps-monitor
npm ci
cargo fetch --locked --manifest-path src-tauri/Cargo.toml
```

Para um fork, substitua a URL pela do seu repositório. Preserve `package-lock.json` e `src-tauri/Cargo.lock` para reproduzir as dependências. A instalação inicial precisa de rede para os registros npm e Cargo.

## Executar com dados reais

```sh
npm run tauri -- dev
```

O Tauri inicia o Vite na porta 1420 e o núcleo Rust. O app aparece no Indicador; clique nele para abrir o Painel. Esse modo usa as configurações, cache e integrações reais do usuário, inicia coletas locais e acessos de rede conforme as preferências. Credenciais ficam no keyring; status line e hooks só são instalados após ação explícita na interface. O token do Claude é lido em memória e nunca renovado pelo app.

`npm run dev` sozinho inicia apenas o frontend no navegador; não fornece o núcleo nem as APIs nativas. Para ver dados conhecidos sem coleta real, use o protótipo ou o modo nativo de fixtures abaixo. Esses recursos de desenvolvimento não existem no build de produção.

## Verificações

Execute da raiz, na ordem usada pelo CI (o build da UI cria `dist`, lido por `generate_context!` durante a compilação Rust):

```sh
cargo fmt --check --manifest-path src-tauri/Cargo.toml
npm run check
npm test
npm run build
cargo clippy --all-targets --locked --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo test --locked --manifest-path src-tauri/Cargo.toml
cargo clippy --all-targets --locked --features fixtures --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo test --locked --features fixtures --manifest-path src-tauri/Cargo.toml --lib fixtures
npm run tauri -- build --no-bundle
```

Para uma iteração curta, `npm test -- src/lib/ai/AiModule.test.ts` executa um arquivo da UI (use um caminho existente); `cargo test --locked --manifest-path src-tauri/Cargo.toml nome_do_teste` filtra testes Rust. A suíte completa usa portas injetáveis e fixtures sanitizadas, sem exigir contas reais.

## Compilar pacotes locais

```sh
npm run tauri -- build
# Ou selecione um formato:
npm run tauri -- build --bundles deb
npm run tauri -- build --bundles appimage
```

A configuração atual gera `.deb` em `src-tauri/target/release/bundle/deb/` e AppImage em `src-tauri/target/release/bundle/appimage/`. O executável fica em `src-tauri/target/release/kvojps-monitor`. `--no-bundle` compila somente o executável. O empacotador pode baixar ferramentas na primeira execução; sem rede, um build do executável não comprova que o empacotamento funciona.

Não habilite `fixtures` em release: a compilação é recusada. Pacotes locais não recebem automaticamente os checksums e atestados de uma release publicada. Em um fork, revise nome, identificador, versão e ícones em `src-tauri/tauri.conf.json`, metadados em `src-tauri/Cargo.toml` e `package.json`, além de URLs do projeto e workflows de release antes de distribuir.

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

## Imagens sanitizadas

Para imagens do app nativo com dados de exemplo, inicie `npm run dev:fixtures` e use a captura nativa acima, com saída temporária fora do repositório. Isso captura a UI real do Tauri alimentada por snapshots de exemplo; identifique esse modo na legenda, sem apresentar o protótipo HTML como versão distribuída.

Antes de copiar PNG/JSON para `docs/assets/`, abra a imagem, revise todos os textos visíveis e metadados dos dois arquivos conforme [sanitização](agents/sanitizacao.md). Não publique caminhos pessoais, contas, sessões, endereços de rede reais ou conteúdo privado. A captura não substitui essa revisão. Se uma imagem precisar mostrar coleta real, sanitize os dados na origem antes de capturar; não grave nem imprima dados brutos no repositório ou terminal compartilhado.

## Validação deste guia

Com Rust 1.99.1, as duas verificações Clippy com `-D warnings` falharam em código preexistente (`src-tauri/src/ai/operation.rs:35`, lints `wrong_self_convention` e `new_ret_no_self`). Os demais resultados e as limitações de execução estão registrados no [ticket 42](../.scratch/kvojps-monitor/issues/42-guia-compilacao-forks.md). A integração do link deste guia e da política de forks no README da v1 pertence ao [ticket 23](../.scratch/kvojps-monitor/issues/23-readme.md).
