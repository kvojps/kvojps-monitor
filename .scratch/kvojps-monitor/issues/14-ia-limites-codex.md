# 14: Limites do Codex

**What to build:** o Módulo IA mostra, para o Codex, o % usado e o horário de reinício das janelas de 5 h e semanal, o plano detectado e a fonte e idade de cada leitura. Leitura antiga com janela válida mostra o % com "atualizado há X"; janela já reiniciada mostra "Reiniciado às HH:MM · sem leitura nova" sem barra; o app nunca assume 0%. Sem `codex`: "Codex não encontrado · instale e faça login no CLI". Botão ↻. Esta fatia cria a UI do Módulo IA que o Claude reaproveita. Ver spec, "Módulo IA" e ticket 01 do mapa.

**Blocked by:** 09 (Coleta resiliente)

**Status:** ready-for-agent

- [ ] `codex app-server` (JSON-RPC via stdio) de longa duração: `account/read` (detecção e `planType`) e `account/rateLimits/read`; escuta `account/rateLimits/updated`
- [ ] Consulta de garantia a cada 5 min; a cada 60 s com o Módulo visível
- [ ] Fallback: snapshots `rate_limits` nos rollouts `~/.codex/sessions/**/rollout-*.jsonl`
- [ ] Regras de dado desatualizado, incluindo "nunca assumir 0% após reinício de janela" (teste explícito)
- [ ] Planos ChatGPT Plus/Pro
- [ ] Snapshot no cache em disco; ↻ com espera de 10 s
- [ ] Testes com respostas reais gravadas do app-server e rollouts; testes de UI por fixtures (cheio, desatualizado, reiniciado, não instalado)

## Integração com a persistência (ticket 09)

Conforme escopo confirmado pelo usuário no ticket 09, integrar o snapshot deste coletor ao cache em disco quando seu contrato existir. Reutilizar as portas de persistência e adicionar projeção tipada sanitizada: nunca persistir credenciais nem conteúdo de commits/cards/prompts/respostas. Restaurar o último dado na inicialização; para Rede, persistir apenas o último teste de velocidade.
