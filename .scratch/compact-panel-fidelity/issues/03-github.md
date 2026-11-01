# 03: GitHub fiel ao B

Type: task
Status: resolved
Blocked by: 01, 02

**What to build:** reescrever o resumo do GitHub a partir do protótipo, com dados reais, até ficar igual ao B no app nativo.

- [x] Cabeçalho “GITHUB” em maiúsculas com ↻/⚙ por ícone; sem subtítulo de fonte/horário.
- [x] Total anual com “contribuições no ano”, mapa de oito semanas com “Últimas 8 semanas”, barras de 14 dias com “Atividade · últimos 14 dias”, “Repo em destaque · N commits / 7 dias”, nome do repo e título do último commit. Sem textos extras (privadas, pico, soma de 8 semanas).
- [x] Falha, offline, credencial ausente/inválida e antigo como linha condicional no corpo, no vocabulário do B.
- [x] Testes de UI por fixtures; sem rolagem em 1100 × 700 e 1000 × 700 com as fixtures do B.
- [x] Captura no app nativo lado a lado com `compact-B.png` e **aprovação do usuário** registrada aqui.

## Comments

Origem: [mapa de fidelidade](../map.md), entrevista de 2026-10-31.

2026-11-01 (implementação): `GithubModule.svelte` reescrito com a marcação e o CSS do protótipo (cabeçalho como o do cartão IA e Agentes, `strong` 2,8em, mapa de 8 semanas por opacidade, barras `--band-steel`, bloco `.featured`). Saíram o subtítulo “Fonte · atualizado”, “inclui N privadas”, soma de 8 semanas, pico de 14 dias e “contribuições em AAAA” (agora “contribuições no ano”). Estados condicionais viram linhas `small` no topo do corpo: “⚠” âmbar para falha, credencial inválida (com “Reconectar”) e GitHub não conectado; texto neutro para pausa, “Atualizando…”, “Atualizado há X” e sem leitura. O mapa e as barras mantêm nomes acessíveis por dia; dias futuros continuam fora. Testes de UI atualizados, incluindo “nada além do B no estado normal” (244 passam), `svelte-check` sem erros. Pendente: medir rolagem em 1100 × 700 e 1000 × 700 e a captura nativa com aprovação, que dependem do usuário enquanto “Captura nativa para agentes” estiver aberto.

## Answer

Aprovado pelo usuário em 2026-11-01 após conferência no app nativo (`npm run dev:fixtures`) lado a lado com `compact-B.png`.

- Resumo do GitHub reescrito a partir do protótipo: “GITHUB” com ↻/⚙ redondos, total com “contribuições no ano”, mapa de 8 semanas por opacidade, barras de 14 dias, “Repo em destaque · N commits / 7 dias”, nome e título do último commit.
- Fora do estado normal: subtítulo de fonte/horário, privadas, soma de 8 semanas, pico de 14 dias e o ano no rótulo do total.
- Estados condicionais como linhas `small` no topo do corpo: ⚠ âmbar para falha, credencial inválida (com “Reconectar”) e não conectado; texto neutro para pausa, atualização, idade e sem leitura.
- Sem divergências da spec.
