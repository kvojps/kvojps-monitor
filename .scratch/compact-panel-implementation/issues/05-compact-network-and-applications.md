# 05: Resumo de Rede e detalhe de Aplicativos de rede

**What to build:** consultar métricas de Rede no resumo compacto e abrir Aplicativos de rede para identificar taxas de download/upload por aplicativo e Outros.

**Blocked by:** 01 — Composição compacta e janela adaptada à área útil.

**Status:** ready-for-agent

- [ ] Mostrar Qualidade e Segurança separadas, conexão/VPN, latência da Internet, teste de velocidade em Mbps, jitter/perda, métricas do gateway e gráfico de tráfego com duas séries e taxas atuais, conforme B.
- [ ] Preservar motivos de Atenção/Risco; nunca afirmar “Rede segura”. Separar capacidade medida em Mbps de tráfego em KB/s ou MB/s, usando amostras reais sem inventar tendência ou preencher lacunas.
- [ ] Manter idade/data do teste acessível por ponteiro/foco e sinalização direta se antigo/indisponível; preservar acionamento, desabilitação, limites e estados indisponível/em andamento/falha da v1.
- [ ] Abrir detalhe com nomes e taxas de download/upload, até cinco aplicativos e Outros com sua explicação; não inventar atribuição por processo além dos dados da fonte. Preservar escala comum local das barras.
- [ ] Limitar detalhe a até 680 px/90% da largura e 80% da altura, com rolagem e foco restrito. Fechar/Escape restauram foco ao acionador; Escape sem detalhe oculta o Painel. Preservar contexto válido e estabilidade do foco nas atualizações.
- [ ] Testar fixtures de desconexão, VPN, motivos de risco/atenção, ausência/antiguidade/falha de leitura, estados do teste e lista vazia/densa de aplicativos, além de comandos, teclado e resumos acessíveis dos gráficos.
- [ ] Preservar coleta e cadências; abrir a sobreposição não multiplica coleta nem muda fontes. Manter o seam e guardrails existentes.

## Comments

Origem: especificação aprovada do Painel compacto e revisão de Rede do complemento visual cinematográfico. Divisão aprovada em 2026-10-31. Não depende do ticket de Agentes: reutilizar eventual mecanismo comum sem tornar essa reutilização um bloqueio artificial.
