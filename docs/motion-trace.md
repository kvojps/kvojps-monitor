# Trace da Coreografia combinada

A decoração usa exclusivamente animações CSS. Não há `requestAnimationFrame`,
intervalos ou temporizadores decorativos em JavaScript. Apenas anéis de marcação,
brilho, faixas, bordas e fundo dos títulos se movem; medições, gráficos, textos e
alertas recebem alterações somente dos snapshots reais.

## Controle de execução

O núcleo informa a visibilidade inicial com `initial_state.panelVisible`, consultando
`WebviewWindow.is_visible()`. O evento nativo `panel-visibility` acompanha a
reabertura de uma janela existente e seu fechamento. Fechar destrói a webview,
encerrando seu contexto JavaScript e animações. Uma nova webview começa sem
animação e só a habilita depois de restaurar visibilidade e preferências.

`document.hidden` é uma restrição adicional, nunca a prova de visibilidade nativa.
Redução manual ou do sistema remove animações, transições e rolagem suave; mudanças
do sistema são acompanhadas em execução. Os Módulos cobertos pelas Configurações
também não animam.

## Medição no ticket 11

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
6. Execute os três pares de 60 s, alternando a ordem, e demais medições descritas
   no ticket 11. Registre versões, hardware e limitações no relatório daquele ticket.

Os testes automatizados de UI validam a política de execução, suas mudanças,
limpeza das inscrições, recriação e estabilidade do instrumento com fixture.
Eles não executam o compositor CSS nem substituem as medições nativas.
