# 05: Resumo de Rede e detalhe de Aplicativos de rede

**What to build:** consultar métricas de Rede no resumo compacto e abrir Aplicativos de rede para identificar taxas de download/upload por aplicativo e Outros.

**Blocked by:** 01 — Composição compacta e janela adaptada à área útil.

**Status:** resolved

- [x] Mostrar Qualidade e Segurança separadas, conexão/VPN, latência da Internet, teste de velocidade em Mbps, jitter/perda, métricas do gateway e gráfico de tráfego com duas séries e taxas atuais, conforme B.
- [x] Preservar motivos de Atenção/Risco; nunca afirmar “Rede segura”. Separar capacidade medida em Mbps de tráfego em KB/s ou MB/s, usando amostras reais sem inventar tendência ou preencher lacunas.
- [x] Manter idade/data do teste acessível por ponteiro/foco e sinalização direta se antigo/indisponível; preservar acionamento, desabilitação, limites e estados indisponível/em andamento/falha da v1.
- [x] Abrir detalhe com nomes e taxas de download/upload, até cinco aplicativos e Outros com sua explicação; não inventar atribuição por processo além dos dados da fonte. Preservar escala comum local das barras.
- [x] Limitar detalhe a até 680 px/90% da largura e 80% da altura, com rolagem e foco restrito. Fechar/Escape restauram foco ao acionador; Escape sem detalhe oculta o Painel. Preservar contexto válido e estabilidade do foco nas atualizações.
- [x] Testar fixtures de desconexão, VPN, motivos de risco/atenção, ausência/antiguidade/falha de leitura, estados do teste e lista vazia/densa de aplicativos, além de comandos, teclado e resumos acessíveis dos gráficos.
- [x] Preservar coleta e cadências; abrir a sobreposição não multiplica coleta nem muda fontes. Manter o seam e guardrails existentes.

## Comments

Origem: especificação aprovada do Painel compacto e revisão de Rede do complemento visual cinematográfico. Divisão aprovada em 2026-10-31. Não depende do ticket de Agentes: reutilizar eventual mecanismo comum sem tornar essa reutilização um bloqueio artificial.


## Answer

Implementado em 2026-10-31:

- Resumo de Rede mantém Qualidade/Segurança separadas, conexão/VPN, latência, teste em Mbps com idade por foco/ponteiro (direta se antigo/indisponível), jitter/perda, gateway e gráfico de duas séries em KB/s/MB/s; motivos de Atenção/Risco preservados, sem “Rede segura”.
- Lista por aplicativo saiu da superfície principal: acionador “Aplicativos de rede” com resumo visível (contagem ou estado) abre diálogo modal nativo (até 680 px/90% × 80%, rolagem) com até cinco aplicativos e Outros com explicação e escala comum local.
- Fechar/Escape fecham o detalhe e restauram foco ao acionador sem alterar a Âncora; Escape sem detalhe oculta o Painel. Atualizações de snapshot mantêm o detalhe aberto e o foco.
- Abrir o detalhe não reporta nova visibilidade nem dispara coleta; núcleo, fontes, cadências e guardrails sem alteração.
- Validação: 231 testes UI, 271 unitários Rust e 3 de hooks; check e build aprovados.
- Revisão: Spec sem lacunas funcionais (asserção da Âncora adicionada). Standards sem violações; renomeação e contagem explícita aplicadas. Pendência de manutenção: o ciclo do diálogo (abrir/fechar/Escape/restaurar foco) duplica o de Agentes; extrair um componente comum fica para refatoração posterior, sem bloquear Trello.

Validação nativa no GNOME/Wayland (confinamento de foco, dimensões do diálogo) permanece no ticket 07; não houve falha de leitura de Rede modelada no snapshot além de ausência (sem leitura) e idade do teste.
