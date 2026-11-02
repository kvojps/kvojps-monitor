# Evidências — ticket 39

Validação em 02/11/2026 no WebKitGTK 4.1, motor usado pelo Tauri no Linux, montando `OperationsStrip.svelte` com os tokens do app e snapshots/transições pela interface pública.

- Regressão reproduzida antes da correção: duas mensagens curtas ocupavam colunas iguais com aproximadamente 490 px vazios em cada coluna, nos dois temas. A asserção de largura pelo conteúdo falhou. Após a correção, os mesmos 24 cenários passaram.
- Matriz: uma mensagem, duas mensagens curtas, seis Módulos simultâneos e resumos com comprimentos diferentes/agrupamento de Codex e Claude × claro/escuro × 1400 px/12 px, 760 px/12 px e 320 px/18 px (texto a 150%).
- `geometry.json`: intervalos fixos de 16 px entre resumos, sem espaço excedente dentro das mensagens curtas, sem sobreposição e sem transbordamento horizontal. O resumo de excedentes apresenta +3, +4 ou +5 Módulos conforme a largura. Mensagens longas são truncadas com reticências.
- `check.py` verifica a entrada de snapshots e a saída dos resultados pelo relógio injetado. Nos cenários de uma/duas mensagens, a faixa retorna a “Bordo em repouso”. Anúncios acessíveis permanecem completos; os marcadores sensíveis das fixtures não aparecem.
- Capturas `<cenário>-<tema>-<largura>-<fonte>.png`. Inspeção visual: `two-dark-1400-12.png` mostra mensagens curtas consecutivas; `long-light-320-18.png` mostra texto ampliado com reticências e +5 Módulos; a mesma matriz cobre os dois temas. O CSS de movimento e a Órbita contínua não foram alterados.

Para reproduzir: executar `npm run dev -- --port 1422` e, numa sessão gráfica com Python GI, GTK 3 e WebKit2 4.1, executar da raiz `/usr/bin/python3 .scratch/kvojps-monitor/validation/39/check.py`. O harness não integra o produto. Os dados operacionais vêm da fixture `src/lib/fixtures/operations/simultaneous.json` e de snapshots de cancelamento, sem novas mensagens no produto.

Validação automatizada: `npm run check` sem erros/avisos; 19 testes focados da Faixa de bordo; suíte completa com 376 testes Vitest e 305 testes unitários Rust. Os três testes de integração Rust passaram após execução fora do sandbox, necessária para acesso a sockets locais.

## Standards

Revisão paralela desde `61a08bce3605067b6356beb48e25e27e0248a370`: nenhuma violação documentada e nenhuma heurística da baseline apontada. CSS localizado, sem mudança de domínio, E/S ou privacidade; validação pela interface pública com snapshots e relógio injetado; histórico do ticket e artefatos seguem as convenções locais.

## Spec

Nenhum achado. Largura pelo conteúdo, intervalo fixo e redução com reticências atendem ao ticket. Agrupamento, prioridade, permanência, anúncios acessíveis e movimento preservados. As evidências dos 24 cenários cobrem os critérios de aceite; revisão também inspecionou capturas de mensagens curtas e de texto ampliado com excedentes.

Total: Standards 0 achados; Spec 0 achados.
