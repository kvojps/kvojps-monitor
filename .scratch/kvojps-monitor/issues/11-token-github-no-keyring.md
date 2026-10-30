# 11: Token clássico do GitHub no keyring

**What to build:** sem `gh`, posso colar um token clássico com escopo `repo` nas Configurações → GitHub; o app o valida antes de salvar, mostra o erro junto do campo ("Token inválido", "Falta o escopo repo", "Sem conexão, tente de novo") ou "Conectado como <login>", e o guarda só no GNOME Keyring. Sem keyring, recusa salvar e explica o porquê. As Configurações mostram "via gh · <login>" ou "token próprio". Esta fatia define o padrão de credenciais usado pelo Trello. Ver spec, "Configurações e primeira execução" e ticket 15 do mapa.

**Blocked by:** 09 (Coleta resiliente)

**Status:** ready-for-agent

- [ ] Porta de keyring (Secret Service via crate `keyring`); credenciais nunca em texto puro
- [ ] Sem keyring disponível: recusa salvar com explicação (teste explícito)
- [ ] Validação por `viewer { login }` + escopo `repo` antes de gravar; falha não salva nada
- [ ] Salvar uma nova credencial libera o bloqueio de 401/403 do ticket 09
- [ ] Configurações → GitHub mostra a fonte da credencial e o login
- [ ] Testes de UI do formulário (cada erro, sucesso, sem keyring)
