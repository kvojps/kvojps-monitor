# 14: Composição fixa sem dados na produção

Type: task
Status: ready-for-agent
Blocked by: None (13 resolved).

**What to build:** levar aos Módulos de produção a decisão da [Composição fixa sem dados](13-composition-without-data.md) (variante C), substituindo as linhas `small` soltas que hoje ficam no lugar do corpo quando falta a leitura.

- [ ] Área única no corpo de GitHub, Trello, Rede e Sistema, e por fonte no cartão IA e Agentes (Codex, Claude, Agentes), para sem leitura, falha sem leitura anterior, credencial/fonte ausente e Trello sem quadro; tom neutro × âmbar e ação como na resposta do ticket 13. Textos em `strings.json`.
- [ ] Antigo continua com dado apagado e linha `small`; linhas condicionais do topo continuam no topo do corpo.
- [ ] Faltas parciais só no item afetado (sensor, swap, bateria, semana, teste de velocidade, aplicativos, aguardando no Codex), inclusive a caixa do aviso de Agentes no lugar de sumir.
- [ ] Testes de UI por fixtures para cada estado ausente de cada Módulo.
- [ ] Conferência no app nativo (`KVOJPS_FIXTURES`, `scripts/capture-panel.sh`) com as fixtures de cada estado ausente, em 1100 × 700 e 1000 × 700, temas escuro e claro, sem rolagem; aprovação do usuário.

## Comments

Origem: resposta do ticket 13 (2026-11-01). Protótipo de referência: `src/lib/compact-prototype/AbsentPrototype.svelte` no branch local `prototype/composition-without-data`.
