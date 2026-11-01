# Stack para Indicador e Painel no GNOME/Wayland

Pesquisa feita em 2026-10-30. Alvo: Ubuntu 26.04 LTS, GNOME 50 (Wayland), extensão `ubuntu-appindicators@ubuntu.com` habilitada.

## Pergunta

Qual stack entrega melhor um Indicador na barra superior que, ao clicar, abre um Painel compacto, inicia com o login, guarda segredos no keyring, envia notificações nativas e pode ser empacotada como `.deb` + AppImage no GitHub Actions gratuito? Opções avaliadas: Tauri v2, Electron, extensão do GNOME Shell, Python+GTK4/libadwaita e uma arquitetura híbrida.

## TL;DR

- **Nenhum app comum (Tauri, Electron, Python) consegue abrir o Painel com um único clique esquerdo** no GNOME do Ubuntu. Isso vem do código da extensão AppIndicator, não de limitação das stacks. Um clique simples abre o menu, ou não faz nada se o ícone não tiver menu. O `Activate` do app só é chamado com **duplo clique**, e somente quando o app expõe esse método.
- **Nenhum app comum consegue posicionar sua janela perto do ícone no Wayland.** O protocolo `xdg-shell` não tem posição global. O GTK4 removeu `gtk_window_move()`. O Mutter não implementa `wlr-layer-shell`. O Electron documenta que no Wayland "programmatically changing the global window coordinates is prohibited".
- **Só código rodando dentro do GNOME Shell (uma extensão) controla clique e posição.** Ela pode tratar o clique simples e pode mover a janela com `Meta.Window.move_frame()`, ou desenhar o próprio popup ancorado no ícone.
- **Recomendação:** arquitetura **híbrida**. Um app Tauri v2 faz todo o trabalho (Rust para sistema, segredos, notificações e autostart; webview para a UI escura). Ele expõe uma interface D-Bus e um ícone SNI como fallback. Uma **extensão GNOME fina e opcional** dá o clique único e o posicionamento sob o ícone.

---

## 1. Comportamento do clique no ícone AppIndicator (GNOME/Ubuntu)

### O que a especificação diz

A especificação StatusNotifierItem define três ações:

- `Activate(x, y)` é a ativação primária, "typically a consequence of user input", normalmente o clique esquerdo.
- `SecondaryActivate` é normalmente o clique do meio.
- `ContextMenu` é normalmente o clique direito.
- A propriedade `ItemIsMenu` indica que "the item only support the context menu".

Fonte: <https://specifications.freedesktop.org/status-notifier-item/latest/status-notifier-item.html>

### O que a extensão do Ubuntu faz de fato

Arquivo `indicatorStatusIcon.js`, upstream `master` em <https://github.com/ubuntu/gnome-shell-extension-appindicator/blob/master/indicatorStatusIcon.js>. A cópia instalada em `/usr/share/gnome-shell/extensions/ubuntu-appindicators@ubuntu.com/` na máquina alvo (GNOME Shell 50.1) tem a mesma lógica:

```js
// vfunc_button_press_event
if (event.get_button() === Clutter.BUTTON_MIDDLE) { ... this._indicator.secondaryActivate(...); return Clutter.EVENT_STOP; }
if (event.get_button() === Clutter.BUTTON_SECONDARY) { this.menu.toggle(); ... }
const doubleClickHandled = this._maybeHandleDoubleClick(event);   // 2º clique -> this._indicator.open() -> Activate
if (doubleClickHandled === Clutter.EVENT_PROPAGATE &&
    event.get_button() === Clutter.BUTTON_PRIMARY &&
    this.menu.numMenuItems) {
    if (this._indicator.supportsActivation !== false)
        this._waitForDoubleClick().catch(logError);   // espera o double-click-time e então abre o MENU
    else
        this.menu.toggle();
}
```

Em `appIndicator.js`, `open()` chama `ActivateAsync(x, y)`. `supportsActivation` vem da introspecção D-Bus, `!!interfaceInfo.lookup_method('Activate')`. A extensão **ignora `ItemIsMenu`**. Fonte: <https://github.com/ubuntu/gnome-shell-extension-appindicator/blob/master/appIndicator.js>

**Consequências práticas:**

| Situação do app | Clique esquerdo simples | Duplo clique | Clique do meio |
|---|---|---|---|
| Tem menu e expõe `Activate` | abre o **menu**, com atraso igual ao double-click-time | `Activate` (app abre o painel) | `SecondaryActivate` |
| Tem menu e **não** expõe `Activate` | abre o menu imediatamente | abre o menu | `SecondaryActivate` |
| **Sem menu** e expõe `Activate` | **nada acontece** | `Activate` | `SecondaryActivate` |

Existem issues abertas pedindo clique simples para ativar, sem solução até hoje:

- #611 "Left-click to open": <https://github.com/ubuntu/gnome-shell-extension-appindicator/issues/611>
- #490: <https://github.com/ubuntu/gnome-shell-extension-appindicator/issues/490>
- #120 "Optional setting to disable menu?": <https://github.com/ubuntu/gnome-shell-extension-appindicator/issues/120>

O próprio Chromium/Electron reconhece o problema no código. `OnSecondaryActivate` também dispara o `click`, com este comentário: *"gnome-shell-extension-appindicator requires a double-click to activate which is non-obvious, so allow middle-click to activate"*. Fonte: <https://chromium.googlesource.com/chromium/src/+/main/chrome/browser/ui/views/status_icons/status_icon_linux_dbus.cc>

### Por stack (só SNI, sem extensão própria)

**Tauri v2.** A documentação do tray diz, sobre eventos de mouse: "Linux: Unsupported. The event is not emitted even though the icon is shown". O menu aparece no clique esquerdo e no direito. Fonte: <https://v2.tauri.app/learn/system-tray/>

- O Tauri usa o crate `tray-icon` com a feature `libappindicator`, que exige GTK3. Fontes: <https://github.com/tauri-apps/tauri/blob/dev/crates/tauri/Cargo.toml> e <https://github.com/tauri-apps/tray-icon>
- O Ubuntu 26.04 traz `libayatana-appindicator3-1` **0.5.94**. Fonte: <https://packages.ubuntu.com/resolute/libayatana-appindicator3-1>
- O XML D-Bus dessa versão **não tem método `Activate`**, só `Scroll`, `SecondaryActivate` e `XAyatanaSecondaryActivate`. Fonte: <https://github.com/AyatanaIndicators/libayatana-appindicator/blob/0.5.94/src/notification-item.xml>
- O sinal `activate-event` só aparece nas versões 0.6.x. Fonte: <https://github.com/AyatanaIndicators/libayatana-appindicator/blob/master/src/app-indicator.c>
- **Resultado no alvo:** clique esquerdo = menu. O caminho viável é um item "Abrir painel" no menu (2 cliques).
- **Alternativa:** usar o crate `ksni` diretamente, ou a feature `ksni` do `tray-icon`. Ele implementa `activate(x, y)` em Rust puro via D-Bus, sem GTK. Isso dá duplo clique ou clique do meio para abrir o painel. Fonte: <https://docs.rs/ksni/latest/ksni/trait.Tray.html>

**Electron.** O tray usa StatusNotifierItem e o evento `click` é "emitted when the tray icon receives an activation". No Linux só `click` é suportado, e não há `right-click`, `double-click` etc. Fonte: <https://www.electronjs.org/docs/latest/api/tray>

- No GNOME do Ubuntu, `click` dispara com duplo clique ou clique do meio, conforme o código do Chromium citado acima.

**Python+GTK4.** Não há biblioteca SNI para GTK4 no Ubuntu. A `libayatana-appindicator3` é GTK3.

- Existe a `libayatana-appindicator-glib`, "100% GTK-free", que exporta menus via `org.gtk.Menus`. Não verifiquei se ela suporta ativação nem se a extensão do Ubuntu a renderiza. Fonte: <https://github.com/AyatanaIndicators/libayatana-appindicator-glib>
- Na prática seria preciso implementar o SNI à mão com `Gio.DBus`. O comportamento de clique é o mesmo da tabela.

**Extensão GNOME Shell.** É um `PanelMenu.Button` próprio, com controle total do clique simples, sem passar pelo AppIndicator.

## 2. Posicionar o Painel perto do ícone no Wayland

- **Protocolo.** `xdg_toplevel` não tem requisição de posição. Só `xdg_popup` com `xdg_positioner` posiciona, e sempre relativo a uma superfície pai do próprio cliente. O ícone pertence ao Shell, não ao app. Fonte: <https://wayland.app/protocols/xdg-shell>
- **GTK4.** `gtk_window_move()` e `gtk_window_set_position()` foram removidos. "Any APIs that deal with global (or root) coordinates have been removed in GTK 4". Fonte: <https://docs.gtk.org/gtk4/migrating-3to4.html>
- **Electron.** No Wayland, `getPosition()` e `getBounds()` retornam 0,0 porque "introspecting or programmatically changing the global window coordinates is prohibited". Fonte: <https://www.electronjs.org/docs/latest/api/base-window>
- **layer-shell.** A tabela de compatibilidade de `wlr-layer-shell-unstable-v1` marca o **Mutter como não suportado**. KWin, Sway, Hyprland, COSMIC etc. suportam. Fontes: <https://wayland.app/protocols/wlr-layer-shell-unstable-v1> e o pedido antigo, sem resolução, <https://gitlab.gnome.org/GNOME/mutter/-/issues/973>. Portanto `gtk4-layer-shell` não serve no GNOME.
- **Plugin `positioner` do Tauri.** As posições relativas ao tray dependem de `on_tray_event`, que não é emitido no Linux (ver seção 1). A documentação não lista uma limitação específica para Linux. Fonte: <https://v2.tauri.app/plugin/positioner/>

**Workarounds possíveis:**

1. **Extensão GNOME move a janela.** Código no Shell pode chamar `meta_window_move_frame(window, user_op, root_x, root_y)` ("Moves the window to the desired location"). A extensão sabe onde está o próprio botão no painel e pode reposicionar a janela do app ao mapear, identificando-a pelo `wm_class`/app-id. Fonte: <https://gnome.pages.gitlab.gnome.org/mutter/meta/method.Window.move_frame.html>
2. **Extensão desenha o próprio painel.** Um `PopupMenu` em St fica ancorado no ícone nativamente, com o mesmo visual dos menus do sistema. A UI fica limitada a St/Clutter + CSS. Não dá para usar GTK/Adw nem webview no processo do Shell (ver seção 4).
3. **Rodar o app via XWayland** (`GDK_BACKEND=x11` / `--ozone-platform=x11`). O Ubuntu 26.04 mantém XWayland para apps X11 ("You can still run applications developed for X.org through the XWayland compatibility layer"). Fonte: <https://documentation.ubuntu.com/release-notes/26.04/summary-for-lts-users/>. **Não verificado** se o Mutter respeita a posição pedida por um cliente X11. Além disso, as coordenadas de `Activate(x, y)` podem não bater com o espaço do XWayland em escala fracionária. É frágil e vai na contramão do Wayland, então não recomendo como solução principal.
4. **Aceitar o posicionamento padrão do Mutter** (janela centralizada e pequena, sem decoração). É a solução mais simples e robusta sem extensão.

## 3. Requisitos transversais

**Autostart (XDG).** É um `.desktop` em `~/.config/autostart/`. A especificação define `Hidden` e `OnlyShowIn`. Fonte: <https://specifications.freedesktop.org/autostart/latest/>

- Tauri: o `tauri-plugin-autostart` usa o crate `auto-launch`. No Linux o padrão é "`.desktop` files in `~/.config/autostart/`", com opção de systemd user. Fontes: <https://v2.tauri.app/plugin/autostart/> e <https://github.com/zzzgydi/auto-launch>
- Para apps em sandbox, o portal `Background.RequestBackground` com `autostart`/`commandline`. Fonte: <https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Background.html>
- Uma extensão já roda com o Shell, mas o daemon dela continua precisando de autostart ou de ativação D-Bus.

**Segredos (Secret Service).** É a API D-Bus `org.freedesktop.Secret.Service`, implementada pelo GNOME Keyring. Fonte: <https://specifications.freedesktop.org/secret-service/latest/>

- Rust: crate `keyring` 4.x com o store `dbus-secret-service` ou `zbus-secret-service`. Fonte: <https://docs.rs/keyring/latest/keyring/>
- Electron: `safeStorage` usa o backend `gnome_libsecret` no GNOME. Se não houver keyring, cai para `basic_text`, com "hardcoded plaintext password". A API síncrona está deprecada e será removida no Electron 46. Fonte: <https://www.electronjs.org/docs/latest/api/safe-storage>
- Python/GJS: `gir1.2-secret-1` (libsecret 0.21.7 no 26.04). Fonte: <https://packages.ubuntu.com/resolute/gir1.2-secret-1>

**Notificações.**

- Tauri: `tauri-plugin-notification` (Linux suportado). Fonte: <https://v2.tauri.app/plugin/notification/>
- Electron: usa libnotify (Desktop Notifications Spec), e o GNOME é citado. Fonte: <https://www.electronjs.org/docs/latest/tutorial/notifications>
- Python: `Gio.Notification`.
- Extensão: `MessageTray` do Shell.

**Empacotamento (.deb + AppImage, GitHub Actions gratuito).**

- Tauri v2 gera deb, rpm, AppImage, Flatpak e Snap. Fonte: <https://v2.tauri.app/distribute/>
- A action oficial `tauri-apps/tauri-action@v1` cria a Release. As dependências do runner são `libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`. Fonte: <https://v2.tauri.app/distribute/pipelines/github/>
- O Ubuntu 26.04 tem `libwebkit2gtk-4.1-0` 2.52.6. Fonte: <https://packages.ubuntu.com/resolute/libwebkit2gtk-4.1-0>
- electron-builder suporta AppImage, deb, rpm, snap, flatpak etc. Os alvos padrão são AppImage e Snap. Fonte: <https://www.electron.build/docs/linux>
- Python+GTK4: o `.deb` é simples, porque depende de `python3-gi` 3.56 e `gir1.2-adw-1` 1.9 do sistema. O AppImage é trabalhoso, pois exige empacotar GTK4, libadwaita e PyGObject sem ferramenta oficial equivalente. Fontes: <https://packages.ubuntu.com/resolute/python3-gi> e <https://packages.ubuntu.com/resolute/gir1.2-adw-1>
- Extensão GNOME: distribuída como zip. Um `.deb` pode instalar em `/usr/share/gnome-shell/extensions/`. AppImage não se aplica.
- AppImage em Ubuntu recente pode exigir `libfuse2t64`, que existe no 26.04: <https://packages.ubuntu.com/resolute/libfuse2t64>

## 4. Comparação

| Critério | Tauri v2 | Electron | Extensão GNOME (pura) | Python + GTK4/libadwaita |
|---|---|---|---|---|
| Clique simples abre painel | Não (menu → "Abrir painel"). Com `ksni`: duplo clique ou clique do meio | Não (duplo clique ou clique do meio) | **Sim** | Não (SNI manual, duplo clique) |
| Painel perto do ícone | Não | Não | **Sim** (popup St ancorado) | Não |
| Memória | Média. WebKitGTK do sistema roda processos Web e Network separados; backend Rust é leve | **Alta**. Chromium empacotado, multi-processo | **Mínima** (roda dentro do `gnome-shell`) | Baixa a média (intérprete + GTK4) |
| Manutenção entre versões do GNOME | Baixa (depende só de SNI, WebKitGTK e libappindicator) | Baixa | **Alta**. API interna sem estabilidade, `shell-version` por versão, quebras como ESM no 45 | Baixa |
| Acesso a sistema (rede, processos, `~/.claude`, `~/.codex`) | Ótimo (Rust nativo) | Ótimo (Node) | Possível (Gio assíncrono, `Gio.Subprocess`), mas roda **no processo do compositor**: I/O bloqueante trava a sessão | Ótimo |
| UI moderna "tech" escura | **Ótima** (HTML/CSS livre) | **Ótima** | Limitada (St + CSS restrito, sem GTK/Adw/web) | Boa, mas com cara "GNOME/Adwaita"; customizável via CSS GTK |
| Segredos / notificações / autostart | keyring crate / plugin / plugin | safeStorage / libnotify / manual | libsecret via GI / MessageTray / implícito | libsecret / Gio.Notification / manual |
| .deb + AppImage em CI grátis | **Sim, oficial** (`tauri-action`) | Sim (electron-builder) | Só zip/.deb | .deb fácil, AppImage difícil |
| Tamanho do download | Pequeno no .deb (usa WebKitGTK do sistema). O AppImage fica maior porque embute WebKitGTK | Grande (Chromium) | Minúsculo | Pequeno no .deb |

Observações sobre a extensão pura:

- As diretrizes de revisão do extensions.gnome.org proíbem binários ("MUST NOT include binary executables or libraries"). Elas proíbem importar `Gtk`/`Gdk`/`Adw` no processo do Shell e exigem limpar tudo em `disable()`. Fonte: <https://gjs.guide/extensions/review-guidelines/review-guidelines.html>. Essas regras só valem para publicação no EGO, mas indicam boas práticas.
- Manutenção: no GNOME 45 houve migração obrigatória para ESM, sem compatibilidade com versões anteriores. Fonte: <https://gjs.guide/extensions/upgrading/gnome-shell-45.html>. O guia do 50 lista mudanças menores. Fonte: <https://gjs.guide/extensions/upgrading/gnome-shell-50.html>
- A própria extensão AppIndicator declara `shell-version` 45 a 51, o que mostra o ritmo de atualização necessário. Fonte: <https://github.com/ubuntu/gnome-shell-extension-appindicator/blob/master/metadata.json>

**Memória:** não encontrei medições primárias comparáveis para este cenário. Os valores acima são qualitativos, baseados na arquitetura de cada runtime. Recomendo medir no protótipo com `smem -k -P <nome>` (PSS), somando todos os processos filhos (WebKit/Chromium).

## 5. Arquitetura híbrida (extensão fina + app/daemon)

```
┌──────────── gnome-shell (extensão fina, ~200 linhas GJS) ────────────┐
│ PanelMenu.Button com ícone/label                                     │
│  clique esq. → D-Bus  com.kvojps.Monitor.TogglePanel()                │
│  ao mapear janela app-id=kvojps-monitor → Meta.Window.move_frame()    │
│  (opcional) label no painel atualizado via sinal D-Bus               │
└──────────────────────────────────────────────────────────────────────┘
                  │ D-Bus de sessão
┌──────────── kvojps-monitor (Tauri v2) ───────────────────────────────┐
│ Rust: coleta (rede, processos, ~/.claude, ~/.codex), keyring,        │
│       notificações, autostart XDG, serviço D-Bus                     │
│ Webview: Painel compacto, sem decoração, tema escuro                  │
│ Ícone SNI (fallback quando a extensão não está instalada):           │
│       menu "Abrir painel" / duplo clique                             │
└──────────────────────────────────────────────────────────────────────┘
```

**Vantagens:**

- A extensão fica pequena e sem lógica, então o custo de manutenção por versão do GNOME é baixo. Ela usa só `PanelMenu`, `Gio.DBusProxy` e `global.display`/`Meta.Window`, que são APIs estáveis na prática.
- Todo I/O fica fora do compositor.
- O app funciona sozinho em qualquer desktop com SNI, como KDE ou o próprio GNOME sem a extensão.

**Custos e riscos:**

- A extensão precisa ser habilitada pelo usuário (`gnome-extensions enable`). No Wayland, uma extensão recém-instalada só é carregada após novo login, porque não há restart do Shell no Wayland. A documentação de desenvolvimento usa sessão aninhada (`gnome-shell --devkit --wayland`) para testar. Fonte: <https://gjs.guide/extensions/development/creating.html>
- O `.deb` pode instalar a extensão em `/usr/share/gnome-shell/extensions/`. O AppImage pode só oferecer o zip ou instruções.
- O reposicionamento via `move_frame` acontece depois que a janela aparece. Pode haver um "pulo" visual que precisa ser validado em protótipo.

**Variante "painel dentro do Shell":** a extensão desenha o painel em St, com dados vindos do daemon via D-Bus. O resultado é UX perfeita (popup nativo ancorado e memória mínima), mas a UI fica limitada e o acoplamento à API do Shell é maior. Faz sentido se o Painel for realmente minúsculo, com poucas linhas de status.

## Lacunas / não verificado

- Se o Mutter 50 respeita a posição pedida por clientes XWayland (workaround 3).
- Se a extensão do Ubuntu renderiza menus exportados pela `libayatana-appindicator-glib` (org.gtk.Menus).
- Números de memória (medir no protótipo).
- Comportamento de `ksni` quando há menu: o duplo clique continua necessário, conforme a tabela da seção 1.

## Recomendação

1. **Stack principal: Tauri v2.** É o melhor equilíbrio entre UI moderna escura (web), acesso total ao sistema em Rust, memória bem menor que a do Electron e empacotamento `.deb` + AppImage oficial via `tauri-action` no GitHub Actions gratuito. Segredos via crate `keyring` (Secret Service), notificações via plugin, autostart via plugin (XDG `~/.config/autostart`).
2. **Ícone:** usar SNI com menu contendo "Abrir painel" como primeiro item. Considerar `ksni` no lugar do tray padrão, para ter `Activate` (duplo clique) e `SecondaryActivate` (clique do meio). Aceitar que, só com AppIndicator, **o clique simples sempre abre o menu**, e que a janela abre onde o Mutter decidir (centralizada).
3. **Se "clique simples abre o painel sob o ícone" for requisito duro:** adicionar uma **extensão GNOME fina e opcional**, distribuída junto no `.deb`. Ela só cria o botão no painel, chama `TogglePanel` via D-Bus e reposiciona a janela com `Meta.Window.move_frame`. Toda a lógica continua no app Tauri.
4. **Descartar:** Electron (memória e tamanho sem ganho funcional, já que tem as mesmas limitações de clique e posição), Python+GTK4 (sem SNI pronto para GTK4 e AppImage difícil) e extensão pura (UI limitada, código no processo do compositor, manutenção a cada versão do GNOME).
5. **Próximo passo sugerido:** protótipo de 1 dia com Tauri + `ksni` + extensão fina, medindo PSS e verificando o "pulo" de reposicionamento no GNOME 50.
