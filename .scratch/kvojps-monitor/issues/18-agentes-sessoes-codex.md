# 18: Sessões do Codex

**What to build:** o Módulo Agentes passa a listar também as Sessões de agente do Codex, junto das do Claude Code. Sem hooks, aparecem Em andamento, Turno concluído, Com falha e Encerrada, e o Módulo indica que "Aguardando" do Codex está indisponível. Ver spec, "Módulo Agentes".

**Blocked by:** 17 (Sessões do Claude Code por polling)

**Status:** ready-for-agent

- [ ] SQLite `~/.codex/state_5.sqlite` em modo read-only, tabela `threads` (cwd, título, criação, última atualização, branch)
- [ ] Último evento do rollout (`task_started`/`task_complete`/`turn_aborted`) + processo vivo definem o estado; `turn_aborted` = Turno concluído
- [ ] Indicação "Aguardando indisponível sem Detecção instantânea" no Módulo
- [ ] Adapter isolado (formatos instáveis); testes com SQLite e rollouts reais gravados
- [ ] Testes de UI com sessões dos dois provedores

## Integração de ciclo de vida (ticket 10)

Integração futura autorizada pelo usuário no ticket 10: usar o estado compartilhado de `lifecycle`, aguardar conectividade e `Status::ready_for(posição)` (fontes remotas separadas por 3 s), pausar HTTP/consultas sem faixa de erro e preservar dados com idade. Consumir cada geração de reconexão/retomada uma vez, reiniciar relógios e verificar/reiniciar subprocessos persistentes mortos quando existirem. Preservar o bloqueio de credenciais recusadas. Definir posições distintas das fontes remotas ao integrar (GitHub usa 0).
