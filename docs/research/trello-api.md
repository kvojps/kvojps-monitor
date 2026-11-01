# Trello no plano gratuito: ler quadro e mover card

Pesquisado em 2026-10-30. Fontes primárias: documentação oficial do Trello em developer.atlassian.com, o changelog oficial, o anúncio oficial da Atlassian no fórum de desenvolvedores e a página de preços do Trello.

## Pergunta

A API REST do Trello permite, no plano Free e sem custo, ler listas e cards de um quadro e mover um card entre listas usando uma API key e um token pessoal? Como o usuário consegue essas credenciais hoje? Quais são os limites de taxa? Dá para usar webhook num app local ou é preciso fazer polling? Que restrições recentes a Atlassian impôs? Num app open-source, cada usuário cria a própria key ou o app pode embarcar uma?

## Resposta curta

**Sim.** Com uma API key e um token pessoal, a API REST lê as listas e os cards de um quadro e move um card de lista. Para mover, basta um `PUT /1/cards/{id}` com `idList`. A documentação oficial não restringe a API por plano. Os limites são por key e por token, não por plano. O plano Free inclui Power-Ups ilimitados, e um Power-Up é o pré-requisito para gerar a key. Webhooks exigem uma URL alcançável pelos servidores do Trello, o que é inviável para um app de bandeja local sem túnel. O caminho é polling. A Atlassian lançou OAuth 2.0 (3LO) em setembro de 2026, mas declarou que **não há cronograma de descontinuação** de key e token e que tokens de API continuam sendo a opção recomendada para automações.

## 1. Operações necessárias

| Necessidade | Endpoint | Escopo |
|---|---|---|
| Listas do quadro, com cards | `GET /1/boards/{id}/lists?cards=open` (parâmetro `cards`) | `read:board:trello` |
| Cards abertos do quadro | `GET /1/boards/{id}/cards` | `read:board:trello` |
| Histórico de mudanças (para polling incremental) | `GET /1/boards/{boardId}/actions` (parâmetro `filter`) | `read:board:trello` |
| Mover card de lista | `PUT /1/cards/{id}` com `idList` (e `pos` opcional) | `read:board:trello`, `write:board:trello` |

Fontes:
- https://developer.atlassian.com/cloud/trello/rest/api-group-boards/
- https://developer.atlassian.com/cloud/trello/rest/api-group-cards/

No fluxo de key e token, o escopo é escolhido na autorização: `scope=read,write` cobre ler e mover. Os valores aceitos são `read`, `write` e `account`. Fonte: https://developer.atlassian.com/cloud/trello/guides/rest-api/authorization/

A autenticação pode ir na query string (`?key=...&token=...`) ou no header `Authorization: OAuth oauth_consumer_key="{apiKey}", oauth_token="{apiToken}"`. O header é preferível porque mantém o token fora de URLs e logs. Fontes:
- https://developer.atlassian.com/cloud/trello/guides/rest-api/api-introduction/
- https://developer.atlassian.com/cloud/trello/guides/rest-api/authorization/

## 2. Custo e plano Free

- A página de preços lista, no Free, "Unlimited Power-Ups per board", até 10 colaboradores e até 10 quadros por Workspace. Ela não fala de API. Fonte: https://trello.com/pricing
- Nenhuma página oficial consultada condiciona a API REST a um plano pago. Os limites de taxa são definidos por key e por token, sem menção a plano. Fonte: https://developer.atlassian.com/cloud/trello/guides/rest-api/rate-limits/
- Ressalva: não encontrei uma frase oficial que diga literalmente "a API é gratuita no plano Free". A conclusão vem da ausência de restrição na documentação e do fato de que gerar a key exige só um Power-Up, que o Free permite. Agregadores de terceiros dizem que a API está incluída em todos os planos, mas não são fonte primária.

## 3. Como o usuário obtém as credenciais hoje

O endereço antigo `trello.com/app-key` não aparece mais na documentação. O fluxo atual passa pelo portal de administração de Power-Ups, em https://trello.com/apps/admin:

1. **Criar um Power-Up ou app** no portal. A documentação diz: "For generating an API key, you first need to have created a Trello Power-Up." O formulário pede Workspace dono, e-mail, autor e nome. A "Iframe Connector URL" só é necessária para um Power-Up de verdade, que roda dentro do Trello. Por padrão, o app só pode ser gerenciado por admins do Workspace dono. Num Workspace pessoal, o próprio usuário é admin. Nada exige publicar ou listar o app. Fonte: https://developer.atlassian.com/cloud/trello/guides/power-ups/managing-apps/
2. **Gerar a API key**: abrir o Power-Up, ir à aba **Trello Auth** e clicar em "Generate a new API Key". Fonte: https://developer.atlassian.com/cloud/trello/guides/rest-api/api-introduction/
3. **Gerar o token**: na mesma página, clicar no link "Token" ao lado da key e aprovar com "Allow". O token aparece na tela. A mesma coisa se faz pela rota `https://trello.com/1/authorize?key=...&scope=read,write&expiration=never&response_type=token&name=...`. Sem `return_url`, o token é exibido no navegador para o usuário copiar. `expiration` aceita `1hour`, `1day`, `30days` ou `never`. Fontes:
   - https://developer.atlassian.com/cloud/trello/guides/rest-api/api-introduction/
   - https://developer.atlassian.com/cloud/trello/guides/rest-api/authorization/

Avisos oficiais: "It is ok for your API key to be publicly available, but a token should never be publicly available." Tokens "grant access to the entire user's account" e devem ser guardados com segurança. Fonte: https://developer.atlassian.com/cloud/trello/guides/rest-api/authorization/

## 4. Limites de taxa

Fonte: https://developer.atlassian.com/cloud/trello/guides/rest-api/rate-limits/

- **300 requisições a cada 10 s por API key.**
- **100 requisições a cada 10 s por token.**
- `/1/members/` tem limite próprio de 100 requisições a cada 900 s. Rotas aninhadas como `/1/members/me/boards` não contam nesse limite.
- Quem estoura recebe HTTP 429 com `API_KEY_LIMIT_EXCEEDED` ou `API_TOKEN_LIMIT_EXCEEDED`. A documentação avisa que, se os 429 de uma key passarem de 200, o Trello devolve 429 para o restante das requisições dessa key.
- Os headers `x-rate-limit-api-key-*` e `x-rate-limit-api-token-*` informam o intervalo, o máximo e quanto resta.
- Também existem `API_TOKEN_DB_LIMIT_EXCEEDED` para chamadas caras e `API_TOO_MANY_CARDS_REQUESTED` para respostas grandes demais, por exemplo cards com actions.

Para um app de bandeja com polling a cada 30–60 s, o consumo fica ordens de grandeza abaixo desses limites.

## 5. Webhooks vs. polling

Fonte: https://developer.atlassian.com/cloud/trello/guides/rest-api/webhooks/

- Ao criar o webhook, o Trello envia um `HEAD` para a `callbackURL` e só cria o webhook se a resposta for 200. Depois, entrega os eventos por `POST` a partir da faixa de IPs 104.192.142.240/28. A URL precisa ser alcançável pelos servidores do Trello. Um certificado SSL inválido impede a criação, e o SSL só funciona na porta 443.
- Se a entrega falha, o Trello tenta de novo 3 vezes, após 30, 60 e 120 s. O webhook é desativado depois de 30 dias de falhas consecutivas e mais de 1.000 tentativas falhas.
- As requisições trazem o header `X-Trello-Webhook`, com HMAC-SHA1 em base64 do corpo concatenado com a callbackURL, assinado com o app secret.

**Conclusão:** num app local sem IP público, o webhook só funciona com um túnel ou relay público, o que fere a premissa "tudo local e custo zero". Polling é a escolha natural:
- `GET /1/boards/{id}/lists?cards=open` periodicamente, ou
- `GET /1/boards/{id}/actions` para buscar só as mudanças.

Também não convém usar o websocket interno do Trello, que é o que o cliente web usa. Desde 15/11/2024, ele não aceita mais autenticação por key e token na query string. Fonte: changelog de 13/11/2024 em https://developer.atlassian.com/cloud/trello/changelog/

## 6. Mudanças recentes da Atlassian

Fonte: https://developer.atlassian.com/cloud/trello/changelog/

- **16/04/2025, RFC-89 "Introducing OAuth2 to Trello"**: a Atlassian anunciou que o mecanismo atual seria substituído por OAuth 2.0 (3LO), com escopos granulares, restrição de recursos e expiração de token. Fonte: https://community.developer.atlassian.com/t/rfc-89-introducing-oauth2-to-trello/90359
- **15/09/2026, OAuth 2.0 3LO disponível**: apps novos podem adotar já e apps existentes podem migrar quando quiserem. O post oficial diz: "There's no deprecation timeline". Um cronograma só será definido quando houver alternativa segura para server-to-server, bots e processos sem usuário logado. Para esses casos, os tokens de API continuam sendo a recomendação. O post também menciona cliente público ou confidencial, PKCE e refresh tokens rotativos. Fonte: https://community.developer.atlassian.com/t/oauth-2-0-3lo-is-now-available-for-trello/102691
- **Guia de migração** "OAuth 2.0 for existing apps": a troca de token revoga o token Trello Auth original. Fonte: https://developer.atlassian.com/cloud/trello/guides/rest-api/oauth-2-for-existing-apps
- **13/11/2024**: o websocket deixou de aceitar key e token na query string (ver seção 5).
- **22/07/2024**: `card.idShort` passou a mudar quando um card troca de quadro. A recomendação é usar `card.id`.
- **2025**: algumas rotas antigas foram descontinuadas, como `PUT /board/:id/labelNames`, `PUT /board/:id/myPrefs/showListGuide`, a rota legada de `memberPrivacy` e endpoints SCIM. Nenhuma afeta ler listas e cards ou mover card.

**Risco:** key e token não têm data de fim, mas a direção declarada é OAuth 2.0. Vale isolar a autenticação num módulo próprio, para trocar depois sem refazer o resto.

## 7. App open-source: key própria ou key embarcada?

Pela documentação, as duas opções são possíveis:

- **Key embarcada e fluxo de autorização por token.** A key pode ser pública. O app abriria `https://trello.com/1/authorize?key=<key do projeto>&name=kvojps-monitor&scope=read,write&expiration=never&response_type=token` e o usuário colaria o token exibido. Sem `return_url`, nenhuma origem precisa ser configurada. Para redirecionar automaticamente com `return_url`, a origem tem que estar entre as "allowed origins" da key. Não verifiquei se o Trello aceita `http://localhost:PORT` como origem.
  - Prós: menos atrito para o usuário, que não precisa criar Power-Up.
  - Contras:
    - O limite de 300 req/10 s por key é compartilhado entre todos os usuários.
    - A regra de bloqueio após 200 respostas 429 afeta todos ao mesmo tempo.
    - A key fica atrelada ao Power-Up e ao Workspace do mantenedor, que vira responsável perante a Atlassian, inclusive pelas obrigações de compliance e privacidade de membros associadas a Power-Ups.
    - Se a key for revogada, todos os usuários quebram.
- **Cada usuário cria a própria key**: cria um Power-Up no Workspace dele, gera key e token e cola no app.
  - Prós: isolamento total, nenhum vínculo do usuário com o mantenedor, e combina com "zero custo, tudo local, credenciais do próprio usuário".
  - Contras: o setup tem 3 ou 4 passos manuais no portal. Um guia no README ou um wizard resolve.

## Recomendação

- Para ler o quadro e mover cards, use a API REST com **API key e token pessoal**. Funciona no plano Free, sem custo, e continua suportada sem data de descontinuação.
- Use **polling**, de preferência `GET /1/boards/{id}/lists?cards=open` a cada 30–60 s, ou `/actions` para buscar só as mudanças. Mova cards com `PUT /1/cards/{id}` e `idList`. Webhooks ficam de fora porque exigem URL pública.
- Mande as credenciais no header `Authorization: OAuth oauth_consumer_key=..., oauth_token=...` e guarde o token no keyring do sistema (Secret Service), nunca em texto puro nem no repositório.
- Como o projeto é open-source, o padrão deve ser **cada usuário criar a própria key** em https://trello.com/apps/admin e gerar o token com `scope=read,write`, seguindo um passo a passo no README. Opcionalmente, o app pode montar a URL de `/1/authorize` a partir da key do usuário para simplificar a geração do token. Não embarque uma key do mantenedor.
- Encapsule a autenticação num módulo isolado, prevendo uma migração futura para OAuth 2.0 3LO.
