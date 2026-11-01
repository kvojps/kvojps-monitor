# 04: Rede fiel ao B

Type: task
Status: resolved
Blocked by: 01, 02

**What to build:** reescrever o resumo de Rede a partir do protótipo, com dados reais, até ficar igual ao B no app nativo.

- [x] Cabeçalho “REDE” em maiúsculas; sem subtítulo e sem selo numerado.
- [x] Linha de status do B com dados reais: tipo de conexão, “Qualidade: X”, “Segurança: Y”. Sem caixas de Qualidade/Segurança. **Reverte** a divergência aprovada no ticket 05 anterior (conexão abaixo da latência).
- [x] Latência grande com “Internet”; botão de teste “↓ N · ↑ N / Mbps · testar ↻ / Teste · HH:MM” (no lugar de “Fixture”); linha Jitter/Perda/Gateway; linha `small` do gateway.
- [x] Gráfico de tráfego de **largura inteira** com duas séries, como no B, e taxas ↓/↑ abaixo; resumo acessível mantido.
- [x] “N aplicativos + Outros ↗” abre o detalhe com o layout do protótipo, acrescido da explicação de Outros e da escala comum exigidas pela v1.
- [x] VPN ativa, motivos de Atenção/Risco, avisos e estados do teste (indisponível, em andamento, falha, offline) e desconexão como linha condicional no corpo, no vocabulário do B. Nunca “Rede segura”.
- [x] Testes de UI por fixtures; sem rolagem em 1100 × 700 e 1000 × 700 com as fixtures do B.
- [x] Captura no app nativo lado a lado com `compact-B.png` e **aprovação do usuário** registrada aqui.

## Comments

Origem: [mapa de fidelidade](../map.md), decisões Q10, Q16, Q17, Q19 da entrevista de 2026-10-31.

2026-11-01 (implementação): `NetworkModule.svelte` reescrito com a marcação e o CSS do protótipo: “REDE” como `h2` em maiúsculas sem ↻/⚙ (o B não tem), linha de status “Wi-Fi|Cabeada · Qualidade: X · Segurança: Y”, latência grande com “Internet”, botão “↓ N · ↑ N / Mbps · testar ↻ / Teste · HH:MM”, linha Jitter/Perda/Gateway, linha `small` do gateway, gráfico de largura inteira (amostras do último minuto espalhadas como no B) com taxas ↓/↑ abaixo e “N aplicativos + Outros ↗”. Saíram as caixas de Qualidade/Segurança, “VPN inativa”, “Mbps · capacidade”, idade por foco/ponteiro e as chaves de legenda do gráfico. O detalhe segue o layout do protótipo (artigos com nome e “↓ · ↑”, “Fechar ×”) acrescido das barras em escala comum e da explicação de Outros. Linhas condicionais logo abaixo da linha de status: “⊘ Sem conexão” (substitui a linha de status; latência vira “—”), “VPN ativa”, motivos de Atenção (âmbar) e Risco (vermelho) com ⚠ (o nome do estado quando o núcleo não manda motivo), “Testando velocidade…”, “⚠ Não foi possível medir a velocidade”, “⊘ Teste indisponível…” (só conectado) e “⚠ Teste antigo · há X · DD/MM HH:MM” após 6 h. Unidades como no B (“0%”, “20ms” visual). Testes do módulo e do Painel atualizados, incluindo “nada além do B no estado normal” (242 passam), `svelte-check` sem erros. Pendente: rolagem em 1100 × 700 e 1000 × 700 e captura nativa com aprovação, que dependem do usuário enquanto “Captura nativa para agentes” estiver aberto.

## Answer

Aprovado pelo usuário em 2026-11-01 após conferência no app nativo (`npm run dev:fixtures`) lado a lado com `compact-B.png`.

- Resumo de Rede reescrito a partir do protótipo: “REDE” sem ↻/⚙, linha de status “Wi-Fi|Cabeada · Qualidade: X · Segurança: Y”, latência com “Internet”, botão “↓ · ↑ / Mbps · testar ↻ / Teste · HH:MM”, Jitter/Perda/Gateway, linha do gateway, gráfico de largura inteira com taxas ↓/↑ e “N aplicativos + Outros ↗”.
- Fora do estado normal: caixas de Qualidade/Segurança, “VPN inativa”, “Mbps · capacidade”, idade do teste por foco/ponteiro e chaves de legenda do gráfico.
- Detalhe de aplicativos no layout do protótipo, acrescido das barras em escala comum e da explicação de Outros.
- Estados condicionais como linhas `small` logo abaixo da linha de status: desconexão (substitui a linha de status), VPN ativa, motivos de Atenção/Risco, teste em andamento, falha, indisponível (só conectado) e antigo após 6 h.
- Sem divergências da spec.

