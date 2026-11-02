# Proposta de revisão do visual do Painel

Proposta pronta para implementação, consolidada em 01/11/2026. Não substitui a spec vigente nem autoriza alterações no app. Decisões e validações vivem nos [tickets do mapa](map.md); este documento fornece o texto proposto e o encaminhamento para implementação.

## Alterações propostas à spec da v1

### Fonte da verdade e composição

Na seção **Visual do Painel → Fonte da verdade**, manter o protótipo B como referência de grade, conteúdo e hierarquia, acrescentando como exceções explícitas a assinatura KVOJPS, a Faixa de bordo e o movimento aprovados neste mapa. A regra de não acrescentar elementos ao B passa a admitir somente essas exceções e as divergências já documentadas na spec.

Em **Janela e composição**, substituir a regra do topo por:

> Sem título geral nem navegação por Módulos. Assinatura KVOJPS estática integrada à faixa superior, conforme C — Assinatura na faixa; linha de ações gerais e Configurações com lado esquerdo livre. Preservar a grade B, seus instrumentos e a hierarquia das métricas. A marca não representa estado nem atividade. Manter o tratamento nos temas claro e escuro e em largura reduzida, com contraste e encaixe validados.

Referência de tipografia e recorte: [Como integrar a assinatura KVOJPS ao visual atual?](issues/01-assinatura-kvojps.md#answer).

### Nova subseção: Faixa de bordo

> Faixa global compacta conforme A — Faixa de bordo, com mensagens escritas em pt-BR sobre operações e transições reais. Preservar estados, explicações e ações nos Módulos. Sem áudio nem histórico visível.
>
> Manter indicação de operação enquanto ela estiver realmente em andamento; mostrar o resultado real por 4 segundos. Operações rápidas podem mostrar diretamente o resultado. Temporizadores controlam somente permanência visual, nunca progresso ou conclusão. Cancelamento tem resultado próprio, distinto de sucesso.
>
> Agrupar operações por fonte, com um resumo por Módulo e sem carrossel. Prioridade: falha, pausa e atenção antes de atividade e sucesso. Limitar a faixa ao espaço disponível; mostrar mensagens prioritárias e um resumo como “+3 Módulos” para os demais, sem expandir a grade. Os detalhes e as ações de todos os Módulos continuam acessíveis.
>
> Atualizações solicitadas pelo usuário e retomadas após pausa mostram início em IA, GitHub e Trello; sucessos automáticos ficam silenciosos, falhas e recuperações reais aparecem. Distinguir o Provedor de IA consultado antes de agrupar. Incluir início e resultado reais de teste de velocidade e movimentação de cards, sem identificar cards ou listas.
>
> Em Agentes, comunicar transições para Aguardando você, Turno concluído, Com falha e Encerrada, agrupando mudanças simultâneas; Em andamento permanece no Módulo. Turno concluído não afirma sucesso. Em Rede, comunicar mudanças de conexão, VPN e categoria de Qualidade e Segurança, sem mensagens por amostra ou sondagem; não inferir troca de rede de snapshots que apenas continuam conectados. Em Sistema, comunicar entrada e saída dos alertas existentes de temperatura, disco e bateria, sem criar limites novos. Ausência normal de sensor ou bateria não é falha.
>
> Reabrir o Painel estabelece a referência atual, sem reapresentar estados existentes como eventos novos. Snapshot repetido não é nova operação; fim de `refreshing` isolado não comprova sucesso. Não inferir resultado de operação cancelada ou descartada por mudança de conexão ou suspensão.
>
> Nunca apresentar indisponibilidade como sucesso ou valor zero. Depois dos 4 segundos na faixa, manter no Módulo a explicação e a ação correspondentes enquanto a condição persistir.
>
> Anúncios acessíveis agrupados e não interruptivos para resultados solicitados pelo usuário, falhas, recuperações e mudanças relevantes acima. Inícios automáticos são silenciosos para leitores de tela, inclusive retomadas indicadas visualmente. Não anunciar amostras ou estados repetidos. Esses anúncios não alteram as notificações do sistema; alertas de Sistema continuam restritos ao Painel.
>
> Mensagens contêm apenas fonte, evento e estado operacional. Excluir credenciais e conteúdo de prompts, respostas, commits e cards; em Agentes, excluir também nome de sessão, projeto, branch e motivo textual de espera. Não persistir histórico operacional nem acrescentar conteúdo sensível a logs, cache ou notificações.

Referências: [Como mostrar operações reais com mensagens de bordo?](issues/02-mensagens-de-bordo.md#answer), [Quais eventos reais alimentam a Faixa de bordo?](issues/05-eventos-de-bordo.md#answer) e [Como consolidar as melhorias aprovadas na spec da v1?](issues/04-consolidacao.md#answer).

### Substituição da subseção Movimento

> Repertório A — Órbita contínua: anéis decorativos com 32 segundos por volta; faixa superior com variação suave de opacidade em ciclo de 6 segundos, mínimo de 55%; brilho em gradiente restrito às bordas de 2 px, intensidade inicial de 75%, percurso de 10 segundos por trecho, ida e volta em 20 segundos, easing suave e defasagens de −3/−6/−9 segundos entre posições dos cartões. Entrada das bordas em 180 ms e resposta de hover em 80 ms. Controles de intensidade e duração pertencem somente ao protótipo.
>
> Dados são imediatamente legíveis. Assinatura, arcos de medição, números, textos e alertas não se movem nem piscam. Decoração não indica atividade ou progresso; não acrescentar marcador animado de atividade na Faixa de bordo nem animação local de conclusão. Preservar foco e equivalente de interação por teclado.
>
> Redução manual ou do sistema interrompe animações e transições e remove o brilho itinerante; redução do sistema prevalece. Não executar animações com Painel oculto ou coberto por Configurações. Preservar destruição da webview ao fechar. Implementação decorativa somente em CSS, mantendo os guardrails existentes.

Referência visual: [Que movimentos tornam o Painel mais expressivo sem atrapalhar a leitura?](issues/03-movimento.md#answer). A aprovação do protótipo não comprova desempenho nativo.

## Critérios de aceite a incorporar

- Validar a composição integrada nos temas claro e escuro, tema do sistema, largura reduzida, alvos de 1100 × 700 e 1000 × 700 e texto ampliado, preservando as regras responsivas existentes. Faixa e assinatura não escondem métricas, alertas ou ações.
- Exercitar seis Módulos com mensagens simultâneas: prioridade e resumo de excedentes, sem carrossel, expansão da grade ou perda de acesso a detalhes. Verificar também operação rápida, longa, falha, pausa, recuperação e cancelamento.
- Cobrir resultado por 4 segundos com relógio injetável, ausência de conclusão fabricada, snapshots repetidos, referência inicial ao reabrir, identificação de operações/provedores e resultados descartados. Não usar `updatedAt` como identidade de operação.
- Validar indisponibilidade total e parcial sem zeros fabricados, explicações persistentes nos Módulos e independência entre fontes.
- Verificar teclado, foco, ordem de leitura, contraste WCAG AA e anúncios agrupados com leitor de tela; mensagens não roubam foco. Metáforas mantêm fonte e resultado claros.
- Verificar redução manual e do sistema, pausa com Painel oculto e em Configurações, e retomada sem eventos antigos reapresentados.
- Medir no app nativo resposta visual ≤ 100 ms, 60 fps em tela de 60 Hz, CPU adicional média da decoração ≤ 2 pontos percentuais de um núcleo, ausência de animação oculta e memória, seguindo o protocolo existente em `docs/desempenho.md`. Metas permanecem obrigatórias; protótipos não substituem essa validação.
- No núcleo, toda E/S continua pelas portas existentes, com fixtures reais e relógio falso. Na UI, usar fixtures de snapshots e transições. Cobrir explicitamente a sanitização de mensagens, logs, cache e notificações com dados sensíveis sentinela. Preservar os testes dos demais guardrails de privacidade.

## Encaminhamento de implementação

Após adoção desta proposta na fonte da verdade, abrir trabalho de implementação no tracker da v1, com esta sequência de dependências:

1. **Contrato de operações e transições:** suprir somente as lacunas identificadas em [Quais eventos reais alimentam a Faixa de bordo?](issues/05-eventos-de-bordo.md#answer), distinguindo fonte/provedor, origem da operação, identidade e resultado real. Preservar estados canônicos, cadências, coleta e notificações existentes.
2. **Composição e Faixa de bordo:** integrar assinatura e faixa à grade existente; consumir o contrato, agrupar e priorizar mensagens, aplicar permanência, sanitização e anúncios acessíveis. Depende do contrato para integração real.
3. **Movimento aprovado:** aplicar o repertório CSS, redução de movimento e pausa. Pode avançar independentemente do contrato, mas sua validação final usa a composição integrada.
4. **Validação integrada e nativa:** executar os critérios acima e registrar evidências de acessibilidade e desempenho, aproveitando a pendência de validação nativa da v1.

Não ampliar fontes de dados, privilégios, coleta em segundo plano, configurações de produto ou grade. Não implementar durante este mapa. Ajustes de encaixe e contraste são verificados na implementação; mudanças que contrariem estas escolhas voltam ao usuário antes de execução.
