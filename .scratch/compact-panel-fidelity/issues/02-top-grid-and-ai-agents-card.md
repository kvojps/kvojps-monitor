# 02: Topo, grade B e cartão IA e Agentes

Type: task
Status: ready-for-agent
Blocked by: 01

**What to build:** reescrever a moldura do Painel e o cartão “IA e Agentes” a partir de `CompactPrototype.svelte`, com dados reais, até ficarem iguais ao B no app nativo.

- [ ] Topo como no B: três barras (lilás, ciano, âmbar, proporções 2/5/1) e linha com “⚙ Configurações” em texto à direita; lado esquerdo vazio (sem “Demonstração · leituras independentes”). Configurações mantém formulários, primeira execução, retorno e restauração de foco/Âncora.
- [ ] Grade B: colunas 1,2/1/1; o cartão IA e Agentes ocupa a coluna esquerda inteira, independente das linhas de GitHub/Trello e Rede/Sistema. Remover a barra lilás lateral, selos numerados e entalhes de borda. Bordas superiores coloridas por Módulo e brilho de borda como no protótipo.
- [ ] Cabeçalho como no B: “IA E AGENTES” em maiúsculas e um único par ↻/⚙, com dica por ponteiro/foco. ↻ atualiza as duas fontes; cada uma mostra a própria falha no corpo, e uma não apaga a outra. ⚙ abre Configurações na seção de IA, com a de Agentes logo abaixo.
- [ ] IA como no B: dois anéis lado a lado (Codex ciano, Claude lilás, sempre), nome, “5 h · reinicia HH:MM”, barra “Semana N%”, “Reinício DD/MM/AAAA”, “Leitura HH:MM”. Sem textos extras (“app-server · atualizado”, “reinicia em X h Y min”, “uso alto” no anel). Uso alto, reinício sem leitura, falha, credencial e sem leitura viram linha condicional no corpo, no vocabulário do B.
- [ ] Agentes como no B: anel por Estado da sessão com a contagem no centro e rótulo **“Em andamento”** (não “Trabalhando”), “N sessões · inclui encerradas”, legenda com pontos coloridos e contagens, dica por ponteiro/foco com projeto/nome, alerta em caixa “⚠ N aguardando você · Ver sessões ↗” no rodapé do cartão. Sem a linha “● AGUARDANDO VOCÊ 01” no topo. Anel zero/indisponível conforme a spec compacta.
- [ ] Detalhe Sessões de agente com o layout do protótipo (nome em negrito, linha provedor · estado, linha `small`) acrescido de duração e motivo exigidos pela v1; foco, Escape e sessão direcionada por notificação preservados.
- [ ] Movimento só nos anéis, barra do topo e brilho de borda; redução manual/sistema e pausa com Painel oculto.
- [ ] Testes de UI por fixtures de snapshot para o estado B e os estados condicionais; sem rolagem do Painel em 1100 × 700 e 1000 × 700 com as fixtures do B.
- [ ] Captura no app nativo lado a lado com `compact-B.png` e **aprovação do usuário** registrada aqui.

## Comments

Origem: [mapa de fidelidade](../map.md), decisões Q3, Q5–Q9, Q15–Q19 da entrevista de 2026-10-31. Os Módulos IA e Agentes continuam independentes em dados, snapshots, estados e Âncora; o cartão único é só apresentação.
