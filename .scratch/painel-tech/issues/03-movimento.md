# Que movimentos tornam o Painel mais expressivo sem atrapalhar a leitura?

Type: prototype
Status: resolved
Labels: wayfinder:prototype
Parent: [Aprimorar a identidade tech do Painel](../map.md)

## Question

Qual intensidade e repertório de animações melhoram abertura, interação e atualização do Painel atual? Comparar propostas no contexto B e separar decoração de atividade real. Validar durações, frequência, transições, simultaneidade, comportamento reduzido e pausa com Painel oculto ou coberto por Configurações. Preservar foco, legibilidade e metas de desempenho; explicitar quais limites atuais da spec precisam mudar.

## Comments

- 01/11/2026: protótipo descartável em [MotionPrototype.svelte](../../../src/lib/motion-prototype/MotionPrototype.svelte), no contexto do Painel existente com fixtures. Executar `npm run prototype:motion` e abrir `http://127.0.0.1:1424/motion-prototype.html?variant=C`. A — Órbita contínua; B — Resposta aos eventos; C — Coreografia de bordo. Preservada a grade por instrução do mapa. Controles de cenário manual, reabertura, tema claro, largura reduzida, redução de movimento e simulação de pausa; respeita também redução do sistema e página oculta. A simulação mantém a prévia visível para comparação; não reproduz a destruição da webview nativa. Sem eventos reais, duração de resultado ou temporizador de conclusão. Dados, textos e medições não animam. A entrada anima somente bordas; assinatura estática. `npm run check` sem erros ou avisos. Aguardando escolha humana e validação de repertório, durações e frequência. Nenhuma alteração da spec; captura em branch local `prototype/*` após validação, sem push. Metas de CPU e fluidez não foram medidas neste protótipo.

- 01/11/2026: corrigida fixture com redução de movimento ligada indevidamente, que aplicava a regra global de supressão. Isolada decoração nativa para comparar os repertórios sem interferência. Reprodução via Chromium/CDP: antes, duas órbitas no DOM e zero animações; depois, C com duas animações de órbita e brilho da faixa ativos. Checagem Svelte sem erros ou avisos.

- 01/11/2026: usuário indicou A e pediu bordas suaves, mas notórias. Prévia A ajustada com brilho em gradiente restrito à borda de 2 px, percurso alternado de 10 s por trecho e intensidade inicial de 75%, ajustáveis nos controles (6–18 s; 30–100%). Defasagem entre cartões evita sincronismo. Efeito decorativo sem significado de atividade; redução de movimento ou pausa remove o brilho. Aguardando avaliação do ajuste; não é resolução final nem alteração da spec.


## Answer

O usuário escolheu **A — Órbita contínua**, pediu bordas “bem suaves mas notórias” e aprovou a prévia ajustada com “Combinado”.

- Preservar composição B, dados imediatamente legíveis, assinatura estática e Faixa de bordo aprovada. Decoração não indica atividade ou progresso real.
- Órbitas decorativas: 32 s por volta. Faixa superior: variação suave de opacidade em ciclo de 6 s, mínimo de 55%; textos permanecem estáticos.
- Bordas: brilho em gradiente restrito a 2 px, intensidade inicial de 75%, percurso de 10 s por trecho com ida e volta (20 s para o ciclo completo), easing suave e defasagens de −3/−6/−9 s entre posições dos cartões. Usar o padrão da prévia aprovada como referência; controles de intensidade e duração pertencem somente ao protótipo.
- Entrada das bordas em 180 ms e resposta de hover em 80 ms, sem atrasar a leitura ou deslocar foco. Números, textos, medições e alertas não se movem nem piscam. A não acrescenta marcador animado de atividade na Faixa de bordo nem animação local de conclusão.
- Redução manual ou do sistema interrompe animações e transições e remove o brilho itinerante. Com Painel oculto ou coberto por Configurações, não executar animações. Preservar destruição da webview ao fechar; a simulação conserva a prévia visível apenas para comparação.
- Revisões propostas à spec: órbitas de 48 s para 32 s, repertório da faixa superior, brilho itinerante nas bordas e transições de entrada/interação. Preservar contraste e metas de resposta ≤ 100 ms, 60 fps e CPU adicional média ≤ 2 pontos percentuais de um núcleo. Desempenho nativo permanece por medir.
- Protótipo preservado na branch **local** `prototype/panel-motion`, commit `e2770bd2476ca0f80353e556303c7e3e679771aa`. Nessa branch, executar `npm run prototype:motion` e abrir `http://127.0.0.1:1424/motion-prototype.html?variant=A`. Sem push; arquivos descartáveis retirados da main.
- Validação: `npm run check` sem erros ou avisos; Chromium/CDP confirmou animações de órbitas, faixa e bordas de A em execução após correção da fixture. Aprovação visual pelo usuário; sem medição nativa de desempenho.

A spec não foi alterada; a revisão fica para **Como consolidar as melhorias aprovadas na spec da v1?**. Não surgiram novos tickets. A próxima decisão da fronteira é **Quais eventos reais alimentam a Faixa de bordo?**.
