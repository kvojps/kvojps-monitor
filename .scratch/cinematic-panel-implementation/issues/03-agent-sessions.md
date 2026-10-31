# 03: Agentes com intervenção e sessão direcionada

**What to build:** reconhecer intervenções e acompanhar Sessões de agente reais na composição aprovada, chegando à sessão correta ao abrir uma notificação.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** ready-for-agent

- [ ] Mostrar projeto/nome, provedor, Estado da sessão, motivo e duração; contadores e alerta da navegação derivam dos snapshots, sem rótulos ou códigos fixos da fixture.
- [ ] Preservar os cinco Estados da sessão, resumo de Encerradas sem lista adicional/expansor e retenção de até 2 h/máximo 5 conforme a v1.
- [ ] Notificação abre o Painel, alcança e destaca a Sessão de agente correspondente, inclusive após recriar a webview; não confundir esse destino temporário com a última âncora escolhida pelo usuário.
- [ ] Exibir aviso de detecção de espera do Codex indisponível e falha de coleta, mantendo dados utilizáveis com idade.
- [ ] Testar estados cheio/vazio/antigo/erro e os cinco Estados da sessão, destino por notificação, contadores, teclado e nomes acessíveis; preservar regras de agrupamento, duração mínima e preferências de notificação.
- [ ] Confirmar por testes no seam existente que notificações e logs/cache não expõem conteúdo de sessões, prompts ou respostas.
