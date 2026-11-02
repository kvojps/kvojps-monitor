# Como mostrar operações reais com mensagens de bordo?

Type: prototype
Status: resolved
Labels: wayfinder:prototype
Parent: [Aprimorar a identidade tech do Painel](../map.md)

## Question

Onde e por quanto tempo apresentar mensagens escritas de inspiração Star Trek, mantendo claros a fonte e o resultado real? Comparar faixa compacta com indicação local e validar atualização simultânea, operação muito rápida, pausa sem rede, falha e repouso. Decidir se há histórico visível, como evitar ruído para leitores de tela e como tratar fontes contínuas sem inventar etapas ou progresso.

Exemplos iniciais para discussão, não textos finais: “Estabelecendo enlace com GitHub…”, “Sincronizando registros do Trello…”, “Telemetria atualizada.” Falhas e ações necessárias devem continuar explícitas em pt-BR. Não incluir conteúdo de commits, cards, prompts ou credenciais no registro operacional.

## Comments

- 01/11/2026: protótipo descartável em [OperationsPrototype.svelte](../../../src/lib/operations-prototype/OperationsPrototype.svelte). Rodar `npm run prototype:operations` e abrir `http://127.0.0.1:1423/operations-prototype.html?variant=A`. A — Faixa de bordo; B — Indicação por Módulo; C — Faixa + indicação local, com registro expansível dos metadados do cenário atual. Fixtures locais e seleção manual de seis cenários; não conecta eventos reais nem implementa duração de mensagens. Tema claro e largura reduzida disponíveis. A assinatura aprovada é mantida como referência. Aguardando avaliação humana de posição, histórico, duração e política de anúncios; ticket permanece claimed. Não houve revisão da spec. Capturar em branch local `prototype/*` após validação, sem push.

- 01/11/2026: o usuário escolheu **A — Faixa de bordo** na comparação visual. Posição escolhida; duração, histórico, simultaneidade e política de anúncios ainda aguardam validação. Esta escolha substitui a recomendação inicial de combinar faixa e indicação local; os estados e ações canônicos dos Módulos continuam sendo preservados.

## Answer

O usuário escolheu **A — Faixa de bordo** e confirmou as três regras de comportamento com “sim às 3”.

- Apresentar mensagens operacionais na faixa compacta global do protótipo A, preservando a composição B e a assinatura aprovada. Preservar os estados, detalhes de falhas e ações canônicos nos Módulos.
- Manter a mensagem enquanto a operação estiver em andamento. Após resultado real, exibi-lo por 4 segundos e voltar ao repouso. Operações rápidas podem mostrar diretamente o resultado, sem duração artificial ou etapas inventadas. O temporizador controla apenas a permanência visual, nunca a conclusão da operação.
- Sem histórico visível. Agrupar operações simultâneas por fonte, sem carrossel; falhas e pausas têm prioridade na faixa. A prioridade não remove detalhes nem ações do Módulo correspondente.
- Rede, Sistema e Agentes comunicam mudanças relevantes, sem mensagem a cada amostra. Acompanhamento contínuo não implica uma operação que termina nem conclusão inferida por tempo.
- Leitores de tela recebem anúncios agrupados e não interruptivos; não anunciar cada amostra. Fonte e resultado permanecem explícitos em pt-BR. Sem áudio; apenas metadados, sem credenciais ou conteúdo de prompts, respostas, commits ou cards.
- Protótipo preservado na branch **local** `prototype/panel-operations`, commit `0c5de3cbc9388b20f0479d88ebaa232215c92a6d`. Nessa branch, executar `npm run prototype:operations` e abrir `/operations-prototype.html?variant=A` na porta 1423. Sem push. Os arquivos descartáveis foram retirados da main.
- Validação: `npm run check` sem erros ou avisos. O protótipo compara posição e cenários por seleção manual; não valida integração com eventos reais, temporização ou política final de anúncios. Esses critérios deverão ser verificados na implementação.

A revisão da spec fica para a consolidação do mapa; esta resolução não implementa a proposta no app. A identificação dos eventos reais e das mudanças relevantes por fonte fica no ticket **Quais eventos reais alimentam a Faixa de bordo?**.
