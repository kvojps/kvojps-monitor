# Validação integrada — ticket 10

Registro de 2026-10-31, baseline `c69ad108ee72881defa18b76c195b48ef44c28c3`.
O ticket foi resolvido por instrução explícita do usuário em 2026-10-31.
As lacunas de evidência nativa e comparação visual integral abaixo permanecem
registradas; não foram declaradas executadas ou aprovadas.

## Reprodução

A superfície `index.html` monta o Painel real com as fixtures existentes, sem
comandos Tauri, credenciais, rede externa ou coleta. Apenas para desenvolvimento;
não é uma entrada do build de produção. Cada parâmetro escolhe uma fixture:
`?theme=light&ai=claude-usage&github=invalid-credential&network=vpn`.

Execute `npm run dev` e abra
`http://localhost:1420/.scratch/cinematic-panel-implementation/validation/index.html`.
Para repetir a varredura, inicie um Chromium descartável:

```sh
chromium --headless --disable-gpu --no-first-run --user-data-dir=/tmp/kvojps-validation-chromium --remote-debugging-port=9222 about:blank
node .scratch/cinematic-panel-implementation/validation/audit.mjs > /tmp/panel-matrix.json
```

O script usa Node 24 e CDP, sem dependências novas. Usa a primeira página do
navegador: execute somente com o perfil descartável indicado. As capturas ficam
em `/tmp/panel-{tema}-{largura}.png`. O relatório preservado é `matrix.json`.

## Condições e resultados

Chromium headless local, viewport de 1000 px de altura, larguras de 1440, 800,
560 e 900 px. Nesta última, densidade emulada 2; demais, 1. Ubuntu Sans/Ubuntu
ou fallback local da pilha do app. Movimento reduzido para capturas estáveis.
Texto a 200% é ampliação uniforme dos tamanhos computados, sem ampliação
cumulativa por herança; não equivale à escala nativa GNOME.

183 cenários: 61 por tema escuro/claro/sistema. Cada tema percorre todas as
fixtures dos seis Módulos a 560 px, as quatro larguras, texto a 200% e
Configurações. Sistema usou a preferência clara da webview nesta execução;
alternância real do portal ainda depende da validação nativa.

Nenhum overflow horizontal nos contêineres Estação/Módulo/Rede/Configurações
verificados. Rolagem intencional do quadro Trello é permitida. Isso não prova
que cada texto truncado ou sobreposição interna seja aceitável: inspeção visual
integral continua necessária. A matriz de estados e recuperações também é
exercitada pelos testes de UI de cada Módulo, além da renderização integrada.

A varredura calcula luminância relativa sRGB e razão WCAG do texto sobre as
superfícies computadas, incluindo composição de opacidade. Exclui decoração,
SVG, controles desabilitados, valores/placeholders de campos de entrada e texto
oculto para leitores de tela. Os mínimos de Configurações cobrem os textos
medidos, não os valores/placeholders dos campos. Não é uma
auditoria completa de acessibilidade, nem inspeciona antialiasing, gradientes ou
todos os detalhes da árvore acessível nativa.

| Medição | Escuro | Claro |
| --- | --- | --- |
| Menor contraste textual na matriz | 6,15:1 | 4,89:1 |
| Menor contraste textual em Configurações | 6,17:1 | 5,92:1 |
| Borda dos campos sobre superfície | 5,69:1 | 4,52:1 |
| Acento dos gráficos sobre superfície secundária | 11,39:1 | 4,89:1 |
| Série secundária sobre superfície secundária | 5,32:1 | 4,32:1 |

As três últimas linhas são cálculos diretos dos tokens sRGB. Faixas estruturais,
grade e células graduadas do mapa não são controles; os gráficos mantêm resumo
textual e rótulos das séries. A cor da Estação continua independente do estado.

Correções: substituída por tokens secundários com contraste a opacidade que reduzia texto de cache/credencial inválida a
3,04:1 no escuro e 2,91:1 no claro; contadores de Agentes passaram de 1,82:1 no
claro para token textual apropriado. Gráfico GitHub e upload da Rede agora têm
cores com contraste no claro, sem alterar as faixas estruturais. Dados com
credencial inválida mantêm tratamento visual secundário e idade conforme a v1. Campos de
Configurações têm borda identificável. O botão de teste de velocidade descreve
valores/unidades ou ausência de medição, além da idade, para tecnologia assistiva;
a idade continua disponível por foco e ponteiro.

Testes integrados confirmam ordem das Estações, teclado, nomes/valores,
recuperação de uma fonte sem afetar as outras, retorno de Configurações e
preservação de foco, seleção e rolagem. Os testes dos Módulos cobrem offline
com cache/idade sem faixa global de erro, estados específicos e recuperações.

## Comparação com H

A branch local `prototype/cinematic-panel-composition` (apagada em 2026-11-01) foi extraída em
`/tmp/kvojps-reference-source` e servida em modo dev, com dependências locais.
A referência só monta em `import.meta.env.DEV`: seu build estático é vazio e
não pode servir de evidência. Capturas em `/tmp/reference-dark-{largura}.png`.

Inspeção das capturas escuras de 1440 px: preservadas as três Estações, faixas,
curvas, hierarquia dos instrumentos e ordem dos Módulos. Diferenças de dados e
altura decorrem das fixtures reais e dos estados, sem usar demonstrações como
leituras. O laboratório da referência tem controles e navegação próprios.
Comparação integral lado a lado em claro/sistema, segunda fileira, tamanhos
intermediário/560 px e escalas ainda não foi concluída. Não há aceite visual
integral declarado neste registro.

## Guardrails de privacidade

Cobertura existente, executada nesta rodada no seam de portas do núcleo:

| Regra | Testes explícitos |
| --- | --- |
| Só salvar no keyring; recusar sem ele | `github::credentials_tests::validated_token_is_saved_only_in_keyring_and_publishes_identity`, `unavailable_keyring_refuses_saving_and_keeps_setup_state`; `trello::tests::without_keyring_connecting_is_refused` |
| Cache/logs sem credenciais ou conteúdo | `persistence::tests::disk_cache_restores_counts_but_never_credentials_or_commit_content`, `info_logs_have_only_metadata_and_rotate_with_three_one_megabyte_files`; `trello::tests::restart_restores_last_board_without_card_titles_or_credentials` |
| Notificações só com metadados | `agents::tests::waiting_notification_uses_only_session_metadata_and_replaces_previous`, `hook_content_never_reaches_notifications_or_the_published_snapshot` |
| Alterações explícitas, aditivas e reversíveis | `ai::claude::tests::settings_never_change_without_the_click`, `connecting_chains_the_existing_status_line_and_disconnecting_restores_it_exactly`; `agents::hooks::tests::installation_requires_a_command_and_removal_preserves_third_party_hooks` |
| Token Claude só em memória, sem renovação | `ai::claude::tests::usage::the_token_is_only_read_never_renewed_written_or_kept`, `restart_shows_the_cached_reading_and_never_the_token` |

## Verificações

- `npm test`: 212 testes em 12 arquivos aprovados.
- `npm run check`: zero erros/avisos.
- `npm run build`: build de produção aprovado.
- `cargo test --manifest-path src-tauri/Cargo.toml`: 271 testes unitários aprovados;
  dois testes de sockets bloqueados pelo sandbox. Reexecução de `--test hook_mode`
  fora do sandbox: três aprovados. Total: 274.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`: aprovado.
- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`: aprovado após corrigir
  formatação preexistente da lista de fixtures de Sistema.
- `git diff --check`: aprovado. Não há script de lint/formatação JS no projeto.

## Aceitação humana pendente

A sessão informa Wayland e `DISPLAY=:0`; isso não comprova interação nativa.
O usuário respondeu “ok” à solicitação de conferência, sem resultados observados.
Ainda registrar: versão do GNOME/compositor, monitor/escala, Indicador, segunda
instância, área útil/posição, Escape/perda de foco, retorno de Configurações,
notificação direcionada e eventuais limitações reais do compositor. Nenhuma dessas
verificações foi marcada como aprovada por equivalência ao Chromium.

Também permanecem os sinais `SettingChanged` reais e atualização ao vivo Tauri,
autostart no login, movimentação do AppImage e abertura de logs/link. A resposta
positiva de release depende de existir publicação deste app. Não alterar outro
repositório para fabricar evidência. CPU/fps/memória pertencem ao ticket 11.

## Revisão em dois eixos

Padrões: zero violações documentadas e zero smells; uma limitação de cobertura
identificada (valores/placeholders dos inputs), explicitada acima. Spec: um
apontamento sobre a exigência de esmaecer dados com credencial inválida; corrigido
com tokens secundários legíveis, preservando idade e sinalização de estado.
Os itens de aceitação humana continuam pendentes.
