# Trace da Órbita contínua

A decoração usa exclusivamente animações CSS. Não há `requestAnimationFrame`,
intervalos ou temporizadores decorativos em JavaScript. Apenas anéis decorativos,
faixas superiores e brilho nas bordas se movem; medições, gráficos, textos e
alertas recebem alterações somente dos snapshots reais.

## Controle de execução

O núcleo informa a visibilidade inicial com `initial_state.panelVisible`, consultando
`WebviewWindow.is_visible()`. O evento nativo `panel-visibility` acompanha a
reabertura de uma janela existente e seu fechamento. Fechar destrói a webview,
encerrando seu contexto JavaScript e animações. Uma nova webview começa sem
animação e só a habilita depois de restaurar visibilidade e preferências.

`document.hidden` é uma restrição adicional, nunca a prova de visibilidade nativa.
Redução manual ou do sistema remove animações, transições e rolagem suave; mudanças
do sistema são acompanhadas em execução. Abrir Configurações pausa toda a decoração do Painel, inclusive as faixas
superiores. O brilho itinerante desaparece enquanto a decoração está pausada.

## Medição nativa

1. No mesmo build, tema, dimensões, fontes e alimentação, abra o inspetor da
   webview e grave o timeline de quadros/animações. Registre separadamente a coleta
   legítima. Não exporte DOM, snapshots, tráfego HTTP ou conteúdo de fontes.
2. Observe `document.documentElement.dataset.decoration`: `running` ou `paused`.
   A marca User Timing `panel-decoration` contém apenas `{ state }`. Cada mudança
   substitui a marca anterior para limitar memória; o timeline deve estar gravando
   para capturar as mudanças. A marca indica a política, não mede consumo ou fps.
3. Compare movimento normal e reduzido manualmente; altere também a preferência
   do sistema enquanto o Painel está aberto. Confira que arcos, números, gráficos,
   texto e alertas não oscilam entre atualizações reais.
4. Feche pelo Indicador/Escape/perda de foco. Confirme no trace de processos que a
   webview foi destruída e não há trabalho decorativo residual. O inspetor pode
   perder a conexão nesse momento: um trace apenas de navegador não comprova o
   requisito. Registre também a memória dos processos e CPU do app/webview.
5. Reabra e confira restauração das preferências, âncora e movimentos permitidos.
   Repita com redução ativa e após mudá-la com o Painel fechado.
6. Execute os três pares de 60 s, alternando a ordem, e as demais medições descritas
   em `docs/desempenho.md`. Registre versões, hardware e limitações junto ao relatório da rodada.

Os testes automatizados de UI validam a política de execução, suas mudanças,
limpeza das inscrições, recriação e estabilidade do instrumento com fixture.
Eles não executam o compositor CSS nem substituem as medições nativas.

## Parâmetros da decoração

- Anéis: volta linear em 32 s; somente os círculos decorativos recebem transformação.
- Faixas superiores: opacidade entre 100% e 55% em ciclo suave de 6 s; assinatura estática.
- Bordas: máscara exclui o interior e limita o gradiente a 2 px, opacidade inicial de 75%,
  percurso suave de 10 s por trecho (20 s de ida e volta), defasagens de −3/−6/−9 s.
- Entrada da borda superior: 180 ms. Hover e foco dentro do cartão respondem em 80 ms,
  com realce do brilho; outlines de foco existentes permanecem disponíveis sem movimento.
- Não há marcador animado na Faixa de bordo, animação de conclusão ou controle de duração/intensidade.

Inspeção de CSS com fixtures na WebKitGTK: órbitas de 32000 ms, faixas de 6000 ms,
gradientes alternados de 10000 ms e defasagens conferidas via `document.getAnimations()`.
Em 600 × 700, documento com 600 px e cartões com 592 px, sem transbordamento horizontal;
foco por teclado com resposta de 80 ms e brilho em opacidade 1.
Máscara `exclude`, padding de 2 px e opacidade de 0,75 conferidos nos temas claro e escuro;
faixa amostrada no meio do ciclo com opacidade de 0,55. Configurações e ocultação removem
as animações; retomada restaura o movimento e o foco sem alterar o instrumento. Redução
remove também o brilho. A suíte de UI cobre redução manual e mudanças da preferência do
sistema. Essa inspeção não mede CPU, fps ou memória; use o roteiro de `docs/desempenho.md`
para medições nativas.
