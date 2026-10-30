# 06: Teste de velocidade

**What to build:** no Módulo Rede posso rodar um teste de velocidade completo sob demanda; um teste leve roda sozinho ao conectar e a cada 6 h, nunca em conexão limitada (metered), e pode ser desligado nas Configurações → Rede. A última velocidade aparece sempre com a idade e passa a pesar na Qualidade. Ver spec, "Módulo Rede".

**Blocked by:** 04 (Qualidade da rede e tráfego total), 09 (Coleta resiliente)

**Status:** ready-for-agent

- [ ] Medição pela Cloudflare (speed.cloudflare.com, HTTPS) via porta HTTP
- [ ] Completo sob demanda (botão); leve (~25 MB ↓, ~10 MB ↑) ao conectar e a cada 6 h; nunca em metered
- [ ] Interruptor "teste de velocidade automático" nas Configurações → Rede, ligado por padrão
- [ ] Velocidade pesa na Qualidade só se medida há < 6 h na rede atual: Ruim < 2 Mbps de descida, Regular < 10 Mbps
- [ ] Último resultado no cache em disco e exibido com idade, inclusive sem conexão
- [ ] Testes com relógio falso: agendamento de 6 h, metered bloqueia, resultado antigo não pesa
