# 09: Coleta resiliente: backoff, credencial inválida, cache em disco e logs

**What to build:** falhas transitórias não me incomodam: o app tenta de novo com backoff e só mostra "Falha ao atualizar há X" após 3 falhas seguidas, sem notificação. Uma credencial recusada (401/403) deixa o último dado esmaecido com a idade e uma faixa "Credencial inválida · Reconectar", e o app para de tentar até a credencial mudar. Logo após o login vejo o último dado conhecido, vindo de um cache em disco sem credenciais. Há um arquivo de log só com metadados. Aplicado ao GitHub nesta fatia; IA e Trello entram nesse mecanismo depois. Ver spec, "Ritmo de coleta" e "Logs".

**Blocked by:** 08 (GitHub via gh)

**Status:** ready-for-agent

- [ ] Backoff em timeout, 5xx e 429: 30 s → 1 → 2 → 4 → 8 → 15 min, respeitando `Retry-After`
- [ ] "Falha ao atualizar há X" após 3 falhas seguidas; sem notificação
- [ ] 401/403: para de tentar até a credencial mudar; faixa "Credencial inválida · Reconectar"; dado esmaecido com idade (teste explícito)
- [ ] Cache em disco em `~/.cache/kvojps-monitor/` para os snapshots de IA, GitHub, Trello e último teste de velocidade; carregado na inicialização; nunca contém credenciais (teste explícito)
- [ ] Logs em `~/.local/state/kvojps-monitor/`, rotação de 3 × 1 MB, nível `info`, só fonte, evento, status HTTP e duração; nunca credenciais, prompts, respostas nem títulos (teste explícito)
- [ ] Testes de UI por fixtures (desatualizado, falha, credencial inválida)
