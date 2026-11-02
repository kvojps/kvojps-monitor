# 32: Aplicar o movimento Órbita contínua

**What to build:** aplicar o repertório decorativo aprovado à composição com assinatura e Faixa de bordo, preservando leitura imediata, interação e redução de movimento.

**Blocked by:** 27 — Integrar assinatura e Faixa de bordo às atualizações do GitHub.

**Status:** done

- [x] Aplicar anéis decorativos com 32 segundos por volta e variação suave de opacidade da faixa superior em ciclo de 6 segundos, mínimo de 55%, sem animar os textos.
- [x] Aplicar brilho em gradiente restrito às bordas de 2 px, intensidade inicial de 75%, percurso de 10 segundos por trecho e ida e volta em 20 segundos, easing suave e defasagens de −3/−6/−9 segundos entre posições dos cartões.
- [x] Aplicar entrada das bordas em 180 ms e resposta de hover em 80 ms, preservando foco e equivalente por teclado. Não criar controles de produto para intensidade ou duração.
- [x] Manter assinatura, arcos de medição, números, textos e alertas estáticos e imediatamente legíveis. Decoração não representa progresso ou atividade; não criar marcador animado na faixa nem animação local de conclusão.
- [x] Redução manual ou do sistema interrompe animações e transições e remove brilho itinerante; preferência do sistema prevalece. Painel oculto ou coberto por Configurações não executa animações; fechamento continua destruindo a webview.
- [x] Implementar decoração somente em CSS, preservando guardrails e metas de desempenho. Verificar temas, largura reduzida, teclado, redução, pausa e retomada com fixtures da UI e inspeção das animações; evidência nativa integrada fica no ticket 33.


## Notas da implementação

- Repertório aplicado em `src/motion.css`: órbitas de 32 s; faixa superior de 6 s
  (55–100%) sem afetar a assinatura; gradiente com máscara de 2 px e intensidade
  inicial de 75%, percurso alternado de 10 s por trecho e defasagens de −3/−6/−9 s.
  Entrada das bordas em 180 ms; hover e foco dentro do cartão respondem em 80 ms.
- A política de execução agora exige área do Painel ativa, além de preferências
  restauradas e visibilidade nativa/documento. Pausa remove animações, transições
  e brilho itinerante. Redução do sistema continua prevalecendo. Sem nova E/S,
  temporizador decorativo, controle de produto ou alteração no fechamento nativo.
- TDD no seam existente da UI com fixtures: teste reproduziu decoração ativa em
  Configurações e passou após a correção. Cobertura de retorno por teclado,
  restauração do foco, estabilidade da leitura e redução do sistema na retomada.
- Inspeção WebKitGTK com fixtures em 1100 × 700 e 600 × 700, temas claro e escuro:
  tempos/defasagens via `document.getAnimations()`, máscara `exclude` e 2 px,
  opacidade inicial 0,75 e mínima da faixa 0,55. Largura reduzida sem transbordamento
  horizontal; foco responde em 80 ms. Configurações/ocultação/redução sem animações;
  brilho removido durante redução; retomada preserva leitura e foco.
- Protocolo e evidência resumidos em `docs/motion-trace.md`. CPU, fps, memória e
  evidência nativa integrada permanecem no ticket 33; inspeção CSS não os substitui.
- Validação: 352 testes de UI, 305 testes unitários Rust e 3 testes de integração
  de hooks passaram (integração fora do sandbox para permitir sockets locais).
  Typecheck sem erros/avisos, build e `git diff --check` passaram.
- Revisão `code-review` em dois eixos contra `d9b815be7490d7b21abe0d4f1a51d8efb16a5556`:
  Standards sem achados; Spec sem divergências, com evidência de inspeção registrada acima.
