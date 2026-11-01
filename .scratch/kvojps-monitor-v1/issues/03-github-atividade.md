# GitHub: calendário do ano e repo destaque dos últimos 7 dias

Nota de sanitização (ticket 40): contagens e horários do exemplo de repositório privado são sintéticos; não representam atividade real do mantenedor.

Type: research
Status: resolved
Blocked by: 
Map: [kvojps-monitor v1](../map.md)

## Question

Com um token pessoal gratuito (ou o `gh` já autenticado), como obter: (a) o calendário de contribuições do ano corrente igual ao do perfil web (incl. contribuições privadas se o usuário quiser); (b) o repo com mais contribuições do usuário nos últimos 7 dias, a data da última atividade nele e quantos commits do usuário ele tem? Avaliar GraphQL contributionsCollection, escopos necessários, limites de rate e cache.

## Answer

**(a) Calendário**: funciona já com o token do `gh` (`gh auth token`): `viewer.contributionsCollection(from: <1º jan>, to: <31 dez>).contributionCalendar` — validado: 1.580 contribuições, igual ao perfil web; privadas entram como contagem anônima (`restrictedContributionsCount`). Janela máxima de 1 ano.

**(b) Repo destaque (7 dias)**: `contributionsCollection.commitContributionsByRepository` vem vazio para repos privados sem o escopo `read:user` (o `gh` atual tem `gist, read:org, repo, workflow`). Caminho que funciona hoje numa query: `viewer.repositories` ordenado por `PUSHED_AT` + `history(author:{id}, since:)` na branch padrão → ex.: `kvojps/example-project`, 42 commits na janela, 240 no total, último em 2026-10-30. Limitação: conta só commits (não PRs/issues/reviews) e só a branch padrão. Para contar tudo: `gh auth refresh -h github.com -s read:user`.

**Custos/limites**: 1 ponto por query; 5.000 pts/h. Sem ETag em GraphQL → cache local (calendário TTL ~1h, destaque 10–15 min). Token fine-grained não recomendado.

Achados completos: [`docs/research/github-atividade.md`](../../../docs/research/github-atividade.md) (trazido da branch `research/github-atividade`, commit `dfd3af8`, apagada em 2026-11-01).
