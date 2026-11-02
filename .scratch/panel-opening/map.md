# Abertura cinematográfica do Painel

Labels: wayfinder:map

## Destination

Definir a experiência de abertura do Painel e seus critérios de comportamento, pronta para orientar uma alteração da spec e posterior implementação.

## Notes

- Planejamento; implementação fora deste mapa. Consultar as skills grilling e domain-modeling; usar prototype no ticket visual.
- Preferências confirmadas na conversa: sequência tech e cinematográfica dentro da própria janela do Painel, fundo escuro, linhas luminosas, mensagens de preparo e transição suave para os Módulos.
- Exibir na primeira abertura após iniciar o app e nas reaberturas. O login automático permanece silencioso, sem abrir janela.
- Duração mínima total de 1,2 segundo. Se o preparo levar mais, continuar até estar pronto, sem adicionar outros 1,2 segundo ao final.
- Mensagens desejadas incluem “Preparando o Painel…” e “Carregando Módulos…”. Precisam representar atividades reais; o significado de prontidão ainda será decidido.
- A história 12 da spec atual exige cache imediato e abertura “sem tela de carregamento”. O usuário solicitou e confirmou a mudança de direção; a revisão concreta dessa regra será registrada no ticket de comportamento antes de alterar a spec.
- Contexto observado: o processo inicia só o Indicador; cada abertura cria uma webview visível e restaura o estado depois; fechar destrói a janela. Referências: `src-tauri/src/panel.rs`, `src-tauri/src/lib.rs`, `src/lib/Panel.svelte`, `.scratch/kvojps-monitor/spec.md`.
- Tracker local conforme `docs/agents/issue-tracker.md`; branches de pesquisa ou protótipo permanecem locais conforme AGENTS.md.

## Decisions so far

- [Definir prontidão e ciclo de abertura do Painel](issues/01-opening-readiness.md): preparo local sem bloqueio de rede, mínimo de 1,2 segundo, espera limitada a 5 segundos, recuperação de falhas e alternativa estática para movimento reduzido.

- [Validar sequência visual cinematográfica de abertura](issues/02-opening-visual.md): Arquitetura (C) aprovada, com identidade “kvojps monitor”, malha luminosa em perspectiva e composição inferior esquerda.

## Not yet specified

Nenhuma decisão pendente. Destino alcançado: contrato de comportamento e direção visual definidos para orientar a revisão da spec e a implementação posterior.

## Out of scope

- Implementar a experiência ou alterar a spec nesta sessão de criação do mapa.
- Redesenhar os Módulos ou adicionar uma janela ao login automático.
- Adicionar progresso ou etapas fictícias para preencher a duração mínima.
