# 04: Resumos compactos de GitHub e Sistema

**What to build:** consultar atividade do GitHub e métricas de Sistema em resumos compactos nas posições de B, mantendo informações, ações e alertas reais.

**Blocked by:** 01 — Composição compacta e janela adaptada à área útil.

**Status:** resolved

- [x] GitHub mostra total identificado como anual, mapa de oito semanas, barras de 14 dias e Repo em destaque com commits em 7 dias e título real do último commit.
- [x] Preservar cálculo do Repo em destaque, contagem anônima de contribuições privadas e ações de atualizar/configurar por ícones acessíveis, com descrição por ponteiro/foco e destino correto.
- [x] Sistema mostra CPU, temperatura, RAM/swap/disco e bateria conforme o complemento anterior, preservando alertas de disco, temperatura e bateria e ausências reais de sensores, swap ou bateria.
- [x] Preservar hierarquia e legibilidade da composição B, sem acrescentar blocos de detalhe dispensados nem transformar valores de fixture em constantes.
- [x] Distinguir carregamento/sem leitura, vazio, antigo, offline, falha e credencial ausente/inválida onde aplicáveis; manter cache utilizável com idade e recuperação. Não apresentar ausência como zero.
- [x] Cobrir os estados com fixtures de snapshot, incluindo ausência de atividade, credencial inválida e ausências/alertas de Sistema; testar comandos e acessibilidade dos gráficos, temas e redução de movimento.
- [x] Preservar fontes, coleta por visibilidade, cadências e guardrails existentes.

## Comments

Origem: especificação aprovada do Painel compacto e cobertura de GitHub/Sistema do complemento visual cinematográfico. Divisão aprovada em 2026-10-31.


## Answer

Implementado em 2026-10-31:

- GitHub compacto com total anual junto ao mapa de oito semanas, barras de 14 dias
  de 26 px e Repo em destaque com dados reais; preservados cálculo, títulos em
  memória e contagem anônima de contribuições privadas.
- Atualizar/configurar junto ao título, por ícones com nomes e descrições acessíveis
  por foco/ponteiro e dispensa por Escape; mantidos destinos, intervalo e bloqueios.
  Offline indica coleta pausada, mantendo cache e idade.
- Sistema compacto com CPU/temperatura, barras de RAM/swap/disco e bateria;
  preservados valores, meters acessíveis, ausências reais, idade e alertas textuais.
- Fontes, coleta, cadências, snapshots e guardrails do núcleo sem alteração.
- Validação: 224 testes UI, 271 testes unitários Rust e 3 testes de hooks; check e
  build aprovados. Auditoria com 57 cenários sem overflow horizontal nos contêineres
  medidos, contraste textual mínimo de 5,39:1 e dicas por foco/ponteiro/Escape.
- Revisão paralela: Standards sem violações e uma sugestão de manutenção para
  compartilhar descrições de ações com IA; Spec sem achados.

[Evidências, capturas e limites](../validation/github-system.md). Validação nativa
integrada e comparação dos seis resumos permanecem no ticket 07. A decisão anterior
aceita de representar falha de Sistema como leitura antiga permanece vigente.

**Reaberto em 2026-10-31 por fidelidade ao protótipo:** no app nativo, o resultado diverge do design e do conteúdo do protótipo B. A correção segue em [compact-panel-fidelity](../../compact-panel-fidelity/map.md).
