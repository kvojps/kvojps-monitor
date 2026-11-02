# Validar sequência visual cinematográfica de abertura

Labels: wayfinder:prototype
Type: prototype
Status: resolved
Blocked by: 01

## Question

Como deve ser a sequência visual dentro do Painel para transmitir a estética tech e cinematográfica desejada, respeitando o contrato decidido em [Definir prontidão e ciclo de abertura do Painel](01-opening-readiness.md)?

Criar com a skill prototype um artefato descartável para reação do usuário e vinculá-lo aqui. Explorar composição, linhas luminosas, mensagens ligadas ao preparo real, cadência no mínimo de 1,2 segundo, continuidade quando houver demora e transição para o conteúdo. Validar legibilidade, temas claro/escuro e movimento reduzido sem presumir a aprovação estética do usuário.

Resolver apenas por troca com o usuário; registrar a direção aprovada e os critérios que orientarão a implementação posterior. O protótipo não é a implementação de produção.

## Comments

### Protótipo para avaliação visual

Artefato descartável: [OpeningPrototype.svelte](../../../src/lib/opening-prototype/OpeningPrototype.svelte), montado sobre o componente real do Painel com fixtures locais e portas sem mutações. Executar `npm run prototype:opening`; abrir `http://127.0.0.1:1422/opening-prototype.html?variant=A`.

- **A — Convergência**: identidade central, linhas diagonais convergentes.
- **B — Circuito lateral**: identidade à esquerda, trilho vertical e ramificações luminosas.
- **C — Arquitetura**: identidade na base, malha em perspectiva ocupando a janela.

Setas na barra inferior ou no teclado alternam `?variant=A`, `B` e `C`. Controles externos ao Painel permitem repetir, fechar, selecionar tema claro, movimento reduzido e preparo rápido, demorado, limite local ou falha da interface. Estado e tempo são expostos para discussão. As mensagens acompanham etapas simuladas; os controles não acionam E/S nativa. O cenário de limite local ilustra o aviso de recuperação; não implementa a recuperação real por área.

Verificação: `npm run check` passou sem erros ou avisos. Avaliação visual pelo usuário pendente; nenhuma direção aprovada. Após a escolha, preservar os artefatos em branch local `prototype/*`, conforme a skill e AGENTS.md; sem promover código de protótipo para produção. A spec permanece para o handoff.


## Answer

Direção aprovada pelo usuário: **C — Arquitetura**, com a correção explícita do nome exibido para **kvojps monitor**.

### Critérios visuais para o handoff

- Adotar a composição de Arquitetura: malha de linhas luminosas em perspectiva sobre fundo escuro, com a identidade na região inferior esquerda.
- Exibir o nome **kvojps monitor**, nessa grafia, em lugar do título genérico “PAINEL” da primeira versão do protótipo. A correção se refere à identidade na abertura; não renomeia o conceito de domínio Painel.
- Preservar as mensagens vinculadas ao preparo real, a passagem suave para os Módulos e o contrato completo de [Definir prontidão e ciclo de abertura do Painel](01-opening-readiness.md).
- A abertura permanece escura também quando o Painel usa tema claro; a transição revela o tema escolhido.
- Com movimento reduzido, apresentar a mesma composição estática, sem animação das linhas nem transição animada, conforme o contrato já acordado.

### Fonte e limites

Protótipo preservado na branch local `prototype/panel-opening`, com entrada `opening-prototype.html` e comando `npm run prototype:opening`. Sem push. A preferência C e o nome foram confirmados diretamente pelo usuário; não houve solicitação de combinar outras variantes.

A correção do nome e a seleção padrão C foram aplicadas ao artefato descartável. O protótipo serve como referência de composição, não como implementação do ciclo real de prontidão. A revisão da história 12 e a implementação ficam para o handoff, fora deste mapa.
