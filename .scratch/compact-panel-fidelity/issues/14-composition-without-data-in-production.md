# 14: Composição fixa sem dados na produção

Type: task
Status: resolved
Blocked by: None (13 resolved).

**What to build:** levar aos Módulos de produção a decisão da [Composição fixa sem dados](13-composition-without-data.md) (variante C), substituindo as linhas `small` soltas que hoje ficam no lugar do corpo quando falta a leitura.

- [x] Área única no corpo de GitHub, Trello, Rede e Sistema, e por fonte no cartão IA e Agentes (Codex, Claude, Agentes), para sem leitura, falha sem leitura anterior, credencial/fonte ausente e Trello sem quadro; tom neutro × âmbar e ação como na resposta do ticket 13. Textos em `strings.json`.
- [x] Antigo continua com dado apagado e linha `small`; linhas condicionais do topo continuam no topo do corpo.
- [x] Faltas parciais só no item afetado (sensor, swap, bateria, semana, teste de velocidade, aplicativos, aguardando no Codex), inclusive a caixa do aviso de Agentes no lugar de sumir.
- [x] Testes de UI por fixtures para cada estado ausente de cada Módulo.
- [x] Conferência no app nativo (`KVOJPS_FIXTURES`, `scripts/capture-panel.sh`) com as fixtures de cada estado ausente, em 1100 × 700 e 1000 × 700, temas escuro e claro, sem rolagem; aprovação do usuário.

## Comments

Origem: resposta do ticket 13 (2026-11-01). Protótipo de referência: `src/lib/compact-prototype/AbsentPrototype.svelte` no branch local `prototype/composition-without-data`.

Implementação (2026-11-01):

- `src/lib/AbsentArea.svelte`: área única da variante C (◌ neutro, ⚠ âmbar, ação opcional), usada por GitHub, Trello, Rede, Sistema, Codex, Claude e Agentes. Na coluna de um provedor e no lugar de Agentes a altura mínima acompanha `--dial`, então a composição do cartão IA e Agentes se mantém mesmo com tudo ausente.
- Ações: “Configurar”/“Reconectar” abrem as Configurações; “Tentar de novo” usa o mesmo ↻ (mesmo intervalo de 10 s). Sem ação onde o app não tem o que fazer: Codex/Claude não encontrados, token do Claude expirado, falha de Agentes (não há comando de atualizar Agentes).
- Antigo: GitHub e Trello apagam o dado sempre que há leitura anterior e o estado não é `ready` (antes só com credencial inválida); Sistema apaga com a leitura antiga; Codex/Claude com falha e leitura anterior apagam anel e barra por opacidade, mantendo a cor do provedor; Agentes com falha e última leitura apagam anel, legenda e caixa.
- **Divergências do ticket 13 (aprovadas):**
  - Aplicativos sem medição: o núcleo manda `processTraffic: null` tanto no aquecimento quanto quando a atribuição não funciona, sem distinguir. “Aplicativos indisponíveis · só o total da interface” seria falso nos primeiros segundos; o botão diz “Aplicativos de rede · sem medição ↗” (vocabulário da Rede).
  - Agentes com falha e leitura anterior, sem sessão aguardando: o ticket 12 proíbe afirmar “Nenhuma sessão aguardando você” sem leitura que sustente. A caixa fica no lugar e diz “Nenhuma sessão aguardando na última leitura (HH:MM)”, apagada com o resto.
  - Agentes ausente quando `failed` sem `lastSuccessAt`, mesmo com sessões encerradas em memória.
- Correções vistas na captura: espaço depois de “⚠”/“▰” (o Svelte cortava o espaço final do `<span>`; “⚠Falha”) e idade abaixo de 1 min dizendo “agora”, não “este minuto”, no GitHub e no Trello.
- Fixtures novas para conferência: `no-reading` (todos os Módulos, `null`), `failure-no-reading` (IA, Agentes, Trello), `ai=sources-missing`, `network=partial` (sem teste e sem aplicativos).
- Conferência nativa por captura (`scripts/capture-panel.sh`), 1100 × 700 escuro e 1000 × 700 claro, `overflowing: []` em todos os casos: sem leitura (todos), falha sem leitura e parciais, fonte/credencial ausente e Codex sem Detecção instantânea, antigo, e o estado B (sem mudança).

## Answer

Aprovado pelo usuário em 2026-11-01 (“Pode aprovar”). A variante C está na produção: área única por Módulo (e por fonte no cartão IA e Agentes) quando falta a leitura inteira; dado antigo apagado sob a linha; faltas parciais no próprio item; caixa do aviso de Agentes sempre presente. Divergências aprovadas: botão “Aplicativos de rede · sem medição” (o núcleo não distingue aquecimento de indisponível) e caixa “Nenhuma sessão aguardando na última leitura (HH:MM)” após falha dos Agentes. Commits `2b0a3f5`, `d8f36de`, `3d2a251`.
