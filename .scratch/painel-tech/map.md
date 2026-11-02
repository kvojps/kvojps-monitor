# Aprimorar a identidade tech do Painel

Labels: wayfinder:map

## Destination

Definir melhorias incrementais do visual atual do Painel: assinatura KVOJPS, mensagens escritas de bordo ligadas a operações reais e animações mais presentes. Encerrar com decisões validadas e uma proposta de revisão da spec pronta para implementação, preservando a composição do protótipo B.

## Notes

- Usar wayfinder, grilling e domain-modeling; nos tickets visuais, usar prototype. Este mapa planeja; não autoriza implementar as propostas no app.
- Em 01/11/2026, o usuário esclareceu: “A direção visual já é a que eu tenho só quero melhorá-la”; concordou com as demais recomendações da rodada inicial: assinatura KVOJPS, mensagens escritas sem áudio e atividade real por Módulo.
- Preservar grade, conteúdo e hierarquia atuais; futurismo elegante, tipografia técnica e brilho controlado. Propostas adicionais cabem quando contribuírem para esse aprimoramento.
- Base: [spec da v1](../kvojps-monitor/spec.md), em especial Visual do Painel. A inclusão da marca e a ampliação de movimento revisam restrições explícitas; detalhes precisam de validação antes de alterar a fonte da verdade.
- Preservar temas, contraste, teclado, redução de movimento, pausa quando oculto, privacidade e metas de desempenho. Metáforas não substituem estados canônicos nem explicações claras de falha.
- Levantamento local: o topo não mostra a marca; IA, GitHub e Trello já têm `refreshing` e mensagens locais. Agentes, Rede e Sistema não têm o mesmo contrato. Não supor progresso ou conclusão a partir de temporizadores.
- Tickets sem Status estão abertos e sem responsável; dependências seguem o tracker local. Não há necessidade identificada de pesquisa externa.

## Decisions so far

- [Como integrar a assinatura KVOJPS ao visual atual?](issues/01-assinatura-kvojps.md): aprovada C — Assinatura na faixa, com KVOJPS na faixa superior e linha de ações livre, também no tema claro e em largura reduzida.
- [Como mostrar operações reais com mensagens de bordo?](issues/02-mensagens-de-bordo.md): aprovada A — Faixa de bordo, operações reais agrupadas por fonte, resultado por 4 segundos, sem histórico e anúncios não interruptivos apenas para mudanças relevantes.

## Not yet specified

- Ajustes adicionais de acabamento que a comparação com os protótipos revelar, sem redesenhar o Painel.

## Out of scope

- Trocar a direção visual, reorganizar a grade B ou reconstruir o Painel.
- Voz audível, efeitos sonoros e abertura que atrase a leitura dos dados.
- Implementar as melhorias durante o mapeamento ou ampliar as fontes de dados do produto.
