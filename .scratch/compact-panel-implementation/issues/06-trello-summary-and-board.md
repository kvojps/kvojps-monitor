# 06: Resumo do Trello e quadro sobreposto com movimentação real

**What to build:** acompanhar o quadro do Trello em um resumo permanente e abrir o quadro completo dentro do Painel para consultar e mover cards com atualização imediata e recuperação em caso de falha.

**Blocked by:** 01 — Composição compacta e janela adaptada à área útil.

**Status:** ready-for-agent

- [ ] Mostrar nome real do quadro, contagem de cards/listas, contagem por lista, prévia de card e acesso ao quadro completo; manter atualizar/configurar e recuperação com ações acessíveis.
- [ ] Abrir quadro amplo, referência 1040 × 610 no alvo de 1100 × 700, limitado a 96% da largura e 92% da altura. Usar colunas de cerca de 228 px, rolagem horizontal do quadro e vertical por lista, sem eliminar cards.
- [ ] Mostrar nomes de listas, quantidades, títulos reais, etiquetas e vencimentos; identificar estados sem depender somente da cor e manter legibilidade em listas densas, vazias e sem cards.
- [ ] Selecionar card, escolher “Mover para” e confirmar em “Mover card”; oferecer “Mover para esta lista” na coluna de destino. O seletor alcança listas fora da área horizontal visível. Cancelar seleção não move.
- [ ] Permitir arraste real entre listas como atalho, mantendo caminho completo por teclado. Usar comandos reais de movimentação, com atualização otimista do quadro e resumo; falha devolve à origem e explica o ocorrido.
- [ ] Abrir card no Trello somente por ação explícita ou duplo clique conforme a v1. Não incluir simulação de falha nem controles de laboratório no produto.
- [ ] Restringir foco à sobreposição; Fechar/Escape fecham o detalhe e restauram foco ao acionador. Escape sem detalhe oculta o Painel. Preservar seleção/contexto válidos ao reabrir e durante atualizações, sem roubar foco.
- [ ] Testar fixtures normal, vazio, antigo, offline, falha, credencial ausente/inválida e sem quadro selecionado; testar confirmação/cancelamento, reversão, resumo sincronizado, teclado, arraste, rolagem e abertura externa.
- [ ] Preservar fontes, coleta por visibilidade, consentimentos e guardrails; conteúdo de cards não entra em logs/cache/notificações. Não criar seam paralelo de E/S.

## Comments

Origem: especificação aprovada do Painel compacto e quadro do protótipo aprovado. Divisão aprovada em 2026-10-31. Não depende do ticket de Agentes: a sobreposição deve ser entregue nesta fatia mesmo que o mecanismo comum ainda não exista.
