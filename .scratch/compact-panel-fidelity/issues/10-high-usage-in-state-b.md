# 10: Uso alto no estado B

Type: grilling
Status: resolved
Blocked by: None (12 resolved).

## Question

Com a fixture do B, o Codex está em 91 % na janela de 5 h; pela regra do mapa (“uso alto aparece só quando a condição ocorre”), o cartão mostra “⚠ Uso alto na janela de 5 h”, que `compact-B.png` não tem. A linha fica, some, ou muda de forma (limiar, texto, posição)?

## Comments

Origem: ticket 02 (2026-11-01). Hoje: limiar 85 %, linha `small` âmbar sob o provedor; anel mantém a cor do provedor; `aria-valuetext` diz “uso alto”.

Bloqueado pelo [Preenchimento vertical do cartão IA e Agentes](11-ai-agents-vertical-fill.md) a pedido do usuário (2026-11-01): a altura do conteúdo do cartão decide primeiro; a linha de uso alto se encaixa no resultado.

Desbloqueado em 2026-11-01: o [Preenchimento vertical](11-ai-agents-vertical-fill.md) decidiu por instrumentos que crescem. A linha de uso alto, se ficar, só encolhe os anéis; não abre vão.

Bloqueado pelo [Alinhamento da distribuição vertical dos Módulos](12-vertical-distribution-alignment.md) a pedido do usuário (2026-11-01): a distribuição vertical decide primeiro.

Desbloqueado em 2026-11-01: o [Alinhamento da distribuição vertical](12-vertical-distribution-alignment.md) estendeu os instrumentos que crescem aos cartões de apoio. A linha de uso alto, se ficar, só encolhe os anéis.

## Answer

Resolvido em 2026-11-01 (grilling com o usuário: “sim, concordo com as duas”).

- **A linha visível sai, em qualquer porcentagem.** Somem “⚠ Uso alto na janela de 5 h” e “⚠ Uso alto na semana”. O número no anel e a barra semanal já dizem quanto foi usado. O estado B fica igual a `compact-B.png`.
- **Por quê:** a spec da v1 não tem história de uso alto e deixa o “Alerta de Limite de uso a 80%” fora da v1. A linha veio do Painel cinematográfico ([IA em instrumentos](../../cinematic-panel-implementation/issues/02-ai-instruments.md)), onde o anel ficava âmbar a partir de 85 % e o texto evitava que o aviso dependesse só da cor. No compacto, o anel mantém sempre a cor do provedor, então o texto perdeu o motivo e ficou como resíduo cinematográfico. A nota do mapa que listava “uso alto da IA” entre as exigências da v1 ausentes no B foi corrigida.
- **O leitor de tela continua dizendo “uso alto”:** `aria-valuetext` anuncia “91% · uso alto” a partir de 85 %, no anel de 5 h e na barra semanal. Isso não aparece na tela.
- **Aplicado:** `src/lib/ai/AiModule.svelte` (linhas condicionais removidas), `src/lib/strings.json` (`highFiveHour`/`highWeekly` removidos), teste “names high usage only to assistive technology…” em `AiModule.test.ts`.
