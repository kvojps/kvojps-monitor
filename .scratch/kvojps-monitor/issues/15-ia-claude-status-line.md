# 15: Claude via status line

**What to build:** nas Configurações → IA clico "Conectar ao Claude Code"; o app instala um wrapper de status line que encadeia minha status line existente e passa a ler os limites de 5 h e semanal do Claude, com o plano detectado. "Desconectar" restaura exatamente a status line original (ou remove a chave). Se outra ferramenta trocar a status line, vejo "Status line desconectada · Reconectar" e o app não reescreve sozinho. Sem leitura: "Abra uma sessão do Claude Code para ler os limites". Ver spec, "Módulo IA" e ticket 13 do mapa.

**Blocked by:** 14 (Limites do Codex)

**Status:** ready-for-agent

- [ ] Wrapper em `~/.local/share/kvojps-monitor/claude-statusline`; `statusLine.command` do `~/.claude/settings.json` só muda após o clique
- [ ] Wrapper repassa stdin ao comando original, devolve a saída dele e copia só `rate_limits` (`five_hour`, `seven_day`) para o cache
- [ ] Comando original guardado na configuração do app; Desconectar restaura ou remove a chave, preservando o resto do arquivo (teste explícito de encadeamento e restauração)
- [ ] Troca por terceiros detectada → aviso com Reconectar, sem reescrever
- [ ] inotify no cache da status line atualiza o snapshot
- [ ] Plano por `subscriptionType` de `~/.claude/.credentials.json` (Pro/Max); detecção = `claude` no PATH + credenciais
- [ ] Regras de dado desatualizado do ticket 14 aplicadas ao Claude
- [ ] Testes de UI por fixtures (conectado, desconectado por terceiros, sem leitura)

## Integração com a persistência (ticket 09)

Conforme escopo confirmado pelo usuário no ticket 09, integrar o snapshot deste coletor ao cache em disco quando seu contrato existir. Reutilizar as portas de persistência e adicionar projeção tipada sanitizada: nunca persistir credenciais nem conteúdo de commits/cards/prompts/respostas. Restaurar o último dado na inicialização; para Rede, persistir apenas o último teste de velocidade.
