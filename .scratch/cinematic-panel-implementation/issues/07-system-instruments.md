# 07: Sistema com instrumentos e alertas

**What to build:** acompanhar CPU, temperatura, memória, disco e bateria na seleção visual aprovada, distinguindo alertas e componentes ausentes.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** ready-for-agent

- [ ] Destacar CPU e temperatura e mostrar barras de RAM/swap/disco e bateria com contexto real, sem constantes da fixture.
- [ ] Não acrescentar histórico de CPU, todos os discos/espaço livre, tempo restante da bateria ou expansores; preservar a seleção de H sem alterar coleta ou critérios de alerta do núcleo.
- [ ] Tratar ausência de sensor, swap ou bateria conforme os dados reais, sem simular valores. Alertas de disco/temperatura/bateria têm identificação textual ou ícone além da cor.
- [ ] Mostrar idade e estado de coleta; falha preserva cache utilizável e ausência de dados oferece explicação no instrumento afetado.
- [ ] Testar snapshots de desktop/notebook, cheio, vazio, antigo, erro, alertas e componentes ausentes, com valores e instrumentos acessíveis.
- [ ] Confirmar pelo seam existente pausa/retomada das coletas condicionadas à visibilidade e preservação das regras de alerta, sem criar notificações ou coletas decorativas.
