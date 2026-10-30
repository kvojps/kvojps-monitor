# Critério do repo destaque no Módulo GitHub

Nota de sanitização (ticket 40): contagens e horários do exemplo de repositório privado são sintéticos; não representam atividade real do mantenedor.

Type: grilling
Status: resolved
Blocked by: 03
Map: [kvojps-monitor v1](../map.md)

## Question

O "repo com mais contribuições nos últimos 7 dias" conta só commits na branch padrão (funciona com o token atual do `gh`), ou commits + PRs + issues + reviews (exige pedir o escopo `read:user` ao usuário)? E o "nº de commits" exibido é o da janela de 7 dias, o total do usuário no repo, ou ambos?

## Answer

O **Repo em destaque** é o repositório com mais **commits seus na branch padrão** nos últimos 7 dias (via `history(author:)`; termo no [GLOSSARY](../../../GLOSSARY.md)).

- **Critério**: só commits, sem PRs/issues/reviews. Funciona com o token atual do `gh` (escopo `repo`), sem pedir `read:user`. Descartados: contribuições via `contributionsCollection` (exige conceder `read:user` uma vez pelo navegador) e o híbrido por escopo (duas regras para o mesmo bloco). PRs e reviews ficam para "PRs aguardando revisão".
- **Números**: os dois, janela em destaque e total secundário — `42 commits em 7 dias · 240 no total`.
- **Última atividade**: seu último commit na branch padrão, em tempo relativo ("há 2 h"), com o título do commit truncado em uma linha. Não usa `pushedAt`.
- **Janela e universo**: móvel, 7×24 h até agora; repos com afiliação dono/colaborador/membro de org, os 20 com push mais recente, forks incluídos. Empate: ganha o commit mais recente.
- **Vazio**: "Sem commits nos últimos 7 dias", sem destaque substituto.
