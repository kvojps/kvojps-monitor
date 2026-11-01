# Especificação visual — Painel cinematográfico

Status: aprovado integralmente pelo usuário em 2026-10-31; pronto para implementação.
Origem: [Consolidar a especificação visual validada do Painel](issues/03-visual-specification.md).

## Autoridade e referência visual

Este documento consolida as decisões do [mapa Painel cinematográfico para trabalho](map.md). É o complemento visual aprovado da [spec da v1](../kvojps-monitor/spec.md); as substituições de apresentação ficam delimitadas abaixo. A execução da implementação está fora deste esforço de planejamento. Regras de domínio, privacidade, credenciais, notificações e coleta permanecem as da v1, exceto pela interpretação de visibilidade aprovada neste esforço.

Referência aprovada: composição **H — Estações de comando**, com **Coreografia combinada**, em `src/lib/cinematic-prototype/`, branch local `prototype/cinematic-panel-composition` (apagada em 2026-11-01). Execute `npm run prototype:cinematic` e abra `/cinematic-prototype.html?variant=H&motion=combined`. A Rede usa a revisão visual de `NetworkCompact.svelte`, aprovada com “Perfeito.” após rejeição dos arranjos textuais. As variantes G/I e o arquivo `assets/network-text-layouts.svelte.txt` são histórico, não alternativas aprovadas.

O protótipo é uma fonte visual, não uma implementação: utiliza fixtures, ações simuladas e estados sintéticos. Não copiar seus valores, nomes de provedores, estados codificados ou regras genéricas de erro para produção. O documento `composition-prototype.md` registra rodadas históricas; menções antigas a expansores ou aprovação pendente não substituem as resoluções dos tickets.

## Composição, janela e navegação

- Abrir pelo Indicador, com os mesmos gestos e garantia de instância única da v1.
- Janela sem decoração, ocupando 95% da largura e da altura da área útil do monitor escolhido pelo compositor; respeitar escala e barras do sistema. O protótipo de navegador não comprova posicionamento nativo.
- Seis Módulos na mesma superfície, sem telas de detalhe e sem expansores. Configurações é a única área separada, com botão “Voltar ao Painel”.
- Três colunas de larguras iguais, com cabeçalhos e duas fileiras alinhados:

| Estação | Primeira fileira | Segunda fileira |
| --- | --- | --- |
| Capacidade | IA | Sistema |
| Execução | Agentes | Trello |
| Observação | GitHub | Rede |

- Larguras acima de 850 px: três estações; entre 651 e 850 px: estações empilhadas, com seus dois Módulos lado a lado; até 650 px: uma coluna. Desfazer alinhamento de alturas quando empilhado. Referência de janela estreita: 560 px.
- Preservar a simetria sem cortar conteúdo, encolher texto para caber ou criar espaço de rolagem para dados extras de IA/GitHub. Rolagem do Painel em telas pequenas é necessária para alcançar os seis Módulos; listas reais podem crescer.
- Navegação por nome de Módulo rola até sua seção e permite foco pelo teclado. Para preservar “último Módulo”, guardar a última âncora escolhida na navegação. Retornar de Configurações restaura o contexto anterior.
- Notificação de Sessão de agente abre o Painel e alcança/destaca a sessão correspondente. Alerta de Agentes na navegação acompanha os dados reais.
- Escape oculta o Painel, inclusive em Configurações; o botão de retorno é a ação para voltar. Perda de foco oculta, preservando o comportamento nativo da v1.
- Cada Módulo identifica nome e atualização da sua fonte. Exibir cache prontamente e atualização discreta, sem bloqueio global. Atualização manual apenas onde já existe, mantendo o intervalo mínimo da v1.

## Linguagem visual e tokens

Influência LCARS/Star Trek: preto profundo, ciano e azul elétrico; faixas segmentadas, curvas técnicas, lavanda e âmbar estruturais. Cor da estação não representa estado da fonte. Evitar mais molduras ou texto apenas para preencher espaço.

| Papel | Escuro | Claro |
| --- | --- | --- |
| Fundo | `#02070d` | `#f3f7fb` |
| Superfície | `#06111d` | `#e4edf5` |
| Superfície secundária | `#0a1d2c` | `#d5e3ef` |
| Texto | `#d8f5ff` | `#132c40` |
| Texto secundário | `#9bb9c7` | `#38566b` |
| Linha | `#173c51` | `#9ab2c6` |
| Acento visual | `#4ce5ff` | ajustar ao contraste sobre fundo claro |

Faixas de estação de referência: lavanda `#b7a4df`, âmbar `#e9b783`, ciano `#81cce5`. Tema claro preserva geometria e hierarquia com os tokens claros; não basta clarear o fundo. Oferecer escuro/claro/sistema; sistema acompanha alterações do ambiente.

Tipografia: Ubuntu Sans ou família sem serifa local equivalente; números em monoespaçada tabular. Títulos de Módulo 16–17 px; títulos gerais 22–30 px; números principais 36–66 px conforme o instrumento; dados secundários 10–12 px como referência do protótipo. Microtexto de 8–9 px fica restrito à decoração/metadados auxiliares. Nenhuma informação indispensável deve depender de texto minúsculo. Espaçamento de Módulo em torno de 20 px e vãos entre estações de 18–22 px.

A implementação deve verificar contraste WCAG AA: 4,5:1 em texto comum, 3:1 em texto grande e controles necessários à identificação. Ajustar tokens locais quando preciso; aprovação visual não é comprovação de contraste.

## Cobertura dos Módulos

### IA

Dois instrumentos circulares para uso da janela de 5 h de Codex e Claude, com percentual e reinício; duas barras para uso semanal; identificação/idade da leitura e ações de atualizar/configurar. O usuário confirmou que essa seleção é suficiente: não acrescentar calendário, janelas/buckets adicionais, planos ou uma área rolável de detalhes para preencher a cobertura da v1.

A simplificação é da apresentação regular. Preservar estados explicativos e caminhos de recuperação: CLI ausente, sem leitura, status line desconectada, token expirado e consentimento para `oauth/usage`. Reconectar só por ação explícita. Jamais supor 0% depois do reinício de uma janela: remover o arco/barra correspondente e mostrar “Reiniciado às HH:MM · sem leitura nova”. Manter o dado válido antigo com idade. Sem alteração na aquisição dos limites.

### Agentes

Manter a linguagem visual aprovada: destaque de intervenção, Sessões de agente com projeto/nome, provedor, Estado da sessão, motivo e duração; contadores derivados dos dados reais. “Aguardando você” e o código Claude da fixture não são rótulos fixos.

Preservar os cinco Estados da sessão, destaque da sessão direcionada por notificação, aviso de detecção de espera do Codex indisponível e falha de coleta. O usuário dispensou acrescentar uma lista de Encerradas; manter o resumo da composição H, sem expansor. Isso não altera retenção de até 2 h/máximo 5 ou regras de notificação.

### GitHub

Total anual em destaque, mapa das últimas oito semanas, barras dos últimos 14 dias e Repo em destaque com commits em 7 dias e título do último commit, como em H; atualizar/configurar no próprio Módulo. O usuário dispensou calendário anual completo e dados adicionais de detalhe, sem acrescentar rolagem. Total anual continua identificado como anual, sem sugerir que as oito semanas mostradas são todo o ano.

Preservar cálculo do Repo em destaque, contagem anônima de contribuições privadas, vazio sem atividade e recuperação de credencial/fonte. Não trocar uma fonte ausente por números de demonstração. Nenhuma mudança na coleta ou armazenamento.

### Trello

Quadro em colunas e cards com título, etiquetas e vencimento. Manter seleção de card/lista de destino, movimento otimista com reversão e mensagem de falha; duplo clique abre o card no navegador. Atualizar/configurar permanecem acessíveis. Listas extensas usam o espaço do quadro com rolagem quando necessária; não esconder cards para forçar simetria. Preservar estados vazios, offline, credencial inválida e sem quadro selecionado.

### Rede — revisão visual aprovada

- Qualidade e Segurança separadas, com nomes canônicos; conexão cabeada/sem fio, VPN e ausência de conexão conforme dados reais. Nunca dizer “Rede segura”. Motivos de Atenção/Risco aparecem quando existirem.
- Latência de Internet em destaque, como na apresentação original. Ao lado, instrumento menor com download/upload do último teste em Mbps, acionável para novo teste.
- Jitter e perda da Internet e latência do gateway em indicadores pequenos. Jitter/perda do gateway em faixa secundária, sem tabela ou cabeçalhos repetidos.
- Gráfico de tráfego com duas séries distintas e taxas atuais ↓/↑. Usar amostras disponíveis sem inventar tendência, preencher lacunas ou animar números artificialmente.
- Aplicativos em barras pareadas de download/upload, nome e valores; até cinco apps mais “Outros”. A dica de Outros explica UDP/QUIC, processos do sistema e conexões curtas. As barras dos aplicativos usam escala comum local e não representam capacidade do link.
- Separar unidades e significado: Mbps é capacidade medida no teste; KB/s ou MB/s é tráfego. No gráfico, escala deriva das amostras, não do último teste de velocidade.
- Idade/data do teste acessível por foco e ponteiro; o protótipo usa `title`, mas produção precisa oferecer a informação também por teclado. Exibir diretamente se antigo/indisponível para evitar interpretação como dado atual.
- Preservar teste indisponível/em andamento/com falha, desabilitação e limites da v1; rótulo/ícone deve comunicar o estado. Se não houver medição, usar ausência explícita, não 0.

A revisão recupera a hierarquia do gráfico e da latência, rejeitando a rodada de tabelas por excesso de texto. Dados do protótipo vêm de fixtures independentes identificadas como demonstração; os totais de fontes diferentes não constituem uma leitura coerente única. Produção utiliza snapshots reais coerentes.

### Sistema

CPU e temperatura em destaque, barras de RAM/swap/disco, bateria com informação contextual. Preservar alertas de disco/temperatura/bateria e ausência de sensor, swap ou bateria conforme dados reais. Não mostrar “Carregada” ou temperatura de fixture como constantes.

O usuário dispensou ampliar a apresentação com histórico de CPU, todos os discos/espaço livre e tempo restante da bateria. Manter a seleção visual de H, sem acrescentar blocos ou expansores; essa redução não altera coleta nem critérios de alerta do núcleo.

## Configurações e primeira execução

Área separada, com a mesma linguagem visual e formulários legíveis. Preservar todos os fluxos da v1 e da implementação atual: Claude (conectar/desconectar status line, consentimento `oauth/usage`, diagnóstico), hooks e seus passos de confiança, notificações, token GitHub, Trello (key/token/autorização/quadro/desconexão), velocidade automática. Incluir preferências de tema e redução de animações.

Primeira execução, autostart, atualização de versão e abertura de logs continuam requisitos da v1; sua ausência no protótipo não autoriza removê-los nem indica que já existem em produção. Usar a mesma superfície de Configurações, sem criar uma tela extra de detalhe por Módulo. Erro de validação fica junto do campo e sucesso identifica conexão. Nunca inserir explicações de implementação na UI sem necessidade para a decisão do usuário.

Credenciais só no GNOME Keyring; sem keyring recusar salvar. Hooks e status line mudam apenas após clique explícito, aditivamente e de forma reversível. Token Claude apenas em memória e sem renovação. Logs/cache/notificações respeitam guardrails da v1; nenhum dado novo é coletado para decoração.

## Estados e acessibilidade

- Distinguir normal, atenção, falha, sem dado, antigo, desconectado e credencial inválida por texto/ícone, sem depender só da cor. Não replicar o erro global sintético do laboratório.
- Falha com cache utilizável mantém o último snapshot com idade e a informação de falha segundo a v1. Sem dados, substituir o instrumento afetado por explicação e ação pertinente. Offline mantém cache com idade sem faixa de erro de rede.
- Preservar independência de Provedores de IA e fontes: falha em uma não apaga as outras. Atualizações não alteram arbitrariamente foco, seleção ou posição de rolagem.
- Navegação e ações por teclado, foco visível, ordem de leitura coerente com as estações, nomes acessíveis para instrumentos, valores e estados. Ícones decorativos ficam fora da árvore acessível.
- Gráficos possuem resumo textual dos valores; diferenças entre séries também têm rótulos, não só cor. Metadados em hover devem ter equivalente por foco; não esconder ação necessária em tooltip.
- Listas densas não sobrepõem ações nem cortam texto indispensável. Sem expansores de detalhe; não acrescentar a lista de Encerradas nem os blocos adicionais de Sistema dispensados pelo usuário.

## Movimento, visibilidade e coleta

Anéis decorativos: 48 s por volta; arcos de medição ficam imóveis entre atualizações reais. Brilho decorativo: 4 s por trecho, alternando ida/volta (8 s de ciclo completo), conforme CSS do protótipo aprovado. Textos/alertas não piscam. Decoração não indica atividade ou coleta.

A [Coreografia combinada](issues/04-more-panel-motion.md) amplia essa base com fluxo luminoso nas faixas superiores (3,5 s, com fases deslocadas), varredura decorativa nas bordas dos Módulos (7 s, defasada entre colunas), deriva de fundo nos títulos das estações (9 s) e deslocamento de 2 px dos botões de navegação ao apontador (transição de 150 ms). Dados, gráficos, texto e alertas permanecem estáveis. Esses efeitos também obedecem à redução, pausa e orçamento de desempenho abaixo; a aprovação visual não constitui medição.

Redução manual ou preferência de redução do sistema interrompe decoração e remove rolagem animada/transições. Redução do sistema prevalece. Com Painel oculto, não há trabalho de animação, callbacks ou timers decorativos; `document.hidden` do protótipo não prova o comportamento Tauri. Sincronizar com visibilidade nativa na implementação.

Coletas condicionadas a Módulo visível passam a considerar o conjunto de Módulos que intersectam a área efetivamente visível do Painel. Oculto/Configurações: conjunto vazio. Fora da área de rolagem: Módulo não visível. Preservar cadências, pausa sem rede e retomada; não multiplicar coletas por notificação de interseção. A porta atual de um único Módulo precisará representar múltiplos visíveis; detalhe de implementação fica para execução.

## Verificação e critérios de aceitação

1. Validar lado a lado com H e Coreografia combinada em temas escuro/claro/sistema e larguras de três colunas, intermediária e 560 px; testar também escala de texto/tela. Sem perda de ações ou campos acordados.
2. Fixtures de snapshot por Módulo: cheio, vazio, antigo, credencial inválida e erro; incluir estados específicos de IA, cinco Estados da sessão, Sistema sem sensores/bateria e Rede com VPN, desconexão, alerta, velocidade falha/em andamento e cinco aplicativos. Testar comandos de UI e recuperação, não pixels do protótipo.
3. Testar privacidade no seam de E/S do núcleo, incluindo regras do keyring, conteúdo de logs/cache/notificações, alterações aditivas/reversíveis de hooks e não renovação do token Claude. Não criar seam paralelo.
4. Verificar Indicador, dimensionamento, posicionamento, fechamento e notificação direcionada manualmente no GNOME/Wayland. Registrar limitações reais do compositor, sem declarar equivalência por teste do navegador.
5. Desempenho na máquina do usuário: mesmo build, tema, dimensões, fontes/fixtures e condição de alimentação. Após estabilizar, comparar movimento ligado e reduzido em três pares de rodadas de 60 s, alternando ordem e registrando baseline, hardware e versão. Somar CPU do app e processos da webview. Orçamento provisório: acréscimo médio máximo de 2 pontos percentuais de um núcleo para animação; não confundir com 2% da máquina inteira ou orçamento total da coleta.
6. Resposta visual a interações em até 100 ms; alvo de 60 fps durante movimento em tela de 60 Hz (ou equivalente), verificando traço de quadros e travamentos durante ações/listas densas. Registrar resultado observado e condições; o alvo não é medição já obtida.
7. Com Painel oculto, verificar ausência de trabalho de animação por instrumentação/trace, distinguindo coleta de dados legítima. Revalidar regra de memória da v1 e comportamento de ocultar/destruir webview.

Aprovações até aqui são visuais e de critérios. CPU, latência de resposta, fps, contraste, integração nativa e memória não foram medidos nesta consolidação. `npm run check` aprovou a última revisão do protótipo, sem erros ou avisos.

## Substituições delimitadas da v1

| Regra anterior | Direção deste esforço |
| --- | --- |
| Painel compacto ~440×600 | 95% da área útil e composição responsiva |
| Trilho vertical, detalhe de um Módulo | Navegação por âncoras, seis Módulos juntos, só Configurações separada |
| Minimalista e acento verde-água | LCARS cinematográfico, preto/ciano/azul com faixas âmbar/lavanda |
| Janelas/buckets e detalhes completos de IA | Seleção visual de H, sem acrescentar dados/rolagem |
| Calendário anual completo e detalhes de GitHub | Total anual, oito semanas e 14 dias de H, Repo em destaque resumido |
| Lista recolhida de Encerradas em Agentes | Resumo de H, sem lista adicional ou expansor |
| Histórico de CPU, todos os discos/espaço livre e tempo de bateria | Seleção visual de Sistema em H, sem blocos adicionais |
| Um Módulo selecionado define coleta visível | Conjunto de Módulos que intersectam a área visível |

Nenhuma substituição muda credenciais, fontes, regras de Segurança/Qualidade da rede ou notificações. Reduções adicionais não explicitadas precisam de validação.

## Validação integral

Documento aprovado integralmente pelo usuário em 2026-10-31, incluindo memória da última âncora e critérios objetivos de acessibilidade e desempenho. Nenhuma decisão de planejamento permanece pendente. As verificações de implementação e medições descritas acima continuam necessárias na execução.
