# Consolidar a especificação visual validada do Painel

Type: grilling
Status: resolved
Map: [Painel cinematográfico para trabalho](../map.md)
Blocked by: 01, 02, 04

## Question

O protótipo e as decisões aprovadas definem o redesign completo com clareza suficiente para implementação, preservando todos os comportamentos atuais?

Consolidar a especificação visual com referência ao protótipo, cobertura de cada Módulo e Configurações, dimensões e navegação, tokens, estados, movimento, acessibilidade e critérios de aceitação. Conferir a cobertura contra a spec da v1 e a implementação atual. Registrar quais decisões visuais substituem as anteriores e obter validação do usuário antes de considerar o destino alcançado. Não implementar o redesign como parte deste ticket.


## Comments

### Retomada da consolidação após aprovação do movimento

Incorporada à [especificação visual](../spec.md) a Coreografia combinada aprovada: referência com `motion=combined`, ciclos e deslocamento dos efeitos, extensão das regras de redução/pausa e manutenção dos critérios de desempenho. Retirada a pendência de exploração de movimento já resolvida. Conferência das aprovações registradas não identificou outra omissão; continuam pendentes a decisão sobre última âncora e a validação integral pelo usuário. Ticket permanece reivindicado, sem resolução ou autorização de implementação.

### Seleção de Agentes/Sistema e nova exploração de movimento

O usuário dispensou as duas propostas adicionais: lista visível de Encerradas em Agentes e ampliação de Sistema com histórico de CPU, todos os discos/espaço livre e tempo restante da bateria. Manter a seleção de dados da composição aprovada, sem acrescentar esses blocos. A simplificação é de apresentação; não altera estados reais, coleta, retenção ou notificações.

A pedido do usuário, criado [Intensificar o movimento visual do Painel](04-more-panel-motion.md). A validação integral da especificação aguarda essa exploração; a consolidação passa a ser bloqueada por ela. Este ticket continua reivindicado, sem resolução.

### Rede visual aprovada e especificação consolidada

O usuário aprovou a revisão visual de Rede (“Perfeito.”): manter latência e gráfico em destaque, velocidade no instrumento lateral, aplicativos em barras pareadas e medições secundárias discretas. A rodada textual anterior foi rejeitada.

Rascunho consolidado em [Especificação visual — Painel cinematográfico](../spec.md), com matriz de substituições da v1, cobertura dos seis Módulos/Configurações, temas, dimensões, visibilidade, estados, acessibilidade e protocolo de desempenho. A aprovação de Rede não equivale à aprovação integral do documento. Pendências explícitas: apresentação de Encerradas sem expansor, cobertura de Sistema e validação integral. Nenhuma remoção desses dados foi presumida.

Vocabulário: retirada a qualificação “compacta” da definição de Painel em `GLOSSARY.md`, coerente com a ampliação já autorizada; o termo continua designando a mesma janela, sem criar conceito novo.

### Revisão visual da Rede — tabelas rejeitadas

O usuário considerou a rodada compacta excessivamente textual e preferiu a apresentação anterior. Arranjos textuais preservados em `../assets/network-text-layouts.svelte.txt` como histórico, sem aprovação.

Revisão em H recupera latência grande e gráfico de tráfego original; velocidade aparece em instrumento lateral clicável (teste simulado), aplicativos em barras pareadas de download/upload com valores e Gateway em faixa secundária. Conexão/VPN e motivos de alerta preservados. Removidos seletor de arranjos e tabela. Dados de demonstração compostos continuam identificados; data do teste acessível pelo título do botão. Fonte: `src/lib/cinematic-prototype/NetworkCompact.svelte`. `npm run check`: zero erros e avisos. Aguardando avaliação visual; não constitui especificação aprovada.

### Exploração de Rede compacta — aguardando avaliação

O usuário confirmou a importância das informações de Rede ausentes e pediu acomodá-las resumindo o Módulo. Aplicada a skill prototype em exploração auxiliar desta decisão, sem alterar a UI de produção.

Em `/cinematic-prototype.html?variant=H&network=matrix`, comparar A — Matriz, B — Duas faixas e C — Linhas pelos botões no próprio Módulo Rede. Incluem conexão/VPN, Segurança e motivos no cenário de alerta, latência/jitter/perda de Internet e gateway, teste de velocidade com data e botão simulado, tráfego atual/histórico e aplicativos/Outros. Valores ausentes continuam ausentes.

Fonte descartável: `src/lib/cinematic-prototype/NetworkCompact.svelte`. Demonstração composta explicitamente identificada, usando fixtures existentes de processos, velocidade e alerta; histórico da fixture de processos tem uma única amostra e não inventa tendência. Nenhuma E/S real nem persistência. `npm run check`: zero erros e avisos. Densidade e encaixe visual ainda aguardam avaliação do usuário; nenhuma variante aprovada e ticket permanece aberto.

### Primeira rodada de consolidação — respostas do usuário

- IA e GitHub: o usuário considera suficientes os dados já expostos no protótipo; não acrescentar janelas adicionais nem calendário anual completo e não introduzir rolagem para esses dados. Esta escolha autoriza a redução dessa apresentação frente à v1, sem alterar coleta ou regras de negócio.
- Aprovada abertura em 95% da área útil, adaptação a telas menores, fechamento ao perder foco e Escape para ocultar; Configurações terá botão de retorno ao Painel.
- Aprovada visibilidade por interseção com a área de rolagem para as coletas condicionais; Módulos fora dela não contam como visíveis.
- Aprovado orçamento provisório incremental das animações de até 2 pontos percentuais de um núcleo de CPU, comparando movimento ligado e reduzido em três rodadas de 60 s na máquina do usuário; resposta visual até 100 ms, alvo de 60 fps e nenhum trabalho de animação com Painel oculto. Critérios ainda não medidos.
- Rede permanece pendente: o usuário pediu identificar quais medições atuais não aparecem na composição H antes de decidir sua apresentação. Nenhuma remoção adicional foi presumida.

### Pendência para os critérios de aceitação

Ao consolidar, definir orçamento mensurável de CPU para animações com baseline na máquina do usuário e fechar o protocolo de verificação de fluidez (proposta: resposta em até 100 ms, alvo de 60 fps e ausência de trabalho de animação com Painel oculto). A aprovação visual em [Validar linguagem visual, movimento e leitura no trabalho](02-motion-and-readability.md) não equivale a uma medição de desempenho.

## Answer

Em 2026-10-31, o usuário respondeu “ok” à rodada final, aprovando ambas as recomendações: lembrar a última âncora escolhida na navegação e restaurar o contexto ao retornar de Configurações; aprovar integralmente a [Especificação visual — Painel cinematográfico](../spec.md) como complemento visual da v1, incluindo acessibilidade e critérios de desempenho sujeitos a medição.

O documento incorpora composição H, revisão de Rede, seleção de dados aprovada e Coreografia combinada, com substituições da apresentação anterior delimitadas. O destino do mapa foi alcançado: especificação visual validada e pronta para implementação. Não restaram decisões nem fog a graduar; implementação e verificações de produção ficam para execução posterior.
