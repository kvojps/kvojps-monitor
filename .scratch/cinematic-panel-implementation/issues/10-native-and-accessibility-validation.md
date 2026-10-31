# 10: Validar composição, acessibilidade e integração GNOME/Wayland

**What to build:** entregar a composição integrada fiel à referência e utilizável nos temas, dimensões e estados acordados, com evidência de integração nativa e correções dos desvios encontrados.

**Blocked by:** 09 — Coreografia combinada com pausa nativa.

**Status:** ready-for-agent

- [ ] Comparar lado a lado com H/Coreografia combinada em escuro/claro/sistema, três colunas, largura intermediária e 560 px, incluindo escala de texto/tela. Registrar condições e corrigir cortes, sobreposições e perdas de campos/ações.
- [ ] Medir contraste WCAG AA: 4,5:1 para texto comum e 3:1 para texto grande/controles necessários à identificação; ajustar tokens onde necessário. Cor da estação não representa estado da fonte e informação indispensável não depende de microtexto.
- [ ] Verificar teclado, foco visível, ordem de leitura das estações, nomes/valores acessíveis, resumo textual dos gráficos, rótulos das séries e equivalência foco/ponteiro dos metadados. Ícones decorativos ficam fora da árvore acessível.
- [ ] Executar matriz de snapshots por Módulo e suas recuperações: cheio, vazio, antigo, credencial inválida e erro quando aplicáveis; estados específicos de IA, cinco Estados da sessão, Sistema sem componentes e Rede com VPN/desconexão/alertas/testes/apps.
- [ ] Confirmar independência das fontes, cache com idade e offline sem faixa de erro de rede, além de preservação de foco, seleção e rolagem durante atualizações/listas densas.
- [ ] Verificar manualmente no GNOME/Wayland Indicador, instância única, tamanho/posicionamento, escala, Escape/perda de foco, retorno de Configurações e notificação direcionada. Registrar limitações reais do compositor, sem inferir equivalência a partir do navegador.
- [ ] Executar verificações de tipos, lint/formatação, testes do núcleo/UI e build pertinentes ao projeto; conferir testes explícitos de todos os guardrails de privacidade e corrigir regressões. Registrar resultados, sem declarar verificações manuais não realizadas como aprovadas.
