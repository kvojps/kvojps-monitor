# Escolher a stack

Type: grilling
Status: resolved
Blocked by: 06, 05
Map: [kvojps-monitor v1](../map.md)

## Question

Com base na pesquisa de stack e nos requisitos de rede/permissões, qual stack, arquitetura de processos (UI vs serviço auxiliar privilegiado) e forma de empacotamento a v1 adota?

## Answer

- **Stack**: Tauri v2 — núcleo Rust (coleta, keyring, notificações, autostart) + UI em webview. UI em **Svelte 5 + TypeScript + Vite**, CSS próprio com tokens (sem Tailwind/kit de componentes).
- **Indicador**: só bandeja via crate `ksni`. Clique simples → menu com "Abrir painel"; duplo clique ou clique do meio → Painel direto. Painel = janela sem decoração, posicionada pelo Mutter. Sem extensão GNOME na v1 ([ADR 0001](../../../docs/adr/0001-indicador-sem-extensao-gnome.md)).
- **Processos**: um único processo de usuário sem privilégio; o núcleo Rust detém todo o estado e chama `codex app-server` e `claude agents --json` como subprocessos. Serviço eBPF privilegiado, se existir, é componente separado e opcional via D-Bus — decidido em [Modo do tráfego por processo](14-trafego-por-processo.md).
- **Núcleo → UI**: o núcleo empurra um snapshot por Módulo via eventos Tauri quando algo muda; a UI só renderiza e envia comandos pontuais (mover card, rodar teste de velocidade, salvar credencial). Notificações saem do núcleo, funcionam com o Painel fechado.
- **Ciclo de vida do Painel**: ao fechar, a webview é ocultada. Medir memória no protótipo de [Layout e visual do Painel](10-visual-do-painel.md); se passar de ~150 MB em repouso, destruir ao fechar.
- **Autostart**: ligado na primeira execução, desligável nas Configurações; `.desktop` em `~/.config/autostart/`; no AppImage, reescrito a cada execução com o caminho atual do arquivo.
- **Segredos**: GNOME Keyring via Secret Service (crate `keyring`). Sem keyring disponível, o app recusa salvar e explica nas Configurações — nunca texto puro.
- **Empacotamento**: `.deb` + AppImage via `tauri-action` em runner `ubuntu-24.04`; suporte declarado Ubuntu 24.04+ com GNOME; outras distros "pode funcionar". Sem Flatpak/Snap.
