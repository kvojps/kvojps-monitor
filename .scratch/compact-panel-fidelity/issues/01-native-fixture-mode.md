# 01: Modo de fixtures nativo para desenvolvimento

Type: task
Status: resolved
Blocked by: None (can start immediately).

**What to build:** permitir abrir o app nativo (Tauri/WebKitGTK) em desenvolvimento exibindo as mesmas fixtures do protótipo B, para comparar o Painel de produção lado a lado com `compact-B.png` no mesmo motor de renderização e na mesma janela real.

- [x] Injetar as fixtures por portas falsas no seam único de E/S (sem caminhos paralelos na UI): IA `claude-usage`, Agentes `states`, GitHub/Trello/Rede `full`, Sistema `notebook`. Permitir escolher outra fixture por Módulo, para os estados não demonstrados pelo B.
- [x] Ativação só em desenvolvimento (flag de build ou variável no `tauri dev`); o build de produção não contém o modo nem as fixtures. Um teste garante isso.
- [x] No modo de fixtures: nenhuma coleta real, nenhuma leitura do keyring, nenhuma escrita em `~/.claude/settings.json`/`~/.codex/hooks.json`, nenhuma chamada de rede e nenhuma notificação real. Ações de escrita (mover card, testar velocidade) respondem de forma simulada e em memória.
- [x] Relógio fixado no instante da fixture `claude-usage`, como nas validações anteriores.
- [x] Documentar o comando de execução em `.scratch/compact-panel-fidelity/validation/README.md`.

## Comments

Origem: [mapa de fidelidade](../map.md), decisão Q12 da entrevista de 2026-10-31 (opção a). O app nativo hoje só mostra dados reais; a validação anterior foi feita só no Chromium e não captou a diferença.

## Answer

Resolvido em 2026-10-31. Divergência aprovada pelo usuário nesta sessão:

- **Injeção no nível do snapshot, não em portas brutas.** As fixtures do B são
  snapshots da UI (`src/lib/<módulo>/fixtures/*.json`), não saídas brutas que os
  coletores analisam; alimentar os seis coletores com portas falsas exigiria fixtures
  brutas novas e não garantiria o mesmo snapshot do B. No modo de fixtures nenhum
  coletor inicia: `src-tauri/src/fixtures.rs` publica os snapshots por
  `snapshot::publish` (o mesmo caminho de `initial_state` e `snapshot:<módulo>`) e
  registra coletores substitutos que respondem aos comandos em memória. A UI não muda.
- **Ativação:** feature Cargo `fixtures` (desligada por padrão), `npm run dev:fixtures`;
  escolha por Módulo em `KVOJPS_FIXTURES="ai=claude-failure,network=poor"`.
  `compile_error!` impede build `--release` com a feature; fixtures lidas do
  repositório em tempo de execução, nunca embutidas.
- **Relógio:** script de inicialização da webview fixa `Date.now()`/`new Date()` em
  `1793448000000`, como nas validações anteriores.
- Preferências e autostart em `$TMPDIR/kvojps-monitor-fixtures`; boas-vindas puladas.
- Testes: `fixtures::tests` (seleção, empacotamento, mover card ida/volta, teste de
  velocidade simulado, guarda de que o modo não referencia nenhuma porta real) e
  `build_tests::production_build_excludes_fixture_mode`. CI roda clippy e testes com a
  feature. Comando documentado em [validation/README.md](../validation/README.md).

Pendente: a primeira abertura nativa com `npm run dev:fixtures` não foi feita nesta
sessão, porque já havia uma instância `tauri dev` do usuário rodando (instância única).
O ticket 02 a exercita ao começar.
