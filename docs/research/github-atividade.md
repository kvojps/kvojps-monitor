# GitHub: calendário do ano e repo destaque dos últimos 7 dias

Nota de sanitização: contagens e horários do exemplo de repositório privado são sintéticos; não representam atividade real do mantenedor.

Data da pesquisa: 2026-10-30. Todas as consultas abaixo foram executadas de verdade nesta máquina (somente leitura, via `gh api graphql`, conta `kvojps`), e as conclusões técnicas vêm dessas execuções. Os resultados relativos ao repositório privado foram substituídos por exemplos sintéticos. O token nunca foi impresso.

## Pergunta

Com um token pessoal gratuito, ou com o `gh` já autenticado, como obter:

- **(a)** o calendário de contribuições do ano corrente, igual ao do perfil web (com contribuições privadas, se o usuário quiser);
- **(b)** o repo com mais contribuições do usuário nos últimos 7 dias, a data da última atividade nele e quantos commits do usuário ele tem (total e na janela).

E também quais escopos são necessários, quais são os limites de rate e como fazer cache.

## Resumo

- **(a) funciona hoje com o token do `gh`.** `viewer.contributionsCollection(from, to).contributionCalendar` devolveu **1.580 contribuições em 2026**, o mesmo número do fragmento HTML do perfil (`https://github.com/users/kvojps/contributions?from=2026-01-01&to=2026-12-31`, que mostra "1,580 contributions in 2026"). As contribuições privadas já entram nesse total como contagem anônima, porque o perfil tem "private contributions" ligado. Custo: 1 ponto.
- **(b) com `contributionsCollection` NÃO funciona com o token atual do `gh`.** Os escopos dele são `gist, read:org, repo, workflow`, sem `read:user`. Nos últimos 7 dias, o resultado foi `totalCommitContributions: 0`, `commitContributionsByRepository: []` e `restrictedContributionsCount: 43`, embora o repo privado `kvojps/example-project` tenha 42 commits do usuário na janela. A documentação explica por quê: contribuições em repos privados só entram com o escopo opcional `read:user`.
- **(b) funciona com o token atual por outro caminho**, com `history(author:{id}, since:)` do branch padrão de cada repo. Uma única query, ao custo de 1 ponto, lista os repos mais recentes (`orderBy: PUSHED_AT`) e devolve, para cada um, os commits na janela, os commits totais e a data do último commit.

## Fontes primárias

| # | Fonte | URL |
|---|---|---|
| S1 | Referência GraphQL: Users (`User.contributionsCollection`, `ContributionsCollection`, `ContributionCalendar*`, `CreatedCommitContribution*`) | https://docs.github.com/en/graphql/reference/users |
| S2 | Rate limits e limites de query da API GraphQL | https://docs.github.com/en/graphql/overview/rate-limits-and-query-limits-for-the-graphql-api |
| S3 | Escopos de OAuth apps (`repo`, `read:user`, header `X-OAuth-Scopes`) | https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps |
| S4 | Gerenciar personal access tokens (fine-grained vs classic) | https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens |
| S5 | Contribuições no perfil (UTC, privadas anonimizadas) | https://docs.github.com/en/account-and-profile/concepts/contributions-on-your-profile |
| S6 | Por que minhas contribuições não aparecem (branch padrão, e-mail, forks, até 24 h) | https://docs.github.com/articles/why-are-my-contributions-not-showing-up-on-my-profile |
| S7 | REST: Events (30 dias, 300 eventos, latência de 30 s a 6 h) | https://docs.github.com/en/rest/activity/events |
| S8 | Boas práticas REST (requisições condicionais, 304) | https://docs.github.com/en/rest/using-the-rest-api/best-practices-for-using-the-rest-api |

## (a) Calendário do ano corrente

### O que a API oferece

- `User.contributionsCollection(from, to, organizationID)` é "the collection of contributions this user has made to different repositories". Se `from` for omitido, o padrão é um ano atrás; se `to` for omitido, o padrão é o momento atual ou `from` + 1 ano [S1].
- O intervalo máximo é **1 ano**. Isso foi validado: `from: 2025-01-01, to: 2026-10-30` devolve o erro `"The total time spanned by 'from' and 'to' must not exceed 1 year"`. O ano civil inteiro (`2026-01-01T00:00:00Z` a `2026-12-31T23:59:59Z`) é aceito.
- `contributionCalendar` tem `colors`, `isHalloween`, `months`, `totalContributions` e `weeks`. Cada `ContributionCalendarDay` traz `date`, `contributionCount`, `contributionLevel` ("relative to other days"), `color` (hex) e `weekday` (1 = segunda; na prática, 0 = domingo) [S1].
- `contributionYears` lista os anos com atividade. Para esta conta, o resultado foi `[2026, 2025, …, 2020]`.

### Query (validada, custo 1)

```graphql
query($from: DateTime!, $to: DateTime!) {
  viewer {
    contributionsCollection(from: $from, to: $to) {
      startedAt endedAt
      hasAnyRestrictedContributions
      restrictedContributionsCount
      contributionCalendar {
        totalContributions
        colors
        weeks {
          firstDay
          contributionDays { date weekday contributionCount contributionLevel color }
        }
      }
    }
  }
  rateLimit { cost remaining }
}
```

```sh
gh api graphql -F query=@calendar.graphql \
  -f from=2026-01-01T00:00:00Z -f to=2026-12-31T23:59:59Z
```

Resultado observado: 53 semanas, `totalContributions: 1580`, `restrictedContributionsCount: 1247`, `colors: ['#9be9a8','#40c463','#30a14e','#216e39']` e dias vazios com `#ebedf0`. A primeira semana começa em 2026-01-01 (quinta, `weekday: 4`), então o app precisa alinhar a grade por `weekday`, como o perfil web faz.

### Contribuições privadas no calendário

- Por padrão, o gráfico do perfil mostra só atividade pública. O usuário pode optar por incluir repos privados, com detalhes anonimizados [S5].
- Quando essa opção está ligada, a API já soma as privadas em `contributionCount` e `totalContributions`, **mesmo sem `read:user`**. Elas aparecem como `restrictedContributionsCount` ("contributions made by the user that the viewer cannot access") [S1]. Por isso o total da API bateu com o perfil web (1.580). Se o usuário quiser ver só as públicas, basta subtrair `restrictedContributionsCount` do total. Por dia, não dá para separar sem `read:user`.
- **Fuso horário:** as contribuições são registradas em UTC [S5]. Uma observação nossa: sem `from`/`to`, a API devolveu `startedAt: 2025-10-26T03:00:00Z`, ou seja, uma janela alinhada a UTC-3. Para ficar igual à web, use `from`/`to` explícitos e compare com o fragmento HTML se houver divergência na borda do dia.
- **Regras do que conta:** commits só contam se estiverem no branch padrão ou em `gh-pages`, se usarem um e-mail ligado à conta (ou o noreply) e se não forem em fork. Podem levar **até 24 h** para aparecer [S6].

## (b) Repo destaque dos últimos 7 dias

### Opção 1: `contributionsCollection` (precisa de `read:user`)

Campos relevantes [S1]:

- `commitContributionsByRepository(maxRepositories: Int = 25)` agrupa por repo. Cada grupo tem `contributions` (uma `CreatedCommitContributionConnection`).
- `CreatedCommitContributionConnection.totalCount` é "the total count of commits across days and repositories in the connection", ou seja, **commits do usuário na janela**.
- `CreatedCommitContribution` tem `commitCount` ("commits … on this day to this repository"), `occurredAt` e `repository`. A granularidade é **diária**: o valor observado foi `occurredAt: …T07:00:00Z`, não a hora real do commit.
- Existem também `pullRequestContributionsByRepository`, `issueContributionsByRepository` e `pullRequestReviewContributionsByRepository`, para quem quiser um "destaque" que vá além de commits.
- A descrição do tipo diz: "Contributions in private and internal repositories are only included with the optional read:user scope." [S1]

```graphql
query($from: DateTime!, $to: DateTime!) {
  viewer {
    contributionsCollection(from: $from, to: $to) {
      totalCommitContributions
      restrictedContributionsCount
      commitContributionsByRepository(maxRepositories: 25) {
        repository { nameWithOwner isPrivate pushedAt url }
        contributions(first: 100, orderBy: {field: OCCURRED_AT, direction: DESC}) {
          totalCount                      # commits na janela
          nodes { occurredAt commitCount } # nodes[0].occurredAt = último dia ativo
        }
      }
      pullRequestContributionsByRepository(maxRepositories: 25) { repository { nameWithOwner } contributions { totalCount } }
      issueContributionsByRepository(maxRepositories: 25) { repository { nameWithOwner } contributions { totalCount } }
      pullRequestReviewContributionsByRepository(maxRepositories: 25) { repository { nameWithOwner } contributions { totalCount } }
    }
  }
}
```

**Resultado com o token atual do `gh`** (`X-OAuth-Scopes: gist, read:org, repo, workflow`):

- Janela de 7 dias: tudo zerado, `restrictedContributionsCount: 43`.
- No ano: só aparecem os 3 repos públicos (`my-desktop-apps` com 246 commits, `kvojps.github.io` com 64 e `sbsi-2027-article` com 20) e mais 1.247 restritas.

Ou seja, o escopo `repo` **não basta**, mesmo para os repos privados do próprio usuário. Isso bate com [S1]. [S3] não diz que `repo` implica `read:user`; `read:user` é "Grants access to read a user's profile data".

Para habilitar, o próprio usuário precisa rodar o comando abaixo, que é interativo e abre o navegador. Esse caminho **não foi validado aqui**:

```sh
gh auth refresh -h github.com -s read:user
```

Outra saída é usar um PAT classic com `repo` + `read:user`. Depois, confira os escopos com `gh api -i /rate_limit | grep -i x-oauth-scopes` (não imprime o token) [S3].

### Opção 2: `history(author:)` por repo (funciona já, validada)

Uma query só, ao custo de **1 ponto**: lista os repos com push mais recente e conta os commits do usuário no branch padrão de cada um.

```graphql
query($authorId: ID!, $since: GitTimestamp!) {
  viewer {
    repositories(first: 10, orderBy: {field: PUSHED_AT, direction: DESC},
                 ownerAffiliations: [OWNER, COLLABORATOR, ORGANIZATION_MEMBER]) {
      nodes {
        nameWithOwner isPrivate pushedAt url
        defaultBranchRef {
          target {
            ... on Commit {
              window: history(author: {id: $authorId}, since: $since) { totalCount }
              all:    history(author: {id: $authorId}) { totalCount }
              last:   history(first: 1, author: {id: $authorId}) { nodes { committedDate messageHeadline url } }
            }
          }
        }
      }
    }
  }
  rateLimit { cost remaining }
}
```

```sh
ID=$(gh api graphql -f query='query{viewer{id}}' --jq .data.viewer.id)
gh api graphql -F query=@top-repo.graphql -f authorId="$ID" \
  -f since="$(date -u -d '7 days ago' +%Y-%m-%dT%H:%M:%SZ)"
```

Exemplo sintético para o repositório privado, com `since = 2026-10-23T12:00Z`:

| repo | pushedAt | commits na janela | commits totais do usuário | último commit |
|---|---|---|---|---|
| kvojps/example-project (privado) | 2026-10-30T12:35:00Z | **42** | 240 | 2026-10-30T12:30:00Z |
| kvojps/sbsi-2027-article | 2026-09-26 | 0 | 20 | 2026-09-26T01:52:22Z |
| kvojps/my-desktop-apps | 2026-09-15 | 0 | 248 | 2026-09-15T03:02:20Z |

O destaque é a linha com o maior `window.totalCount`. No exemplo sintético, a diferença de uma contribuição ilustra outra atividade possível, como a criação de um branch ou repositório; não descreve atividade real da conta.

Detalhes da opção 2:

- `history` cobre apenas o ref escolhido, aqui o branch padrão. Isso coincide com a regra do perfil [S6]. Commits em outros branches não contam em nenhum dos dois caminhos.
- `author: {id}` casa os e-mails verificados da conta, então faz o mesmo papel da regra de e-mail do perfil.
- A opção 2 conta **só commits**. PRs, issues e reviews ficam de fora. Para um app pessoal centrado em commits, isso basta.
- Como `first: 10` ordenado por push é uma heurística, um repo com mais commits, mas push mais antigo que os 10 primeiros, ficaria de fora. Em 7 dias isso é muito improvável, e `first` pode ir até 100 [S2].
- "Data da última atividade": use `pushedAt` (qualquer push) ou `last.nodes[0].committedDate` (último commit do usuário). Na opção 1, `occurredAt` dá apenas o dia.

### Alternativa REST (não recomendada para isto)

`GET /users/{username}/events` inclui eventos privados quando o usuário está autenticado. Ele aceita um fine-grained token com a permissão de usuário "Events" (read). Porém, só traz os últimos 30 dias, no máximo 300 eventos, e "is not built to serve real-time use cases", com latência de 30 s a 6 h [S7]. Serve para "última atividade", não para contagem de commits.

## Escopos: classic vs fine-grained

| Necessidade | Classic PAT / token do `gh` | Fine-grained PAT |
|---|---|---|
| (a) calendário com privadas anonimizadas | qualquer token; validado com o `gh` atual | todo token tem leitura de repos públicos [S4]; o calendário deve funcionar (não validado) |
| (b) opção 1 com repos privados | `read:user` (+ `repo` para metadados do repo) [S1][S3] | **não documentado** se existe um equivalente a `read:user` para `contributionsCollection`; não validado |
| (b) opção 2 (`history`) em repos privados | `repo` (validado) | acesso aos repos + permissão "Contents: read" (não validado aqui) |

Limites do fine-grained relevantes [S4]: cada token acessa recursos de **um único dono** (usuário ou org), não acessa várias orgs ao mesmo tempo e não contribui em repos onde o usuário é colaborador externo. Expira em 30 dias por padrão. Para um app pessoal que talvez olhe repos de orgs, o classic (ou o `gh`) é mais simples.

## Rate limit e cache

- O limite primário da GraphQL é **5.000 pontos/hora por usuário**, PAT incluído [S2]. O header observado foi `X-Ratelimit-Limit: 5000`.
- Custo de cada query: soma das requisições por conexão ÷ 100, com mínimo de 1 [S2]. **As três queries acima custaram 1 ponto cada** (validado via `rateLimit { cost }`).
- Limites secundários [S2]: no máximo 2.000 pontos/minuto e 100 requisições concorrentes. `first`/`last` aceitam de 1 a 100. Requisições que passam de 10 s são cortadas e podem descontar pontos extras.
- O custo de polling é desprezível. Por exemplo, calendário + destaque a cada 15 min dá cerca de 192 pontos/dia, contra 120.000 disponíveis.
- **Cache:** [S2] não documenta ETag ou requisições condicionais para GraphQL. O `304` que não consome rate limit está documentado apenas para REST [S8]. Como o perfil pode levar até 24 h para refletir commits [S6], vale manter um cache local em arquivo, por exemplo em `~/.cache/kvojps-monitor/`:
  - calendário do ano: TTL de 1 h ou mais;
  - destaque de 7 dias: TTL de 10 a 15 min;
  - em erro ou sem rede, mostrar o último valor salvo com o timestamp.
- **Token no app:** em tempo de execução, use `gh auth token` (lê do keyring) ou o Secret Service/libsecret. Nunca grave o token em log ou em config.

## Recomendação

1. **(a) calendário:** use `viewer.contributionsCollection(from: 1º jan, to: 31 dez).contributionCalendar` com o token do `gh` (`gh auth token`). Ele já bate com o perfil web (1.580 em 2026), privadas incluídas. Para mostrar só públicas, subtraia `restrictedContributionsCount` (só no total). Cache de 1 h.
2. **(b) repo destaque:** use a **opção 2** (`viewer.repositories` por `PUSHED_AT` + `history(author:{id}, since:)`). Ela funciona hoje com o escopo `repo`, custa 1 ponto e entrega numa única query os commits na janela, os commits totais e a data/hora do último commit. Cache de 10 a 15 min.
3. **Opcional:** se o destaque precisar contar PRs, issues e reviews, peça ao usuário para rodar `gh auth refresh -h github.com -s read:user` e troque para a opção 1, mantendo a 2 como fallback quando `restrictedContributionsCount > 0` e a lista vier vazia.
4. **Evite** fine-grained PAT neste caso: o comportamento com `contributionsCollection` privado não está documentado, e a restrição a um único dono atrapalha. Evite também a REST Events (30 dias, latência de até 6 h). Custo total: zero.

Nota de privacidade: o nome do projeto privado foi substituído por um exemplo. Não usar os nomes de exemplo para localizar repositórios reais.
