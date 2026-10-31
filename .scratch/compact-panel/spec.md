# Especificação visual — Painel compacto

Status: aprovada pelo usuário em 2026-10-31; verificações de implementação permanecem obrigatórias.
Origem: [Painel compacto sem rolagem](map.md) e [Consolidar a especificação do Painel compacto](issues/03-compact-specification.md).

## Autoridade e referência

Este documento complementa a [especificação visual cinematográfica](../cinematic-panel/spec.md) e substitui somente as regras de apresentação delimitadas abaixo. Fontes, coleta, privacidade, credenciais, cache, notificações e regras de negócio continuam conforme a [spec da v1](../kvojps-monitor/spec.md) e o complemento vigente. Não implementar produção durante este planejamento.

Referência: **B — Núcleo lateral**, entrada `compact-prototype.html?variant=B`, código em `src/lib/compact-prototype/`, branch local `prototype/compact-panel-composition`, sem push. Execute `npm run prototype:compact`. Fixtures e ações simuladas não são fontes de dados de produção. O [registro do protótipo](composition-prototype.md) contém capturas, medições e limites observados.

O usuário escolheu B, aprovou a revisão do Trello com “Perfeito”, confirmou a janela de aproximadamente 1100 × 700 e aceitou os demais detalhes com “Se for como está no protótipo concordo com tudo”. Essa aprovação cobre a referência apresentada; não comprova testes nativos ou estados que ela não demonstrou.

## Janela e composição

- Janela de aproximadamente **1100 × 700 em dimensões lógicas**, respeitando área útil e escala do monitor. Substituir o dimensionamento de 95% da área útil. Não maximizar nem entrar em tela cheia para mostrar os resumos ou abrir detalhes.
- Sem título/subtítulo geral, navegação redundante por Módulos ou faixas de Estação. Preservar a linguagem LCARS com estrutura compacta, temas e hierarquia legível.
- Coluna esquerda com IA e Agentes agrupados visualmente; nas outras duas colunas, GitHub e Rede acima, Trello e Sistema abaixo. Usar proporções da composição B, sem fundir os Módulos, fontes ou snapshots de IA/Agentes.
- Visão principal sem rolagem no alvo de 1100 × 700, com fixtures e texto padrão. O protótipo também coube em 1000 × 700. Listas ilimitadas não precisam caber no resumo.
- Quando a área útil, texto ampliado ou conteúdo maior impedirem o alvo, permitir reorganização e rolagem, sem cortar conteúdo, reduzir automaticamente fonte ou esconder alertas. Referência do protótipo: duas colunas até 950 px e uma até 650 px. Respeitar área útil quando 1100 × 700 não couber.
- Limites observados: texto a 125% exige rolagem no alvo; nomes longos em B exigiram 11 px adicionais por causa da reserva dos controles do laboratório. São condições de teste, não autorização para prometer ausência de rolagem em qualquer conteúdo/escala.
- Configurações continua acessível no topo e como área separada com retorno ao Painel; preservar seus formulários e ações reais. Controles flutuantes do laboratório, mensagem de demonstração e dimensões de depuração não fazem parte do produto.

## Resumos e ações

- **IA:** instrumentos de Claude/Codex com uso em 5 h, reinício, barras semanais e idade/horário da leitura. Preservar independência dos provedores e as recuperações já definidas. Ações por ícone junto ao título, com nome acessível e descrição por ponteiro/foco; atualizar/configurar continuam direcionando às fontes corretas.
- **Agentes:** anel com quantidade em andamento no centro, segmentos por Estado da sessão e legenda com contagens. O denominador é o conjunto de sessões ativas mais encerradas ainda retidas, como no protótipo; manter retenção da v1. Legenda identifica os cinco estados e permite consultar projeto/nome por ponteiro e foco. “Aguardando você” permanece como alerta direto e acionável.
- **GitHub:** total anual, mapa de oito semanas, barras de 14 dias e Repo em destaque com commits em 7 dias e título do último commit. Preservar ações e estados do complemento anterior.
- **Trello:** nome do quadro, contagem de cards/listas, contagem por lista, prévia de card e acesso ao quadro completo. Contagens acompanham a movimentação. Títulos reais não são substituídos por textos fixos da fixture.
- **Rede:** Qualidade e Segurança separadas, conexão, latência da Internet, teste de velocidade em Mbps, jitter/perda, métricas do gateway e gráfico de tráfego com duas séries e taxas atuais. Aplicativos e Outros ficam acessíveis no detalhe. Nunca afirmar “Rede segura”; preservar motivos de risco/atenção e estados do teste.
- **Sistema:** CPU, temperatura, RAM/swap/disco e bateria, com alertas e ausências reais conforme o complemento anterior.

## Detalhes sobrepostos

- Anel, legenda e alerta de Agentes abrem **Sessões de agente**. O detalhe reúne sessões e encerradas ainda retidas, com projeto/nome, provedor, estado e motivo quando houver. Preservar duração e demais informações da v1; o horário inicial da fixture não substitui esses requisitos.
- **Aplicativos de rede** abre uma lista com nomes e taxas de download/upload, incluindo Outros e sua explicação. Não inventar atribuição por processo além dos dados disponíveis na fonte.
- Detalhes de sessões/processos seguem a área central do protótipo: até 680 px de largura/90% da janela, altura até 80%, com rolagem quando necessária.
- Quadro Trello abre dentro do Painel, com área ampla: referência 1040 × 610 no alvo de 1100 × 700, limitada a 96% da largura e 92% da altura. Colunas de cerca de 228 px, rolagem horizontal do quadro e vertical por lista. Nenhum card é eliminado para forçar a composição.
- Mostrar nome de lista, quantidade, títulos, etiquetas e vencimentos dos cards; manter legibilidade e identificação dos estados. Não depender somente de cor.
- Selecionar um card, escolher **Mover para** e confirmar em **Mover card**. Alternativamente usar **Mover para esta lista** na coluna de destino. Arraste entre listas é atalho; caminho por teclado permanece disponível. O seletor alcança listas fora da área horizontal visível.
- Movimento otimista com atualização do resumo; na falha, devolver à origem e explicar a falha. Cancelar seleção não move. Abrir no Trello por ação explícita ou duplo clique, conforme a v1. Simulação de falha é controle de laboratório e não vai para produção.
- Abrir detalhe mantém o contexto principal. **Fechar** e Escape fecham primeiro o detalhe e restauram foco ao acionador; Escape sem detalhe oculta o Painel. Detalhes modais limitam o foco à área aberta; manter seleção/contexto ao reabrir, quando ainda válidos.
- Notificação de sessão abre o Painel e o detalhe da sessão direcionada, alcançando/destacando-a inclusive se encerrada e ainda retida. Preservar as regras da v1 para alvos indisponíveis; não apontar para outra sessão.

## Estados, acessibilidade e domínio

Estados não demonstrados pelo protótipo herdam as regras anteriores: carregamento/sem leitura, vazio, antigo, offline, falha e credencial inválida; nunca converter ausência de leitura em zero. Cache utilizável permanece com idade e sinalização pertinente. IA reiniciada sem nova leitura remove arco/barra afetados e explica o reinício. Falha de uma fonte não apaga as demais.

No anel, zero sessões com leitura válida é vazio explícito e não produz divisão por zero; leitura indisponível precisa ser distinguida de zero. Quantidades e segmentos derivam dos dados reais, incluindo encerradas somente durante a retenção. A aprovação da fixture cheia não elimina esses critérios.

Foco visível, ordem de leitura coerente, ações e descrições por teclado, resumos acessíveis de gráficos e contraste AA continuam obrigatórios. Atualizações não roubam foco nem invalidam seleção arbitrariamente. Preservar temas escuro/claro/sistema, redução manual/sistema e pausa de decoração com Painel oculto. O movimento simplificado do laboratório não substitui o movimento aprovado no complemento anterior.

O agrupamento visual não redefine **Módulo** nem cria uma nova **Estação**. A apresentação compacta dispensa as faixas de Estação; a taxonomia existente não precisa de alteração no glossário. **Âncora** continua representando o Módulo escolhido; preservar o contexto em retornos e reabertura sem exigir o trilho de navegação removido. Detalhes de sessão direcionada complementam o acesso pela notificação.

Preservar coleta por visibilidade efetiva e suas cadências; Configurações/Painel oculto continuam sem Módulos visíveis. Uma sobreposição não autoriza multiplicar coleta nem mudar fontes. Manter o seam único de E/S e os guardrails de credenciais, hooks e logs/cache/notificações.

## Substituições delimitadas

| Complemento anterior | Painel compacto |
| --- | --- |
| Janela de 95% da área útil | Aproximadamente 1100 × 700, respeitando área útil/escala |
| Três Estações, seis Módulos com faixas/cabeçalhos amplos | Composição B, IA/Agentes na coluna esquerda, cabeçalhos compactos |
| Navegação por nomes e título geral | Resumos simultâneos, ações locais e Configurações no topo |
| Listas de agentes e aplicativos na superfície principal | Anel/resumos com detalhes sobrepostos |
| Trello integral na superfície principal | Resumo permanente e quadro completo sobreposto com movimentação |
| Escape sempre oculta o Painel | Com detalhe aberto fecha o detalhe; fora dele oculta o Painel |

Nenhuma outra redução da cobertura funcional anterior é autorizada. Em especial, manter recuperação de fontes, alertas, consentimentos, fluxos de Configurações, guardrails e notificações.

## Verificação na implementação

1. Comparar produção com B e o quadro aprovado em 1100 × 700 e 1000 × 700, temas escuro/claro/sistema, texto ampliado e nomes longos; verificar fallback e alcance de todas as ações. Não esconder overflow para passar.
2. Fixtures de snapshot: normal, vazio, antigo, falha, offline, credencial ausente/inválida; IA reiniciada sem leitura; anel zero/indisponível/cinco estados/encerradas; listas densas e sem cards; Rede com motivos e teste indisponível/em andamento/falha; Sistema com ausências/alertas. Cenários não exercitados no protótipo são testes obrigatórios, não resultados já obtidos.
3. Exercitar movimentação, reversão, seleção preservada, arraste real e alternativa por teclado, rolagem entre listas, abertura externa e atualização dos resumos. Verificar foco, Escape, fechamento e sessão direcionada por notificação.
4. Validar no GNOME/Wayland a janela real, dimensões lógicas/escala, monitor, detalhes dentro da área útil, abertura/ocultação e instância única. Navegador não comprova comportamento Tauri.
5. Manter os critérios de contraste, latência, fps, orçamento de animação, pausa com Painel oculto e memória da especificação anterior; medi-los na implementação. Preservar testes dos guardrails no seam de E/S existente.

Evidência já obtida: medições do navegador, movimentação/reversão em memória, retorno do foco por Escape, dicas por foco e redução manual; `npm run check` sem erros/avisos na revisão do código. Não há medição nativa, teste real de arraste, auditoria completa de contraste/desempenho nem execução de todos os estados acima.
