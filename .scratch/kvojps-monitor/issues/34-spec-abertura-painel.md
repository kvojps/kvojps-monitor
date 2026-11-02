# 34: Incorporar a abertura aprovada à spec

**What to build:** atualizar a fonte da verdade da v1 com o contrato de abertura e o visual aprovados no mapa. Este ticket documenta a divergência autorizada pelo usuário em relação à história 12 atual; não implementa a experiência.

**Blocked by:** nenhum.

**Status:** ready-for-agent

- [x] Substituir a história 12, que exige cache imediato e ausência de tela de carregamento, pela revisão proposta em Definir prontidão e ciclo de abertura do Painel.
- [x] Incorporar à spec todos os critérios do contrato: prontidão local, rede em segundo plano, mínimo total de 1,2 segundo incluindo transição, limite de preparo local de 5 segundos, recuperação parcial, falha da interface, cancelamento, reabertura e movimento reduzido.
- [x] Registrar na seção visual a direção Arquitetura (C): fundo escuro, malha luminosa em perspectiva, identidade inferior esquerda com a grafia exata “kvojps monitor”, revelando depois o tema escolhido.
- [x] Registrar o motivo da substituição na seção “Decisões e porquês”, vinculando as decisões de origem; manter Painel como termo de domínio.
- [x] Conferir consistência com Âncora, boas-vindas, abertura por notificação, login silencioso, acessibilidade e metas de desempenho existentes. Não inventar mudanças nesses fluxos; dúvidas que exijam mudar o contrato voltam ao usuário.
- [x] Revisar o diff documental para assegurar que os contratos completos foram incorporados sem promover o código descartável a produção.

## Fontes aprovadas

- [Definir prontidão e ciclo de abertura do Painel](../../panel-opening/issues/01-opening-readiness.md).
- [Validar sequência visual cinematográfica de abertura](../../panel-opening/issues/02-opening-visual.md).
- [Abertura cinematográfica do Painel](../../panel-opening/map.md).

## Comments

Concluído em 02/11/2026: história 12 substituída; contrato completo incorporado em “Abertura do Painel”; direção Arquitetura (C) registrada em “Visual do Painel”; motivo e fontes vinculados em “Decisões e porquês”. Referências de cache e cadência alinhadas ao preparo local. Conferidos Âncora, boas-vindas, notificação, login silencioso, acessibilidade e desempenho. Nenhum código descartável promovido a produção.

Revisão documental em dois eixos: Standards e Spec sem achados. Checagem de tipos sem erros ou avisos; 353 testes de UI passaram. Testes Rust executados fora do sandbox para permitir sockets Unix.
