# 16: Opt-in oauth/usage do Claude

**What to build:** nas Configurações → IA posso ligar (desligado por padrão) a leitura via endpoint não documentado `oauth/usage`, após um aviso claro de API não documentada e zona cinzenta nos termos. Ligado, os limites ficam sempre atualizados e aparecem semanais por modelo quando houver. Token expirado mostra "Token do Claude expirado · abra o Claude Code" e volta à status line; o app nunca renova o token. Ver spec, "Módulo IA" e ticket 13 do mapa.

**Blocked by:** 15 (Claude via status line)

**Status:** ready-for-agent

- [ ] `GET api.anthropic.com/api/oauth/usage` com header `anthropic-beta: oauth-2025-04-20` a cada 5 min, no backoff do ticket 09
- [ ] Token de `~/.claude/.credentials.json` lido só em memória; nunca renovado nem gravado (teste explícito)
- [ ] 401 → volta à status line e mostra a mensagem de token expirado
- [ ] Semanais por modelo só quando não-nulos; campos desconhecidos ignorados
- [ ] Com as duas fontes, vale a leitura mais recente de cada janela
- [ ] Testes com respostas gravadas; testes de UI do aviso e do estado expirado
