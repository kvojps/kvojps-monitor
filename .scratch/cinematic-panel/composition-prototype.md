# Protótipo de composição do Painel

Artefato descartável para [Definir a composição e a navegação do Painel cinematográfico](issues/01-panel-composition.md). Composição H aprovada pelo usuário; regras finais de estados, movimento e cobertura funcional ainda dependem dos próximos tickets.

Execute `npm run prototype:cinematic` e abra http://localhost:1421/cinematic-prototype.html?variant=H.

## Composições atuais para comparar

- **G — Ponte integrada:** IA e Agentes em destaque, Rede e Sistema numa coluna de apoio, GitHub e Trello na base.
- **H — Estações de comando:** três colunas organizadas por função: capacidade (IA/Sistema), execução (Agentes/Trello), observação (GitHub/Rede).
- **I — Console panorâmico:** telemetria numa faixa superior; sessões e atividade ao lado de um quadro de trabalho amplo.

As três seguem a influência LCARS e exibem os seis Módulos no mesmo Painel. As setas inferiores ou do teclado alternam `?variant=G`, `H` e `I`. Links antigos abrem G. A navegação por Módulo rola até a área correspondente. Somente Configurações troca de tela; Escape retorna ao Painel. Os controles ficam nos Módulos, com detalhes expansíveis para todas as janelas de uso, calendário completo e medições adicionais. A movimentação de cards ocorre no quadro, em memória.

Há simulação de janela de 560 px e adaptação a larguras menores; o modo largo ocupa quase toda a largura disponível. Não simula posicionamento GNOME, abertura pelo Indicador ou dimensionamento nativo Tauri.

## Histórico da exploração

A primeira rodada (Central panorâmica, Órbita de trabalho, Instrumento de foco) está preservada na [referência da primeira rodada](assets/composition-round-one.svelte.txt). A segunda rodada explorou Parede de telemetria, Ponte de comando e Mesa de operações; o usuário escolheu a linha da Ponte de comando e pediu influência de Star Trek. Fontes preservadas na [referência da segunda rodada](assets/composition-round-two.svelte.txt) e nos [instrumentos da segunda rodada](assets/telemetry-round-two.svelte.txt).

O usuário pediu novas composições mais organizadas e dispensou a tela separada de detalhe por Módulo, mantendo apenas Configurações como tela separada. As três opções atuais seguem essa orientação. Ainda não há composição final aprovada.

## Inventário a preservar

Inventário observado em `src/lib/Panel.svelte` e nos componentes dos Módulos e Configurações:

| Área | Consulta e ações a acomodar |
| --- | --- |
| Painel | Acesso pelo Indicador, último Módulo, atualização da fonte, fechar com Escape, direcionamento para Sessão de agente de uma notificação |
| IA | Limites de uso de Codex e Claude, janelas e reinícios, planos, origem e idade da leitura, estados indisponíveis e falhas, atualizar, configurar e reconectar |
| Agentes | Sessões atuais e encerradas, projeto, nome, provedor, Estado da sessão, duração, motivo de espera e destaque da sessão direcionada |
| GitHub | Calendário de contribuições, contribuições restritas, Repo em destaque e último commit, atualizar, estados de credencial, falha e leitura antiga |
| Trello | Quadro, listas, cards, etiquetas, vencimento, seleção e movimentação entre listas, abertura de card, atualizar, configurar e estados de falha |
| Rede | Segurança da rede, Qualidade da rede, motivos de alerta, conexão/VPN, gateway e internet com latência/jitter/perda, teste de velocidade, histórico e tráfego por processo |
| Sistema | CPU e histórico, temperatura, RAM, swap, discos e espaço livre, bateria e alertas, indisponibilidade de sensores |
| Configurações | Hooks e notificações de agentes; conectar/desconectar Claude e consentimento de uso; token GitHub; chave/token Trello, autorização, seleção de quadro e desconexão; velocidade automática |

## Dados e limites da exploração

Reutiliza componentes existentes com fixtures: IA `claude-usage`, Agentes `waiting`, GitHub e Trello `full`, Rede `good`, Sistema `notebook`. O selo de demonstração permanece visível. Datas das fixtures não são atualizadas para parecerem dados ao vivo. A visão geral usa instrumentos próprios da exploração: medidores circulares e barras de limites, contagem de sessões, tráfego com as duas amostras disponíveis, contribuições e quadro de cards. As datas e os números permanecem os das fixtures. Os detalhes mantêm os componentes atuais; este ticket compara principalmente composição, espaço e navegação. Estados alternativos e acabamento/movimento completos pertencem à próxima decisão.

Ações externas são simuladas e exibem retorno explícito. Movimentar cards altera apenas uma cópia em memória. Configurações não persistem. Não há chamadas ao núcleo, HTTP, keyring ou arquivos de configuração do usuário.

Verificação da segunda rodada: `npm run check` passou sem erros ou avisos; renderização Svelte no servidor das três composições confirmou conteúdo das fixtures; servidor local respondeu por HTTP na porta 1421. Build da entrada confirmou a exclusão do protótipo em produção pelo gate de desenvolvimento. Inspeção visual e aprovação permanecem pendentes.

Após a validação, preservar o protótipo numa branch local `prototype/*` e registrar a decisão no ticket. Não promover o código descartável ao app neste mapa.

## Refinamento da Ponte de comando — influência de Star Trek

O usuário escolheu a direção da Ponte de comando e pediu um toque de Star Trek. A composição E agora explora influência LCARS: moldura curva, faixas segmentadas, navegação em blocos e cabeçalhos coloridos, com preto/ciano e detalhes âmbar/lavanda. As cores das molduras distinguem áreas; os estados seguem identificados por texto. As composições D e F continuam disponíveis para comparação. Aguardando validação visual deste refinamento; a spec de produção permanece a mesma.

Verificação do refinamento: `npm run check`, zero erros e avisos.

## Verificação da terceira rodada

`npm run check`: zero erros e avisos. Renderização Svelte das composições G, H e I confirmou os seis Módulos e os conteúdos das fixtures. Inspeção visual pelo usuário segue pendente.

## Escolha e ajuste de simetria — Estações de comando

O usuário escolheu H, considerou os dados já expostos suficientes e dispensou os expansores. Na H, os quatro expansores foram removidos. G e I continuam como comparações históricas da rodada. A abertura sem parâmetro agora seleciona H.

As três estações usam colunas de larguras iguais e linhas compartilhadas: cabeçalhos, primeira fileira de Módulos e segunda fileira de Módulos ficam alinhados. Ações e indicadores inferiores recebem espaçamento para acompanhar a distribuição. Em telas menores, a composição se reorganiza sem exigir essas alturas compartilhadas.

Verificação: `npm run check` sem erros ou avisos; renderização de H confirmou os seis Módulos e ausência de expansores. A distribuição revisada ainda aguarda inspeção visual do usuário. O pedido de exibir apenas os dados atuais foi registrado como direção visual; eventual redução de funções da v1 precisa ser conciliada na especificação final, sem alterar o app de produção neste protótipo.

## Aprovação da composição

O usuário aprovou a distribuição simétrica final de H. A decisão está registrada no ticket de origem. Fonte preservada na branch local `prototype/cinematic-panel-composition`, sem push. O protótipo continua descartável; o app de produção não foi redesenhado.
