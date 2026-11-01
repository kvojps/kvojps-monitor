# Mapa: Fidelidade do Painel compacto ao protótipo B

Label: wayfinder:map
Status: open

## Destination

O Painel compacto de produção, visto no app nativo (Tauri/WebKitGTK no GNOME/Wayland), igual em design e conteúdo ao protótipo B — Núcleo lateral (`src/lib/compact-prototype/CompactPrototype.svelte`, `.scratch/compact-panel/assets/compact-B.png`), com cada Módulo aprovado pelo usuário em comparação lado a lado.

## Notes

- Origem: entrevista de 2026-10-31 após o usuário ver no app nativo que “todas as áreas desenvolvidas estão diferentes do que planejamos, na maioria com mais coisas”. Implementação anterior: [compact-panel-implementation](../compact-panel-implementation/issues/).
- **Causa:** a implementação partiu dos Módulos do Painel cinematográfico e os podou até caber, em vez de usar o protótipo como molde. O resíduo cinematográfico explica os excessos.
- **Regra-mãe:** “Temos que ser fiéis ao design e conteúdo do protótipo.” O protótipo B é a fonte da verdade visual e de conteúdo. A [spec compacta](../compact-panel/spec.md) e a [spec da v1](../kvojps-monitor/spec.md) mandam só em comportamento, dados, estados não demonstrados pelo B e guardrails.
- **Abordagem:** reescrever o resumo de cada Módulo com a marcação e o CSS do protótipo como molde, ligando dados reais e estados da produção. Não podar os componentes atuais.
- **No estado normal, nada que o B não mostra.** Saem: subtítulos “Fonte · atualizado HH:MM”, selos numerados (02/04/05/06), barra lateral lilás, entalhes nas bordas, caixas de Qualidade/Segurança, caixa de capacidade, botões textuais do Trello, textos extras de GitHub e IA, títulos em caixa mista. Idade da leitura só onde o B mostra (“Leitura HH:MM” na IA; “Teste · HH:MM” na Rede).
- **Exigências da v1 ausentes no B** (VPN ativa, motivos de Atenção/Risco, avisos do teste de velocidade, alertas do Sistema, uso alto da IA, falha/offline/credencial/antigo/sem leitura) aparecem **só quando a condição ocorre**, como linha no corpo do cartão no vocabulário do B (texto `small` colorido, como “⚠ 1 aguardando você”), sem novos elementos de cabeçalho. Anéis de IA mantêm sempre a cor do provedor (Codex ciano, Claude lilás).
- **Textos fixos do protótipo** viram dados reais no mesmo formato e posição: tipo de conexão, “Qualidade: X”, “Segurança: Y”, estado real da bateria, “Teste · HH:MM” no lugar de “Fixture · HH:MM”, cores reais das etiquetas do Trello.
- **Única troca de palavra:** o centro do anel de Agentes diz “Em andamento” (Estado da sessão do glossário), não “Trabalhando”.
- **Fora do produto:** “Demonstração · leituras independentes” (lado esquerdo da linha do topo fica vazio), rodapé de depuração, controle flutuante, nota “Detalhe ilustrativo” e retornos simulados.
- **Movimento:** só o dos elementos que existem no B (anéis, barra do topo, brilho de borda), com redução manual/sistema e pausa com Painel oculto. Descartar o movimento cinematográfico restante.
- **Aceite:** sem rolagem em 1100 × 700 **e** 1000 × 700 com as fixtures do B (IA `claude-usage`, Agentes `states`, GitHub/Trello/Rede `full`, Sistema `notebook`); comparação no app nativo, lado a lado com `compact-B.png`; cada Módulo fecha só com aprovação do usuário.
- Privacidade, credenciais, fontes, coleta, cadências, notificações e o seam único de E/S permanecem fora da revisão. Branches `prototype/*` e `research/*` ficam locais.

## Decisions so far

- Entrevista de 2026-10-31 (este mapa): todas as decisões acima confirmadas pelo usuário (“Concordo com tudo”, “sim, pode criar”).
- Divergências antes aprovadas voltam ao B: conexão na linha de status da Rede ([ticket 05 anterior](../compact-panel-implementation/issues/05-compact-network-and-applications.md)) e ícones ↻/⚙ no cabeçalho do Trello ([ticket 06 anterior](../compact-panel-implementation/issues/06-trello-summary-and-board.md)).
- “IA e Agentes” vira um único cartão visual; os Módulos continuam independentes em dados, estados e Âncora. Sem termo novo no glossário; “Trabalhando” registrado como termo evitado para Estado da sessão.

- [Modo de fixtures nativo para desenvolvimento](issues/01-native-fixture-mode.md): feature Cargo `fixtures` (`npm run dev:fixtures`, `KVOJPS_FIXTURES` por Módulo) publica os snapshots do B pelo caminho normal, sem coletores; injeção no nível do snapshot, não em portas brutas (divergência aprovada).
- [Topo, grade B e cartão IA e Agentes](issues/02-top-grid-and-ai-agents-card.md): moldura e cartão reescritos a partir do protótipo; ↻ do cartão atualiza só a IA (divergência aprovada); relógio das fixtures corrigido para 30/10 20:00; conferência nativa e aprovação movidas para tickets próprios.
- [GitHub fiel ao B](issues/03-github.md): resumo reescrito do protótipo, sem textos extras; estados condicionais como linhas `small` no topo do corpo; aprovado no app nativo.
- [Rede fiel ao B](issues/04-network.md): resumo e detalhe de aplicativos reescritos do protótipo; linha de status com dados reais; estados condicionais como linhas `small` abaixo dela; aprovado no app nativo.
- [Trello fiel ao B](issues/05-trello.md): resumo e quadro completo reescritos do protótipo; ↻/⚙ por ícone; estados condicionais como linhas `small` no topo do corpo; aprovado no app nativo.
- [Sistema fiel ao B](issues/06-system.md): resumo reescrito do protótipo; GB só sob a RAM, bateria em texto; cabeçalho de fonte/horário removido do Painel; estados condicionais como linhas `small` no topo do corpo; aprovado no app nativo.
- [Captura nativa para agentes](issues/08-agent-native-capture.md): `scripts/capture-panel.sh` pede à instância viva (só feature `fixtures`) snapshot da webview + JSON de rolagem; B sem rolagem em 1100/1000 × 700, escuro e claro; Vite frio pode tirar o CSS de Módulos (captura recusa); Painel abre em 1048 × 648 no Wayland (vai para a validação integrada).
- [Conferência nativa do topo e cartão IA e Agentes](issues/09-native-review-top-and-ai-agents.md): aprovado no app nativo; textos `small` na fonte normal em todos os Módulos (como no B); Agentes distribuído na altura do cartão; Trello sem prévia do primeiro card e nomes de lista em uma linha (divergência aprovada).

## Tickets

1. [Modo de fixtures nativo para desenvolvimento](issues/01-native-fixture-mode.md)
2. [Topo, grade B e cartão IA e Agentes](issues/02-top-grid-and-ai-agents-card.md)
3. [GitHub fiel ao B](issues/03-github.md)
4. [Rede fiel ao B](issues/04-network.md)
5. [Trello fiel ao B](issues/05-trello.md)
6. [Sistema fiel ao B](issues/06-system.md)
7. [Validação integrada nativa](issues/07-integrated-native-validation.md)
8. [Captura nativa para agentes](issues/08-agent-native-capture.md)
9. [Conferência nativa do topo e cartão IA e Agentes](issues/09-native-review-top-and-ai-agents.md)
10. [Uso alto no estado B](issues/10-high-usage-in-state-b.md)

## Not yet specified

Nenhuma decisão aberta.

## Out of scope

- Mudanças de coleta, fontes, credenciais, privacidade, notificações ou regras de negócio.
- Novos recursos além do que o protótipo B e a v1 já definem.
