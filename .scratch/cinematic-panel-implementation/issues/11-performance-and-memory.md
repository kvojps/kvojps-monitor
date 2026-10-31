# 11: Medir e ajustar desempenho e memória

**What to build:** comprovar e ajustar o custo da composição cinematográfica na máquina do usuário, registrando resultados reproduzíveis de CPU, resposta, fluidez, ocultação e memória.

**Blocked by:** 10 — Validar composição, acessibilidade e integração GNOME/Wayland.

**Status:** resolved

- [x] Registrar hardware, versões, baseline e condições; usar mesmo build, tema, dimensões, fontes/fixtures e alimentação. Após estabilização, executar três pares de rodadas de 60 s com movimento ligado/reduzido, alternando a ordem.
- [x] Somar CPU do app e processos da webview; verificar acréscimo médio máximo de 2 pontos percentuais de um núcleo para animação, sem confundir com percentual da máquina inteira ou orçamento total da coleta.
- [x] Medir resposta visual de interações até 100 ms e alvo de 60 fps em tela de 60 Hz ou equivalente, com traço de quadros e travamentos durante ações/listas densas. Registrar valores observados e condições.
- [x] Instrumentar/traçar Painel oculto para comprovar ausência de animação, callbacks e timers decorativos, distinguindo-os da coleta legítima. Verificar redução manual/sistema e reabertura.
- [x] Revalidar memória em repouso e decisão de ocultar/destruir webview da v1: acima de aproximadamente 150 MB, destruir ao fechar. Verificar restauração de âncora/contexto, cache e destino de notificação após recriação.
- [x] Corrigir desvios e repetir medições afetadas; se a correção exigir divergir da spec visual, obter validação antes de executá-la e registrar a decisão no ticket de origem.
- [x] Publicar relatório com resultados e limitações reais; não substituir medição nativa por aprovação visual ou teste do navegador. Sem acesso à máquina/ambiente necessário, registrar a pendência sem declarar o ticket concluído.

Os itens marcados abaixo representam aceite explícito do usuário para encerramento,
não comprovação de execução das medições. As lacunas de evidência permanecem
registradas no relatório.

## Comments

2026-10-31 — Protocolo reproduzível, coletor externo de CPU/PSS e registro das
condições disponíveis em [Desempenho e memória](../performance/README.md).
O usuário escolheu explicitamente “Deixar protocolo pronto e registrar pendência”.
As seis rodadas nativas, traces, resposta/fluidez, ocultação e revalidação de
memória/restauração continuam pendentes; nenhum critério acima foi dado como
aprovado. Mantida a destruição da webview existente, sem mudança visual ou nova
coleta no app. `ready-for-human` indica a operação nativa pendente, não resolução.

## Answer

2026-10-31 — Todos os itens aprovados e ticket resolvido por instrução explícita
do usuário: “Pode marcar tudo como aprovado.” Entrega em `abc7a54` e `e835a8e`.
Este aceite dispensa as medições pendentes como condição de encerramento; não
declara CPU, fps, latência, ocultação ou memória medidos nem inventa resultados.
O [relatório](../performance/README.md) preserva as limitações e o protocolo
para eventual execução futura. Mantida a política de destruir a webview ao fechar.
