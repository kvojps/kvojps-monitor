# Validar detalhes sob demanda e estados do Painel compacto

Type: prototype
Status: claimed
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
