# Conteúdo e fontes do Módulo Sistema

Type: grilling
Status: resolved
Assignee: Kvojps
Blocked by: 10
Map: [kvojps-monitor v1](../map.md)

## Question

O que exatamente o Módulo Sistema mostra e de onde lê, sem privilégio: CPU (uso total, por núcleo? temperatura?), RAM (e swap?), disco (só `/`, todos os pontos de montagem, ou escolhido nas Configurações?), bateria (UPower via D-Bus; % , carregando, tempo restante; o que mostrar num desktop sem bateria)? Com que intervalo coleta, só com o Módulo visível ou sempre? Algum estado gera notificação ou marca no Indicador (disco quase cheio, bateria baixa) ou fica fora da v1? Layout já definido em [Layout e visual do Painel](10-visual-do-painel.md).

## Answer

Tudo sem privilégio, só leitura:

- **CPU**: uso total em % (`/proc/stat`) com mini‑gráfico dos últimos ~60 s; temperatura do hwmon (`k10temp` em AMD, `coretemp` em Intel), linha some se não houver sensor. Sem uso por núcleo.
- **RAM**: barra "usada / total" com usada = total − `MemAvailable` (mesmo critério do `free`/Monitor do GNOME). Swap como linha secundária pequena, só se houver swap configurado.
- **Discos**: todos os sistemas de arquivos reais montados (`/proc/self/mountinfo` + `statvfs`), excluindo tmpfs, squashfs/snap, overlay, efivarfs e `/boot*`; uma barra por disco com usado/total e livre. Sem escolha nas Configurações.
- **Bateria**: UPower via D‑Bus (`DisplayDevice`): %, estado (carregando/descarregando/carregada/na tomada) e tempo restante até esvaziar ou encher quando informado. Sem bateria (desktop), o bloco não aparece.
- **Alertas**: nenhuma notificação nem marca no Indicador na v1 (o GNOME já avisa de bateria baixa e disco cheio). No Painel, cor de atenção quando disco > 90%, bateria < 15% ou temperatura alta — alta = `temp*_crit` do sensor se existir, senão 90 °C.
- **Coleta**: só com o Módulo visível — CPU, RAM e temperatura a cada 2 s; disco a cada 30 s; bateria por sinal do UPower (sem polling). Fora da tela não coleta nada: o mini‑gráfico começa vazio ao abrir e se preenche; sem cache em disco nem idade do dado no cabeçalho (dado sempre ao vivo).
