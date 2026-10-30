# 17: Sessões do Claude Code por polling

**What to build:** o Módulo Agentes lista minhas Sessões de agente do Claude Code, cada uma com Estado da sessão (cor), projeto (pasta do cwd), nome, detalhe e tempo. A sessão Aguardando você fica destacada no topo; as Encerradas ficam numa seção recolhida por até 2 h (no máximo 5). Enquanto alguma sessão estiver Aguardando você, o Indicador ganha o ponto de acento e o ícone de Agentes no trilho espelha esse ponto. Ver spec, "Módulo Agentes" e ticket 02 do mapa.

**Blocked by:** 03 (Módulo Sistema ao vivo)

**Status:** ready-for-agent

- [ ] `claude agents --json --all` a cada ~2 s via porta de processo, sempre
- [ ] Mapeamento para os Estados da sessão: `busy`/`waiting` (com `waitingFor`)/`idle` e background `working`/`blocked`/`done`/`failed`/`stopped`
- [ ] Encerradas: só as vistas vivas pelo app, por 2 h, no máximo 5, não persistidas entre execuções
- [ ] Ponto de acento no Indicador e no trilho enquanto houver Aguardando você
- [ ] Testes do núcleo com JSON real gravado e relógio falso; testes de UI por fixtures (vazio, aguardando, encerradas)
