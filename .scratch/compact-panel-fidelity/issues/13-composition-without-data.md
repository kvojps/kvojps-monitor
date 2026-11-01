# 13: Composição fixa sem dados

Type: prototype
Status: open
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
