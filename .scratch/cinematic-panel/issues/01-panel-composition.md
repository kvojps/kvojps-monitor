# Definir a composição e a navegação do Painel cinematográfico

Type: prototype
Status: resolved
Map: [Painel cinematográfico para trabalho](../map.md)

## Question

Qual composição, tamanho e navegação oferecem a intensidade cinematográfica desejada, preservando a consulta e as ações dos seis Módulos e das Configurações?

Explorar com protótipo local uma central de comando maior, incluindo a alternativa de visão geral com detalhe por Módulo. Validar com o usuário a hierarquia, a densidade, o acesso às ações e o comportamento em telas menores. Usar fixtures existentes para representar dados; exemplos precisam ser identificáveis como demonstração. Não alterar a UI de produção. Registrar o inventário das funções atuais que a composição precisa acomodar.

## Comments

### Exploração de composição — aguardando avaliação

Protótipo disponível em [Protótipo de composição do Painel](../composition-prototype.md), com inventário das funções atuais, três composições, fixtures utilizadas e limites de simulação. Código descartável em `src/lib/cinematic-prototype/`, entrada independente `cinematic-prototype.html`; execução por `npm run prototype:cinematic`.

`npm run check`: sem erros ou avisos. Avaliação visual do usuário pendente; o ticket permanece reivindicado, sem decisão registrada.

### Segunda rodada — dados expostos e instrumentos

Após a primeira avaliação, o usuário pediu mais sugestões e dados apresentados como painéis. Confirmou a orientação proposta: vários dados visíveis por Módulo, destaque para IA e Agentes e janela quase do tamanho da tela.

Novas composições no mesmo protótipo: Parede de telemetria, Ponte de comando e Mesa de operações (`?variant=D`, `E`, `F`). A primeira rodada foi preservada no [registro do protótipo](../composition-prototype.md). A visão geral agora inclui instrumentos baseados nas fixtures e acesso para ampliar cada Módulo. Aguardando avaliação visual; nenhuma composição aprovada e ticket ainda aberto.

### Direção escolhida — Ponte de comando com influência de Star Trek

O usuário escolheu a linha visual da Ponte de comando e pediu um toque de Star Trek. Atualizada a composição E com influência LCARS: faixas curvas, navegação segmentada, cabeçalhos por Módulo e detalhes em âmbar e lavanda sobre a base preta/ciano. Os números e instrumentos continuam usando as mesmas fixtures; as cores estruturais não representam estados dos dados. Alertas continuam textuais.

A escolha da direção não encerra a validação de tamanho, densidade e navegação. Acabamento atualizado disponível em `?variant=E`; aguardando reação do usuário. `npm run check` passou sem erros ou avisos.


### Terceira rodada — Painel completo, apenas Configurações separada

O usuário pediu composições mais organizadas seguindo a influência Star Trek e dispensou a tela específica de detalhe por Módulo. Esclareceu que apenas Configurações pode ficar fora do Painel completo.

Criadas Ponte integrada, Estações de comando e Console panorâmico (`?variant=G`, `H`, `I`). Todos os Módulos ficam visíveis na mesma superfície; navegação rola para a seção, dados adicionais expandem inline e o quadro permite movimentar cards em memória. Configurações mantém a única tela separada, com retorno ao Painel. Código da segunda rodada preservado nos assets do [registro do protótipo](../composition-prototype.md).

`npm run check`: sem erros ou avisos. Renderização das três composições confirmou os seis Módulos e os dados das fixtures. Aguardando avaliação do usuário; ticket ainda reivindicado, sem resolução final.


### Composição escolhida — Estações de comando

O usuário escolheu H e aprovou a organização por estações. Pediu retirar os expansores, pois os dados na tela são suficientes, e tornar a distribuição mais simétrica.

H atualizada: três colunas de mesma largura, cabeçalhos e duas fileiras de Módulos alinhados por linhas compartilhadas, expansores removidos e ações inferiores alinhadas. H passa a ser a opção inicial. Configurações permanece a única tela separada.

`npm run check` sem erros ou avisos; renderização confirmou seis Módulos sem expansores. Aguardando avaliação da distribuição revisada antes de encerrar a decisão. A conciliação entre a seleção visual de dados e a preservação de funções da v1 fica explícita para a consolidação da especificação; nenhuma função de produção foi alterada.


## Answer

Composição **H — Estações de comando** aprovada pelo usuário após o ajuste final de simetria (resposta: “Perfeito.”).

- Painel completo com os seis Módulos; apenas Configurações abre uma tela separada.
- Três estações de mesma largura: Capacidade (IA/Sistema), Execução (Agentes/Trello) e Observação (GitHub/Rede).
- Cabeçalhos e duas fileiras de Módulos alinhados; adaptação para disposição empilhada em telas menores.
- Influência visual de Star Trek/LCARS, com geometria curva e faixas segmentadas, preto/ciano e detalhes âmbar/lavanda.
- Dados já visíveis são suficientes para esta composição: expansores retirados. Ações permanecem nos próprios Módulos.

Artefato e inventário: [Protótipo de composição do Painel](../composition-prototype.md). Fonte preservada na branch local `prototype/cinematic-panel-composition` (sem push). Execute `npm run prototype:cinematic` e abra `/cinematic-prototype.html?variant=H`.

Validação técnica: `npm run check`, zero erros e avisos; renderização confirmou os seis Módulos sem expansores. Aprovação visual do usuário cobre a distribuição final. Movimento, estados alternativos, tema e critérios de fluidez seguem em [Validar linguagem visual, movimento e leitura no trabalho](02-motion-and-readability.md). A cobertura final dos comportamentos da v1 será conciliada em [Consolidar a especificação visual validada do Painel](03-visual-specification.md).
