# 11: Medir e ajustar desempenho e memória

**What to build:** comprovar e ajustar o custo da composição cinematográfica na máquina do usuário, registrando resultados reproduzíveis de CPU, resposta, fluidez, ocultação e memória.

**Blocked by:** 10 — Validar composição, acessibilidade e integração GNOME/Wayland.

**Status:** ready-for-agent

- [ ] Registrar hardware, versões, baseline e condições; usar mesmo build, tema, dimensões, fontes/fixtures e alimentação. Após estabilização, executar três pares de rodadas de 60 s com movimento ligado/reduzido, alternando a ordem.
- [ ] Somar CPU do app e processos da webview; verificar acréscimo médio máximo de 2 pontos percentuais de um núcleo para animação, sem confundir com percentual da máquina inteira ou orçamento total da coleta.
- [ ] Medir resposta visual de interações até 100 ms e alvo de 60 fps em tela de 60 Hz ou equivalente, com traço de quadros e travamentos durante ações/listas densas. Registrar valores observados e condições.
- [ ] Instrumentar/traçar Painel oculto para comprovar ausência de animação, callbacks e timers decorativos, distinguindo-os da coleta legítima. Verificar redução manual/sistema e reabertura.
- [ ] Revalidar memória em repouso e decisão de ocultar/destruir webview da v1: acima de aproximadamente 150 MB, destruir ao fechar. Verificar restauração de âncora/contexto, cache e destino de notificação após recriação.
- [ ] Corrigir desvios e repetir medições afetadas; se a correção exigir divergir da spec visual, obter validação antes de executá-la e registrar a decisão no ticket de origem.
- [ ] Publicar relatório com resultados e limitações reais; não substituir medição nativa por aprovação visual ou teste do navegador. Sem acesso à máquina/ambiente necessário, registrar a pendência sem declarar o ticket concluído.
