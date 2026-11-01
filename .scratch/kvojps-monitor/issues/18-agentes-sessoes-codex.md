# 18: Sessões do Codex

**What to build:** o Módulo Agentes passa a listar também as Sessões de agente do Codex, junto das do Claude Code. Sem hooks, aparecem Em andamento, Turno concluído, Com falha e Encerrada, e o Módulo indica que "Aguardando" do Codex está indisponível. Ver spec, "Módulo Agentes".

**Blocked by:** 17 (Sessões do Claude Code por polling)

**Status:** done

- [x] SQLite `~/.codex/state_5.sqlite` em modo read-only, tabela `threads` (cwd, título, criação, última atualização, branch)
- [x] Último evento do rollout (`task_started`/`task_complete`/`turn_aborted`) + processo vivo definem o estado; `turn_aborted` = Turno concluído
- [x] Indicação "Aguardando indisponível sem Detecção instantânea" no Módulo
- [x] Adapter isolado (formatos instáveis); testes com SQLite e rollouts reais gravados
- [x] Testes de UI com sessões dos dois provedores

## Integração de ciclo de vida (ticket 10)

Integração futura autorizada pelo usuário no ticket 10: usar o estado compartilhado de `lifecycle`, aguardar conectividade e `Status::ready_for(posição)` (fontes remotas separadas por 3 s), pausar HTTP/consultas sem faixa de erro e preservar dados com idade. Consumir cada geração de reconexão/retomada uma vez, reiniciar relógios e verificar/reiniciar subprocessos persistentes mortos quando existirem. Preservar o bloqueio de credenciais recusadas. Definir posições distintas das fontes remotas ao integrar (GitHub usa 0).

## Implementação

Adapter isolado para o SQLite em modo somente leitura, via porta de arquivos,
e eventos de rollout. Processos Codex são associados às threads pelos descritores
abertos em `/proc`; associação verificada no host. Snapshots identificam o provedor,
carregam branch e última atualização, e compartilham a ordenação e o histórico em
memória do Módulo. Falhas de consulta preservam a última leitura e não encerram
sessões. Polling local continua sem rede; não há HTTP, credenciais ou subprocesso
persistente novo neste ticket.

Fixtures locais sanitizadas de SQLite, início, conclusão e interrupção; falha é
uma variante explícita do schema. Testes no seam do núcleo cobrem os estados,
privacidade, acesso somente leitura, falhas injetadas e histórico. Fixture de UI
reúne os dois provedores e a indicação de Aguardando indisponível.

## Correção: sessões ausentes por processos sem acesso ao executável

A verificação do coletor no host reproduziu `failed=true, active=0` apesar de
haver rollout aberto correspondente ao SQLite. Processos do mesmo usuário
(`systemd`, `(sd-pam)` e `ssh-agent`) negavam a leitura de `/proc/<pid>/exe` e
abortavam toda a coleta. Agora o adapter consulta também `comm` nesse caso:
processos que não são Codex são ignorados; falhas de acesso de candidatos Codex
continuam preservando a última leitura. Regressão exercitada no seam do núcleo.
Após a correção, o mesmo diagnóstico no host retornou `failed=false, active=1`.
O harness temporário foi removido após confirmar o resultado.
