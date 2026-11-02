# 32: Aplicar o movimento Órbita contínua

**What to build:** aplicar o repertório decorativo aprovado à composição com assinatura e Faixa de bordo, preservando leitura imediata, interação e redução de movimento.

**Blocked by:** 27 — Integrar assinatura e Faixa de bordo às atualizações do GitHub.

**Status:** ready-for-agent

- [ ] Aplicar anéis decorativos com 32 segundos por volta e variação suave de opacidade da faixa superior em ciclo de 6 segundos, mínimo de 55%, sem animar os textos.
- [ ] Aplicar brilho em gradiente restrito às bordas de 2 px, intensidade inicial de 75%, percurso de 10 segundos por trecho e ida e volta em 20 segundos, easing suave e defasagens de −3/−6/−9 segundos entre posições dos cartões.
- [ ] Aplicar entrada das bordas em 180 ms e resposta de hover em 80 ms, preservando foco e equivalente por teclado. Não criar controles de produto para intensidade ou duração.
- [ ] Manter assinatura, arcos de medição, números, textos e alertas estáticos e imediatamente legíveis. Decoração não representa progresso ou atividade; não criar marcador animado na faixa nem animação local de conclusão.
- [ ] Redução manual ou do sistema interrompe animações e transições e remove brilho itinerante; preferência do sistema prevalece. Painel oculto ou coberto por Configurações não executa animações; fechamento continua destruindo a webview.
- [ ] Implementar decoração somente em CSS, preservando guardrails e metas de desempenho. Verificar temas, largura reduzida, teclado, redução, pausa e retomada com fixtures da UI e inspeção das animações; evidência nativa integrada fica no ticket 33.
