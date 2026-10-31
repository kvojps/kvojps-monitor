# 11: Token clássico do GitHub no keyring

**What to build:** sem `gh`, posso colar um token clássico com escopo `repo` nas Configurações → GitHub; o app o valida antes de salvar, mostra o erro junto do campo ("Token inválido", "Falta o escopo repo", "Sem conexão, tente de novo") ou "Conectado como <login>", e o guarda só no GNOME Keyring. Sem keyring, recusa salvar e explica o porquê. As Configurações mostram "via gh · <login>" ou "token próprio". Esta fatia define o padrão de credenciais usado pelo Trello. Ver spec, "Configurações e primeira execução" e ticket 15 do mapa.

**Blocked by:** 09 (Coleta resiliente)

**Status:** feito

- [x] Porta de keyring (Secret Service via crate `keyring`); credenciais nunca em texto puro
- [x] Sem keyring disponível: recusa salvar com explicação (teste explícito)
- [x] Validação por `viewer { login }` + escopo `repo` antes de gravar; falha não salva nada
- [x] Salvar uma nova credencial libera o bloqueio de 401/403 do ticket 09
- [x] Configurações → GitHub mostra a fonte da credencial e o login
- [x] Testes de UI do formulário (cada erro, sucesso, sem keyring)

## Implementação e validação

- Porta de keyring injetável e adapter Secret Service (`keyring`, sem backend alternativo), reutilizável para credenciais do Trello. Todas as operações rodam na thread do coletor.
- Validação GraphQL `viewer { login }` e escopo exato `repo` no header `X-OAuth-Scopes` antes de gravar; falhas preservam a credencial anterior. Erros são tipados, sem corpo da resposta ou segredo.
- `gh auth token` continua prioritário e consultado a cada coleta. Sem autenticação no CLI, o token próprio é lido do keyring somente em memória. Salvar com sucesso libera o bloqueio de autenticação e agenda atualização completa.
- Configurações → GitHub mostra a fonte e formulário mascarado, com erros junto do campo e confirmação com login. O campo é limpo após envio; submissões duplicadas são bloqueadas. O atalho “Colar token clássico” leva do Módulo às Configurações.
- Fixtures de snapshot publicadas pelo núcleo cobrem `gh` e token próprio. Testes cobrem ausência de keyring, falhas de validação sem gravação, prioridade do CLI, retomada após 401, reinício, privacidade do cache/logs e formulário de UI.
- `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (122 testes), `npm run check`, `npm test` (49 testes) e `npm run build`: aprovados.
- Base da revisão: `3ef4cb1d56e1878de0e5bd587c65025361b93cea` (HEAD anterior ao trabalho).
- Verificação manual no GNOME/Keyring real não realizada nesta execução.

### Revisão final

- **Standards:** sem achados restantes. A resposta de validação (corpo e header de escopos) foi capturada e anonimizada; o núcleo interpreta esses dados com testes para `repo` exato, escopos parciais/ausentes e corpos inválidos.
- **Spec:** sem achados restantes. Mudanças automáticas de credencial forçam o calendário completo, evitando misturar contas; o login do token próprio continua visível após reabrir Configurações. Ambos com testes de regressão.
