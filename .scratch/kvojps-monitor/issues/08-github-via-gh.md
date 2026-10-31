# 08: GitHub via gh: calendário e Repo em destaque

Nota de sanitização (ticket 40): contagens e horários do exemplo de repositório privado são sintéticos; não representam atividade real do mantenedor.

**What to build:** com o `gh` autenticado, o Módulo GitHub mostra o calendário de contribuições do ano corrente com o total (privadas como contagem anônima) e o Repo em destaque com "42 commits em 7 dias · 240 no total" e meu último commit nele (tempo relativo, título truncado em uma linha). Sem destaque: "Sem commits nos últimos 7 dias". Botão ↻ com espera de 10 s entre cliques. Sem `gh`, estado vazio com "Configurar" oferecendo "Rodar `gh auth login`" (o token clássico chega no ticket 11). Ver spec, "Módulo GitHub" e ticket 12 do mapa.

**Blocked by:** 03 (Módulo Sistema ao vivo)

**Status:** feito

- [x] Portas de processo (`gh auth token` a cada atualização, nada copiado) e cliente HTTP criadas
- [x] Calendário via GraphQL `contributionsCollection(from: 1º jan, to: 31 dez)` + `restrictedContributionsCount`, a cada 1 h
- [x] Repo em destaque: `viewer.repositories` (dono/colaborador/membro de org, 20 com push mais recente, forks incluídos) + `history(author, since)` na branch padrão, janela móvel de 7×24 h, empate pelo commit mais recente; não usa `pushedAt`; a cada 15 min, 5 min com o Módulo visível
- [x] ↻ manual por Módulo com espera de 10 s
- [x] Abrir o Painel mostra o último snapshot e atualiza em segundo plano com indicador discreto se passou do intervalo
- [x] Testes do núcleo com respostas GraphQL gravadas e relógio falso (empate, janela vazia, virada de ano); testes de UI por fixtures (cheio, vazio, sem gh)

## Notas da implementação

- Autenticação por `gh auth token` em cada atualização, com o segredo somente na pilha da chamada HTTP. Portas `Processos` e `Http` injetáveis; o cliente HTTP usa HTTPS com timeout de 30 s.
- Calendário consultado a cada hora, incluindo renovação ao mudar o ano UTC. `totalContributions` é exibido sem somar novamente `restrictedContributionsCount`, apresentado separadamente como contagem anônima.
- Universo de 20 repositórios com todas as afiliações pedidas e forks incluídos; janela `since`/`until` com duração de 7×24 h e total sem janela filtrado pelo mesmo autor. Branch ausente é ignorada.
- O snapshot anterior permanece no Painel durante a atualização; o indicador discreto também vem do núcleo. A espera manual de 10 s existe no núcleo e na UI.
- “Configurar” oferece a instrução para executar `gh auth login` no terminal e atualizar em seguida. Token próprio/keyring e as Configurações completas pertencem ao ticket 11.
- Fixtures GraphQL capturadas nesta máquina, anonimizadas antes do versionamento; janela vazia e empate são variantes documentadas. Fixtures da UI geradas e verificadas pelo núcleo.
- Sem persistência de títulos ou credenciais. Cache/backoff/401 são do ticket 09; conectividade/retomada/login escalonado, do ticket 10. Até lá, falhas preservam o snapshot e as tentativas automáticas têm espaçamento mínimo de 30 s.
- Verificação visual do Painel no GNOME não foi realizada nesta execução.

## Validação

- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` e `cargo test`: aprovados (76 testes Rust).
- `npm run check`, `npm test` e `npm run build`: aprovados (28 testes de UI; nenhum diagnóstico Svelte).
- Revisão em dois agentes: Standards sem violações obrigatórias; Spec sem achados restantes. Sugestão não bloqueante: substituir gradualmente `Value`/strings de estado por tipos de domínio.
