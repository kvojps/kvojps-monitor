# 04: Rede fiel ao B

Type: task
Status: ready-for-agent
Blocked by: 01, 02

**What to build:** reescrever o resumo de Rede a partir do protótipo, com dados reais, até ficar igual ao B no app nativo.

- [ ] Cabeçalho “REDE” em maiúsculas; sem subtítulo e sem selo numerado.
- [ ] Linha de status do B com dados reais: tipo de conexão, “Qualidade: X”, “Segurança: Y”. Sem caixas de Qualidade/Segurança. **Reverte** a divergência aprovada no ticket 05 anterior (conexão abaixo da latência).
- [ ] Latência grande com “Internet”; botão de teste “↓ N · ↑ N / Mbps · testar ↻ / Teste · HH:MM” (no lugar de “Fixture”); linha Jitter/Perda/Gateway; linha `small` do gateway.
- [ ] Gráfico de tráfego de **largura inteira** com duas séries, como no B, e taxas ↓/↑ abaixo; resumo acessível mantido.
- [ ] “N aplicativos + Outros ↗” abre o detalhe com o layout do protótipo, acrescido da explicação de Outros e da escala comum exigidas pela v1.
- [ ] VPN ativa, motivos de Atenção/Risco, avisos e estados do teste (indisponível, em andamento, falha, offline) e desconexão como linha condicional no corpo, no vocabulário do B. Nunca “Rede segura”.
- [ ] Testes de UI por fixtures; sem rolagem em 1100 × 700 e 1000 × 700 com as fixtures do B.
- [ ] Captura no app nativo lado a lado com `compact-B.png` e **aprovação do usuário** registrada aqui.

## Comments

Origem: [mapa de fidelidade](../map.md), decisões Q10, Q16, Q17, Q19 da entrevista de 2026-10-31.
