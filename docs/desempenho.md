# Medição de desempenho e memória

Protocolo para medir critérios do Painel que não têm resultados registrados no app nativo. A validação da versão atual não mediu CPU, latência, quadros ou memória em ambiente GNOME/Wayland controlado. A única referência histórica é a do primeiro esqueleto (~298 MB de PSS com núcleo e WebKit, sem Codex), que motivou destruir a webview ao fechar.

Alvos: acréscimo médio de CPU da decoração ≤ 2 pontos percentuais **de um núcleo** (não 2 % da máquina); resposta visual ≤ 100 ms; 60 fps em tela de 60 Hz; nenhum trabalho de animação com o Painel oculto.

## Preparar

1. `npm run build` e `cargo build --release --manifest-path src-tauri/Cargo.toml`. Registre commit, `sha256sum` do binário e versões de Rust, Tauri/Wry e WebKitGTK. Use o mesmo binário em todas as rodadas.
2. Registre monitor, resolução, Hz, escala, tamanho efetivo do Painel, tema fixo, alimentação e perfil energético; mantenha constantes. Registre fontes habilitadas e quantidade de itens por Módulo, sem títulos, conteúdo ou credenciais. Use fontes reais, não o modo de fixtures.
3. Abra pelo Indicador e espere 60 s. Não rode testes/builds junto.
4. `pgrep -x kvojps-monitor` dá o núcleo; `ps -eo pid,ppid,comm` (sem argumentos) separa WebKitWebProcess, WebKitNetworkProcess e filhos de coleta (`codex app-server`). Confira a árvore antes e depois de cada rodada; processo novo ou que saiu invalida a rodada.

## Coletor

[`scripts/perf-collect.py`](../scripts/perf-collect.py) lê só contadores de CPU, identidade e PSS de `/proc` dos PIDs informados; não lê argumentos, ambiente, DOM, arquivos do app nem rede. Falha sem resultado parcial se um processo some ou muda.

```sh
python3 scripts/perf-collect.py --app 1000 --webview 1001 1002 --collectors 1003 --mode full > pair-1-full.json
```

Padrão: 60 s, amostra de 1 s. Some `summary.app.mean_cpu_core_percent` e `summary.webview.mean_cpu_core_percent`: 100 % é uma CPU lógica ocupada (**não dividir pelo número de CPUs**). Reporte coletores à parte. PSS em KiB (÷ 1024 para MiB), por grupo. O script não detecta processos novos nem mede fps ou latência.

## CPU da decoração: três pares

Ordem **ligado/reduzido**, **reduzido/ligado**, **ligado/reduzido**, alternando a redução de animações nas Configurações (preferência do sistema desligada, senão ela prevalece). Espere 60 s após cada troca, depois capture 60 s sem interação e sem inspetor aberto. Delta = ligado − reduzido; aceite pela média dos três, preservando negativos e dispersão. Ruído alto pede repetição.

## Resposta, fluidez e ocultação

Siga também [motion-trace.md](motion-trace.md). Capture timeline da webview e trace com Sysprof na sessão GNOME, sem exportar DOM, títulos ou dados reais. Pelo menos 20 interações por cenário (abrir/voltar de Configurações, abrir detalhes, rolar listas densas, quadro do Trello): entrada até o primeiro quadro com a mudança, máximo e p95 ≤ 100 ms. Quadros: p95, maior intervalo, perdidos e travamentos com ação.

Feche por Esc, Indicador e perda de foco (capturas separadas) e confira a destruição da webview e trabalho residual. Captura fechada de 60 s com `--mode closed --webview` (sem PIDs) só se nenhum WebKit do app restar. Verifique redução manual, do sistema durante a execução e mudança com Painel fechado; restaure a preferência do sistema.

## Memória e reabertura

Mantenha a destruição da webview (`panel::close`). Meça PSS aberto/reduzido, aberto/ligado e fechado. Reabra e confira Âncora, cache com idade, preferências e comportamento offline; repita após reiniciar o app e via notificação real de Agentes (detalhe de Sessões com a sessão destacada, sem perder a Âncora).
