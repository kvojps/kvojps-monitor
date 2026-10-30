# 18: Sessões do Codex

**What to build:** o Módulo Agentes passa a listar também as Sessões de agente do Codex, junto das do Claude Code. Sem hooks, aparecem Em andamento, Turno concluído, Com falha e Encerrada, e o Módulo indica que "Aguardando" do Codex está indisponível. Ver spec, "Módulo Agentes".

**Blocked by:** 17 (Sessões do Claude Code por polling)

**Status:** ready-for-agent

- [ ] SQLite `~/.codex/state_5.sqlite` em modo read-only, tabela `threads` (cwd, título, criação, última atualização, branch)
- [ ] Último evento do rollout (`task_started`/`task_complete`/`turn_aborted`) + processo vivo definem o estado; `turn_aborted` = Turno concluído
- [ ] Indicação "Aguardando indisponível sem Detecção instantânea" no Módulo
- [ ] Adapter isolado (formatos instáveis); testes com SQLite e rollouts reais gravados
- [ ] Testes de UI com sessões dos dois provedores
