# Validar detalhes sob demanda e estados do Painel compacto

Type: prototype
Status: resolved
Map: [Painel compacto sem rolagem](../map.md)
Blocked by: 01

## Question

Como acessar detalhes de sessões, Trello e processos sem sobrecarregar a visão principal nem perder ações, alertas e contexto?

Partir da composição validada. Validar com o usuário a área sobreposta, abertura/fechamento por mouse e teclado, retorno de foco, rolagem de listas extensas e conciliação de Escape com o fechamento do Painel. Manter mover cards e acesso direto à sessão indicada por notificação, inclusive encerrada.

Definir o significado e o denominador do anel de sessões: trabalhando no centro, demais estados na legenda, tratamento de encerradas, zero sessões e leitura indisponível. Hover e foco precisam permitir consultar detalhes sem depender apenas de cor ou apontador. Testar espera, falha, dados antigos, ausência de credenciais, carregamento, nomes longos e listas densas com fixtures. Verificar temas, redução de movimento e escala de texto, registrando limites de tamanho e eventual alternativa para áreas menores que a suportada.

## Comments

### 2026-10-31 — Quadro completo com movimentação

Após escolher B — Núcleo lateral, o usuário pediu melhorar o quadro completo do Trello para permitir movimentação. O protótipo agora oferece quadro sobreposto amplo, listas em colunas, rolagem horizontal do quadro e vertical por lista, seleção de card, lista de destino e botão de mover, ações de destino nos cabeçalhos e atalho por arraste.

Movimentação somente em memória, com atualização otimista, contagens no resumo principal e reversão na falha simulada. Abertura no Trello permanece simulada. Escape fecha o detalhe e retorna foco ao acionador; comportamento nativo do Painel ainda não validado.

[Captura do quadro](../assets/trello-board-prototype.png) e [evidência de movimentação/reversão/foco](../assets/trello-prototype-results.json). Em área de navegador 1100 × 700, o detalhe mede 1040 × 610, sem rolagem externa do diálogo; listas e quadro possuem seus próprios espaços de rolagem. `npm run check` sem erros ou avisos.

Aguardando avaliação visual e de interação do usuário. Este ticket permanece aberto: demais detalhes, estados do anel, recuperação, escalas e casos densos ainda precisam de validação.

### 2026-10-31 — Quadro completo do Trello aprovado

O usuário aprovou a revisão apresentada com “Perfeito”. A referência aprovada é o quadro em colunas da composição B, com movimentação por seleção de card/lista de destino e atalho por arraste, atualização do resumo e reversão em falha, conforme o [protótipo](../composition-prototype.md#segunda-rodada--quadro-completo-do-trello).

A aprovação é visual e do fluxo apresentado; não constitui validação do gesto de arraste real nem da integração Trello de produção. O ticket permanece aberto para os demais detalhes e estados descritos na pergunta; esta aprovação não os resolve implicitamente.

## Answer

Após a apresentação das pendências (sessões, processos de rede, estados/acessibilidade e consolidação), o usuário confirmou: “Se for como está no protótipo concordo com tudo”. Ficam aprovados os detalhes de sessões e aplicativos conforme a referência apresentada, além do quadro Trello já aprovado, na composição B e dentro da janela de aproximadamente 1100 × 700 confirmada na conversa.

O anel usa sessões ativas e encerradas ainda retidas como denominador, com quantidade trabalhando no centro e legenda dos cinco estados; detalhes de sessões e processos abrem na área central com rolagem e fechamento/retorno de foco. A aprovação cobre a referência visual e o fluxo mostrado, sem certificar estados que o protótipo não demonstrou. Esses casos preservam as regras anteriores e seguem como critérios de implementação (zero versus leitura indisponível, recuperação, notificações direcionadas, escalas e listas densas), sem nova redução de cobertura.

Fonte visual e evidências no [registro do protótipo](../composition-prototype.md). A consolidação e a revisão final do documento seguem em [Consolidar a especificação do Painel compacto](03-compact-specification.md).
