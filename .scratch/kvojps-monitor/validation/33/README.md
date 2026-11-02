# Validação integrada do Painel — ticket 33

Registro de 02/11/2026, aproximadamente 03:23–03:26 UTC. **Aceite parcial; ticket aberto.**
Não há alteração de código de produção nesta rodada. O ticket 25 permanece intacto.

## Ambiente e reprodução

- Fonte: `b85b426a389b0a1228935d9f6ef41c5fd21e4ba1` (as alterações posteriores desta rodada são testes e evidências).
- Ubuntu 26.04.1 LTS, GNOME Shell 50.1, sessão Wayland (`wayland-0`).
- WebKitGTK 2.52.6; Tauri 2.12.5; Wry 0.57.1; Rust 1.99.1.
- Build **debug**, `cargo build --manifest-path src-tauri/Cargo.toml --features fixtures`, servidor Vite existente na porta 1420.
- SHA-256 do executável capturado: `f13496bea4ef0e9b58a31c11512e8cfc84a36b23b779aaa0afadcf3586db4483`.
- Viewport lógico conferido pelo próprio WebKit; `devicePixelRatio = 1`. Monitor, Hz e perfil energético não foram aferidos; esta rodada não é medição de desempenho.

A tentativa de `npm run dev:fixtures` encontrou a porta 1420 ocupada. A instância
pessoal já existente não respondeu ao comando de captura em 40 s. Ela não foi
encerrada. A captura foi então feita com o executável acima em `dbus-run-session`,
mantendo o compositor Wayland da sessão real e isolando a instância de fixtures.
O processo criado para a validação foi encerrado ao fim das capturas.

Para reproduzir, iniciar o binário com fixtures dentro de uma sessão D-Bus separada
e, **na mesma sessão D-Bus**, executar:

```sh
scripts/capture-panel.sh /tmp/ticket33-1100x700-dark 1100x700 dark
```

Repetir para `1100x700`, `1000x700`, `600x700` e `dark`, `light`, `system`.
Os arquivos JSON preservam a resposta bruta, inclusive o caminho original em
`/tmp`; os PNG correspondentes estão nesta pasta com o mesmo tamanho/tema no nome.
As capturas usam exclusivamente fixtures, sem coletores, credenciais, hooks,
notificações ou dados pessoais. A preferência de aparência fica na configuração
temporária do modo de fixtures.

## Composição nativa observada

| Viewport | Temas solicitados | Área rolável (largura × altura visível/conteúdo) | Resultado |
| --- | --- | --- | --- |
| 1100 × 700 | escuro, claro, sistema | 1064 × 605/605 | Sem rolagem ou transbordamento |
| 1000 × 700 | escuro, claro, sistema | 964 × 605/605 | Sem rolagem ou transbordamento |
| 600 × 700 | escuro, claro, sistema | 592 × 609/1603 | Rolagem vertical; sem transbordamento horizontal |

Todos os relatórios têm `ready: true`, fontes carregadas e CSS dos seis Módulos e
da Faixa de bordo. O documento mantém exatamente a largura/altura solicitadas.
Inspeção visual de [1100 escuro](1100x700-dark.png),
[1000 claro](1000x700-light.png) e [600 claro](600x700-light.png): assinatura e
faixa em repouso presentes; nos dois alvos largos os instrumentos, alertas e ações
cabem; em largura reduzida o conteúdo segue na área com rolagem. Não houve
inspeção visual individual das outras seis imagens, cujas métricas também foram
conferidas. A faixa estava em repouso nas nove capturas; isso **não comprova** a
geometria com seis mensagens simultâneas ou texto ampliado.

`system` resultou em escuro nas três capturas. O D-Bus separado não possui o
StatusNotifierWatcher do GNOME e os portais registraram ausência do serviço
Mutter.ServiceChannel. Portanto isso comprova a renderização da opção restaurada,
mas **não** comprova acompanhar a troca de tema no portal da sessão pessoal,
abertura pelo Indicador nem o comportamento completo da integração GNOME.

## Evidência automatizada

Seams preexistentes da spec: componentes com snapshots e relógio falso; núcleo
com portas de E/S e fixtures. O novo teste em `src/lib/Panel.test.ts`,
`keeps six modules accessible while simultaneous results expire independently of a long query`,
publica os seis snapshots no mesmo lote: consulta Codex longa, sessão aguardando,
falha GitHub, pausa Trello, teste de velocidade cancelado e alertas de Sistema.
Confere prioridade e `+3 Módulos`, região de anúncio agrupado `polite`/atômica,
permanência até 3999 ms e expiração aos 4000 ms, explicação persistente da falha,
ações acessíveis, foco preservado e ausência de sucesso fabricado após mais 60 s.
É um teste de validação do comportamento existente; não foi necessária correção
de produção. Falhas durante sua construção vieram do formato de fixture e da
sincronização do próprio teste, não de desvios comprovados no app.

Cobertura reaproveitada:

- `OperationsStrip.test.ts`: fontes e provedores independentes, resultado real,
  operação rápida/longa, cancelamento, repetição, referência inicial/reabertura,
  prioridade, agrupamento e sentinelas de conteúdo sensível.
- `Panel.test.ts` e testes por Módulo: indisponibilidade total/parcial, leitura
  preservada entre fontes, detalhes e ações por teclado, foco, explicação após a
  faixa expirar, pausa em Configurações e visibilidade nativa.
- `Settings.test.ts`: redução manual e mudanças da preferência do sistema.
- Núcleo: respostas descartadas de IA/Trello/Rede, relógio falso, cache/logs e
  notificações sanitizados; recusa de credenciais sem keyring; alterações de
  hooks/status line somente por comando explícito e preservando terceiros;
  token Claude em memória e sem renovação.

Resultados da rodada:

- `npm run check`: zero erros e avisos.
- `cargo test --manifest-path src-tauri/Cargo.toml`: 305 testes unitários e três
  de integração de hooks aprovados; nenhum doc-test. A execução inicial no sandbox
  bloqueou dois binds de socket Unix com `Operation not permitted`; a repetição
  fora do sandbox passou integralmente.
- `npx vitest run src/lib/Panel.test.ts src/lib/OperationsStrip.test.ts`: 71 testes aprovados.
- `npm test` final: 353 testes aprovados em 15 arquivos.

Esses testes não executam o compositor, não medem contraste renderizado ou fps e
não comprovam anúncios efetivamente falados por leitor de tela.

## Aceite ainda necessário na sessão pessoal

| Critério | Evidência faltante |
| --- | --- |
| Composição | Texto ampliado do GNOME; seis mensagens simultâneas no app nativo; temas claro/escuro do portal ao vivo; acesso a todos os detalhes após rolagem |
| Operações reais | Rápida/longa, falha, pausa, recuperação, cancelamento, descarte, independência e ausência total/parcial; explicações persistentes e referência atual ao reabrir |
| Acessibilidade | Percurso de teclado, foco e ordem de leitura nativos; leitor de tela com anúncios agrupados não interruptivos; contraste AA de textos e controles nos estados e temas |
| Movimento e ciclo de vida | Redução manual/do sistema ao vivo, Configurações, fechamento por Esc/Indicador/perda de foco, destruição de processos da webview, reabertura sem eventos antigos |
| Desempenho | Build release com fontes reais, latência máxima/p95 ≤ 100 ms, quadros a 60 Hz, três pares A/B de CPU de 60 s com média ≤ 2 p.p. de um núcleo, ausência de animação oculta, PSS aberto/fechado/reaberto |

Não foram coletados números de CPU/PSS, traces, latência ou fps. O modo de fixtures
e o build debug não atendem à preparação do [protocolo](../../../../docs/desempenho.md).
Não há ferramenta de interação gráfica/leitor de tela nesta execução para
concluir o percurso manual, nem controle validado das preferências/Indicador na
sessão pessoal. Não se substituiu essa evidência por navegador ou aprovação de
protótipo. Retomar na sessão GNOME normal seguindo o protocolo e
[motion-trace.md](../../../../docs/motion-trace.md), registrando versões, hardware,
hash do release, fontes habilitadas apenas como metadados e traces sanitizados.
Reaproveitar do ticket 25 somente os cenários pertinentes acima; suas demais
pendências não são pré-requisito para este ticket.
