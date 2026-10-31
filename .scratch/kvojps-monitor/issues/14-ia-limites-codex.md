# 14: Limites do Codex

**What to build:** o Módulo IA mostra, para o Codex, o % usado e o horário de reinício das janelas de 5 h e semanal, o plano detectado e a fonte e idade de cada leitura. Leitura antiga com janela válida mostra o % com "atualizado há X"; janela já reiniciada mostra "Reiniciado às HH:MM · sem leitura nova" sem barra; o app nunca assume 0%. Sem `codex`: "Codex não encontrado · instale e faça login no CLI". Botão ↻. Esta fatia cria a UI do Módulo IA que o Claude reaproveita. Ver spec, "Módulo IA" e ticket 01 do mapa.

**Blocked by:** 09 (Coleta resiliente)

**Status:** feito

- [x] `codex app-server` (JSON-RPC via stdio) de longa duração: `account/read` (detecção e `planType`) e `account/rateLimits/read`; escuta `account/rateLimits/updated`
- [x] Consulta de garantia a cada 5 min; a cada 60 s com o Módulo visível
- [x] Fallback: snapshots `rate_limits` nos rollouts `~/.codex/sessions/**/rollout-*.jsonl`
- [x] Regras de dado desatualizado, incluindo "nunca assumir 0% após reinício de janela" (teste explícito)
- [x] Planos ChatGPT Plus/Pro
- [x] Snapshot no cache em disco; ↻ com espera de 10 s
- [x] Testes com respostas reais gravadas do app-server e rollouts; testes de UI por fixtures (cheio, desatualizado, reiniciado, não instalado)

## Integração com a persistência (ticket 09)

Conforme escopo confirmado pelo usuário no ticket 09, integrar o snapshot deste coletor ao cache em disco quando seu contrato existir. Reutilizar as portas de persistência e adicionar projeção tipada sanitizada: nunca persistir credenciais nem conteúdo de commits/cards/prompts/respostas. Restaurar o último dado na inicialização; para Rede, persistir apenas o último teste de velocidade.

## Integração de ciclo de vida (ticket 10)

Integração futura autorizada pelo usuário no ticket 10: usar o estado compartilhado de `lifecycle`, aguardar conectividade e `Status::ready_for(posição)` (fontes remotas separadas por 3 s), pausar HTTP/consultas sem faixa de erro e preservar dados com idade. Consumir cada geração de reconexão/retomada uma vez, reiniciar relógios e verificar/reiniciar subprocessos persistentes mortos quando existirem. Preservar o bloqueio de credenciais recusadas. Definir posições distintas das fontes remotas ao integrar (GitHub usa 0).

## Decisões e desvios desta implementação

- **Fixtures de rollout transcritas, não copiadas.** A cópia automática de `~/.codex/sessions` para o repositório foi recusada pelo classificador de segurança do agente (rollouts trazem prompts e respostas). As linhas `token_count` reais inspecionadas foram transcritas à mão; demais linhas viraram esqueletos só com `timestamp` e tipos; dois `timestamp` foram derivados do nome do arquivo. Tudo registrado em `src-tauri/fixtures/codex/README.md`. As respostas do app-server são captura real (`capture.py`), anonimizada. A notificação `account/rateLimits/updated` não chegou durante a captura e é montada nos testes a partir do `RateLimitSnapshot` gravado.
- **Ciclo de vida (ticket 10):** Codex ocupa a posição 2 entre as fontes remotas (GitHub 0, Trello 1). Sem rede, pausam as consultas `account/rateLimits/read`; notificações do servidor continuam sendo aplicadas. Nova geração: consulta uma vez, zera relógios e reinicia o app-server morto na hora; fora disso, reinício no máximo a cada 30 s.
- **Coleta resiliente (ticket 09):** falha ou ausência de resposta (30 s) em `account/rateLimits/read` aplica o backoff 30 s → 15 min, cai para os rollouts e, após 3 falhas seguidas, mostra "Falha ao atualizar há X". Não há 401/403 nessa fonte: o próprio CLI cuida do token.
- **Atualização esparsa:** `account/rateLimits/updated` só é mesclada sobre uma leitura do app-server; sobre leitura de rollout (ou sem leitura) dispara a leitura completa, para não apresentar valores antigos como novos.
- **Rollouts:** entre as 10 sessões mais recentes, vale o evento `token_count` com janela de horário mais recente (uma sessão iniciada antes pode ter o dado mais novo). Buckets sem janelas (`premium`) são ignorados.
- **Sem `codex` ou sem login:** mostra "Codex não encontrado · instale e faça login no CLI", como pede a spec, mesmo havendo leitura em cache.
- **Cache em disco:** `ai.json` guarda plano, fonte, idade e janelas; nunca e-mail ou IDs de conta (teste explícito).
- Estrutura do snapshot `{refreshing, paused?, codex: {...}}`, para o Claude (ticket 15) entrar como chave irmã.
- Verificação contra o `codex` 0.166.0 real desta máquina: adapter de subprocesso leu plano `plus` e as duas janelas via app-server (teste temporário, não versionado). Verificação visual no GNOME não realizada.
