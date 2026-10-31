# 03: Módulo Sistema ao vivo

**What to build:** com o Módulo Sistema visível, vejo o uso total de CPU com mini‑gráfico dos últimos ~60 s (começa vazio ao abrir) e a temperatura quando houver sensor, RAM usada/total pelo critério do `free` com swap como linha secundária se configurado, uma barra por disco real montado (usado/total e livre) e, em notebook, a bateria (%, estado, tempo restante). Cor de atenção para disco > 90%, bateria < 15% e temperatura alta, sem notificação. Nada é coletado com o Módulo invisível e não há cache em disco.

Esta fatia **define o padrão do núcleo** que as outras seguem: portas injetáveis (sistema de arquivos com raiz falsa, relógio, D-Bus), snapshot por Módulo empurrado à UI, coleta condicionada ao Módulo visível, fixtures reais gravadas nesta máquina e testes de UI por fixtures de snapshot. Ver spec, "Módulo Sistema" e "Testing Decisions".

**Blocked by:** 01 (Esqueleto: Indicador, Painel e trilho)

**Status:** feito

- [x] CPU de `/proc/stat` a cada 2 s com mini‑gráfico de ~60 s; temperatura via hwmon (`k10temp`/`coretemp`), linha some sem sensor
- [x] RAM usada = total − `MemAvailable`; swap só se configurado
- [x] Discos de `/proc/self/mountinfo` + `statvfs` a cada 30 s, excluindo tmpfs, squashfs/snap, overlay, efivarfs e `/boot*`
- [x] Bateria via UPower `DisplayDevice` por sinal; bloco oculto sem bateria
- [x] Atenção: disco > 90%, bateria < 15%, temperatura ≥ `temp*_crit` do sensor ou 90 °C
- [x] Coleta só com o Módulo visível; para ao ocultar
- [x] Testes do núcleo com raiz falsa de `/proc`/hwmon, UPower falso e relógio falso a partir de amostras reais gravadas
- [x] Testes de UI a partir de fixtures de snapshot (notebook, desktop sem bateria, sem sensor, atenção)

## Notas da implementação

- **Disco real** = origem em `/dev/*`, fora das exclusões do ticket e de `/snap/*`, uma barra por origem (subvolumes btrfs e bind mounts não duplicam). Raízes ZFS (origem `rpool/...`) ficam de fora; revisitar se alguém usar ZFS.
- **Temperatura**: lida como "≥ `temp*_crit` se o sensor informa, senão ≥ 90 °C" (um `coretemp` com crit 100 °C a 95 °C não pede atenção). A outra leitura possível é "≥ crit **ou** ≥ 90 °C"; em aberto até decisão do usuário.
- **Fixtures**: `/proc`, hwmon, `statvfs` e o UPower "carregada" foram gravados nesta máquina (notebook AMD, `k10temp`). Sem como gravar aqui, `notebook-descarregando.json` e `desktop.json` derivam da gravação real trocando só os campos usados; o layout `coretemp` é montado no teste.
- **Contrato núcleo → UI**: as fixtures de snapshot da UI (`src/lib/sistema/fixtures/`) são geradas pelo núcleo; `ATUALIZAR_FIXTURES=1 cargo test` regrava e o teste falha se divergirem.

