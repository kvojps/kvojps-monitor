# Limites do Claude pela status line; `oauth/usage` só por opt-in

Os Limites de uso do Claude podem vir de duas fontes locais: o JSON que o Claude Code entrega à status line (documentado, mas só atualiza quando há uma sessão ativa) ou `GET /api/oauth/usage` com o token OAuth de `~/.claude/.credentials.json` (sempre atual, mas não documentado e em zona cinzenta nos termos da Anthropic). Decidimos usar a **status line como fonte padrão** e deixar `oauth/usage` como **opt-in desligado por padrão**, com aviso explícito, consulta a cada 5 min, token lido só em memória e **nunca renovado** pelo app (token expirado volta à status line e pede para abrir o Claude Code). Motivo: o app não deve expor o usuário a risco de conta sem consentimento, nem disputar o token com o próprio Claude Code.

A status line só muda após clique em "Conectar ao Claude Code": o app aponta `statusLine.command` para um wrapper que encadeia o comando existente e só copia `rate_limits`; "Desconectar" restaura o original. Se outro app trocar a status line, o Painel mostra "Status line desconectada" e não reescreve sozinho.

## Considered Options

- **Só `oauth/usage`**: dado sempre atual, mas risco de termos imposto a todos os usuários.
- **Renovar o token quando expira**: evitaria lacunas, mas poderia invalidar a sessão do Claude Code e mexe em credencial que não é do app.
