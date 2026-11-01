# 08: Captura nativa para agentes

Type: task
Status: ready-for-agent
Blocked by: None (can start immediately).

## Question

Como um agente obtém, sem ajuda do usuário, uma captura da janela nativa do Painel em modo de fixtures (1100 × 700 e 1000 × 700) e a medida de rolagem do conteúdo? No ticket 02 o GNOME recusou `org.gnome.Shell.Screenshot` (“Screenshot is not allowed”), então a conferência dependeu do usuário.

- [ ] Achar um caminho que funcione no GNOME/Wayland só em desenvolvimento (por exemplo, captura da própria webview ou comando exclusivo da feature `fixtures`), sem nada no build de produção.
- [ ] Expor também `scrollHeight`/`clientHeight` do Painel, para comprovar “sem rolagem” sem olhar.
- [ ] Documentar em `validation/README.md`.

## Comments

Origem: ticket 02 (2026-11-01). Destrava a conferência dos tickets 03–07.
