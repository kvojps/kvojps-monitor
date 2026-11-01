# Trello no plano gratuito: ler quadro e mover card

Type: research
Status: resolved
Blocked by: 
Map: [kvojps-monitor v1](../map.md)

## Question

A API REST do Trello permite, no plano Free e sem custo, ler listas e cards de um quadro e mover um card entre listas usando uma API key + token pessoal? Como o usuário obtém essas credenciais hoje (Power-Up admin / developer key)? Limites de rate, webhooks vs polling (webhook exige URL pública — inviável localmente?), e restrições recentes da Atlassian à API.

## Answer

Viável e gratuito no plano Free com API key + token pessoal (`scope=read,write`): ler via `GET /1/boards/{id}/lists?cards=open`, mover via `PUT /1/cards/{id}` com `idList`. Key criada em https://trello.com/apps/admin (aba "Trello Auth"); token com `expiration=never`. Limites: 300 req/10s por key, 100/10s por token — polling a cada 30–60s é folgado. Webhooks exigem URL pública → usar polling. Key + token sem prazo de depreciação; OAuth 2.0 (3LO) disponível desde 15/09/2026. Para o app open-source: cada usuário cria sua própria key (guia no README), token no keyring, auth isolada num módulo para migrar a OAuth 2.0 depois. Não verificado: `localhost` como origem permitida de `return_url`.

Achados completos: [`docs/research/trello-api.md`](../../../docs/research/trello-api.md) (trazido da branch `research/trello-api`, commit `66d2d25`, apagada em 2026-11-01).
