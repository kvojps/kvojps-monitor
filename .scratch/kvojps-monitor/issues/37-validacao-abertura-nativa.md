# 37: Validar a abertura no app nativo

**What to build:** verificar a abertura integrada em Tauri/WebKitGTK no Ubuntu/GNOME/Wayland e registrar evidências dos contratos de comportamento e visual.

**Blocked by:** 35 — Implementar prontidão e ciclo de abertura do Painel; 36 — Aplicar Arquitetura à abertura do Painel.

**Status:** ready-for-human

- [ ] Exercitar primeira abertura após iniciar, reaberturas, nova solicitação com janela já aberta, fechamento durante preparo/transição e reabertura imediata; confirmar login automático silencioso e destruição da webview.
- [ ] Medir mínimo total de 1,2 segundo desde a primeira apresentação visível, incluindo transição; comparar preparo rápido, preparo de 3 segundos e limite de 5 segundos, sem intervalo mínimo adicional após demora.
- [ ] Validar ausência de rede/cache, preferências indisponíveis, recuperação por área, falha da interface com tentar novamente/fechar e respostas tardias. Confirmar que a atualização remota não bloqueia abrir.
- [ ] Conferir Arquitetura (C), nome “kvojps monitor”, mensagens verdadeiras, transição para o tema escolhido, restauração da Âncora, destino por notificação e boas-vindas.
- [ ] Verificar temas claro/escuro/sistema, 1100 × 700, 1000 × 700, largura reduzida, texto ampliado, contraste, teclado, foco e leitor de tela; redução manual e do sistema sem animação/transição.
- [ ] Aplicar as metas e o protocolo de desempenho vigentes da spec, registrando ambiente, tempos, traces e memória/PSS na abertura, fechamento e reabertura; confirmar ausência de trabalho decorativo após fechar. Evidência de navegador ou protótipo não substitui o app nativo.
- [ ] Executar verificações pertinentes de núcleo/UI e privacidade; registrar resultados e limitações em `.scratch/kvojps-monitor/validation/37/`. Ausência de acesso ao ambiente nativo não conta como aprovação.
- [ ] Corrigir desvios dentro do contrato aprovado; mudanças de decisão retornam ao usuário. Não encerrar ou alterar pendências alheias do ticket de validação nativa anterior.

## Fontes aprovadas

- [Definir prontidão e ciclo de abertura do Painel](../../panel-opening/issues/01-opening-readiness.md).
- [Validar sequência visual cinematográfica de abertura](../../panel-opening/issues/02-opening-visual.md).
- [Abertura cinematográfica do Painel](../../panel-opening/map.md).

## Comments

### 02/11/2026 — rodada de validação parcial

Resultados e limitações em [validation/37/README.md](../validation/37/README.md).
Check sem erros/avisos, 368 testes UI, 305 unitários Rust e 3 de integração
aprovados; build UI e build nativo debug com fixtures aprovados. A captura
Tauri/WebKitGTK em Wayland com D-Bus isolado falhou com
`metrics script did not answer`, sem PNG nem métricas. Não houve aceite nativo,
medição de desempenho ou correção de produção. Todos os itens continuam abertos
e o status permanece `ready-for-human`. Tickets 25 e 33 não foram alterados.
