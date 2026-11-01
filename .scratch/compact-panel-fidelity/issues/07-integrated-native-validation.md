# 07: Validação integrada nativa

Type: task
Status: resolved
Blocked by: 03, 04, 05, 06, 09, 10, 11, 12, 13, 14

**What to build:** comprovar no app nativo, com o modo de fixtures, que o Painel inteiro está igual ao B, e fechar as pendências nativas deixadas pela implementação anterior.

- [x] Captura nativa do Painel inteiro em 1100 × 700 e 1000 × 700, temas escuro e claro, lado a lado com `compact-B.png` e `compact-B-light-long.png`; sem rolagem com as fixtures do B. Fallback (950/650 px, texto ampliado, nomes longos) sem cortar conteúdo nem esconder alertas.
- [x] Matriz de estados por fixture nos dois temas, conferindo que as linhas condicionais seguem o vocabulário do B e que o estado normal não tem nada além do B.
- [ ] (não executadas; encerrado por decisão do usuário) Pendências nativas da [validação anterior](../../compact-panel-implementation/validation/integrated.md#pendências-nativas): dimensões lógicas/escala, área útil, abertura pelo Indicador, ocultação por Escape/perda de foco, instância única, arraste real no WebKitGTK, Escape com seletor nativo aberto, notificação direcionada, resposta ≤ 100 ms e 60 fps, A/B de CPU e pausa com Painel oculto.
- [x] Guardrails de privacidade executados no seam de portas; build de produção sem modo de fixtures.
- [x] **Aceite visual integral do usuário** registrado aqui e no mapa.

## Comments

Origem: [mapa de fidelidade](../map.md), decisões Q8, Q11, Q12 da entrevista de 2026-10-31.

Achado da [Captura nativa para agentes](08-agent-native-capture.md) (2026-11-01): em 1920 × 1080, escala 1, o Painel abre com área visível de 1048 × 648, não 1100 × 700 (o tao no Wayland desconta 52 px por eixo). Conferir nas dimensões lógicas e decidir se `panel.rs` compensa. Capturas: `scripts/capture-panel.sh`.

### 2026-11-01 — Rodada AFK (agente)

- **B no app nativo** (`scripts/capture-panel.sh`, fixtures do B): 1100 × 700 e 1000 × 700, escuro e claro, `overflowing` vazio e `scrollHeight` = `clientHeight` = 641. Visual igual ao `compact-B.png`, com as divergências já aprovadas (anéis que crescem, Trello sem prévia do card, sem linha de demonstração/rodapé). Fallback 950 × 650: uma coluna, rola 483 px, nada cortado nem escondido.
- **Matriz de estados**: 20 combinações cobrindo todas as fixtures dos seis Módulos, nos dois temas, em 1100 × 700. Linhas condicionais no vocabulário do B (`small` colorido ou área única da variante C). Só uma combinação rola: 9 px, com três alertas de Sistema mais a linha “Teste indisponível” da Rede (`ai=claude-off,agents=waiting,github=no-reading,trello=no-reading,network=processes-empty,system=warning`). **Aceito pelo usuário**: sem rolagem só é exigido com as fixtures do B; em alerta extremo a rolagem vale se nada some.
- **1048 × 648 → 1100 × 700 (decisão do usuário: compensar).** Causa: no Wayland, o tao liga CSD (barra de título vazia mais a sombra do GTK) nas janelas sem decoração, e o tamanho pedido inclui essas margens. `panel.rs` agora aprende a margem uma vez por processo: assim que a alocação do conteúdo coincide com o tamanho que o GTK assentou, compara com o tamanho pedido e pede de novo com a margem somada, sempre limitado à área útil. Conferido no app nativo: abre com área visível de 1100 × 700 (antes 1048 × 648). Testes `window_margin_is_asked_on_top_of_the_visible_target_within_the_work_area` e `margin_is_learned_only_from_a_plausible_allocation`.
- **Guardrails de privacidade no seam de portas**: `cargo test` com 274 testes aprovados, incluindo token só lido, keyring obrigatório, logs e cache só com metadados, hooks aditivos e reversíveis, e `settings.json` só após clique. `npm test` com 271 aprovados e `npm run check` sem erros.
- **Build de produção sem modo de fixtures**: `npm run build` mais `cargo build --release`. O binário não contém `KVOJPS_FIXTURES`, o relógio fixo, `--capture` nem o snapshot do WebKit; `dist/` não contém protótipo nem demonstração. A única ocorrência de `claude-usage` é o nome do arquivo de cache de produção.
- **Incidente**: durante a rodada, um `cargo test` recompilou `target/debug/kvojps-monitor` sem a feature, e a matriz abriu esse binário com coletores reais por alguns minutos. A entrada de autostart do usuário aponta para `target/debug` (o app reescreve a entrada a cada partida).
- **Pendente, com o usuário** (wizard `native-validation.sh` no scratchpad da sessão): Indicador (menu, duplo clique, clique do meio), Escape e perda de foco, instância única, arraste real, Escape com o seletor “Mover para” aberto, notificação direcionada, resposta e fluidez (percepção), A/B de CPU 3 × 60 s no release e Painel oculto.
- **Follow-up to the size fix (2026-11-01):**
  - **Deadlock:** the GTK callback held the margin lock while it called Tauri/GTK, which can re-enter the same callback, so the main thread hung intermittently (captures without an answer). Fixed: the lock is never held across those calls.
  - **Band in the IA e Agentes card:** the leftover past the anéis' cap now goes to the Agents legend (see [Preenchimento vertical do cartão IA e Agentes](11-ai-agents-vertical-fill.md)).
  - **Capture:** it no longer resizes a Panel that's already at the requested size, so it shows the natural opening. It also reports `intelligenceFree` and `dial`.

## Answer

Resolvido em 2026-11-01 com o **aceite visual integral do usuário**: “Pode marcar como resolvido, visualmente está ok.”

- **B no app nativo:** sem rolagem em 1100 × 700 e 1000 × 700, nos temas escuro e claro. O fallback de 950 × 650 rola sem cortar nada. A matriz de 20 combinações cobre todas as fixtures nos dois temas. Uma única rolagem de 9 px, com três alertas de Sistema, foi aceita.
- **1048 × 648 compensado:** o Painel abre com área visível de 1100 × 700 no Wayland (`ca46024`). A sobra do cartão IA e Agentes vai para a legenda de Agentes (`3530970`; divergência aprovada e registrada em [Preenchimento vertical do cartão IA e Agentes](11-ai-agents-vertical-fill.md)).
- **Guardrails de privacidade** no seam de portas e **build de produção sem modo de fixtures** conferidos (274 testes Rust, 271 de UI).
- **Não executado:** as pendências nativas de interação e desempenho, a saber Indicador (menu, duplo clique, clique do meio), Escape e perda de foco, instância única, arraste real, Escape com o seletor nativo, notificação direcionada, resposta ≤ 100 ms e 60 fps, A/B de CPU e pausa com Painel oculto. O usuário encerrou o ticket sem rodar o wizard (`native-validation.sh`, descartável, no scratchpad da sessão). Essas pendências ficam como lacuna de evidência, não como item aprovado.
