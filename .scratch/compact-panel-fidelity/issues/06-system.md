# 06: Sistema fiel ao B

Type: task
Status: resolved
Blocked by: 01, 02

**What to build:** reescrever o resumo de Sistema a partir do protótipo, com dados reais, até ficar igual ao B no app nativo.

- [x] Cabeçalho “SISTEMA” em maiúsculas; sem subtítulo e sem selo numerado.
- [x] CPU “N %” e Temperatura com uma casa decimal (“47,4 °C”) como no B.
- [x] RAM, Swap e Disco / em **%** com barra; GB usados/total só sob a RAM, como no B.
- [x] Bateria como texto verde “▰ Bateria N% · <estado real>”, sem barra.
- [x] Ausências e alertas como linha condicional no corpo, no vocabulário do B.
- [x] Testes de UI por fixtures; sem rolagem em 1100 × 700 e 1000 × 700 com as fixtures do B.
- [x] Captura no app nativo lado a lado com `compact-B.png` e **aprovação do usuário** registrada aqui.

## Comments

Origem: [mapa de fidelidade](../map.md), decisão Q16 da entrevista de 2026-10-31.

2026-11-01 (implementação): `SystemModule.svelte` reescrito com a marcação e o CSS do protótipo: “SISTEMA” como `h2` dentro do Módulo (sem ↻/⚙, como no B), CPU “N<small>%</small>” e Temperatura “47,4<small>°C</small>” com uma casa decimal, medidores RAM/Swap/Disco / com “N%” e barra arredondada, “5,7 / 14,5 GB” só sob a RAM (agora sempre uma casa decimal, como o B; vírgula pt-BR), e “▰ Bateria N% · <Estado>” em texto verde sem barra (estados da bateria com inicial maiúscula, como “Carregada” no B). Saíram o cabeçalho do Painel com “Kernel e UPower · atualizado HH:MM” (último Módulo que o usava; `t.sources`, `t.header` e o CSS `.module-head` removidos) e a barra da bateria; todos os cartões agora têm o mesmo `padding` em `.card`. Linhas condicionais `small` no topo do corpo: “⚠ Leitura antiga · sem atualização há X”, “⚠ Temperatura alta” (valor em âmbar), “⚠ Pouco espaço em /mnt” (`role="alert"`, barra do disco raiz em âmbar quando é ele), “⚠ Bateria baixa” (linha da bateria em âmbar), “Sem swap configurada” (sem o medidor) e “Sem bateria detectada” (sem a linha da bateria). “Medindo…” e “Sem sensor” ficam no lugar do valor, como “—” na Rede. Os medidores mantêm `role="meter"` com “N% · usado / total” no valor acessível. Testes atualizados, incluindo “nada além do B no estado normal” (244 passam), `svelte-check` sem erros, `cargo test` verde. No modo de fixtures o relógio da webview (30/10 20:00) é anterior ao `updatedAt` do `notebook`, então a leitura não aparece como antiga. Pendente: rolagem em 1100 × 700 e 1000 × 700 e captura nativa com aprovação do usuário.

## Answer

Aprovado pelo usuário em 2026-11-01 após conferência no app nativo (`npm run dev:fixtures`) lado a lado com `compact-B.png`.

- Resumo de Sistema reescrito a partir do protótipo: “SISTEMA” sem subtítulo nem ↻/⚙, CPU “N %”, Temperatura com uma casa decimal, RAM/Swap/Disco / em % com barra, GB usados/total (uma casa decimal) só sob a RAM, “▰ Bateria N% · <Estado>” em texto verde sem barra.
- Fora do estado normal: cabeçalho do Painel “Kernel e UPower · atualizado HH:MM” (removido do Painel por completo) e a barra da bateria.
- Estados condicionais como linhas `small` no topo do corpo: leitura antiga, temperatura alta, pouco espaço em disco, bateria baixa, sem swap e sem bateria; “Medindo…” e “Sem sensor” no lugar do valor.
- Sem divergências da spec.
