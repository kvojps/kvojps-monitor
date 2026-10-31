# Implementação do Painel cinematográfico

Decomposição em 11 tickets aprovada pelo usuário em 2026-10-31.

Fonte de verdade: [especificação visual aprovada](../cinematic-panel/spec.md), complemento da [spec funcional da v1](../kvojps-monitor/spec.md). O planejamento visual e seus tickets resolvidos permanecem preservados.

## Tickets e dependências

| Ticket | Bloqueado por |
| --- | --- |
| [01 — Painel em estações com navegação e coleta por visibilidade](issues/01-panel-stations.md) ✅ resolvido | Nenhum |
| [02 — IA com instrumentos de Limite de uso](issues/02-ai-instruments.md) ✅ resolvido | 01 |
| [03 — Agentes com intervenção e sessão direcionada](issues/03-agent-sessions.md) ✅ resolvido | 01 |
| [04 — GitHub com atividade e Repo em destaque](issues/04-github-activity.md) ✅ resolvido | 01 |
| [05 — Trello em quadro interativo](issues/05-trello-board.md) ✅ resolvido | 01 |
| [06 — Rede com instrumentos e tráfego real](issues/06-network-instruments.md) ✅ resolvido | 01 |
| [07 — Sistema com instrumentos e alertas](issues/07-system-instruments.md) ✅ resolvido | 01 |
| [08 — Configurações completas, temas e primeira execução](issues/08-settings-and-onboarding.md) ✅ resolvido | 01 |
| [09 — Coreografia combinada com pausa nativa](issues/09-combined-motion.md) | 02–08 |
| [10 — Validar composição, acessibilidade e integração GNOME/Wayland](issues/10-native-and-accessibility-validation.md) ✅ resolvido | 09 |
| [11 — Medir e ajustar desempenho e memória](issues/11-performance-and-memory.md) | 10 |

Fronteira inicial: 01. Após sua conclusão, 02–08 podem começar independentemente. **01 resolvido em 2026-10-31:** os Módulos ficaram vazios no Painel por decisão do usuário; cada fatia 02–07 volta a montar o conteúdo do seu Módulo (detalhes no ticket 01). `ready-for-agent` indica triagem; não elimina os bloqueios declarados. Medições na máquina do usuário e verificação manual nativa exigem acesso ao ambiente real e resultados registrados, sem substituir evidência por testes do navegador.

## Critérios comuns

- Cada ticket de implementação entrega comportamento verificável e seus testes de estados, comandos e acessibilidade. Os tickets finais verificam a integração e corrigem desvios; não adiam essa cobertura.
- Usar a composição H e a Coreografia combinada como referência visual, sem copiar dados sintéticos ou regras do protótipo para produção.
- Código em inglês e interface em pt-BR. Sem novas coletas para decoração, expansores ou detalhes dispensados na spec.
- Toda E/S do núcleo usa as portas injetáveis existentes e fixtures reais; a UI usa fixtures de snapshot. Não criar seam paralelo.
- Credenciais apenas no GNOME Keyring; sem keyring, recusar salvar. Logs/cache/notificações contêm somente metadados permitidos. Hooks e status line exigem clique explícito, alterações aditivas e reversíveis. Token Claude somente em memória, sem renovação. Cada guardrail deve ter teste explícito.
- Preservar cache com idade, independência das fontes e estados explicativos; falhas não apagam dados válidos de outras fontes. Atualizações preservam foco, seleção e rolagem.
- Qualquer divergência adicional das specs exige validação do usuário antes da execução e registro no ticket de origem.

## Decisões registradas

- **2026-10-31 — ticket 08 resolvido:** usuário confirmou a validação de Configurações, temas e primeira execução. Implementação e revisão em `d86fbd9`, `418065c` e `39d76d5`; [registro de aceitação](issues/08-settings-and-onboarding.md#comments). Ticket 09 desbloqueado. Capturas e medições da validação integrada permanecem no ticket 10.

- **2026-10-31 — ticket 10 resolvido:** encerrado por instrução explícita do usuário, com correções e evidências em `c5dbbab`. [Decisão de encerramento](issues/10-native-and-accessibility-validation.md#answer). Limitações e verificações sem evidência continuam documentadas. Ticket 11 desbloqueado.
