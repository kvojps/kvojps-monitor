# 27: Integrar assinatura e Faixa de bordo às atualizações do GitHub

**What to build:** mostrar a assinatura KVOJPS e uma Faixa de bordo compacta no Painel, com mensagens ligadas às atualizações reais do GitHub, preservando métricas, estados e ações dos seis Módulos. Esta é a primeira fatia completa do contrato operacional até a UI.

**Blocked by:** 26 — Adotar a revisão aprovada do visual do Painel.

**Status:** done

- [x] Integrar a assinatura estática na faixa superior conforme C — Assinatura na faixa e as mensagens conforme A — Faixa de bordo, preservando grade B e lado esquerdo livre na linha de ações.
- [x] Publicar e consumir apenas os sinais operacionais necessários, distinguindo fonte, origem (usuário, automática ou retomada), identidade e resultado real; não usar updatedAt como identidade nem refreshing=false como prova de sucesso.
- [x] Mostrar início de atualização solicitada e retomada após pausa; sucesso automático permanece silencioso, enquanto falha e recuperação reais aparecem. Operação rápida pode mostrar diretamente o resultado; cancelamento, quando existente, é distinto de sucesso.
- [x] Manter indicação enquanto a operação estiver realmente em andamento e resultado por 4 segundos; relógio injetável controla somente permanência visual. Repetição de snapshot não cria operação e reabertura estabelece referência atual sem reapresentar eventos antigos.
- [x] Agrupar por fonte em um resumo por Módulo, priorizar falha, pausa e atenção antes de atividade e sucesso, e resumir excedentes como “+3 Módulos”. Sem carrossel, histórico ou expansão da grade; detalhes e ações permanecem acessíveis nos Módulos.
- [x] Manter indisponibilidade explícita, sem zeros ou sucessos fabricados, e explicação e ação no Módulo enquanto a condição persistir, mesmo após sumir da faixa.
- [x] Oferecer anúncios agrupados não interruptivos para resultados solicitados, falhas e recuperações, sem roubar foco; inícios automáticos, inclusive retomadas visuais, são silenciosos para leitores de tela. Não alterar notificações do sistema.
- [x] Validar composição nos temas claro, escuro e do sistema, largura reduzida, 1100 × 700, 1000 × 700 e texto ampliado; verificar teclado, ordem de leitura e contraste WCAG AA.
- [x] Cobrir com fixtures de snapshots e transições os seis Módulos simultâneos, prioridades, operação rápida/longa, falha, pausa, recuperação, cancelamento, repetição e reabertura. Apenas GitHub precisa de integração real neste ticket; os demais são cenários de contrato para as próximas fatias.
- [x] Toda E/S do núcleo passa pelas portas existentes, com fixtures reais e relógio falso. Mensagens só contêm fonte, evento e estado operacional; dados sensíveis sentinela comprovam sanitização de mensagens, logs, cache e notificações, sem histórico operacional persistido. Preservar testes dos demais guardrails de privacidade.

## Notas da implementação

- Assinatura C integrada à faixa superior e Faixa de bordo A no rodapé, mantendo a grade B e a linha de ações livre à esquerda.
- GitHub publica `operation` transitória nos snapshots: identidade sequencial independente de `updatedAt`, origem (`user`, `automatic`, `resume`) e estado real. Pausa e recuperação usam o ciclo de vida e o resultado da consulta; não há histórico persistido. Nenhuma nova E/S ou notificação do sistema.
- A UI estabelece a referência na abertura, deduplica transições por identidade/estado, mantém atividade até resultado real e usa relógio injetável somente para os 4 segundos do resultado. Prioriza falha, pausa e atenção, limita os resumos ao espaço e anuncia resultados agrupados em região `polite`. Fixtures exercitam os seis Módulos; somente GitHub produz sinais reais nesta fatia.
- O Módulo GitHub mantém os estados, métricas e ações existentes, incluindo explicação e configuração quando a credencial fica indisponível com atividade em cache. Falhas transitórias continuam seguindo a política existente; a linha de falha persistente aparece após três falhas.
- Validação: `npm run check` sem erros ou avisos; 335 testes de UI, 283 testes unitários Rust e 3 testes de integração passaram. Os testes de integração de hooks precisaram executar fora do sandbox para criar sockets locais. `npm run build` e `git diff --check` passaram.
- Capturas nativas em 1100 × 700 escuro e 1000 × 700 claro sem rolagem; 760 × 700 com tema do sistema usa apenas a rolagem vertical prevista. WebKitGTK com texto em 150%, nos tamanhos 1000 × 700 e 760 × 700, mantém a faixa compacta, sem transbordamento horizontal, com fonte prioritária e “+5 Módulos”. Contraste da assinatura: 13,44:1 escuro e 5,94:1 claro; texto da faixa: 16,66:1 e 12,13:1. Teclado e preservação de foco cobertos na UI; sem novos controles na faixa.
- Revisão em dois eixos (`code-review`) contra `0696792ae2d03c1006fbe68db6d1fa895b1c3adc`: Standards e Spec sem achados restantes. Corrigido anúncio acessível de operações distintas com resultado textual idêntico, com teste de regressão; origem e estado no Rust agora usam enums serializáveis. Suítes completas, typecheck e build repetidos após essas correções.
