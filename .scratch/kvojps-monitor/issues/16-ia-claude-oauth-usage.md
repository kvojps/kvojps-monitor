# 16: Opt-in oauth/usage do Claude

**What to build:** nas Configurações → IA posso ligar (desligado por padrão) a leitura via endpoint não documentado `oauth/usage`, após um aviso claro de API não documentada e zona cinzenta nos termos. Ligado, os limites ficam sempre atualizados e aparecem semanais por modelo quando houver. Token expirado mostra "Token do Claude expirado · abra o Claude Code" e volta à status line; o app nunca renova o token. Ver spec, "Módulo IA" e ticket 13 do mapa.

**Blocked by:** 15 (Claude via status line)

**Status:** feito

- [x] `GET api.anthropic.com/api/oauth/usage` com header `anthropic-beta: oauth-2025-04-20` a cada 5 min, no backoff do ticket 09
- [x] Token de `~/.claude/.credentials.json` lido só em memória; nunca renovado nem gravado (teste explícito)
- [x] 401 → volta à status line e mostra a mensagem de token expirado
- [x] Semanais por modelo só quando não-nulos; campos desconhecidos ignorados
- [x] Com as duas fontes, vale a leitura mais recente de cada janela
- [x] Testes com respostas gravadas; testes de UI do aviso e do estado expirado

## Integração de ciclo de vida (ticket 10)

Integração futura autorizada pelo usuário no ticket 10: usar o estado compartilhado de `lifecycle`, aguardar conectividade e `Status::ready_for(posição)` (fontes remotas separadas por 3 s), pausar HTTP/consultas sem faixa de erro e preservar dados com idade. Consumir cada geração de reconexão/retomada uma vez, reiniciar relógios e verificar/reiniciar subprocessos persistentes mortos quando existirem. Preservar o bloqueio de credenciais recusadas. Definir posições distintas das fontes remotas ao integrar (GitHub usa 0).

## Decisões e desvios desta implementação

- **Opt-in:** guardado em `~/.config/kvojps-monitor/claude-usage.json` (`enabled`). Em Configurações → IA, "Ler via oauth/usage…" abre o aviso (API não documentada, zona cinzenta nos termos, risco de bloqueio ou revogação, sem renovação do token), e o opt-in só liga em "Entendi o risco, ligar". Desligar volta à status line na hora e descarta a leitura do endpoint.
- **Token:** `accessToken` e `expiresAt` são lidos de `~/.claude/.credentials.json` só com o opt-in ligado, para uma requisição, e nunca guardados, logados ou gravados (`refreshToken` nunca é lido). Um token com `expiresAt` vencido conta como expirado sem requisição. Um 401 ou um token vencido bloqueia esse token até o Claude Code renová-lo, conferido a cada 5 s. Para que um reinício do app não reenvie o token recusado, a configuração do opt-in guarda só o `expiresAt` dele (metadado, não o token).
- **Cadência:** a cada 5 min, com o Módulo visível ou não; ↻ força a leitura (mínimo 10 s entre cliques). Backoff de 30 s a 15 min com `Retry-After`; "Falha ao atualizar há X" após 3 falhas seguidas. Uma resposta 200 que não traz nenhuma das duas janelas é registrada como falha (`httpStatus` 200) e entra no backoff.
- **Ciclo de vida:** posição 3 na sequência das fontes remotas (GitHub 0, Trello 1, Codex 2). O Claude publica a própria pausa (`claude.paused`), que esconde a linha de falha.
- **Mescla das fontes:** as janelas principais vêm da leitura mais recente, e a mais antiga só preenche uma janela que falte. Os semanais por modelo (`seven_day_opus`, `seven_day_sonnet`, quando não-nulos) vêm só do `oauth/usage`. Fonte e idade mostradas são as da leitura mais recente. `seven_day_oauth_apps` e `seven_day_cowork` são por produto, não por modelo, e ficam de fora. O bucket principal aparece como "Todos os modelos" quando há semanais por modelo.
- **Fixture:** `oauth-usage.json` foi transcrito da resposta real registrada na pesquisa do ticket 01 (plano Pro, buckets por modelo nulos), sem captura nova. `capture.py` regrava a resposta atual quando o usuário quiser. Ver `src-tauri/fixtures/claude/README.md`.
- **Fora de escopo, notado na revisão:** backoff, `paused()` e o cliente HTTP repetem o padrão de Codex, Trello e GitHub. Extrair um agendador comum fica para outra mudança.
- O app não fez nenhuma chamada real ao endpoint, e não houve verificação visual no GNOME.
