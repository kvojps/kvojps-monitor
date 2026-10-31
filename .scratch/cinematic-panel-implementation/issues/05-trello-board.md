# 05: Trello em quadro interativo

**What to build:** consultar e mover cards no quadro integrado à estação, preservando acesso às listas reais e recuperação de falhas.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** ready-for-human

- [x] Mostrar colunas e cards com título, etiquetas e vencimento conforme a composição aprovada; listas extensas têm rolagem, sem esconder cards para obter simetria.
- [x] Selecionar card/lista de destino e mover com resultado otimista; falha reverte o movimento e informa o ocorrido. Duplo clique abre no navegador, com alternativa acessível por teclado.
- [x] Atualizar/configurar permanecem acessíveis; respeitar intervalo manual mínimo de 10 s e preservar seleção/foco durante atualização.
- [x] Cobrir cheio, listas vazias, sem quadro, sem credencial, credencial inválida, antigo, offline e falha com cache utilizável.
- [x] Testar fixtures, comandos de abertura/movimento e reversão, incluindo listas densas sem sobreposição de ações ou corte de informação indispensável.
- [x] Preservar coleta/armazenamento; testar que credenciais e conteúdo de cards ficam fora de logs/cache/notificações.

## Registro da implementação (2026-10-31)

- **Módulo Trello no Painel:** `TrelloModule` volta à estação Execução como na composição H (o protótipo reutilizava o próprio módulo): nome do quadro, colunas com faixa superior e canto técnico, cards com título (até 2 linhas), etiquetas em pontos coloridos e vencimento (atenção se vencido). Listas longas rolam dentro da própria coluna (máx. 420 px) e o quadro rola na horizontal; nenhum card é escondido.
- **Mover e abrir:** clique no card e depois na lista (ou no card de outra lista) continua igual, com resultado otimista, reversão e “Não foi possível mover”. Alternativa por teclado: cada card é um botão (Enter/Espaço seleciona, `aria-pressed`); com um card selecionado, o título de cada outra lista vira o botão “Mover para {lista}” e o card mostra “Abrir {card} no navegador”. Duplo clique segue abrindo. Offline ou com credencial recusada a seleção ainda dá acesso a abrir, sem destinos de movimento. A dica só em hover (`title`) saiu, como no GitHub.
- **Ações:** “Atualizar Trello” (10 s entre cliques manuais) e “Configurar Trello” sempre no Módulo, inclusive sem credencial e sem quadro (substituem o botão “Configurar” do estado vazio). Credencial recusada mantém o texto da v1, “Credencial inválida · Reconectar”, com “Reconectar” abrindo Configurações.
- **Estados:** sem leitura ainda (antes do primeiro snapshot), sem credencial (com aviso do token pendente), sem quadro, quadro sem listas (“Quadro sem listas”, explicação de um caso que antes ficava em branco), lista vazia, antigo (cache sem títulos), offline, falha sustentada e credencial inválida, todos com idade quando há cache. Novas fixtures `stale` e `offline` geradas pelo núcleo em `ui_fixtures_match_published_snapshots`.
- **Atualizações:** seleção, foco e rolagem do Painel e de cada lista se mantêm quando chega um snapshot; a seleção cai só se o card sumir do quadro.
- **Privacidade:** coleta e armazenamento inalterados. Cache e logs sem credenciais nem títulos já cobertos por `restart_restores_last_board_without_card_titles_or_credentials` e `failed_move_keeps_the_last_board_and_its_clock_and_logs_only_metadata`; o coletor do Trello não tem porta de notificação.
- **Pendente (usuário):** conferência visual nativa de listas densas (sobreposição/corte não é verificável no jsdom), nas três larguras e nos temas escuro/claro.
