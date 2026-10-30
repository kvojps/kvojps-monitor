# 04: Qualidade da rede e tráfego total

**What to build:** no Módulo Rede vejo a Qualidade da rede como Boa / Regular / Ruim, com latência, jitter e perda até o gateway e a internet, um gráfico de tráfego total em tempo real e as etiquetas "Cabeada", "VPN ativa" e "Sem conexão" quando aplicável. A velocidade ainda não entra na classificação (ticket 06). Ver spec, "Módulo Rede".

**Blocked by:** 03 (Módulo Sistema ao vivo)

**Status:** ready-for-agent

- [ ] Porta D-Bus do NetworkManager (conexão ativa, tipo, VPN, conectividade) com fixtures gravadas
- [ ] Ping sem root via ping socket (fallback: tempo de conexão TCP 443) ao gateway e à internet a cada 5 s, sempre, janela móvel de 2 min
- [ ] Regras de Qualidade: Ruim (perda > 5%, internet > 300 ms, sem internet com gateway respondendo); Regular (perda > 1%, internet > 100 ms, gateway > 30 ms, jitter > 30 ms); Boa no resto
- [ ] Tráfego total pelo delta de `/proc/net/dev` a cada 1 s, sempre, com gráfico
- [ ] Etiquetas Cabeada / VPN ativa; sem conexão, o eixo mostra "Sem conexão"
- [ ] Testes do núcleo com relógio falso cobrindo cada limiar; testes de UI por fixtures (boa, ruim, sem conexão, VPN)
