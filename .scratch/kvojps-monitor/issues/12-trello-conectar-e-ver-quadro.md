# 12: Trello: conectar e ver o quadro

**What to build:** numa tela só das Configurações → Trello eu colo a key, clico "Gerar token" (abre a autorização com a key preenchida), colo o token e escolho o quadro entre os abertos. O Módulo Trello mostra as listas em colunas, cada card com título (até 2 linhas), etiquetas como pontos coloridos e data de entrega (cor de atenção se vencida). Duplo clique abre o card no navegador. Botão ↻. "Desconectar" apaga key e token. Ver spec, "Módulo Trello" e ticket 04 do mapa.

**Blocked by:** 11 (Token clássico do GitHub no keyring)

**Status:** feito — pendente: substituir as fixtures provisórias por uma captura real (`src-tauri/fixtures/trello/capture.py`)

- [x] Key e token (`scope=read,write`, `expiration=never`) no keyring; sem captura via `return_url`
- [x] Validação por `GET /1/members/me` + listagem de quadros antes de salvar
- [x] Leitura por `GET /1/boards/{id}/lists?cards=open` a cada 5 min, 30 s com o Módulo visível; entra no backoff/cache/401 do ticket 09
- [x] Auth isolada no adapter (migração futura para OAuth 2.0)
- [x] Sem credencial: Módulo visível com estado vazio e "Configurar"
- [x] Desconectar apaga key e token do keyring
- [ ] Testes do núcleo com respostas gravadas (fixtures provisórias no formato documentado; captura real pendente); testes de UI por fixtures (cheio, vazio, vencido, credencial inválida)

## Integração com a persistência (ticket 09)

Conforme escopo confirmado pelo usuário no ticket 09, integrar o snapshot deste coletor ao cache em disco quando seu contrato existir. Reutilizar as portas de persistência e adicionar projeção tipada sanitizada: nunca persistir credenciais nem conteúdo de commits/cards/prompts/respostas. Restaurar o último dado na inicialização; para Rede, persistir apenas o último teste de velocidade.

## Integração de ciclo de vida (ticket 10)

Integração futura autorizada pelo usuário no ticket 10: usar o estado compartilhado de `lifecycle`, aguardar conectividade e `Status::ready_for(posição)` (fontes remotas separadas por 3 s), pausar HTTP/consultas sem faixa de erro e preservar dados com idade. Consumir cada geração de reconexão/retomada uma vez, reiniciar relógios e verificar/reiniciar subprocessos persistentes mortos quando existirem. Preservar o bloqueio de credenciais recusadas. Definir posições distintas das fontes remotas ao integrar (GitHub usa 0).

## Implementação e validação

- Coletor `src-tauri/src/trello/` com portas injetadas (API, keyring, relógio, persistência, processos). A auth fica isolada em `trello/api.rs`: key e token vão só no header `Authorization: OAuth …`; trocar para OAuth 2.0 muda apenas esse arquivo.
- Uma entrada `trello` no keyring guarda key, token, login e o id do quadro escolhido. Conectar valida com `GET /1/members/me` + `GET /1/members/me/boards?filter=open` antes de gravar; falha não altera o que já estava salvo. Sem keyring, recusa. Reconectar a mesma conta mantém o quadro se ele ainda estiver aberto.
- Leitura: `GET /1/boards/{id}/lists?cards=open` (com `card_fields`) a cada 30 s com o Módulo visível e 5 min fora dele; ↻ com espera de 10 s, que também relista os quadros. A lista de quadros só é relida sem quadro escolhido, sem lista conhecida ou no ↻.
- Ticket 09: backoff 30 s → 15 min respeitando `Retry-After`, faixa após 3 falhas, 401/403 bloqueia até a credencial mudar. Cache em disco com quadro, listas, URL curta, cores e datas, nunca títulos de cards, nomes de etiquetas ou credenciais; restaurado como desatualizado. Keyring travado no login mantém o dado restaurado. Logs só com metadados.
- Ticket 10: posição 1 entre as fontes remotas (3 s depois do GitHub), pausa sem rede sem faixa de erro, uma atualização por geração de reconexão/retomada.
- "Vencido" é decidido pelo núcleo a cada publicação (cards com entrega concluída não contam). Duplo clique e "Gerar token" abrem o navegador via `xdg-open` pela porta de processos, só para `https://trello.com/c/<id>`, a página de autorização e `trello.com/apps/admin`.
- UI: `TrelloModule` (colunas, título em até 2 linhas, etiquetas como pontos, data de entrega com cor de atenção, estados vazio/escolher quadro/credencial inválida/falha) e `TrelloSettings` (passo a passo numa tela, seletor de quadro, Desconectar). Fixtures de UI publicadas pelo núcleo.
- `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (149), `npm run check`, `npm test` (69) e `npm run build`: aprovados. Verificação manual no GNOME/Keyring real não realizada.

### Revisão

- Corrigidos: keyring travado apagava o cache restaurado; reconectar descartava o quadro; leitura de quadros a cada atualização; `xdg-open` fora de porta; `Keyring::delete` com padrão silencioso.
- Mantidos de propósito: 5xx/429 na validação aparecem como "Sem conexão" e a faixa de credencial inválida some enquanto offline, iguais ao GitHub. A lógica de backoff/pausa/loop duplica a do GitHub; extrair um coletor remoto compartilhado fica como refatoração futura (tocaria o ticket 11 já entregue).
