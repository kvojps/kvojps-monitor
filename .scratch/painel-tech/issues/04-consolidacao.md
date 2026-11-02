# Como consolidar as melhorias aprovadas na spec da v1?

Type: grilling
Status: resolved
Labels: wayfinder:grilling
Blocked by: 01, 02, 03, 05
Parent: [Aprimorar a identidade tech do Painel](../map.md)

## Question

Também depende de [Quais eventos reais alimentam a Faixa de bordo?](05-eventos-de-bordo.md), para incorporar o contrato operacional sem pressupor eventos inexistentes.

As escolhas de assinatura, comunicação operacional e movimento funcionam juntas, preservando o Painel atual? Validar a composição integrada e definir a proposta final de alteração da spec, critérios de aceite e limites de implementação, incluindo acessibilidade, desempenho, operações concorrentes e dados indisponíveis. Registrar as divergências aprovadas neste ticket antes de atualizar a spec. Identificar somente então o trabalho de implementação necessário.

Depende de [Como integrar a assinatura KVOJPS ao visual atual?](01-assinatura-kvojps.md), [Como mostrar operações reais com mensagens de bordo?](02-mensagens-de-bordo.md) e [Que movimentos tornam o Painel mais expressivo sem atrapalhar a leitura?](03-movimento.md).

## Answer

Em 01/11/2026, o usuário confirmou com “Concordo com tudo” a rodada final de consolidação, preservando as escolhas dos tickets anteriores e aprovando:

- **Concorrência:** limitar a Faixa de bordo ao espaço disponível, mostrar as mensagens prioritárias e resumir as demais com texto como “+3 Módulos”, sem expandir a grade ou usar carrossel. Falha, pausa e atenção vêm primeiro; detalhes e ações continuam nos Módulos.
- **Indisponibilidade:** nunca apresentar ausência como sucesso ou zero; explicação e ação permanecem no Módulo enquanto a condição persistir, mesmo depois dos 4 segundos do resultado na faixa. A reabertura estabelece referência atual, sem anunciar problemas antigos como eventos novos.
- **Aceite integrado:** exigir validação conjunta de assinatura, faixa e movimento em temas claro/escuro, largura reduzida, teclado, leitor de tela, redução de movimento e pausa em Configurações. Preservar as metas de resposta ≤ 100 ms, 60 fps e CPU adicional média ≤ 2 pontos percentuais de um núcleo. Desempenho nativo continua por comprovar.

As divergências aprovadas em relação à spec vigente são: assinatura integrada à faixa superior; Faixa de bordo como exceção à regra de não acrescentar elementos ao B; órbitas de 48 para 32 segundos, brilho de borda de 4 para 10 segundos por trecho e repertório de opacidade/entrada/interação aprovado em **Que movimentos tornam o Painel mais expressivo sem atrapalhar a leitura?**. Linha de ações livre, grade B e estados canônicos permanecem como limites.

A [proposta de revisão da spec](../proposta-revisao-spec.md) contém o texto de integração, critérios de aceite e encaminhamento do trabalho de implementação. Os parâmetros visuais e a seleção de eventos continuam tendo suas decisões de origem nos tickets vinculados; a proposta é um artefato de consolidação.

Não houve alteração da spec ou do app, nem validação nativa da composição integrada. As aprovações visuais anteriores vieram dos protótipos separados; a composição conjunta deverá passar pelos critérios de aceite. Não se identificaram decisões adicionais para chegar ao destino. Ajustes de acabamento restantes são verificações de implementação dentro desses limites; divergências precisam voltar ao usuário.
