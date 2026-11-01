# 13: Composição fixa sem dados

Type: prototype
Status: resolved
Blocked by: None (can start immediately).

## Question

Como cada Módulo mantém a composição do B (mesmos blocos, nas mesmas posições e alturas) em todos os estados, preenchendo o lugar de cada dado ausente com uma mensagem que diz por que ele falta?

- Regra do usuário (2026-11-01): “Temos que manter essa composição em todos os cenários, mesmo se não tiver o dado o espaço que o dado ocuparia é preenchido com uma mensagem de erro.”
- Hoje, sem dado, o Módulo encolhe: some o bloco e fica uma linha `small` solta (por exemplo, “Sem leitura…” no GitHub, Rede, Sistema e Trello; credencial ausente ou escolha de quadro no Trello; aviso de Agentes oculto sem leitura, com falha ou com aguardando indisponível no Codex). A composição muda e os instrumentos que crescem ([Alinhamento da distribuição vertical](12-vertical-distribution-alignment.md)) tomam o espaço que sobra.

## Comments

Origem: conferência do usuário em 2026-11-01, depois da caixa neutra “Nenhuma sessão aguardando você” (ticket 12).

Pontos a decidir no protótipo:

- **Granularidade:** a mensagem vai no lugar de cada bloco (número grande, gráfico, barras, lista) ou numa área única do tamanho do corpo do cartão?
- **Forma do vazio:** instrumento em estado vazio (anel/barra/gráfico apagado, como o “—” e o anel vazio de Agentes) com a mensagem dentro ou sobre ele, ou caixa de mensagem no lugar do instrumento.
- **Vocabulário por causa:** sem leitura ainda, falha da coleta (com idade da última leitura), credencial ausente/inválida, fonte ausente, sem sensor/bateria/swap, medição indisponível, Codex sem Detecção instantânea. Cor (neutra × âmbar × vermelha) e ação (Reconectar, Configurar) de cada uma.
- **Dado antigo × ausente:** com leitura anterior (dado em cache marcado como antigo), o bloco mostra o dado apagado ou a mensagem?
- **Linhas condicionais do topo** (ticket 02): continuam como linha `small` no topo do corpo ou passam para dentro do bloco afetado?
- **Agentes:** a caixa do aviso mostra a mensagem do motivo (sem leitura, falha, aguardando indisponível) em vez de sumir.

Conferir no app nativo (`npm run dev:fixtures` com `KVOJPS_FIXTURES`, `scripts/capture-panel.sh`) com as fixtures de cada estado ausente de cada Módulo, em 1100 × 700 e 1000 × 700, temas escuro e claro, sem rolagem; a composição deve ficar igual à do estado B.

## Answer

Aprovada pelo usuário em 2026-11-01: **variante C, área única no corpo** (“Prefiro a variante C”).

- **Falta a leitura inteira** (sem leitura ainda, falha sem leitura anterior, credencial ou fonte ausente, Trello sem quadro escolhido): o cabeçalho do cartão fica como no B e o corpo do Módulo vira **uma única área** que ocupa a altura dele, com ícone, a mensagem do motivo e, quando existe, um botão de ação (“Configurar”, “Reconectar”, “Tentar de novo”). No cartão IA e Agentes cada fonte tem a própria área: Codex e Claude na coluna de cada anel; Agentes no lugar do anel, da legenda e do aviso.
- **Tom:** “sem leitura” neutro (borda e texto discretos, ícone ◌); falha, credencial e fonte ausente em âmbar (⚠). Vermelho não entra.
- **Antigo** (há leitura anterior): o dado anterior fica **apagado** sob a linha “⚠ Leitura antiga · atualizada há …”; não vira área.
- **Linhas condicionais do topo** (antigo, coleta pausada etc.) continuam como linha `small` no topo do corpo; o instrumento que cresce do cartão ([Alinhamento da distribuição vertical](12-vertical-distribution-alignment.md)) encolhe para absorvê-las.
- **Faltas parciais** só no item afetado, sem área: “Temperatura · sem sensor” com “—”; “Sem swap configurada” sob a barra vazia; “▱ Sem bateria detectada” no lugar da bateria; “Janela semanal indisponível” sob a barra Semana; “Sem teste ainda” no teste de velocidade; aplicativos indisponíveis no botão de aplicativos; no Codex sem Detecção instantânea, a caixa do aviso de Agentes mostra “Aguardando indisponível sem Detecção instantânea · Configurar” em vez de sumir.
- Rede, Sistema e Agentes não têm credencial: sem leitura ou falha caem nas mensagens deles.
- Protótipo (variantes A instrumento vazio, B caixa por bloco, C área única; seis cenários e Módulo afetado; `npm run prototype:absent`) no branch local `prototype/composition-without-data` (commit `0dd2439`, apagada em 2026-11-01), não enviado. Conferido só no Chromium headless em 1100 × 700; a conferência nativa vai para [Composição fixa sem dados na produção](14-composition-without-data-in-production.md).
