# Validação integrada do Painel compacto — ticket 07

Registro de 2026-10-31; baseline `5c998022c508a8d932c491940b9f1d6247eed982`.
Evidência de **navegador** (Chromium 156 headless) sobre o Painel real com fixtures
gravadas. Nada aqui comprova comportamento Tauri/WebKitGTK no GNOME/Wayland; os itens
nativos estão na seção [Pendências nativas](#pendências-nativas).

## Reprodução

A superfície `surface/` monta `Panel.svelte` com as fixtures de `src/lib/*/fixtures`,
sem comandos nativos, credenciais nem coleta. Não entra no build de produção.
Parâmetros: um nome de fixture por Módulo (`none` omite o snapshot), `theme`,
`motion=full`, `long=1` (nomes longos derivados), `dense=1` (40 sessões, 60 cards por
lista, 50 aplicativos derivados), `move=fail` (o núcleo simulado recusa a
movimentação) e `target=<sessão>` (alvo de notificação). Variantes derivadas são
condições de estresse, não leituras novas.

Com Vite em 1420 e um Chromium descartável em CDP 9222
([procedimento](../../cinematic-panel-implementation/validation/README.md)), na raiz:

```sh
SHOTS=/tmp node .scratch/compact-panel-implementation/validation/integrated-audit.mjs > /tmp/integrated.json
```

Relógio fixado no instante da fixture `claude-usage`, como nos tickets 02/04.
Resultado preservado em [integrated-results.json](integrated-results.json).
Ambiente: Ubuntu, GNOME Shell 50.1, Wayland, AMD Ryzen 5 5625U (12 CPUs lógicas),
14 GiB, na tomada, WebKitGTK 2.52.6 instalado (não usado pelo auditor).

## Desvio encontrado e corrigido: rolagem principal em 1100 × 700

Antes da correção, a fixture padrão rolava **491 px** em 1100 × 700. O Painel não
cumpria o alvo de B. Causas medidas:

- **Rede com 779 px** (B ≈ 290 px). A regra `.traffic svg` também alcançava os SVGs
  da legenda das séries, que viravam blocos de 100% × 84 px e quebravam as taxas
  (“177,\n7 KB/s”). Instrumentos, eixos e gráfico mantinham as medidas do
  complemento cinematográfico.
- Como as linhas da grade são compartilhadas, a altura da Rede empurrava Trello e
  Sistema para baixo. A coluna de IA/Agentes também excedia (Agentes 350 px, com
  número de 56 px e anel de 140 px).
- Trello tinha 336 px, com botões de 36 px de altura.

Correções, todas só de apresentação (sem mudança de dados, ações ou textos):

- **Rede:** o seletor do gráfico agora é `.traffic > svg` (34 px). Qualidade e
  Segurança continuam como grupos separados, mais baixos. A conexão/VPN passou
  para baixo da latência (**divergência de B**, onde a conexão fica na linha de
  status; aprovada pelo usuário em 2026-10-31 e registrada no ticket 05). A grade de métricas virou uma linha flexível e o
  botão de aplicativos perdeu a linha própria da legenda. O título “Tráfego total” é oculto visualmente e mantido
  para leitores de tela, como em B. O gráfico continua nomeado com resumo. Jitter,
  perda e latência do gateway ficam em uma linha. Os motivos de Atenção/Risco
  continuam acima dos instrumentos. Grades a partir de 120 px mantêm duas colunas
  em 1000 px.
- **Agentes:** número 30 px, anel 112 px, legenda e alerta mais compactos; anel,
  legenda e alerta continuam acionáveis.
- **Trello:** espaçamentos e botões de 28 px. Os nomes de listas e os títulos de
  cards **não são mais cortados** por reticências/`line-clamp`: quebram linha, como
  em B. O nome dos aplicativos no detalhe de Rede também deixou de usar reticências.
- **Cabeçalhos de Módulo:** 56 → 47 px; rolagem com margem inferior de 8 px.
- **Contraste:** título “Quadro Trello” no tema claro tinha 1,53:1
  (`--band-amber` sobre superfície clara); agora usa `--band-amber-text`.

Capturas após a correção: [escuro 1100](integrated-dark-1100.png),
[claro 1100](integrated-light-1100.png), [escuro 1000](integrated-dark-1000.png).
Comparação visual com `compact-B.png` em 1100 × 700: mesma composição, ordem,
hierarquia e conteúdo. Diferenças restantes: B mostra ações por ícone no Trello,
e a produção mantém os botões textuais previstos no ticket 06. B mostra dois
instrumentos de IA porque sua fixture tem leitura do Claude; a fixture padrão de
produção não tem. Aceite visual integral continua sendo do usuário.

## Composição e fallback

Altura disponível da rolagem principal: 637 px.

| Condição (temas escuro/claro/sistema iguais) | Rolagem principal |
| --- | --- |
| 1100 × 700, fixtures padrão | **0 px** |
| 1100 × 700, listas densas | 0 px |
| 1100 × 700, escala de monitor 1,25/1,5/2 | 0 px |
| 1000 × 700, fixtures padrão | 69 px (Rede com 385 px: cabeçalho, legenda de capacidade, taxas e aplicativos quebram linha) |
| 1100 × 700, texto a 125% | 244 px |
| 1000 × 700, texto a 125% | 279 px |
| 1100 × 700, nomes longos | 189 px |
| 950 × 700 (duas colunas) / 650 × 700 (uma) | 521 / 1097 px |
| Área útil 1100 × 560 / 900 × 600 / 760 × 500 | 138 / 634 / 734 px; último Módulo alcançável |

Em 1000 × 700 o protótipo cabia; a produção rola 69 px. Registrado sem ocultar
overflow. Texto a 125% é ampliação uniforme dos tamanhos computados, não a
escala de texto do GNOME. Escala de monitor no navegador muda só a densidade.

Todos os cenários: nenhum overflow horizontal, nenhum conteúdo cortado por
`overflow: hidden/clip` e nenhum elemento além da largura do Módulo. Fontes
nunca são reduzidas automaticamente.

## Matriz de estados

130 cenários: todas as fixtures de cada Módulo e `none` (sem snapshot), nos temas
escuro e claro. Nenhuma falha de contraste, corte ou escape. Cobre IA reiniciada
sem leitura (`reset`, `weekly-reset`); anel vazio (`empty`/`closed`),
indisponível (`none`, “Sem leitura”), cinco estados e encerradas; Rede com
motivos, VPN, desconexão e teste indisponível/em andamento/falha/offline; Sistema
com ausências e alertas; credencial ausente/inválida, offline, antigo e falha.

A matriz roda nos temas escuro e claro; o tema sistema foi coberto só na tabela
de composição.

Estados que excedem 1100 × 700, com fallback por rolagem: Rede 5–77 px, **inclusive
estados comuns** `good`/`poor`/`vpn` (49 px), cujas fixtures têm teste de velocidade
indisponível e aplicativos sem medição, além de avisos do teste e motivos; GitHub e
Trello com falha/credencial/offline 20–42 px, Agentes vazio 43 px (vazio
explícito sob o anel zerado). A spec exige ausência de rolagem com as fixtures
padrão; nenhum desses casos corta conteúdo ou esconde alerta.

## Acessibilidade

- Contraste textual mínimo: 6,15:1 escuro e 4,89:1 claro no Painel; 5,39:1 nos
  detalhes abertos. A varredura exclui SVG, desabilitados e conteúdo oculto.
- Não textual: controles sem texto precisam de borda, preenchimento ou ícone a
  3:1 do fundo. O mínimo medido foi 13,2:1 no escuro e 5,39:1 no claro. Nos
  detalhes abertos todos os controles têm texto, então essa verificação não se aplica
  (o valor 100 no JSON indica ausência de amostras). Séries
  dos gráficos: cálculo do ticket 10, inalterado.
- Teclado: 18 de 18 controles focáveis alcançados por Tab, todos com nome e
  contorno de foco visível. Ordem: Configurações → IA → Agentes → GitHub → Rede →
  Trello. Sistema não tem controles.
- Árvore acessível: nenhum botão, link, imagem ou medidor sem nome. Mapa do
  GitHub, gráfico de Rede, etiquetas e medidores têm resumos textuais.

## Detalhes sobrepostos

24 cenários (Agentes, Aplicativos de rede e Quadro Trello; 1100 × 700,
1000 × 700, 650 × 700 e área útil 1100 × 560; escuro/claro):

- Dimensões: detalhes 680 × 560 (585 a 650 px, 448 de altura com área útil 560).
  Quadro 1040 × 610 no alvo, 960 em 1000 e 1040 × 515 em área útil 560.
  Sempre dentro da janela.
- Foco entra e permanece no diálogo modal; o primeiro Escape fecha o detalhe e
  devolve o foco ao acionador; o segundo pede ao núcleo para ocultar o Painel.
- Trello, com o caminho completo por teclado: 3 das 8 listas começam fora da área
  visível e todas as 7 de destino estão no seletor. A movimentação atualiza
  quadro e resumo de forma otimista. A falha devolve à origem e explica (“O card
  voltou para Lista 1”). A seleção é preservada ao reabrir, com foco no card. O
  duplo clique pede a abertura externa e os cards são `draggable`.
- Notificação: `s-closed` (encerrada retida) e `s-waiting` abrem o detalhe
  destacando a sessão; alvo inexistente mostra “Sessão da notificação
  indisponível” sem destacar outra.

## Movimento e Painel oculto (instrumentação do navegador)

Movimento completo: decoração `running` e 8 animações ativas. Ao ocultar
(evento de visibilidade simulado), decoração `paused` e 0 animações; ao
reabrir, retomam. Movimento reduzido: 0 animações em todas as fases. Os
relógios de idade por segundo continuam enquanto o webview existe. Em produção,
o webview é destruído ao fechar (ticket 11). Isso não substitui trace nativo.

## Resposta e fluidez no navegador

Fixtures densas, 20 aberturas por detalhe, sem limitar CPU, cliques
programáticos; medido até o segundo quadro após o clique:

| Ação | p95 / máximo |
| --- | --- |
| Sessões de agente (40 sessões) | 34,8 ms |
| Aplicativos de rede (50) | 30,6 ms |
| Quadro Trello (8 listas × 60 cards) | 89,7 ms |

Event Timing: 116 eventos ≥ 16 ms, máximo 40 ms; 21 long animation frames.
Rolagem de 2 s no quadro denso: 120 quadros, p95 16,7 ms, máximo 16,8 ms, nenhum
acima de 20 ms. O quadro denso fica perto do limite de 100 ms já no Chromium.
Medir no WebKitGTK nativo é obrigatório antes de qualquer conclusão.

## Guardrails de privacidade

Executados no seam de portas, nesta rodada, com saída em [guardrails.txt](guardrails.txt): incluindo
os da tabela do [ticket 10](../../cinematic-panel-implementation/validation/README.md#guardrails-de-privacidade)
e `own_token_and_commit_content_never_reach_cache_or_logs`,
`forwarding_strips_content_and_ignores_unrelated_notifications`,
`disconnect_without_keyring_is_refused`. As correções deste ticket não tocam o núcleo.

## Pendências nativas

Não executadas nesta entrega e **não declaradas aprovadas**:

- Janela Tauri real: dimensões lógicas/escala, monitor, área útil, abertura pelo
  Indicador (menu, duplo clique, clique do meio), ocultação por Escape/perda de
  foco, segunda instância abrindo o Painel existente, alternância de tema pelo
  portal e texto ampliado do GNOME.
- Comparação visual nativa com B e o quadro aprovado; arraste real no
  WebKitGTK; Escape com o seletor nativo aberto; notificação real direcionada.
- Resposta ≤ 100 ms e 60 fps no WebKitGTK com traces de quadros; A/B de CPU
  (três pares de 60 s, ≤ 2 p.p. de um núcleo) com o
  [coletor existente](../../cinematic-panel-implementation/performance/README.md);
  trace com Painel oculto, redução manual/sistema e memória/destruição do webview.
