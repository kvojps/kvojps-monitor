# 03: Anel de Agentes e detalhe de Sessões de agente

**What to build:** acompanhar os Estados da sessão em um anel compacto e abrir Sessões de agente para consultar informações completas, inclusive a sessão direcionada por uma notificação.

**Blocked by:** 01 — Composição compacta e janela adaptada à área útil.

**Status:** resolved

- [x] Derivar segmentos e legenda dos cinco Estados da sessão a partir das sessões ativas e encerradas ainda retidas; mostrar a quantidade em andamento no centro. Preservar retenção da v1, até 2 h/máximo 5 encerradas.
- [x] Diferenciar leitura indisponível de zero válido; zero apresenta vazio explícito sem divisão por zero. Manter falha de coleta e aviso de detecção de espera do Codex indisponível.
- [x] Permitir identificar projeto/nome por ponteiro e foco na legenda. Anel, legenda e alerta direto “Aguardando você” abrem o detalhe.
- [x] Mostrar no detalhe projeto/nome, provedor, estado, motivo quando houver, duração e demais informações da v1, incluindo encerradas ainda retidas.
- [x] Limitar o detalhe a até 680 px/90% da largura e 80% da altura da janela, com rolagem; restringir foco à área aberta. Fechar/Escape fecham primeiro o detalhe e restauram foco ao acionador; Escape sem detalhe oculta o Painel.
- [x] Preservar seleção/contexto válidos na reabertura e em atualizações, sem roubar foco.
- [x] Notificação abre Painel e detalhe, alcançando/destacando a sessão exata, inclusive encerrada ainda retida. Tratar alvo indisponível conforme a v1, sem apontar para outra sessão e sem substituir a Âncora persistida pela navegação transitória da notificação.
- [x] Testar por fixtures zero, indisponível, cinco estados, encerradas/expiração da retenção, falha e alvo de notificação, além de teclado, foco, Escape e atualizações. Manter privacidade das notificações e coleta existente.

## Comments

Origem: especificação aprovada do Painel compacto e regras de Sessões de agente da v1. Divisão aprovada em 2026-10-31. O mecanismo de sobreposição pode ser compartilhado, mas este ticket não bloqueia Rede ou Trello: cada fatia deve entregar seu comportamento completo.


Implementado em 2026-10-31: anel e legenda derivam dos snapshots, incluindo encerradas retidas; detalhe modal nativo com restauração de foco e Escape em duas etapas. Notificações mantêm navegação transitória, suportam repetição e alvo recebido em snapshot posterior. Retenção e privacidade continuam cobertas pelos testes existentes do núcleo.

Validação automatizada: checagem Svelte/TypeScript, suíte da UI, build e suíte Rust (incluindo testes de sockets fora do sandbox). Revisão Standards: sem violações; associação explícita estado/cor aplicada após sugestão. Revisão Spec: seleção na reabertura e alcance de alvo tardio corrigidos, com cobertura de integração.

Validação manual pendente no GNOME/Wayland: confinamento nativo do foco, dimensões/rolagem do diálogo e abertura pela notificação real. O jsdom simula somente o ciclo de abertura/fechamento do diálogo; não comprova comportamento nativo.
