# Como integrar a assinatura KVOJPS ao visual atual?

Type: prototype
Status: resolved
Labels: wayfinder:prototype
Parent: [Aprimorar a identidade tech do Painel](../map.md)

## Question

Qual tratamento tipográfico, escala e posição fazem KVOJPS aparecer como identidade tech/futurista sem competir com as métricas nem mudar a composição B? Comparar variações no contexto do Painel atual, incluindo tema claro e largura reduzida. Validar com o usuário a opção escolhida e a revisão da regra do topo esquerdo vazio.

## Comments

- 01/11/2026: protótipo descartável em [IdentityPrototype.svelte](../../../src/lib/identity-prototype/IdentityPrototype.svelte), montando o Painel existente com fixtures locais, sem núcleo nativo. Rodar `npm run prototype:identity`; abrir `http://127.0.0.1:1422/identity-prototype.html?variant=A`. Variações: A — Wordmark técnico; B — Selo de bordo; C — Assinatura na faixa. Controles de tema claro e largura de 760 px; padrão de 1100 × 700. Aguardando avaliação humana; nenhuma revisão da spec aprovada. O protótipo será capturado em branch local `prototype/*` após a escolha; não fazer push. A composição fica preservada conforme as Notes do mapa, em vez de variar a grade como sugere o fluxo genérico de prototype.

## Answer

O usuário escolheu **C — Assinatura na faixa** e confirmou “Fechado”: KVOJPS integrada à faixa superior, mantendo livre a linha de ações, com o mesmo tratamento no tema claro e na largura reduzida.

- Preservar a composição B, a hierarquia das métricas e as ações existentes. A marca é identidade visual estática, sem significado de estado ou atividade.
- Referência visual: tipografia monoespaçada de 11 px, peso 600, espaçamento de 0,2 em; recorte com fundo do Painel sobre a faixa, inclinação de −12° no recorte e compensação de 12° no texto; cor de destaque do tema. São parâmetros do protótipo escolhido, sujeitos à validação de contraste e encaixe na implementação.
- A proposta para a spec é permitir a assinatura integrada à faixa superior, preservando o vazio à esquerda na linha de ações. A fonte da verdade será revista na consolidação do mapa; esta resolução não autoriza implementar no app.
- Protótipo completo preservado na branch **local** `prototype/panel-identity`, arquivos `identity-prototype.html` e `src/lib/identity-prototype/`; executar `npm run prototype:identity` nessa branch e abrir `/identity-prototype.html?variant=C`. Sem push.
- Validação do protótipo: `npm run check` sem erros ou avisos; montagem com fixtures e renderização no Chromium confirmadas. A escolha visual foi validada pelo usuário.

Não surgiram novas decisões para este ticket. A próxima decisão da fronteira é **Como mostrar operações reais com mensagens de bordo?**.
