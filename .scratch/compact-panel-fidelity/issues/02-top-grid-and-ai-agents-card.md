# 02: Topo, grade B e cartão IA e Agentes

Type: task
Status: resolved
Blocked by: 01

**What to build:** reescrever a moldura do Painel e o cartão “IA e Agentes” a partir de `CompactPrototype.svelte`, com dados reais, até ficarem iguais ao B no app nativo.

- [x] Topo como no B: três barras (lilás, ciano, âmbar, proporções 2/5/1) e linha com “⚙ Configurações” em texto à direita; lado esquerdo vazio (sem “Demonstração · leituras independentes”). Configurações mantém formulários, primeira execução, retorno e restauração de foco/Âncora.
- [x] Grade B: colunas 1,2/1/1; o cartão IA e Agentes ocupa a coluna esquerda inteira, independente das linhas de GitHub/Trello e Rede/Sistema. Remover a barra lilás lateral, selos numerados e entalhes de borda. Bordas superiores coloridas por Módulo e brilho de borda como no protótipo.
- [x] Cabeçalho como no B: “IA E AGENTES” em maiúsculas e um único par ↻/⚙, com dica por ponteiro/foco. ↻ atualiza as duas fontes; cada uma mostra a própria falha no corpo, e uma não apaga a outra. ⚙ abre Configurações na seção de IA, com a de Agentes logo abaixo.
- [x] IA como no B: dois anéis lado a lado (Codex ciano, Claude lilás, sempre), nome, “5 h · reinicia HH:MM”, barra “Semana N%”, “Reinício DD/MM/AAAA”, “Leitura HH:MM”. Sem textos extras (“app-server · atualizado”, “reinicia em X h Y min”, “uso alto” no anel). Uso alto, reinício sem leitura, falha, credencial e sem leitura viram linha condicional no corpo, no vocabulário do B.
- [x] Agentes como no B: anel por Estado da sessão com a contagem no centro e rótulo **“Em andamento”** (não “Trabalhando”), “N sessões · inclui encerradas”, legenda com pontos coloridos e contagens, dica por ponteiro/foco com projeto/nome, alerta em caixa “⚠ N aguardando você · Ver sessões ↗” no rodapé do cartão. Sem a linha “● AGUARDANDO VOCÊ 01” no topo. Anel zero/indisponível conforme a spec compacta.
- [x] Detalhe Sessões de agente com o layout do protótipo (nome em negrito, linha provedor · estado, linha `small`) acrescido de duração e motivo exigidos pela v1; foco, Escape e sessão direcionada por notificação preservados.
- [x] Movimento só nos anéis, barra do topo e brilho de borda; redução manual/sistema e pausa com Painel oculto.
- [ ] Testes de UI por fixtures de snapshot para o estado B e os estados condicionais; sem rolagem do Painel em 1100 × 700 e 1000 × 700 com as fixtures do B.
- [ ] Captura no app nativo lado a lado com `compact-B.png` e **aprovação do usuário** registrada aqui.

## Comments

Origem: [mapa de fidelidade](../map.md), decisões Q3, Q5–Q9, Q15–Q19 da entrevista de 2026-10-31. Os Módulos IA e Agentes continuam independentes em dados, snapshots, estados e Âncora; o cartão único é só apresentação.

## Answer

Implementado em 2026-11-01; as verificações nativas pendentes viraram tickets próprios.

- **Feito:** topo B (três barras, “⚙ Configurações” à direita, lado esquerdo vazio), grade B com o cartão IA e Agentes na coluna esquerda, sem barra lateral nem selos numerados (removidos de todos os Módulos), bordas superiores por Módulo e brilho de borda. Cabeçalho único “IA E AGENTES” com ↻/⚙; IA e Agentes continuam seções e Módulos separados (snapshot, visibilidade, Âncora). IA e Agentes reescritos a partir de `CompactPrototype.svelte`; estados condicionais como linha `small` colorida. Movimento só em órbitas dos anéis, barra do topo e brilho de borda (`src/motion.css`). Testes de UI atualizados (243 passam), `svelte-check` sem erros.
- **Divergência aprovada pelo usuário:** ↻ atualiza só a IA. Agentes não tem comando de atualização; o coletor já lê a cada 2 s e recebe hooks na hora, então um ↻ de Agentes não mudaria nada visível. A dica diz “Consultar os limites de uso de Claude e Codex”.
- **Correção do ticket 01:** relógio do modo de fixtures movido para `1793401200000` (30/10/2026 20:00, publicação de `claude-usage`). Às 09:00 de 31/10 a janela de 5 h do Claude já tinha reiniciado e o app nativo mostraria “Reiniciado” onde o B mostra “reinicia 23:30”.
- **Idade da leitura:** só “Leitura HH:MM”; com a data quando a leitura tem mais de um dia. Falha/offline continuam com idade na própria linha.
- **Pendente (movido):** captura do agente no GNOME, conferência nativa com aprovação, e se “⚠ Uso alto” aparece no estado B. Ver [Captura nativa para agentes](08-agent-native-capture.md), [Conferência nativa do topo e cartão IA e Agentes](09-native-review-top-and-ai-agents.md) e [Uso alto no estado B](10-high-usage-in-state-b.md).
