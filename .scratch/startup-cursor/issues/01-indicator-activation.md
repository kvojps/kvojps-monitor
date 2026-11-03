# Cursor de espera ao abrir o Painel pelo Indicador

Status: ready-for-agent
Type: task

## Sintoma e escopo confirmado

Na versão `.deb` 0.1.0, abrir o Painel pelo Indicador deixa o cursor em espera,
embora a janela já responda a cliques. O usuário relata que a espera termina
quando o aplicativo aparece na doca. Corrigir essa espera residual e preparar
0.1.1; preservar a composição de abertura e o carregamento dos Módulos.

## Reprodução

`bash scripts/diagnose-startup-cursor.sh`: duas respostas `s`, confirmadas pelo
usuário numa nova abertura, produziram código 1 e
`FAIL: panel responsive, startup cursor still busy`.

O script registra somente observações booleanas. Não captura tela, processos,
credenciais, sessões, rede ou conteúdo das fontes.

## Investigação

- O launcher instalado não define `StartupNotify`; ainda não há evidência de
  que alterar o launcher resolveria o acionamento pelo Indicador.
- A extensão AppIndicator instalada fornece um token de ativação antes de
  chamar os eventos do Indicador. `ksni` 0.3.7 disponibiliza esse token no
  contexto do callback; o aplicativo não o consultava.
- A API de `ksni` orienta encaminhá-lo ao toolkit antes de apresentar a janela.
  A implementação GTK 3.24 de Wayland utiliza o identificador de inicialização
  na ativação da superfície.
- A correção encaminha o token do evento para a janela GTK na thread principal,
  tanto ao criar quanto ao reapresentar o Painel. O valor permanece em memória.
  Eventos sem token continuam abrindo o Painel normalmente.

Referências primárias:

- [ksni: token do evento](https://docs.rs/ksni/0.3.7/ksni/fn.current_xdg_activation_token.html)
- [GTK: apresentação Wayland](https://github.com/GNOME/gtk/blob/gtk-3-24/gdk/wayland/gdkwindow-wayland.c)

## Validação

Verificações concluídas:

- `cargo fmt --check`: aprovado.
- `npm run check`: zero erros e avisos.
- `npm test`: 376 testes aprovados.
- `npm run build`: aprovado.
- `cargo clippy --all-targets --locked -- -D warnings`: aprovado.
- `cargo test --locked`: 305 testes de biblioteca aprovados; dois testes de
  integração foram bloqueados pelo sandbox ao criar sockets Unix. Reexecutar
  `cargo test --locked --test hook_mode` fora do sandbox aprovou os três testes
  de integração.
- Clippy com `features fixtures`: aprovado; 15 testes filtrados de fixtures
  aprovados.
- `python3 scripts/check-release-version.py v0.1.1`: aprovado.
- Script de diagnóstico: resultados FAIL, PASS e inconclusivo verificados com
  entradas sintéticas, separadamente da reprodução observada pelo usuário.

Pacote `.deb` 0.1.1 amd64 gerado e inspecionado. O launcher mantém o contrato
da versão instalada. O executável extraído desse pacote foi iniciado para
validar o cursor no ambiente afetado. O usuário confirmou que o cursor ficou
normal; a repetição do script com respostas `s` e `n` produziu código 0 e
`PASS: panel responsive, no residual startup cursor`.
O binário local contém caminhos da máquina de compilação e não será enviado
à release pública; os artefatos públicos serão gerados pelo workflow de release.
A suíte atual não oferece um seam que observe o cursor do compositor a partir
de um evento real do AppIndicator. Um teste unitário de encaminhamento isolado
não comprovaria o desaparecimento do sintoma; a confirmação final usa o script
com observação humana no ambiente afetado.

Revisão de sanitização: somente caminhos do projeto, versões públicas e
observações booleanas; sem capturas ou identificadores privados. Não há uma
decisão de domínio nova que justifique alterar o glossário ou criar ADR.

Revisão pré-publicação: os arquivos alterados são código, script de diagnóstico,
manifestos de versão, lockfiles e este ticket. O diff foi revisado integralmente;
não há imagens, fixtures ou binários novos selecionados para Git. Identidades
de autor e committer usam noreply do GitHub. A branch é `main` e, antes deste
trabalho, não havia commits locais pendentes de envio. Publicação 0.1.1 pendente.
