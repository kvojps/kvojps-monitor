# 01: Esqueleto: Indicador, Painel e trilho

**What to build:** o app Tauri v2 (núcleo Rust + UI Svelte 5/TS/Vite) inicia com o login e mostra o Indicador na barra superior do GNOME. Clique simples abre um menu com "Abrir painel"; duplo clique ou clique do meio abrem o Painel direto. O Painel (~440×600, sem decoração, cantos arredondados) tem o trilho vertical com os seis Módulos (IA, Agentes, GitHub, Trello, Rede, Sistema) e Configurações no rodapé, cada Módulo ainda vazio com cabeçalho (nome + hora da última atualização). Fecha com Esc e ao perder o foco e reabre no último Módulo visitado, lembrado entre execuções. Abrir o app de novo só abre o Painel da instância viva. Base visual pronta: tokens de design (escuro por padrão seguindo o tema do sistema, claro suportado, acento verde-água, números monoespaçados tabulares, cores semânticas só para estados) e um único arquivo de strings em pt-BR. Ver spec, "Stack e processos", ADR 0001 e o protótipo em `prototype/painel-visual` (branch apagada em 2026-11-01).

**Blocked by:** None (can start immediately)

**Status:** done

- [x] Indicador via `ksni` com menu "Abrir painel"; duplo clique e clique do meio abrem o Painel; o ícone suporta os estados normal, ponto de acento e badge de Risco (ainda sem fonte que os acione)
- [x] Painel sem decoração, posicionado pelo Mutter, fecha com Esc e ao perder o foco
- [x] Trilho com os seis Módulos e Configurações no rodapé; um Módulo em detalhe por vez; cabeçalho com nome e hora da última atualização
- [x] Último Módulo visitado persiste entre execuções
- [x] Segundo lançamento sinaliza a instância viva para abrir o Painel e sai
- [x] Autostart ligado na primeira execução via `.desktop` em `~/.config/autostart/` (no AppImage, reescrito a cada execução com o caminho atual)
- [x] Tokens de design em CSS próprio (sem Tailwind/kit), tema claro e escuro seguindo o sistema
- [x] Textos da UI num único arquivo de strings pt-BR
- [x] Contrato núcleo → UI estabelecido: um evento de snapshot por Módulo, comandos pontuais da UI e o comando que informa o Módulo visível
- [x] Memória em repouso medida com o Painel fechado e registrada neste ticket; se > ~150 MB, a webview é destruída ao fechar em vez de ocultada
- [x] Vitest + Testing Library configurados com um primeiro teste do trilho

## Memória em repouso (medida em 2026-10-30)

PSS somado do processo e dos filhos WebKit (`/proc/<pid>/smaps_rollup`), build de release, Ubuntu 26.04, GNOME 50 em Wayland, WebKitGTK 2.52.6:

| Situação | Núcleo | WebKitWebProcess | WebKitNetworkProcess | Total |
|---|---|---|---|---|
| Webview criada e oculta (Painel nunca aberto) | 75,4 MB | 102,2 MB | 18,1 MB | **195,7 MB** |
| Webview destruída ao fechar, Painel nunca aberto | 37,0 MB | — | — | **37,0 MB** |
| Painel aberto | 95,6 MB | 129,3 MB | 18,0 MB | 243,0 MB |
| Depois de abrir e fechar | 132,6 MB | — | 22,2 MB | 154,8 MB |

Ocultar passou de ~150 MB, então **a webview é destruída ao fechar** e a janela nasce a cada abertura. Depois do primeiro uso o núcleo segue maior: dos 132,6 MB, só ~38 MB são anônimos; o resto são páginas de arquivo de bibliotecas que o GL carrega e mantém mapeadas (libLLVM ~43 MB, libgallium ~11 MB, WebKit/JSC ~23 MB), limpas e recuperáveis pelo kernel. O `WebKitNetworkProcess` também sobrevive. **Pendência:** depois do primeiro uso o repouso fica em ~155 MB, ainda no limite; investigar render sem GL ou encerrar o `WebKitNetworkProcess` se isso incomodar.

Consequência no comportamento: como a UI é recarregada a cada abertura, fechar nas Configurações e reabrir volta ao último Módulo visitado.
