# 05: Segurança da rede, notificação e badge de Risco

**What to build:** o Módulo Rede mostra a Segurança da rede como Sem alertas / Atenção / Risco (nunca "Segura") com o motivo. Ao entrar em Risco recebo uma notificação nativa, uma vez por conexão, e o Indicador ganha um badge discreto enquanto durar. Ver spec, "Módulo Rede".

**Blocked by:** 04 (Qualidade da rede e tráfego total)

**Status:** ready-for-agent

- [ ] Risco: rede aberta sem OWE, WEP, WPA com TKIP, DNS respondendo a nome aleatório inexistente (esperado NXDOMAIN)
- [ ] Atenção: portal cativo (connectivity check do NM), MAC do gateway mudou na mesma conexão
- [ ] Flags do AP e perfil por eventos do NetworkManager; DNS e portal ao conectar e a cada 10 min
- [ ] Cabeada: valem DNS, portal e MAC; com VPN, avalia o link físico
- [ ] Porta do notificador criada; notificação só ao entrar em Risco, uma vez por conexão (teste explícito)
- [ ] Badge no Indicador só em Risco; sem conexão os dois eixos mostram "Sem conexão" e não há badge
- [ ] Testes de UI por fixtures (sem alertas, atenção, risco)
