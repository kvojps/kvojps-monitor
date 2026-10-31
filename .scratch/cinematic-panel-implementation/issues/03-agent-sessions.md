# 03: Agentes com intervenção e sessão direcionada

**What to build:** reconhecer intervenções e acompanhar Sessões de agente reais na composição aprovada, chegando à sessão correta ao abrir uma notificação.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** ready-for-human

- [x] Mostrar projeto/nome, provedor, Estado da sessão, motivo e duração; contadores e alerta da navegação derivam dos snapshots, sem rótulos ou códigos fixos da fixture.
- [x] Preservar os cinco Estados da sessão, resumo de Encerradas sem lista adicional/expansor e retenção de até 2 h/máximo 5 conforme a v1.
- [x] Notificação abre o Painel, alcança e destaca a Sessão de agente correspondente, inclusive após recriar a webview; não confundir esse destino temporário com a última âncora escolhida pelo usuário.
- [x] Exibir aviso de detecção de espera do Codex indisponível e falha de coleta, mantendo dados utilizáveis com idade.
- [x] Testar estados cheio/vazio/antigo/erro e os cinco Estados da sessão, destino por notificação, contadores, teclado e nomes acessíveis; preservar regras de agrupamento, duração mínima e preferências de notificação.
- [x] Confirmar por testes no seam existente que notificações e logs/cache não expõem conteúdo de sessões, prompts ou respostas.

## Registro da implementação (2026-10-31)

- **Módulo Agentes no Painel:** `AgentsModule` volta a montar o conteúdo como na composição H: destaque de intervenção (contagem de sessões aguardando; sem nenhuma, “Nenhuma intervenção pendente” em vez de “Aguardando você” fixo), lista de Sessões de agente com código do provedor derivado do dado (CC/CX, decorativo), nome, projeto · provedor, Estado da sessão, motivo e duração, e o resumo de contadores no rodapé. Antes do primeiro snapshot, “Sem leitura das sessões de agente ainda”.
- **Contadores:** os cinco Estados da sessão (Em andamento, Aguardando você, Turno concluído, Com falha, Encerradas), todos contados do snapshot. Encerradas aparecem só no contador: sem lista nem expansor; retenção de 2 h/máximo 5 inalterada no núcleo.
- **Alerta da navegação:** espelha `accentDot` (mesma fonte do ponto do Indicador); o nome acessível vem da contagem (“N aguardando você”), sem rótulo fixo.
- **Notificação:** o Painel passa a sessão direcionada ao Módulo, que rola até ela e a destaca (`aria-current`, contorno e texto “Da notificação”, não só cor). Vale para o Painel já aberto (evento `agent-target`) e para a webview recriada (`initialState.agentTarget`). Escolher um Módulo na navegação desfaz o destaque; o destino não altera a última âncora.
- **Motivo de espera:** mapeado como antes; nome de ferramenta só aparece se estiver na mesma lista permitida das notificações do núcleo (“Aguardando aprovação: Bash”); qualquer outro detalhe livre vira “Aguardando sua resposta”.
- **Falha de coleta com idade:** o núcleo passou a publicar `lastSuccessAt` (última consulta bem-sucedida; eventos de hook não contam) no snapshot de Agentes, sem nova coleta. Com falha, o Módulo mantém as sessões e mostra “Não foi possível consultar as sessões de agente · última leitura às HH:MM”.
- **Privacidade:** novo teste pelo seam existente cobre o caminho completo payload de hook do Codex (com `PRIVATE_COMMAND`/`PRIVATE_PROMPT`) → socket → `Monitor` → notificador e snapshot publicado, sem conteúdo. O coletor de Agentes não grava cache em disco nem logs; preferências persistidas já são cobertas por `agent_switches_default_to_enabled_and_persist_without_session_content`.
- **Decisões validadas pelo usuário (2026-10-31), divergências registradas:** (1) uma sessão direcionada que encerrou depois da notificação aparece sozinha na lista, destacada, para que a notificação ainda a alcance (as demais Encerradas seguem só no contador); (2) contadores para os cinco estados, em vez dos três da composição H, para que nenhum estado vivo fique sem contagem; Decisão local: durações acima de 1 h passam a “1 h 01 min”.
- **Revisão:** o alerta da navegação voltou a usar `accentDot`; `lastSuccessAt` deixou de avançar com hooks; o aviso de falha e o destaque de intervenção deixaram de ser regiões vivas, para não repetir anúncios a cada snapshot. Pendências conhecidas: a lista permitida de ferramentas está duplicada entre `notifications.rs` e `AgentsModule.svelte`, e nenhum teste liga as duas; após uma falha, o cabeçalho mostra a hora do snapshot e o aviso mostra a hora da última leitura.
- **Pendente (usuário):** conferência visual nativa nas três larguras e nos temas escuro/claro; abrir uma notificação real com o Painel fechado e aberto.
