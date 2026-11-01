# 08: Captura nativa para agentes

Type: task
Status: resolved
Blocked by: None (can start immediately).

## Question

Como um agente obtém, sem ajuda do usuário, uma captura da janela nativa do Painel em modo de fixtures (1100 × 700 e 1000 × 700) e a medida de rolagem do conteúdo? No ticket 02 o GNOME recusou `org.gnome.Shell.Screenshot` (“Screenshot is not allowed”), então a conferência dependeu do usuário.

- [x] Achar um caminho que funcione no GNOME/Wayland só em desenvolvimento (por exemplo, captura da própria webview ou comando exclusivo da feature `fixtures`), sem nada no build de produção.
- [x] Expor também `scrollHeight`/`clientHeight` do Painel, para comprovar “sem rolagem” sem olhar.
- [x] Documentar em `validation/README.md`.

## Comments

Origem: ticket 02 (2026-11-01). Destrava a conferência dos tickets 03–07.

## Answer

Resolvido em 2026-11-01: o agente captura sozinho, pela própria webview.

- **Caminho:** `scripts/capture-panel.sh <prefixo absoluto> [LxA] [dark|light|system]` com `npm run dev:fixtures` rodando. O segundo lançamento leva `--capture` à instância viva pela instância única; ela reabre o Painel no tamanho lógico e tema pedidos, mede e grava `<prefixo>.png` (snapshot do WebKitGTK) e `<prefixo>.json`. Código só na feature `fixtures` (`src-tauri/src/capture.rs`; dependências opcionais `webkit2gtk`, `javascriptcore-rs` e `cairo-rs`, que já vinham do wry). O teste de build confere que a feature só liga essas dependências opcionais. Documentado em [validation/README.md](../validation/README.md#captura-por-agentes).
- **Rolagem sem olhar:** o JSON traz `scroller` (`scrollHeight`/`clientHeight`) e `overflowing` (todo contêiner rolável que excede a caixa; vazio = sem rolagem).
- **Conferido no app nativo com as fixtures do B:** 1100 × 700 e 1000 × 700, escuro e claro: `overflowing` vazio, `scrollHeight` = `clientHeight` = 641. Visualmente próximo de `compact-B.png` (a aprovação continua com o usuário, na [Conferência nativa do topo e cartão IA e Agentes](09-native-review-top-and-ai-agents.md)).
- **Achado 1, Vite frio:** na primeira partida, o `vite-plugin-svelte` falhou ao carregar o CSS de AiModule, AgentsModule, GithubModule e TrelloSettings; o WebKit manteve a falha nas reaberturas, e o Painel saiu sem o CSS desses Módulos (↻/⚙ abaixo do título, Trello desmontado, falsa rolagem de 51 px). A captura agora lista `styles` e recusa quando falta a folha de um Módulo. Uma conferência humana logo após a primeira partida pode ter visto o mesmo efeito.
- **Achado 2, tamanho real do Painel:** num monitor 1920 × 1080, escala 1, o Painel abre com área visível de **1048 × 648**, não 1100 × 700 (52 px a menos por eixo: o tao no Wayland desconta margens da janela). Também sem rolagem nesse tamanho. Fica para as dimensões lógicas da [Validação integrada nativa](07-integrated-native-validation.md).
