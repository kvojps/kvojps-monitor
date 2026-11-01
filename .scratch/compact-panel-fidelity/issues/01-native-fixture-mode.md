# 01: Modo de fixtures nativo para desenvolvimento

Type: task
Status: ready-for-agent
Blocked by: None (can start immediately).

**What to build:** permitir abrir o app nativo (Tauri/WebKitGTK) em desenvolvimento exibindo as mesmas fixtures do protótipo B, para comparar o Painel de produção lado a lado com `compact-B.png` no mesmo motor de renderização e na mesma janela real.

- [ ] Injetar as fixtures por portas falsas no seam único de E/S (sem caminhos paralelos na UI): IA `claude-usage`, Agentes `states`, GitHub/Trello/Rede `full`, Sistema `notebook`. Permitir escolher outra fixture por Módulo, para os estados não demonstrados pelo B.
- [ ] Ativação só em desenvolvimento (flag de build ou variável no `tauri dev`); o build de produção não contém o modo nem as fixtures. Um teste garante isso.
- [ ] No modo de fixtures: nenhuma coleta real, nenhuma leitura do keyring, nenhuma escrita em `~/.claude/settings.json`/`~/.codex/hooks.json`, nenhuma chamada de rede e nenhuma notificação real. Ações de escrita (mover card, testar velocidade) respondem de forma simulada e em memória.
- [ ] Relógio fixado no instante da fixture `claude-usage`, como nas validações anteriores.
- [ ] Documentar o comando de execução em `.scratch/compact-panel-fidelity/validation/README.md`.

## Comments

Origem: [mapa de fidelidade](../map.md), decisão Q12 da entrevista de 2026-10-31 (opção a). O app nativo hoje só mostra dados reais; a validação anterior foi feita só no Chromium e não captou a diferença.
