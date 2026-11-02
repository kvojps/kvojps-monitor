# Fixtures sintéticas do README

Os arquivos `readme.json` em cada pasta `src/lib/<modulo>/fixtures/` alimentam a captura nativa usada no README. Foram derivados em memória dos snapshots publicados, mantendo os formatos esperados pela UI; antes de gravar, os valores de atividade e identidade foram substituídos por exemplos sintéticos.

- `ai`: percentuais de uso e horários de leitura/reset sintéticos; metadados de plano omitidos e sem credenciais.
- `agents`: sessões, estados, projetos e nomes de exemplo.
- `github`: contagens diárias, total, sequência e atividade do repositório sintéticos; login e repositório usam aliases. As datas do calendário servem apenas como eixo temporal do gráfico.
- `trello`: quadro, listas, cards, IDs, URLs e conta de exemplo.
- `network`: tráfego por aplicativo, velocidade, latência, perda e histórico sintéticos; sem endereço ou identidade de rede.
- `system`: uso de CPU, memória, swap, disco, temperatura e bateria sintéticos.

Para reproduzir a captura Tauri nativa no modo de desenvolvimento, inicie com `KVOJPS_FIXTURES="ai=readme,agents=readme,github=readme,trello=readme,network=readme,system=readme" npm run dev:fixtures`, depois siga o procedimento de captura e sanitização descrito em [`docs/desenvolvimento.md`](../../../docs/desenvolvimento.md#modo-de-fixtures-nativo). Essas fixtures não representam leituras de uma máquina ou conta real.
