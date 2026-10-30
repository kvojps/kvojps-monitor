# Indicador só de bandeja, sem extensão do GNOME Shell

No GNOME/Wayland do Ubuntu, um app comum não consegue abrir o Painel com clique simples no Indicador (a extensão AppIndicator abre o menu; `Activate` só no duplo clique) nem posicionar o Painel sob o ícone (Wayland proíbe coordenadas globais; Mutter não tem layer-shell). Só uma extensão do GNOME Shell resolveria as duas coisas. Decidimos não ter extensão na v1: o Indicador é um ícone de bandeja (`ksni`) cujo clique simples abre um menu com "Abrir painel", duplo clique ou clique do meio abrem o Painel direto, e o Painel é uma janela sem decoração posicionada pelo Mutter. Motivo: uma extensão usa API interna sem estabilidade, exige manutenção a cada versão do GNOME e não existe no AppImage — custo recorrente alto para um app pessoal que funciona sem ela.

## Considered Options

- **Extensão fina opcional no `.deb`** (alterna o Painel via D-Bus e o reposiciona com `Meta.Window.move_frame`): melhor experiência, mas manutenção contínua; fica como possível esforço futuro.
- **Extensão como app inteiro**: UI limitada a St/Clutter e I/O rodando dentro do compositor.
- **XWayland para posicionar a janela**: não verificado e frágil com escala fracionária.
