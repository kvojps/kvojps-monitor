# 07: Tráfego por processo

**What to build:** com o Módulo Rede visível vejo os 5 apps que mais usam rede agora (↓/↑, agrupados por nome) e uma linha "Outros" explicando o que não é atribuível. Ver spec, "Módulo Rede" e o ticket 14 do mapa.

**Blocked by:** 04 (Qualidade da rede e tráfego total)

**Status:** ready-for-agent

- [ ] Contadores TCP por conexão do kernel (`sock_diag`/`tcp_info`), só processos do próprio usuário, sem privilégio
- [ ] Top 5 agrupados por nome do app, taxa ↓/↑ média dos últimos ~5 s, a cada 2 s
- [ ] Linha "Outros" = total da interface − atribuído, com dica "UDP/QUIC, processos do sistema e conexões curtas"
- [ ] Coleta só com o Módulo Rede visível; não pesa na Qualidade
- [ ] Testes do núcleo a partir de respostas `sock_diag` gravadas; testes de UI por fixture
