# Evidências — ticket 38

Validação no WebKitGTK 4.1 em 02/11/2026, com relógio fixo em `1793401200000`.

- 36 combinações: estado normal e oito fixtures assimétricas × claro/escuro × largura de 360 px com fonte de 14 px ou 240 px com fonte de 21 px (150%).
- Fixtures: `src/lib/ai/fixtures/asymmetric.json`, derivadas do snapshot `claude-usage.json`; alternam Claude/Codex, janela de 5 h/semanal e reinício/ausência de leitura na janela.
- `geometry.json` registra as coordenadas dos medidores e das trilhas: mesma altura entre provedores, sem transbordamento horizontal nos 36 casos. O WebKit confirmou suporte a subgrid.
- Capturas `<índice>-<tema>-<largura>-<fonte>.png`: índice -1 é o estado normal; 0–7 seguem a ordem das fixtures. Inspeção visual dos estados assimétricos ampliados nos dois temas: explicações inteiras, sem sobreposição, consumo desconhecido como travessão. A trilha sem valor mantém apenas o espaço, sem preenchimento ou semântica de consumo.
- A suíte existente preserva os casos de ausência total de leitura e ações de configurar, reconectar e tentar novamente; os oito novos casos verificam as ausências por janela e a independência dos provedores.

Para reproduzir, executar Vite na porta 1422 e, numa sessão gráfica com Python GI, GTK 3 e WebKit2 4.1, executar da raiz: `/usr/bin/python3 .scratch/kvojps-monitor/validation/38/check.py`. O harness `index.html` monta o componente real com os tokens do app; não integra o produto.

Validação automatizada: `npm run check` sem erros/avisos; 376 testes Vitest; 305 testes unitários Rust e três testes de integração. Os testes de sockets Rust exigiram execução fora do sandbox.

Revisão em dois eixos desde `8519c13755301420f6d54730c5155e55215d6751`: padrões sem achados; spec sem divergências no código, com comprovação visual inicialmente pendente e registrada nesta pasta.
