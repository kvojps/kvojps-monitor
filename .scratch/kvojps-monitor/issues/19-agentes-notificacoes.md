# 19: Notificações de Agentes

**What to build:** recebo uma notificação nativa quando uma sessão entra em Aguardando você ("Aguardando aprovação: <ferramenta>" ou "Aguardando sua resposta"), quando um turno termina Com falha, e quando um Turno concluído durou ≥ 30 s ("Turno concluído em 4 min 12 s"). Interrupção e Encerrada nunca notificam. No máximo uma notificação viva por sessão; 3+ eventos em 10 s viram uma agrupada ("3 sessões aguardando você"). Clicar abre o Painel no Módulo Agentes com a sessão destacada. Três interruptores nas Configurações → Agentes, todos ligados por padrão. Ver spec, "Módulo Agentes" e ticket 09 do mapa.

**Blocked by:** 05 (Segurança da rede, notificação e badge de Risco), 17 (Sessões do Claude Code por polling)

**Status:** ready-for-agent

- [ ] Título `<Claude Code | Codex> · <projeto>` [`· <nome>`]; corpo sem nenhum trecho de prompt ou resposta (teste explícito)
- [ ] Turno concluído < 30 s não notifica (teste explícito); interrupção e Encerrada nunca notificam
- [ ] Nova notificação da mesma sessão substitui a anterior
- [ ] ≥ 3 eventos em 10 s → uma notificação agrupada (teste explícito com relógio falso)
- [ ] Clique abre o Painel no Módulo Agentes com a sessão destacada (exceção ao "último Módulo visitado")
- [ ] Três interruptores (Aguardando, Turno concluído, Falha), sem separação por provedor; Não perturbe do GNOME sem lógica própria
