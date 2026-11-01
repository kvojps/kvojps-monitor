# 03: Anel de Agentes e detalhe de Sessões de agente

**What to build:** acompanhar os Estados da sessão em um anel compacto e abrir Sessões de agente para consultar informações completas, inclusive a sessão direcionada por uma notificação.

**Blocked by:** 01 — Composição compacta e janela adaptada à área útil.

**Status:** ready-for-agent

- [ ] Derivar segmentos e legenda dos cinco Estados da sessão a partir das sessões ativas e encerradas ainda retidas; mostrar a quantidade em andamento no centro. Preservar retenção da v1, até 2 h/máximo 5 encerradas.
- [ ] Diferenciar leitura indisponível de zero válido; zero apresenta vazio explícito sem divisão por zero. Manter falha de coleta e aviso de detecção de espera do Codex indisponível.
- [ ] Permitir identificar projeto/nome por ponteiro e foco na legenda. Anel, legenda e alerta direto “Aguardando você” abrem o detalhe.
- [ ] Mostrar no detalhe projeto/nome, provedor, estado, motivo quando houver, duração e demais informações da v1, incluindo encerradas ainda retidas.
- [ ] Limitar o detalhe a até 680 px/90% da largura e 80% da altura da janela, com rolagem; restringir foco à área aberta. Fechar/Escape fecham primeiro o detalhe e restauram foco ao acionador; Escape sem detalhe oculta o Painel.
- [ ] Preservar seleção/contexto válidos na reabertura e em atualizações, sem roubar foco.
- [ ] Notificação abre Painel e detalhe, alcançando/destacando a sessão exata, inclusive encerrada ainda retida. Tratar alvo indisponível conforme a v1, sem apontar para outra sessão e sem substituir a Âncora persistida pela navegação transitória da notificação.
- [ ] Testar por fixtures zero, indisponível, cinco estados, encerradas/expiração da retenção, falha e alvo de notificação, além de teclado, foco, Escape e atualizações. Manter privacidade das notificações e coleta existente.

## Comments

Origem: especificação aprovada do Painel compacto e regras de Sessões de agente da v1. Divisão aprovada em 2026-10-31. O mecanismo de sobreposição pode ser compartilhado, mas este ticket não bloqueia Rede ou Trello: cada fatia deve entregar seu comportamento completo.
