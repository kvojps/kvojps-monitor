# 01: Esqueleto: Indicador, Painel e trilho

**What to build:** o app Tauri v2 (núcleo Rust + UI Svelte 5/TS/Vite) inicia com o login e mostra o Indicador na barra superior do GNOME. Clique simples abre um menu com "Abrir painel"; duplo clique ou clique do meio abrem o Painel direto. O Painel (~440×600, sem decoração, cantos arredondados) tem o trilho vertical com os seis Módulos (IA, Agentes, GitHub, Trello, Rede, Sistema) e Configurações no rodapé, cada Módulo ainda vazio com cabeçalho (nome + hora da última atualização). Fecha com Esc e ao perder o foco e reabre no último Módulo visitado, lembrado entre execuções. Abrir o app de novo só abre o Painel da instância viva. Base visual pronta: tokens de design (escuro por padrão seguindo o tema do sistema, claro suportado, acento verde-água, números monoespaçados tabulares, cores semânticas só para estados) e um único arquivo de strings em pt-BR. Ver spec, "Stack e processos", ADR 0001 e o protótipo em `prototype/painel-visual`.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] Indicador via `ksni` com menu "Abrir painel"; duplo clique e clique do meio abrem o Painel; o ícone suporta os estados normal, ponto de acento e badge de Risco (ainda sem fonte que os acione)
- [ ] Painel sem decoração, posicionado pelo Mutter, fecha com Esc e ao perder o foco
- [ ] Trilho com os seis Módulos e Configurações no rodapé; um Módulo em detalhe por vez; cabeçalho com nome e hora da última atualização
- [ ] Último Módulo visitado persiste entre execuções
- [ ] Segundo lançamento sinaliza a instância viva para abrir o Painel e sai
- [ ] Autostart ligado na primeira execução via `.desktop` em `~/.config/autostart/` (no AppImage, reescrito a cada execução com o caminho atual)
- [ ] Tokens de design em CSS próprio (sem Tailwind/kit), tema claro e escuro seguindo o sistema
- [ ] Textos da UI num único arquivo de strings pt-BR
- [ ] Contrato núcleo → UI estabelecido: um evento de snapshot por Módulo, comandos pontuais da UI e o comando que informa o Módulo visível
- [ ] Memória em repouso medida com o Painel fechado e registrada neste ticket; se > ~150 MB, a webview é destruída ao fechar em vez de ocultada
- [ ] Vitest + Testing Library configurados com um primeiro teste do trilho
