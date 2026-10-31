# Mapa: Painel compacto sem rolagem

Label: wayfinder:map

## Destination

Uma especificação visual validada por protótipo para um Painel de aproximadamente 1100 × 700, com todos os resumos visíveis sem rolagem na visão principal, preservando a estética atual e o acesso às funções existentes.

## Notes

- Planejamento com `wayfinder`; implementação de produção fora deste esforço. Consultar `grilling` e `domain-modeling` nas decisões e `prototype` nas explorações visuais.
- O usuário confirmou os três acordos de abertura em 2026-10-31: janela de aproximadamente 1100 × 700; IA e Agentes reunidos visualmente; resumos permanentes e listas extensas em detalhes sob demanda.
- Retirar título e subtítulo gerais; reduzir faixas decorativas e cabeçalhos repetidos. Atualizar/configurar passam a ícones junto aos títulos, com descrição no hover e foco pelo teclado.
- Claude e Codex mantêm seus instrumentos de limites. Sessões de agente ganham anel com quantidade trabalhando no centro, segmentos por Estado da sessão e legenda com contagens; hover/foco identifica as sessões daquele estado. Aguardando você continua como alerta visível.
- Trello, processos de rede e sessões podem abrir detalhes em área sobreposta. Não exigir que listas ilimitadas caibam integralmente na visão principal. A localização e o comportamento desses detalhes ainda precisam de validação.
- Preservar linguagem LCARS, temas, movimento e redução de movimento aprovados; buscar outras economias de espaço sem reduzir a legibilidade nem esconder alertas.
- Referências: [complemento visual vigente](../cinematic-panel/spec.md), [spec da v1](../kvojps-monitor/spec.md) e [vocabulário](../../GLOSSARY.md). Agrupamento visual de IA/Agentes não implica fundir fontes, snapshots ou regras de coleta.
- Esta revisão autoriza explorar substituições de tamanho, composição, navegação e acesso a detalhes do complemento visual vigente. Registrar a autorização nos tickets de origem; consolidar regras novas apenas após validação. Privacidade, credenciais, fontes e regras de negócio permanecem fora da revisão.
- Branches `prototype/*` e `research/*` ficam locais, sem push. Tickets sem Status estão abertos; Blocking usa `Blocked by`.

## Decisions so far

- [Validar a distribuição compacta do Painel](issues/01-compact-composition.md): escolhida B — Núcleo lateral; quadro completo do Trello e movimentação seguem na validação de detalhes.

## Not yet specified

- Ajustes que a avaliação visual revelar para densidade, textos longos e equilíbrio entre instrumentos e informações secundárias.

## Out of scope

- Implementar a compactação no app de produção durante o planejamento.
- Novas integrações, mudanças de coleta, credenciais, privacidade ou regras de notificação.
- Garantir ausência de rolagem em listas ilimitadas ou em qualquer dimensão/escala de tela; a faixa suportada será explicitada na validação.
