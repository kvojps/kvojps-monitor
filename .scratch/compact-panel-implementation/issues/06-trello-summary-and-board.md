# 06: Resumo do Trello e quadro sobreposto com movimentação real

**What to build:** acompanhar o quadro do Trello em um resumo permanente e abrir o quadro completo dentro do Painel para consultar e mover cards com atualização imediata e recuperação em caso de falha.

**Blocked by:** 01 — Composição compacta e janela adaptada à área útil.

**Status:** resolved

- [x] Mostrar nome real do quadro, contagem de cards/listas, contagem por lista, prévia de card e acesso ao quadro completo; manter atualizar/configurar e recuperação com ações acessíveis.
- [x] Abrir quadro amplo, referência 1040 × 610 no alvo de 1100 × 700, limitado a 96% da largura e 92% da altura. Usar colunas de cerca de 228 px, rolagem horizontal do quadro e vertical por lista, sem eliminar cards.
- [x] Mostrar nomes de listas, quantidades, títulos reais, etiquetas e vencimentos; identificar estados sem depender somente da cor e manter legibilidade em listas densas, vazias e sem cards.
- [x] Selecionar card, escolher “Mover para” e confirmar em “Mover card”; oferecer “Mover para esta lista” na coluna de destino. O seletor alcança listas fora da área horizontal visível. Cancelar seleção não move.
- [x] Permitir arraste real entre listas como atalho, mantendo caminho completo por teclado. Usar comandos reais de movimentação, com atualização otimista do quadro e resumo; falha devolve à origem e explica o ocorrido.
- [x] Abrir card no Trello somente por ação explícita ou duplo clique conforme a v1. Não incluir simulação de falha nem controles de laboratório no produto.
- [x] Restringir foco à sobreposição; Fechar/Escape fecham o detalhe e restauram foco ao acionador. Escape sem detalhe oculta o Painel. Preservar seleção/contexto válidos ao reabrir e durante atualizações, sem roubar foco.
- [x] Testar fixtures normal, vazio, antigo, offline, falha, credencial ausente/inválida e sem quadro selecionado; testar confirmação/cancelamento, reversão, resumo sincronizado, teclado, arraste, rolagem e abertura externa.
- [x] Preservar fontes, coleta por visibilidade, consentimentos e guardrails; conteúdo de cards não entra em logs/cache/notificações. Não criar seam paralelo de E/S.

## Comments

Origem: especificação aprovada do Painel compacto e quadro do protótipo aprovado. Divisão aprovada em 2026-10-31. Não depende do ticket de Agentes: a sobreposição deve ser entregue nesta fatia mesmo que o mecanismo comum ainda não exista.


## Answer

Implementado em 2026-10-31:

- Resumo do Trello: nome real do quadro, “N cards · M listas”, contagem por lista, prévia do primeiro card (título real ou aviso de título pendente do cache, etiquetas e vencimento) e “Abrir quadro completo”. Contagens leem a visão otimista e acompanham movimentação e reversão. Atualizar/Configurar, Reconectar e avisos de idade/falha preservados.
- Quadro em diálogo modal nativo (`min(1040px, 96vw)` × `min(610px, 92vh)`), colunas de 228 px com rolagem horizontal do quadro e vertical por lista; nenhum card eliminado. Listas mostram nome e quantidade (também em texto acessível); cards mostram título, etiquetas nomeadas em pt-BR (não só cor) e vencimento com texto “Vencido”/“Entrega concluída”.
- Movimentação: selecionar card → “Mover para” (seletor com todas as outras listas, inclusive fora da área visível) → “Mover card”; ou “Mover para esta lista” na coluna; ou arraste HTML5 real entre listas. “Cancelar seleção” não move. Otimista no quadro e no resumo; falha devolve à origem e explica (“O card voltou para …”), também no resumo se o quadro já tiver sido fechado. Movimentação indisponível (offline/credencial inválida) é explicada e o arraste fica desativado.
- Abrir no Trello só por ação explícita do card selecionado ou duplo clique (seleção mantida). Sem controles de laboratório.
- Fechar/Escape fecham o quadro e restauram foco ao acionador; Escape sem detalhe oculta o Painel. Reabrir preserva seleção válida (com foco no card) e rolagem horizontal; atualizações mantêm diálogo, seleção, foco e rolagem por lista; destino que deixou de existir é descartado.
- `dragDropEnabled: false` na janela para o arraste HTML5 chegar ao webview (o app não recebe arquivos).
- Sem nova E/S: `moveTrelloCard`/`openTrelloCard` existentes; nada novo em logs/cache/notificações.
- Validação: 242 testes UI, 271 unitários Rust e 3 de hooks; check e build aprovados.
- Revisão: Spec — corrigidos duplo clique desmarcando a seleção, destino obsoleto após atualização e resultado antigo persistindo. Standards — sem violações; strings duplicadas/mortas removidas e nomes esclarecidos. Pendências: o ciclo do diálogo agora se repete em Agentes, Rede e Trello (extração comum fica para refatoração); Escape com o seletor nativo aberto fecha o quadro inteiro e deve ser observado na validação nativa.

Validação nativa no GNOME/Wayland (arraste real no WebKitGTK, confinamento de foco, dimensões do diálogo) permanece no ticket 07.

**Reaberto em 2026-10-31 por fidelidade ao protótipo:** no app nativo, o resultado diverge do design e do conteúdo do protótipo B. A correção segue em [compact-panel-fidelity](../../compact-panel-fidelity/map.md).

**Reversão (2026-10-31):** os botões textuais Atualizar/Configurar Trello voltam ao B: ícones ↻/⚙ no cabeçalho. Ver [ticket 05 de fidelidade](../../compact-panel-fidelity/issues/05-trello.md).
