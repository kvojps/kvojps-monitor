# Stack para Indicador e Painel no GNOME/Wayland

Type: research
Status: resolved
Blocked by: 
Map: [kvojps-monitor v1](../map.md)

## Question

Qual stack entrega melhor um Indicador na barra superior do GNOME (Wayland, extensão AppIndicator) que abre um Painel compacto, inicia com o login, guarda segredos no keyring, envia notificações nativas e é empacotável como .deb + AppImage via GitHub Actions? Comparar Tauri v2, Electron, extensão do GNOME Shell e Python+GTK4/libadwaita, com atenção a: posicionamento da janela perto do ícone no Wayland, consumo de memória, manutenção entre versões do GNOME, acesso a recursos de sistema (rede/processos) e facilidade de UI moderna.

## Answer

- **Clique no Indicador**: na extensão AppIndicator do Ubuntu (GNOME Shell 50.1, conferido no código instalado), clique esquerdo simples abre o menu (ou nada, sem menu); `Activate` do app só com duplo clique; clique do meio envia `SecondaryActivate`. Tauri: eventos de clique do tray não são emitidos no Linux e a libayatana-appindicator 0.5.94 do Ubuntu 26.04 não tem `Activate`; a crate `ksni` habilita duplo clique/clique do meio.
- **Posição do Painel no Wayland**: cliente não pode se posicionar (protocolo, GTK4, Electron documentam); Mutter não suporta layer-shell. Só código dentro do GNOME Shell (extensão) consegue: `Meta.Window.move_frame` ou popup próprio ancorado no ícone.
- **Recomendação**: **Tauri v2** como app principal (UI web escura, Rust com acesso total ao sistema, `keyring` crate, plugins oficiais de notificação e autostart, `.deb` + AppImage via action oficial no GitHub Actions gratuito). Indicador como tray padrão com item "Abrir painel" (funciona sem extensão). Se clique simples + Painel ancorado ao ícone for requisito, **extensão GNOME fina opcional** no `.deb` que alterna o Painel via D-Bus e reposiciona a janela; lógica fica no Tauri.
- **Descartados**: Electron (pesado, mesmas limitações), Python+GTK4 (sem lib de tray para GTK4, AppImage difícil), só-extensão (UI limitada, roda no compositor, quebra a cada versão do GNOME).
- **Não verificado**: memória (só qualitativo — medir no protótipo); XWayland permitir posicionamento; "pulo" visível no reposicionamento.

Achados completos: [`docs/research/stack-indicador.md`](../../../docs/research/stack-indicador.md) (trazido da branch `research/stack-indicador`, commit `ee1f803`, apagada em 2026-11-01).
