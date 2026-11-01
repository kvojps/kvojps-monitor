# 07: Validação integrada nativa

Type: task
Status: ready-for-agent
Blocked by: 03, 04, 05, 06, 09, 10, 11, 12, 13

**What to build:** comprovar no app nativo, com o modo de fixtures, que o Painel inteiro está igual ao B, e fechar as pendências nativas deixadas pela implementação anterior.

- [ ] Captura nativa do Painel inteiro em 1100 × 700 e 1000 × 700, temas escuro e claro, lado a lado com `compact-B.png` e `compact-B-light-long.png`; sem rolagem com as fixtures do B. Fallback (950/650 px, texto ampliado, nomes longos) sem cortar conteúdo nem esconder alertas.
- [ ] Matriz de estados por fixture nos dois temas, conferindo que as linhas condicionais seguem o vocabulário do B e que o estado normal não tem nada além do B.
- [ ] Pendências nativas da [validação anterior](../../compact-panel-implementation/validation/integrated.md#pendências-nativas): dimensões lógicas/escala, área útil, abertura pelo Indicador, ocultação por Escape/perda de foco, instância única, arraste real no WebKitGTK, Escape com seletor nativo aberto, notificação direcionada, resposta ≤ 100 ms e 60 fps, A/B de CPU e pausa com Painel oculto.
- [ ] Guardrails de privacidade executados no seam de portas; build de produção sem modo de fixtures.
- [ ] **Aceite visual integral do usuário** registrado aqui e no mapa.

## Comments

Origem: [mapa de fidelidade](../map.md), decisões Q8, Q11, Q12 da entrevista de 2026-10-31.

Achado da [Captura nativa para agentes](08-agent-native-capture.md) (2026-11-01): em 1920 × 1080, escala 1, o Painel abre com área visível de 1048 × 648, não 1100 × 700 (o tao no Wayland desconta 52 px por eixo). Conferir nas dimensões lógicas e decidir se `panel.rs` compensa. Capturas: `scripts/capture-panel.sh`.
