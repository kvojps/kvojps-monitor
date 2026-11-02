# Validação da abertura nativa — ticket 37

Rodada de 02/11/2026. **Ticket aprovado e encerrado por decisão explícita do
usuário em 02/11/2026**, com todo o checklist marcado. A validação técnica desta
rodada permanece incompleta; a aprovação aceita as limitações abaixo e não
acrescenta evidências nativas. Não houve
alteração de produção nem de testes. As pendências dos tickets 25 e 33 permanecem
intactas.

## Ambiente

- Fonte: `21dff9043fcf4b2c0f737b2de117787c7b6a161a`.
- Ubuntu 26.04.1 LTS; sessão declarada Wayland, `wayland-0`, `DISPLAY=:0`.
- Rust 1.99.1; Tauri 2.12.5 e Wry 0.57.1 (Cargo.lock);
  WebKitGTK 2.52.6 (`pkg-config --modversion webkit2gtk-4.1`).
- Build debug com `cargo build --manifest-path src-tauri/Cargo.toml --features fixtures`.
- SHA-256 do executável:
  `82caf2f36023dbd493c4c8c3fd3c37b663a2ca29f3321805d1fe625428aeedd9`.
- Monitor, resolução, Hz, escala, perfil energético e versão do GNOME não foram
  aferidos nesta rodada. Não se trata de uma medição de desempenho.

## Tentativa nativa e resultado

O sandbox bloqueou sockets do Vite e do D-Bus. Após autorização para executar
fora dele, o Vite informou porta 1420 ocupada. O servidor existente não foi
encerrado; sua versão efetivamente servida não foi verificada.

O executável com fixtures iniciou com `dbus-run-session` no compositor Wayland
da sessão. A sessão D-Bus isolada não tinha `org.kde.StatusNotifierWatcher`;
o registro do Indicador falhou. Isso impede comprovar os fluxos reais pelo
Indicador nessa configuração.

Na mesma sessão D-Bus da instância de fixtures, foi executado:

```sh
scripts/capture-panel.sh "$PWD/.scratch/kvojps-monitor/validation/37/1100x700-dark" 1100x700 dark
```

Resultado bruto: [1100x700-dark.json](1100x700-dark.json), com erro
`metrics script did not answer`. Não houve PNG, viewport confirmado ou métricas
de abertura. O erro comprova somente falha da tentativa de captura; não identifica
se a causa está na página servida, no ciclo da janela ou no coletor. Não é prova
de desvio do produto nem de aprovação visual. O roteiro parou nessa falha;
1000 × 700 claro e 600 × 700 sistema não foram executados. As instâncias criadas
para esta rodada foram encerradas, preservando instâncias pessoais preexistentes.

O modo de fixtures desabilita coletores reais e usa configuração temporária;
não fornece evidência de rede, keyring, login automático ou notificações reais.
Não foram preservados logs integrais de processos/portais, capturas de dados
pessoais ou conteúdo de fontes. O único artefato bruto é o erro acima.

## Verificações automatizadas

| Comando | Resultado |
| --- | --- |
| `npm run check` | Zero erros e avisos |
| `npx vitest run src/lib/PanelOpening.test.ts` | 15 testes aprovados |
| `npm test` | 368 testes aprovados em 16 arquivos |
| `npm run build` | Aprovado |
| `cargo test --manifest-path src-tauri/Cargo.toml` fora do sandbox | 305 unitários e 3 de integração aprovados; zero doc-tests |
| `cargo build --manifest-path src-tauri/Cargo.toml --features fixtures` | Aprovado |

A primeira execução Rust no sandbox passou nos 305 unitários, mas falhou em dois
testes de integração por `Operation not permitted` ao criar sockets Unix.
A repetição autorizada fora do sandbox passou integralmente.

Cobertura reaproveitada nos seams existentes (portas injetáveis no núcleo e
componentes com fixtures/relógio na UI):

- `src/lib/PanelOpening.test.ts`: mínimo de 1200 ms desde apresentação,
  preparo de 3000 ms seguido apenas da transição de 240 ms, recuperação por
  timeout/rejeição, descarte tardio, remount, Escape e limpeza de timers,
  falha fatal com tentar novamente/fechar, atualização remota fora da prontidão,
  recuperação parcial e boas-vindas sem comandos de credenciais/hooks.
- O mesmo arquivo verifica temas, redução manual/do sistema, pausa da decoração
  durante abertura, conteúdo inerte até a revelação e substituição da composição
  inicial sem identidade duplicada. São testes de DOM, não de primeiro frame.
- Suíte Rust: recusa de credenciais sem keyring; logs/cache/notificações sem
  conteúdo sensível; hooks por comando explícito, aditivos e reversíveis;
  token Claude em memória e sem renovação. A aprovação é dos contratos sob
  fixtures, não uma inspeção de credenciais pessoais.

## Evidências nativas não coletadas

Todos os cenários do [ticket 37](../../issues/37-validacao-abertura-nativa.md)
não foram comprovados integralmente nesta rodada:

| Área | Evidência necessária |
| --- | --- |
| Ciclo | Primeira abertura, reaberturas, solicitação repetida, fechar durante preparo/transição, reabrir imediatamente, login silencioso e destruição da webview |
| Tempo | Primeiro frame visível e duração total ≥ 1,2 s; preparo rápido, 3 s e limite de 5 s; ausência de mínimo adicional após demora |
| Recuperação | Sem rede/cache, preferências indisponíveis, recuperação por área, falha fatal, tentar novamente/fechar, respostas tardias e atualização remota independente |
| Visual e destino | Arquitetura C desde o primeiro frame, nome e mensagens, tema escolhido, Âncora, notificação real e boas-vindas |
| Acessibilidade | Claro/escuro/sistema, 1100 × 700, 1000 × 700, largura reduzida, texto ampliado, contraste, teclado, foco, leitor de tela e redução manual/do sistema |
| Desempenho | Release com fontes reais, ambiente controlado, latência máxima/p95 ≤ 100 ms, quadros a 60 Hz, três pares de CPU com média adicional ≤ 2 p.p. de um núcleo, traces, PSS aberto/fechado/reaberto e ausência de trabalho decorativo após fechar |

Retomar na sessão GNOME normal com interação gráfica e leitor de tela,
seguindo [desempenho.md](../../../../docs/desempenho.md) e
[motion-trace.md](../../../../docs/motion-trace.md). Registrar hash do release,
fontes apenas como metadados, configuração do monitor e traces sanitizados.
Não há tempos nativos, CPU, PSS, fps ou aprovação visual nesta rodada; testes,
protótipo e navegador não substituem essas evidências.
