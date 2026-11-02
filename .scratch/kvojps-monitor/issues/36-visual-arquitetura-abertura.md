# 36: Aplicar Arquitetura à abertura do Painel

**What to build:** implementar a direção visual C aprovada dentro da própria janela, consumindo o ciclo real de abertura. Reescrever o visual para produção a partir da referência descartável.

**Blocked by:** 35 — Implementar prontidão e ciclo de abertura do Painel.

**Status:** resolved

- [x] Compor fundo escuro, malha de linhas luminosas em perspectiva e identidade inferior esquerda, conforme Arquitetura (C). Exibir exatamente “kvojps monitor”.
- [x] Usar como fonte visual a branch local `prototype/panel-opening`, commit `91d27bf`, entrada `opening-prototype.html`, variante C. Não incorporar controles de simulação, variantes alternativas ou portas falsas ao app.
- [x] Vincular “Preparando o Painel…” e “Carregando Módulos…” às atividades reais correspondentes. Se tudo estiver pronto antes do mínimo, manter apenas a composição, sem mensagem alegando atividade.
- [x] Fazer a passagem suave para os Módulos dentro da duração contratada, sem reiniciar o mínimo após preparo demorado. Respeitar prontidão/recuperação antes de revelar conteúdo.
- [x] Manter abertura escura nos temas claro, escuro e do sistema, revelando o tema restaurado ao entrar no Painel sem flashes intermediários de tema incorreto.
- [x] Respeitar redução manual e do sistema: mesma composição estática, mensagens e duração mínima, sem linhas em movimento ou transição animada. Preferência do sistema prevalece.
- [x] Garantir legibilidade, responsividade, texto ampliado, contraste e operação por teclado; conteúdo coberto não recebe foco/interação. Mensagens não roubam foco nem provocam anúncios repetidos a cada frame.
- [x] Preservar a decoração existente dos Módulos, sem executá-la indevidamente sob a abertura ou após fechar. Não criar preferências de produto para duração ou intensidade.
- [x] Verificar com fixtures de snapshot os estados do ciclo, temas, movimento reduzido e falhas; executar check, testes pertinentes e build. Evidência nativa integrada pertence ao ticket Validar a abertura no app nativo.

## Fontes aprovadas

- [Definir prontidão e ciclo de abertura do Painel](../../panel-opening/issues/01-opening-readiness.md).
- [Validar sequência visual cinematográfica de abertura](../../panel-opening/issues/02-opening-visual.md).
- [Abertura cinematográfica do Painel](../../panel-opening/map.md).


## Implementação e validação

Composição de produção em `src/lib/PanelOpening.svelte`, reescrita a partir da referência C do commit `91d27bf`: fundo escuro, malha em perspectiva e identidade inferior esquerda. Mensagens usam as etapas reais existentes; durante a espera mínima após prontidão, a região de status fica vazia.

A passagem de 240 ms começa após prontidão e após os primeiros 960 ms de apresentação: preparo rápido completa a sequência em 1,2 s; preparo demorado acrescenta apenas a passagem, sem repetir o mínimo. O conteúdo permanece inerte e fora da árvore de acessibilidade até o fim. Movimento reduzido conserva a abertura opaca e estática até a revelação. Linhas aguardam a restauração das preferências antes de animar.

Validação: `npm run check` sem erros/avisos; 367 testes de UI, incluindo 14 da abertura com fixtures (ciclo, recuperação, falha, temas, redução manual e do sistema, decoração); `npm run build`; suíte Rust com 305 testes unitários e 3 de integração. Os testes de socket exigiram execução fora do sandbox. A validação visual integrada e de desempenho nativo permanece no ticket de validação nativa, conforme o escopo.

Revisão em dois eixos pela skill code-review, base `4f8cb99d8fba528a29ad700dab46c3eb1fff11b0`: Standards sem violações documentadas; sugestão de centralizar cancelamento aplicada. Spec identificou desaparecimento prematuro com redução de movimento; corrigido mantendo opacidade até a revelação.


### Correção após avaliação do usuário

Relato: tela branca antes da abertura, composição diferente da referência C e fonte diferente da usada pelo app. A medição no WebKitGTK reproduziu a diferença: título em `system-ui` versus corpo em `"Ubuntu Sans", Ubuntu, Inter, system-ui, sans-serif`.

Correção: título e status usam `--sans`; espaçamento externo volta a 8% como na referência, removendo o limite de 96 px; geometria e identidade recebem camadas explícitas, sem a sombra de texto adicionada na primeira implementação. Fundo `#02070d` definido na configuração da janela/webview nativa e no HTML inicial, antes do carregamento dos módulos JS/CSS, para cobrir o intervalo anterior à montagem do Svelte.

Verificação: medição no WebKitGTK confirmou fontes iguais e fundo escuro após carregamento; `npm run check`, 367 testes UI, `npm run build` e `cargo check` passaram. A ausência do flash desde o primeiro frame nativo ainda requer reabrir o app recompilado; a medição de DOM após carregamento não comprova esse intervalo.


### Primeira apresentação com a identidade

Após o usuário confirmar que ainda via apenas o fundo antes da identidade, a composição estática passou ao HTML inicial, com estilo compartilhado em `src/opening.css`. O Svelte remove essa composição ao montar a abertura real, sem duplicar o nome; a composição inicial não inventa atividades ou animações antes de restaurar as preferências.

A janela nativa é criada invisível e apresentada no término do carregamento da página. Pedidos repetidos enquanto a página carrega não a mostram antecipadamente. O relógio existente aguarda apresentação visível para contar o mínimo.

Validação: 368 testes UI (incluindo regressão da substituição da composição inicial), check sem erros/avisos, build e `cargo check` passaram. A avaliação visual do primeiro frame requer execução do app nativo recompilado.
