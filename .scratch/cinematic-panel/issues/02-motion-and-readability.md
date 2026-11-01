# Validar linguagem visual, movimento e leitura no trabalho

Type: prototype
Status: resolved
Map: [Painel cinematográfico para trabalho](../map.md)
Blocked by: 01

## Question

Quais regras de cor, tipografia, brilho, geometria, instrumentos e animação tornam a composição aprovada intensamente cinematográfica sem prejudicar o trabalho?

Evoluir o protótipo e validar com o usuário estados normais, alertas, falhas, dados ausentes ou desatualizados, listas densas e Configurações. Definir o que é decoração e o que representa dados, movimento contínuo, redução de animações, comportamento com Painel oculto e critérios mensuráveis de fluidez e consumo a verificar na implementação. Resolver explicitamente como manter o suporte atual ao tema claro e ao tema do sistema diante da direção escura acordada.

## Comments

### Laboratório de estados e movimento — aguardando avaliação

Protótipo evoluído sobre **Estações de comando**, preservando a composição aprovada. Execute `npm run prototype:cinematic` e abra `/cinematic-prototype.html?variant=H`.

- Seletor de cenários: normal, alerta, falha, sem dados, desatualizado e lista densa. Cenários são simulações explicitamente identificadas; a lista densa repete uma sessão da fixture para avaliar leitura. Não representa novos dados reais.
- Falha e ausência de dados removem instrumentos e oferecem ação simulada; desatualizado preserva o snapshot com aviso textual.
- Temas escuro, claro e sistema disponíveis para comparação. Tema claro ainda exige avaliação visual, incluindo Configurações.
- Traços decorativos dos anéis giram em 48 s; arcos de medição permanecem fixos. Brilho decorativo varia em 4 s. Redução manual, preferência do sistema e página oculta interrompem animações; valores não oscilam artificialmente.
- `npm run check`: zero erros e avisos. Sem medição de consumo ou fluidez realizada nesta rodada.

Proposta para discussão: manter temas claro e sistema; nunca piscar texto ou alertas; animações só em decoração. Na implementação, verificar ausência de trabalho de animação com Painel oculto, resposta a interações em até 100 ms e alvo de 60 fps durante animações na máquina do usuário. Orçamento de CPU ainda precisa ser acordado e medido, com baseline comparável.

Ticket permanece reivindicado: aprovação visual, critérios finais e resolução dependem da conversa com o usuário. Código descartável ainda em trabalho, sem captura final em branch nesta rodada.

## Answer

O usuário aprovou integralmente a rodada apresentada (“Concordo com tudo.”): movimento decorativo, preservação dos temas e leitura dos cenários, incluindo Configurações, sem solicitar ajustes.

- Manter a composição **Estações de comando** e sua linguagem LCARS: preto profundo, ciano/azul, detalhes âmbar e lavanda, geometria técnica e faixas segmentadas. A tipografia, o brilho e a densidade desta rodada são a referência visual aprovada.
- Movimento contínuo restrito à decoração: traços dos anéis giram lentamente (48 s por volta), com brilho suave (ciclo de 4 s). Arcos que representam medições, valores e textos permanecem imóveis; alertas não piscam. Decoração não representa coleta nem atividade real.
- Oferecer redução manual de animações e respeitar a preferência de redução do sistema. Interromper animações com Painel oculto; não manter trabalho de animação em segundo plano.
- Preservar os temas escuro, claro e sistema; o tema do sistema acompanha a preferência do ambiente.
- Estados de atenção e falha têm aviso textual, sem depender só da cor. Falha e ausência de dados substituem os instrumentos por explicação e ação pertinente; leitura desatualizada mantém o último snapshot explicitamente identificado. Listas densas permitem rolagem. Configurações continua separada do Painel completo.
- Os exemplos são demonstrações identificadas, baseadas em fixtures; a lista densa é uma repetição sintética para avaliação. Não são novas medições.

Fonte: branch local `prototype/cinematic-panel-composition` (apagada em 2026-11-01), arquivos em `src/lib/cinematic-prototype/`. Execute `npm run prototype:cinematic` e abra `/cinematic-prototype.html?variant=H`. Sem push; sem alteração na UI de produção.

Validação técnica já executada: `npm run check`, zero erros e avisos. A aprovação é visual; consumo e fluidez ainda não foram medidos. A consolidação deve fechar os critérios de aceitação propostos (resposta em até 100 ms, alvo de 60 fps e ausência de animação oculta) e definir orçamento de CPU com baseline, sem tratá-los como resultados comprovados.
