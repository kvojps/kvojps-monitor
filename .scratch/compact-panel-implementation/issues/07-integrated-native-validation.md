# 07: Validar o Painel compacto integrado no GNOME/Wayland

**What to build:** entregar o Painel compacto integrado conforme B, corrigindo desvios e registrando evidência de comportamento, acessibilidade e desempenho no app nativo.

**Blocked by:** 02 — Instrumentos e ações compactas de IA; 03 — Anel de Agentes e detalhe de Sessões de agente; 04 — Resumos compactos de GitHub e Sistema; 05 — Resumo de Rede e detalhe de Aplicativos de rede; 06 — Resumo do Trello e quadro sobreposto com movimentação real.

**Status:** ready-for-agent

- [ ] Comparar produção com B e o quadro aprovado em 1100 × 700 e 1000 × 700, nos temas escuro/claro/sistema. No alvo de 1100 × 700, fixtures e texto padrão exibem todos os resumos sem rolagem principal; registrar o resultado em 1000 × 700 sem ocultar overflow para passar.
- [ ] Exercitar texto a 125%, nomes longos, listas densas, área útil reduzida e escala de monitor; permitir fallback/rolagem sem cortar conteúdo, esconder alertas ou reduzir automaticamente fonte. Todas as ações continuam alcançáveis.
- [ ] Conferir a matriz integrada de snapshots normal, vazio, antigo, falha, offline e credenciais ausentes/inválidas; IA reiniciada sem leitura; anel zero/indisponível/cinco estados/encerradas; listas densas/vazias; motivos e estados do teste de Rede; ausências/alertas de Sistema.
- [ ] Exercitar movimentação real do Trello, reversão, resumo sincronizado, seleção preservada, arraste e alternativa por teclado, listas fora da área visível e abertura externa. Conferir foco, Escape, reabertura e sessão direcionada por notificação, incluindo alvo retido e indisponível.
- [ ] Validar no GNOME/Wayland dimensões lógicas/escala, monitor e área útil, detalhes contidos na janela, abertura/ocultação, instância única e caminhos do Indicador previstos no ADR vigente. Não tratar testes do navegador como comprovação nativa.
- [ ] Auditar contraste AA (4,5:1 texto comum; 3:1 texto grande e controles), ordem de leitura, foco visível, nomes/descrições por teclado e resumos textuais dos gráficos; corrigir os desvios encontrados.
- [ ] Medir resposta visual de até 100 ms e alvo de 60 fps em tela de 60 Hz ou equivalente, incluindo listas densas e ações. Registrar condições, traços de quadros e travamentos observados.
- [ ] Medir movimento ligado/reduzido no mesmo build, dimensões, tema, fontes/fixtures e alimentação: após estabilização, três pares de rodadas de 60 s, alternando ordem, registrando baseline, hardware e versão e somando CPU do app/webview. Verificar acréscimo médio máximo provisório de 2 pontos percentuais de um núcleo para animação, conforme o complemento anterior.
- [ ] Com Painel oculto, verificar ausência de trabalho decorativo por instrumentação/trace, distinguindo coleta legítima; verificar redução manual/sistema, cadências por visibilidade e regra de memória/ocultar ou destruir webview da v1.
- [ ] Executar testes de regressão dos guardrails no seam existente: keyring obrigatório; logs/cache/notificações somente com metadados permitidos; hooks/status line alterados apenas por clique explícito, aditivamente e de modo reversível; token Claude só em memória e sem renovação.
- [ ] Registrar evidências, condições e limitações reais, corrigir falhas e repetir verificações afetadas. Qualquer divergência necessária da spec exige aprovação e registro no ticket de origem antes de seguir.

## Comments

Origem: critérios de verificação da especificação aprovada do Painel compacto, complemento visual cinematográfico, spec da v1 e ADR do Indicador. Divisão aprovada em 2026-10-31. As medições do protótipo são evidência de navegador, não resultados nativos já obtidos.
