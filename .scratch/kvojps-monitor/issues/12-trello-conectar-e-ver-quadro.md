# 12: Trello: conectar e ver o quadro

**What to build:** numa tela só das Configurações → Trello eu colo a key, clico "Gerar token" (abre a autorização com a key preenchida), colo o token e escolho o quadro entre os abertos. O Módulo Trello mostra as listas em colunas, cada card com título (até 2 linhas), etiquetas como pontos coloridos e data de entrega (cor de atenção se vencida). Duplo clique abre o card no navegador. Botão ↻. "Desconectar" apaga key e token. Ver spec, "Módulo Trello" e ticket 04 do mapa.

**Blocked by:** 11 (Token clássico do GitHub no keyring)

**Status:** ready-for-agent

- [ ] Key e token (`scope=read,write`, `expiration=never`) no keyring; sem captura via `return_url`
- [ ] Validação por `GET /1/members/me` + listagem de quadros antes de salvar
- [ ] Leitura por `GET /1/boards/{id}/lists?cards=open` a cada 5 min, 30 s com o Módulo visível; entra no backoff/cache/401 do ticket 09
- [ ] Auth isolada no adapter (migração futura para OAuth 2.0)
- [ ] Sem credencial: Módulo visível com estado vazio e "Configurar"
- [ ] Desconectar apaga key e token do keyring
- [ ] Testes do núcleo com respostas gravadas; testes de UI por fixtures (cheio, vazio, vencido, credencial inválida)
