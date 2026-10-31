# 17: Sessões do Claude Code por polling

**What to build:** o Módulo Agentes lista minhas Sessões de agente do Claude Code, cada uma com Estado da sessão (cor), projeto (pasta do cwd), nome, detalhe e tempo. A sessão Aguardando você fica destacada no topo; as Encerradas ficam numa seção recolhida por até 2 h (no máximo 5). Enquanto alguma sessão estiver Aguardando você, o Indicador ganha o ponto de acento e o ícone de Agentes no trilho espelha esse ponto. Ver spec, "Módulo Agentes" e ticket 02 do mapa.

**Blocked by:** 03 (Módulo Sistema ao vivo)

**Status:** ready-for-agent

- [x] `claude agents --json --all` a cada ~2 s via porta de processo, sempre
- [x] Mapeamento para os Estados da sessão: `busy`/`waiting` (com `waitingFor`)/`idle` e background `working`/`blocked`/`done`/`failed`/`stopped`
- [x] Encerradas: só as vistas vivas pelo app, por 2 h, no máximo 5, não persistidas entre execuções
- [x] Ponto de acento no Indicador e no trilho enquanto houver Aguardando você
- [x] Testes do núcleo com JSON real gravado e relógio falso; testes de UI por fixtures (vazio, aguardando, encerradas)

## Integração de ciclo de vida (ticket 10)

Integração futura autorizada pelo usuário no ticket 10: usar o estado compartilhado de `lifecycle`, aguardar conectividade e `Status::ready_for(posição)` (fontes remotas separadas por 3 s), pausar HTTP/consultas sem faixa de erro e preservar dados com idade. Consumir cada geração de reconexão/retomada uma vez, reiniciar relógios e verificar/reiniciar subprocessos persistentes mortos quando existirem. Preservar o bloqueio de credenciais recusadas. Definir posições distintas das fontes remotas ao integrar (GitHub usa 0).

## Implementação

Polling local contínuo pelas portas de processo e relógio, inclusive sem rede e com
o Painel fechado. Snapshots e histórico ficam apenas em memória. Jobs background
terminais sem PID entram no histórico somente se vistos vivos; o resultado final
(falha ou conclusão) permanece como detalhe. Sem instalação de hooks ou notificações,
tratados em tickets próprios. Fixture real capturada em 31/10/2026; demais estados
são variantes explícitas do esquema gravado.
