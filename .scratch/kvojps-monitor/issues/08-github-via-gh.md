# 08: GitHub via gh: calendário e Repo em destaque

Nota de sanitização (ticket 40): contagens e horários do exemplo de repositório privado são sintéticos; não representam atividade real do mantenedor.

**What to build:** com o `gh` autenticado, o Módulo GitHub mostra o calendário de contribuições do ano corrente com o total (privadas como contagem anônima) e o Repo em destaque com "42 commits em 7 dias · 240 no total" e meu último commit nele (tempo relativo, título truncado em uma linha). Sem destaque: "Sem commits nos últimos 7 dias". Botão ↻ com espera de 10 s entre cliques. Sem `gh`, estado vazio com "Configurar" oferecendo "Rodar `gh auth login`" (o token clássico chega no ticket 11). Ver spec, "Módulo GitHub" e ticket 12 do mapa.

**Blocked by:** 03 (Módulo Sistema ao vivo)

**Status:** ready-for-agent

- [ ] Portas de processo (`gh auth token` a cada atualização, nada copiado) e cliente HTTP criadas
- [ ] Calendário via GraphQL `contributionsCollection(from: 1º jan, to: 31 dez)` + `restrictedContributionsCount`, a cada 1 h
- [ ] Repo em destaque: `viewer.repositories` (dono/colaborador/membro de org, 20 com push mais recente, forks incluídos) + `history(author, since)` na branch padrão, janela móvel de 7×24 h, empate pelo commit mais recente; não usa `pushedAt`; a cada 15 min, 5 min com o Módulo visível
- [ ] ↻ manual por Módulo com espera de 10 s
- [ ] Abrir o Painel mostra o último snapshot e atualiza em segundo plano com indicador discreto se passou do intervalo
- [ ] Testes do núcleo com respostas GraphQL gravadas e relógio falso (empate, janela vazia, virada de ano); testes de UI por fixtures (cheio, vazio, sem gh)
