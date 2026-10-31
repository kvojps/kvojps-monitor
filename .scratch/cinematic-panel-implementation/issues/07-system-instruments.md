# 07: Sistema com instrumentos e alertas

**What to build:** acompanhar CPU, temperatura, memória, disco e bateria na seleção visual aprovada, distinguindo alertas e componentes ausentes.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** ready-for-agent

- [x] Destacar CPU e temperatura e mostrar barras de RAM/swap/disco e bateria com contexto real, sem constantes da fixture.
- [x] Não acrescentar histórico de CPU, todos os discos/espaço livre, tempo restante da bateria ou expansores; preservar a seleção de H sem alterar coleta ou critérios de alerta do núcleo.
- [x] Tratar ausência de sensor, swap ou bateria conforme os dados reais, sem simular valores. Alertas de disco/temperatura/bateria têm identificação textual ou ícone além da cor.
- [x] Mostrar idade e estado de coleta; falha preserva cache utilizável e ausência de dados oferece explicação no instrumento afetado.
- [x] Testar snapshots de desktop/notebook, cheio, vazio, antigo, erro, alertas e componentes ausentes, com valores e instrumentos acessíveis.
- [x] Confirmar pelo seam existente pausa/retomada das coletas condicionadas à visibilidade e preservação das regras de alerta, sem criar notificações ou coletas decorativas.

## Registro da implementação (2026-10-31)

- **Módulo Sistema no Painel:** `SystemModule` volta à estação Capacidade com a seleção de H. Antes do primeiro snapshot: “Sem leitura do sistema ainda”.
- **Destaque:** CPU em 46 px com “%” e temperatura em 25 px, cada um como `group` nomeado. Sem duas amostras, a CPU diz “Medindo…”, nunca 0%. Sem sensor, a temperatura diz “Sem sensor”.
- **Barras:** RAM, swap e “Disco /” em `meter`, com `aria-valuetext` “usado / total · %”. O percentual do disco é usado sobre total, coerente com o par exibido (como em H); quem decide o alerta continua sendo o núcleo, pela regra do df. Sem swap: “Sem swap configurada” sob a RAM.
- **Discos:** só o disco `/` (ou o primeiro, se `/` faltar) vira barra. Outro disco com alerta do núcleo aparece numa linha “⚠ Pouco espaço em /mnt/data” (`role="alert"`), preservando o alerta sem listar todos os discos nem o espaço livre.
- **Bateria:** linha inferior com percentual, barra (`aria-valuetext` “100% · carregada”) e estado vindo do UPower (“carregada” só quando o dado diz). Sem bateria ou sem UPower: “Sem bateria detectada”. Tempo restante não aparece.
- **Alertas:** temperatura (“⚠ Temperatura alta”), bateria (“⚠ Bateria baixa”) e disco têm texto e ícone, além da cor `--warn`.
- **Idade e coleta:** o cabeçalho segue com “atualizado HH:MM”. Sem snapshot novo há mais de 10 s (cinco amostras de CPU), o módulo mantém a última leitura esmaecida e mostra “◷ Leitura antiga · sem atualização há …” (`status`). É o estado de falha: o núcleo não publica quando a leitura de memória falha, então o cache em memória continua utilizável. O Sistema segue sem cache em disco.
- **Fixtures do núcleo:** `missing` (sem sensor, swap e bateria) e `starting` (uma amostra de CPU) entram em `ui_snapshot_fixtures_come_from_core`. Os estados vazio e antigo usam `null` e relógio controlado.
- **Removido da UI:** gráfico de histórico de CPU, lista de discos, espaço livre e tempo restante, junto com os textos e formatadores (`bytes`, `duration`) que ficaram sem uso. O núcleo continua coletando o histórico; coleta e critérios não mudaram.
- **Confirmado pelo seam existente:** `hidden_module_collects_nothing`, `hiding_module_stops_collection`, `reopening_module_restarts_empty_history` e `resume_discards_cpu_baseline_and_collects_without_waiting` cobrem pausa e retomada. Os testes de limite (90 °C ou crítico do sensor, disco > 90%, bateria < 15% descarregando) seguem iguais. O módulo não cria notificações nem coletas novas.
- **Pendente (usuário):** conferência visual nativa nas três larguras e nos temas escuro/claro (ticket 10).
- **Decisões para o usuário (revisão de spec):**
  - *Falha × antigo:* o núcleo não publica quando a leitura falha, então a UI não distingue falha de leitura antiga e mostra “Leitura antiga” nos dois casos. Separar exigiria um campo de falha no snapshot do Sistema (formato do núcleo, sem mudar coleta nem critérios).
  - *Linha de alerta de disco fora de `/`:* elemento ausente em H, acrescentado para preservar o alerta de disco sem listar todos os discos.
  - *Pausa por visibilidade:* ao voltar à área visível depois de mais de 10 s, “Leitura antiga” aparece até o snapshot imediato da retomada (menos de 1 s), porque a UI não sabe se a coleta estava pausada.
