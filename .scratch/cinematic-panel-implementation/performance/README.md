# Desempenho e memória — ticket 11

## Estado e limites

Preparação em 2026-10-31, baseline do código
`c675ddb572d92685b47ddd5480d9d976b464dbaa`. **Medição nativa pendente;
ticket não concluído.** O usuário escolheu “Deixar protocolo pronto e registrar
pendência”. Não foram executadas rodadas de CPU/PSS do app, captura de quadros,
latência nem verificação de fechamento/recriação nesta etapa.

Ambiente consultado: Ubuntu 26.04.1 LTS, Linux 7.0.0-38-generic,
AMD Ryzen 5 5625U (6 núcleos / 12 CPUs lógicas), sessão Wayland,
GNOME Shell 50.1 (consulta D-Bus real fora do sandbox), GTK 3.24.52,
WebKitGTK 2.52.6, Node 24.20.0 e Python 3.14.4. `pidstat`, `sysprof-cli`
e `perf` estão instalados; disponibilidade não comprova captura autorizada
ou bem-sucedida. Monitor/Hz/escala, RAM, alimentação e perfil energético ainda
precisam ser registrados no momento da execução.

O binário release existente tinha data 2026-10-31 17:36:47 -0300; sua
correspondência com o baseline não foi comprovada. Não usá-lo como evidência
do build atual. A medição anterior do ticket 01 (aproximadamente 298 MB de PSS,
incluindo núcleo e WebKit, sem Codex) é histórica, não uma revalidação.

## Preparar a execução nativa

1. Execute `npm run build` e
   `cargo build --release --manifest-path src-tauri/Cargo.toml`. Registre commit,
   `sha256sum src-tauri/target/release/kvojps-monitor`, versões de Cargo/Rust,
   Tauri/Wry do lockfile e WebKit/GTK. Use esse mesmo binário nas seis rodadas.
2. Registre monitor, resolução, Hz, escala, dimensões efetivas do Painel,
   fontes, tema fixo, alimentação AC/bateria e perfil energético. Mantenha tudo
   constante. Registre fontes habilitadas, estados e quantidade de itens por
   Módulo, sem títulos, conteúdo ou credenciais. Produção usa fontes reais;
   a superfície de fixtures do ticket 10 é navegador e não vale como nativa.
3. Abra pelo Indicador, deixe as mesmas Estações visíveis e espere 60 s para
   estabilizar. Não execute testes/builds junto da medição. Registre atividade
   externa relevante, coleta legítima e estado de rede. Não desconecte serviços
   nem altere arquivos de credenciais para criar um baseline.
4. Identifique o PID do app com `pgrep -x kvojps-monitor`. Inspecione a árvore
   por `ps -eo pid,ppid,comm` (sem argumentos/ambiente). Separe núcleo,
   WebKitWebProcess, WebKitNetworkProcess e eventuais processos auxiliares
   WebKit da mesma instância. Registre à parte os filhos de coleta, como
   `codex app-server`; não inclua processos de outros apps ou o coletor Python.
   Confira a árvore antes e depois de cada rodada. Um filho novo, saída ou
   reparenting invalida a lista estática: identifique novamente e repita.

## Coletor externo de metadados

`collect.py` lê somente contadores de CPU, identidade de processo e PSS de
`/proc`. Não lê argumentos, ambiente, DOM, snapshots, arquivos do app ou rede.
Não faz parte do núcleo nem adiciona coleta decorativa. Exige acesso a
`smaps_rollup` dos PIDs selecionados; falha sem publicar resultado parcial
quando um processo some, muda de identidade ou não pode ser lido.

Exemplo com PIDs ilustrativos (substituir pelos observados):

```sh
python3 .scratch/cinematic-panel-implementation/performance/collect.py --app 1000 --webview 1001 1002 --collectors 1003 --mode full > /tmp/pair-1-full.json
```

O padrão é 60 s, amostragem de 1 s. O JSON guarda apenas modo, PIDs, durações,
CPU e PSS por grupo. A média de CPU é ponderada pela duração real. Some
`summary.app.mean_cpu_core_percent` e
`summary.webview.mean_cpu_core_percent`: 100% representa uma CPU lógica
ocupada continuamente; **não dividir por 12**. Coletores ficam separados e
devem ser reportados também. Somar PSS evita a dupla contagem de páginas
compartilhadas do RSS; converter KiB para MiB dividindo por 1024. Reportar
PSS do núcleo, WebKit, coletores e total separadamente.

A lista de PIDs é explícita: o script não detecta filhos novos nem processos
que nascem e morrem entre amostras. Essa limitação exige a inspeção de árvore
e trace de processos. Leitura sequencial de `/proc` tem pequeno desvio temporal;
o custo do observador é externo e deve ser mantido igual em todas as rodadas.
O script não mede fps, latência ou callbacks.

## Três pares e decisão de CPU

Faça pares na ordem **ligado/reduzido**, **reduzido/ligado**,
**ligado/reduzido**. Configure a opção em Configurações, volte ao Painel,
confirme a política efetiva e espere novamente 60 s antes de cada captura de
60 s. A preferência de redução do sistema precisa estar desligada nesse A/B;
se prevalecer, não existe rodada de movimento ligado válida. Nenhuma interação
durante as rodadas de repouso. Não deixe inspetor ou profiler aberto nesse A/B;
use capturas separadas para fluidez e registre o custo da instrumentação.

| Par | Ordem | CPU ligado (núcleo + WebKit) | CPU reduzido | Delta em pontos percentuais | PSS ligado/reduzido |
| --- | --- | --- | --- | --- | --- |
| 1 | ligado → reduzido | pendente | pendente | pendente | pendente |
| 2 | reduzido → ligado | pendente | pendente | pendente | pendente |
| 3 | ligado → reduzido | pendente | pendente | pendente | pendente |

Calcule cada delta como ligado menos reduzido e a média dos três deltas.
Aceite provisório: média ≤ 2 pontos percentuais de um núcleo. Preserve os
valores negativos e a dispersão; ruído alto pede repetição, não arredondamento
favorável. Não comparar esse incremento ao orçamento total da coleta.

## Resposta, fluidez e ocultação

Siga também [Trace da Coreografia combinada](../../../docs/motion-trace.md).
Use o inspetor de uma build nativa que o disponibilize; se precisar de build
de diagnóstico, identifique-a separadamente e não misture seus números ao A/B
release. Capture timeline de renderização e quadros da webview e trace de
processos/compositor com Sysprof na sessão GNOME. Não exporte DOM, tráfego,
stacks com conteúdo, títulos de cards ou capturas de dados reais. Revise os
artefatos localmente antes de versionar qualquer exportação; prefira tabelas
de tempos agregados.

Registre pelo menos 20 interações por cenário: navegação por âncoras, abrir/
voltar de Configurações, expansão de listas, rolagem em listas densas e quadro
Trello. Registre contagens/densidade e estado das fontes. Meça entrada até o
primeiro quadro que mostra a mudança, com máximo e p95; alvo ≤ 100 ms.
Tempo até retorno do handler não é resposta visual. Para 60 Hz, alvo de quadros
de aproximadamente 16,67 ms; reporte p95, maior intervalo, quadros perdidos
e travamentos com duração e ação. Ajuste ao período real em outro refresh.
Nenhum valor foi observado ainda.

No mesmo trace, feche por Escape, Indicador e perda de foco (capturas separadas),
observe a destruição da janela/contexto e identifique trabalho residual dos
processos. A marca `panel-decoration` registra a política, não ausência de
callbacks. Distinga polling legítimo dos coletores de trabalho decorativo.
Se WebKit permanecer como processo, sua presença sozinha não prova animação:
confira contexto/tarefas no trace. Faça captura fechada de 60 s com `--mode
closed --webview` sem PIDs somente se nenhum processo WebKit do app restar;
caso contrário, informe os PIDs sobreviventes. O coletor não comprova ausência
de animação; precisa do trace nativo.

Verifique redução manual, redução pelo sistema durante execução e mudança
com Painel fechado; registre o valor original da preferência do sistema e
restaure-o depois. Reabra com e sem redução e confirme que somente a decoração
permitida retorna. Registre o funcionamento observado, sem inferi-lo do DOM.

## Memória e recriação

Mantenha a decisão atual de destruir a webview ao fechar (`panel::close`),
fundamentada na medição histórica acima de aproximadamente 150 MB. Revalide PSS
em repouso aberto/reduzido, aberto/ligado e fechado após estabilização, incluindo
processos sobreviventes. Não decida esconder em vez de destruir sem evidência.

Antes de fechar, escolha outra âncora e registre contexto; reabra e confira
âncora, estado/cache com idade, preferências e comportamento offline. Distinga
o contexto prometido pela spec de seleção/rolagem transitória da webview.
Repita após reiniciar o app e via uma notificação real de Agentes: destino deve
ser Agentes com a sessão destacada, sem perder a âncora normal memorizada.
Não fabrique notificação com conteúdo sensível. Registre falhas e só altere
código com evidência; divergência da spec visual exige validação prévia.

## Entrega pendente

Preencher as seis rodadas, baselines, condições e cálculos; anexar resumo
sanitizado dos traces, latências, fluidez, ocultação, PSS e restauração.
Corrigir desvios comprovados e repetir as medições afetadas antes de encerrar
o ticket. Nesta entrega há protocolo e coletor, sem aceite de desempenho.

## Verificações desta entrega

- Coletor executado por 1 s contra um processo Python descartável: PSS lido,
  grupo WebKit vazio aceito no modo fechado e PID inexistente rejeitado sem
  resultado parcial. É smoke do instrumento, não medição do app.
- `npm run check`: zero erros e avisos; `npm test`: 212 testes aprovados;
  `npm run build`: aprovado.
- `cargo test --manifest-path src-tauri/Cargo.toml`: 271 testes unitários
  aprovados. Dois testes de sockets bloqueados pelo sandbox; reexecução dos
  três testes de `hook_mode` fora do sandbox aprovada. Total: 274 testes Rust.
