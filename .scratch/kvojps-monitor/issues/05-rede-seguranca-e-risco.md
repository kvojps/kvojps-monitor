# 05: Segurança da rede, notificação e badge de Risco

**What to build:** o Módulo Rede mostra a Segurança da rede como Sem alertas / Atenção / Risco (nunca "Segura") com o motivo. Ao entrar em Risco recebo uma notificação nativa, uma vez por conexão, e o Indicador ganha um badge discreto enquanto durar. Ver spec, "Módulo Rede".

**Blocked by:** 04 (Qualidade da rede e tráfego total)

**Status:** resolved

- [x] Risco: rede aberta sem OWE, WEP, WPA com TKIP, DNS respondendo a nome aleatório inexistente (esperado NXDOMAIN)
- [x] Atenção: portal cativo (connectivity check do NM), MAC do gateway mudou na mesma conexão
- [x] Flags do AP e perfil por eventos do NetworkManager; DNS e portal ao conectar e a cada 10 min
- [x] Cabeada: valem DNS, portal e MAC; com VPN, avalia o link físico
- [x] Porta do notificador criada; notificação só ao entrar em Risco, uma vez por conexão (teste explícito)
- [x] Badge no Indicador só em Risco; sem conexão os dois eixos mostram "Sem conexão" e não há badge
- [x] Testes de UI por fixtures (sem alertas, atenção, risco)

## Implementação

- Segurança independente da Qualidade no snapshot, com motivos e estados Sem alertas / Atenção / Risco; desconexão limpa os dois eixos e o badge.
- AP e perfil público acompanhados por sinais D-Bus do NetworkManager; MAC relido na primeira leitura e em eventos. DNS pelo resolvedor do sistema e portal por `CheckConnectivity`, ao conectar e a cada 10 minutos. VPN conserva a avaliação do link físico.
- Porta de notificação com evento fechado (somente metadados), usando notificações nativas D-Bus; no máximo uma por conexão. Indicador acompanha o estado de Risco continuamente.
- Fixtures reais de flags/perfil, vizinhos e resposta do resolvedor, com variações controladas documentadas em `src-tauri/fixtures/rede/README.md`. Snapshots de UI gerados pelo núcleo para Sem alertas, Atenção, Risco e desconexão.
- Revisão nos eixos Standards e Spec: corrigidos portal persistente após recuperação, polling do MAC e cobertura dos adapters por fixtures; flags receberam constantes nomeadas.
