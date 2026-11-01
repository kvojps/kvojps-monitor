# Organizar as Configurações

**Status:** ready-for-agent

## Problem Statement

As Configurações estão bagunçadas. Hoje são oito cartões numa grade automática (Boas-vindas, Geral, Aparência, IA, Agentes, GitHub, Trello, Rede), com alturas muito diferentes e uma ordem sem lógica, então a grade fica desalinhada. Os formulários de credencial ficam sempre abertos, mesmo com o Módulo já conectado:

- O GitHub mostra o campo de token mesmo quando funciona via `gh` e repete "conectado".
- O Trello mostra o passo a passo de chave e token inteiro mesmo com conta e quadro escolhidos.

Cada seção monta título, estado, erro e interruptores de um jeito diferente. Tudo fica numa rolagem só, sem índice, e o atalho "Configurar" apenas rola até o cartão. A seção Rede some enquanto não chega o primeiro snapshot.

## Solution

As Configurações passam a ter um **índice à esquerda** e **uma seção por vez à direita**, com o cabeçalho "CONFIGURAÇÕES" e "← Painel".

- O índice tem dois grupos: **App** (Geral, Aparência) e **Módulos** (IA, Agentes, GitHub, Trello, Rede).
- Um Módulo que precisa configurar ganha uma marca discreta "!" no índice.
- Todas as seções seguem a mesma anatomia: título → linha de estado → ações → preferências → erro junto do controle que falhou.
- Quando um Módulo já está conectado, a seção mostra só um resumo e as ações (Trocar/Desconectar). O formulário de credencial aparece sob demanda ou quando falta credencial.
- Na primeira abertura, as Boas-vindas ocupam o lado direito e somem assim que uma seção é escolhida.
- Em janela estreita, o índice vira uma fileira horizontal acima da seção.

## User Stories

1. Como usuário, quero um índice fixo à esquerda das Configurações, para achar qualquer seção sem rolar a tela inteira.
2. Como usuário, quero o índice dividido em "App" (Geral, Aparência) e "Módulos" (IA, Agentes, GitHub, Trello, Rede), para distinguir preferências do app de configurações de fontes de dados.
3. Como usuário, quero ver só a seção escolhida no lado direito, para não me perder numa grade de cartões desalinhados.
4. Como usuário, quero que a seção selecionada fique destacada no índice, para saber onde estou.
5. Como usuário, quero que entrar nas Configurações pelo botão do topo abra sempre Geral, para ter um ponto de partida previsível.
6. Como usuário, quero que "Configurar" num Módulo do Painel abra direto a seção daquele Módulo, para resolver o problema sem procurar.
7. Como usuário, quero uma marca "!" no índice ao lado de cada Módulo que precisa configurar, para ver de relance o que falta.
8. Como usuário, não quero marcas de "ok" no índice, para que ele fique limpo e só chame atenção quando algo exige ação.
9. Como novo usuário, quero ver as Boas-vindas no lado direito na primeira abertura, com o índice já visível, para entender o que configurar e como navegar.
10. Como novo usuário, quero que "Configurar X" nas Boas-vindas selecione a seção X no índice, para seguir direto ao passo certo.
11. Como novo usuário, quero que as Boas-vindas sumam de vez quando eu escolher qualquer seção, para que a tela única não volte a ocupar espaço; as marcas "!" passam a guiar o resto.
12. Como novo usuário, quero manter o atalho "Conectar ao Claude Code" nas Boas-vindas, para conectar o Claude num clique.
13. Como usuário em janela estreita (até 650 px), quero o índice como uma fileira horizontal rolável acima da seção, para continuar navegando sem rolagem horizontal da página.
14. Como usuário, quero que toda seção comece com uma linha de estado (ponto colorido + texto), para saber a situação daquele Módulo antes de qualquer ação.
15. Como usuário, quero que os erros apareçam logo abaixo do controle que falhou, para entender o que deu errado e onde.
16. Como usuário, quero que interruptores, botões e campos tenham a mesma aparência em todas as seções, para que a tela pareça um produto só.
17. Como usuário, quero textos explicativos curtos e só onde ajudam a decidir, para ler menos.
18. Como usuário, quero a seção IA dividida em um bloco por Provedor de IA (Claude e Codex), cada um com sua linha de estado, para ver a situação de cada provedor separadamente.
19. Como usuário do Claude, quero ver no bloco Claude se a status line está conectada, desconectada ou trocada por terceiros, com Conectar, Desconectar ou Reconectar conforme o caso, para controlar a fonte padrão dos Limites de uso.
20. Como usuário do Claude, quero o opt-in `oauth/usage` dentro do bloco Claude, abaixo da status line, com o mesmo aviso de hoje antes de ativar, para decidir conscientemente.
21. Como usuário do Codex, quero ver no bloco Codex só a linha de estado (detectado · plano, ou não encontrado), porque não há nada para configurar ali.
22. Como usuário do GitHub via `gh`, quero ver uma linha "Conectado via gh · login" e nada de campo de token, para não achar que preciso fazer algo.
23. Como usuário do GitHub, quero o botão "Usar token próprio" que abre o formulário de token só quando eu pedir, para usar um token clássico se quiser.
24. Como usuário do GitHub com token próprio, quero ver "Token próprio · login" uma única vez e o botão "Trocar token", para não ver o estado repetido.
25. Como usuário sem credencial do GitHub (`gh` ausente ou inválido), quero o formulário de token já aberto, junto da instrução para usar o `gh`, para configurar sem cliques extras.
26. Como usuário do Trello conectado, quero ver "Conta X · quadro Y" com os botões "Trocar quadro" e "Desconectar", sem o passo a passo de credencial, para que a seção fique enxuta.
27. Como usuário do Trello, quero que "Trocar quadro" abra a lista de quadros, para mudar o quadro exibido só quando quiser.
28. Como usuário do Trello sem credencial, quero o passo a passo numerado (1. chave, 2. token), para seguir a ordem certa.
29. Como usuário do Trello, quero que a lista de quadros já venha aberta depois de validar a credencial ou quando falta escolher o quadro, para concluir a configuração sem procurar.
30. Como usuário do Trello, quero que a chave digitada continue no campo quando o Painel fecha durante a geração do token no navegador, como hoje.
31. Como usuário de Agentes, quero a Detecção instantânea como primeiro controle da seção, com a nota de confiança do `/hooks` do Codex quando ligada, para entender o passo extra.
32. Como usuário de Agentes, quero os três interruptores de notificação agrupados sob "Notificações", para ver de uma vez o que me avisa.
33. Como usuário, quero a seção Rede sempre no índice, com o interruptor de teste de velocidade automático desabilitado até chegar o primeiro snapshot, para que ela não suma.
34. Como usuário, não quero Sistema no índice enquanto ele não tiver nada configurável, para não abrir uma seção vazia.
35. Como usuário, quero Geral com início com o login, checagem de nova versão, abrir pasta de logs e, no fim, a versão e a linha discreta de nova versão com link para o release.
36. Como usuário, quero Aparência com tema (escuro/claro/sistema) e redução de animações na mesma anatomia das demais seções.
37. Como usuário de teclado ou leitor de tela, quero navegar pelo índice com o teclado, que a seção selecionada seja anunciada e que o foco vá para a seção aberta, para usar as Configurações sem mouse.
38. Como usuário, quero que "← Painel" continue me levando de volta ao Painel de qualquer seção.
39. Como usuário, quero que todos os fluxos de hoje continuem funcionando (validação antes de gravar, falha não salva nada, credenciais só no keyring), para que a reorganização não mude comportamento.

## Implementation Decisions

- **Mudança só na UI**: nenhum comando, evento ou snapshot do núcleo muda. As seções continuam chamando as mesmas portas do Core. Não entra comando novo de remover o token do GitHub.
- **Container das Configurações**:
  - Mantém o índice (dois grupos) e a seção selecionada como estado local.
  - A API pública que o Painel usa para abrir as Configurações num Módulo (hoje "focar um Módulo") passa a selecionar a seção. Sem argumento, seleciona Geral.
  - A grade `auto-fill` de cartões desaparece.
- **Seções do índice**: `general`, `appearance`, `ai`, `agents`, `github`, `trello`, `network`. Sistema não entra.
- **Marca "!"**: derivada da regra de status que já existe para as Boas-vindas ("precisa configurar"). Uma única regra alimenta as Boas-vindas e o índice. Status "verificando", "detectado" e "não precisa" não mostram marca.
- **Boas-vindas**:
  - Continuam controladas pelo flag de primeira abertura que o Painel já recebe.
  - Enquanto ativas e sem seção escolhida, ocupam o lado direito.
  - Escolher qualquer seção (pelo índice ou por "Configurar X") encerra as Boas-vindas nesta abertura.
  - A persistência de "já vistas" continua como hoje.
- **Anatomia compartilhada**:
  - Peças reutilizáveis de linha de estado (ponto colorido por tom: ok, atenção, erro, neutro), interruptor com erro associado e mensagem de erro junto do controle.
  - Elas substituem o CSS global montado à mão no container.
  - Cada seção é escrita como título → estado → ações → preferências.
- **Seção IA**: um bloco por Provedor de IA. Os estados da status line do Claude (conectada, trocada, ausente) e do opt-in `oauth/usage` (desligado, ligado, expirado, aviso aberto) seguem as regras atuais, só reorganizados.
- **Seção GitHub**:
  - O formulário fica recolhido quando há credencial e abre por "Usar token próprio" (fonte `gh`) ou "Trocar token" (fonte token).
  - Fica aberto quando o estado é sem `gh` ou credencial inválida.
  - Depois de salvar com sucesso, recolhe e a linha de estado mostra o novo login.
- **Seção Trello**:
  - Estados de exibição: sem credencial (passo a passo numerado aberto), escolher quadro (lista aberta), conectado (resumo + Trocar quadro + Desconectar), lista aberta por pedido.
  - Escolher um quadro recolhe a lista.
  - Continua a regra de rádios em vez de `<select>` (o popup GTK fecha o Painel).
- **Seção Rede**: sai do container e vira um componente próprio. O interruptor fica desabilitado enquanto não há snapshot.
- **Layout estreito**: até 650 px (mesma referência do Painel), o índice vira uma fileira horizontal rolável acima da seção, sem rolagem horizontal da página.
- **Acessibilidade**:
  - O índice é uma lista de navegação. O item ativo usa `aria-current`. A marca "!" tem texto acessível ("precisa configurar").
  - Ao selecionar uma seção, o foco vai para o título dela.
- **Textos**: novos rótulos em pt-BR no catálogo de strings ("App", "Módulos", "Usar token próprio", "Trocar token", "Trocar quadro", "Notificações", texto acessível da marca).

## Testing Decisions

- **Um bom teste** verifica comportamento visível ao usuário: o que aparece, o que fica oculto, qual seção está ativa e quais comandos do Core são chamados. Nunca estrutura interna, classes CSS ou estado local.
- **Seam principal**: renderizar o Painel com o Core falso e os snapshots de fixture existentes, como o teste atual das Configurações. Ali entram:
  - entrar pelo botão do topo abre Geral;
  - "Configurar" de cada Módulo no Painel abre a seção certa;
  - primeira abertura mostra Boas-vindas, e escolher uma seção as encerra;
  - "Configurar X" nas Boas-vindas seleciona X;
  - marca "!" aparece só nos Módulos que precisam configurar (fixtures de IA desconectada, GitHub sem `gh`, Trello vazio ou escolhendo quadro);
  - Rede presente sem snapshot, com o interruptor desabilitado;
  - Sistema ausente do índice;
  - "← Painel" volta ao Painel;
  - foco e `aria-current` corretos.
- **Testes por seção** (os testes de seção de IA, Agentes, GitHub e Trello que já existem) continuam para os estados recolhido/aberto:
  - GitHub: via `gh`, token próprio, sem credencial, recolher após salvar.
  - Trello: sem credencial, escolher quadro, conectado, Trocar quadro, chave preservada.
  - IA: blocos Claude/Codex e todos os estados da status line e do opt-in.
  - Agentes: Detecção instantânea e Notificações.
- **Prior art**: o teste atual das Configurações (Painel + Core falso + matchMedia controlado) e os testes de seção por fixture JSON de snapshot.
- Os guardrails de privacidade continuam cobertos pelos testes existentes, porque nenhuma regra de credencial muda.

## Out of Scope

- Qualquer mudança no núcleo Rust, em comandos ou em snapshots.
- Comando para remover o token próprio do GitHub.
- Seção Sistema (escolha de discos e similares).
- Mudanças visuais no Painel e nos resumos dos Módulos.
- Persistir a última seção visitada entre aberturas.
- Reabrir as Boas-vindas depois da primeira abertura.

## Further Notes

- O protótipo B não desenha as Configurações; esta spec define o visual delas. A spec da v1 ("Configurações e primeira execução" e "Visual do Painel") deve passar a apontar para cá.
- Sem ADR: a mudança é reversível e não muda nenhuma fronteira técnica. Sem termo novo no GLOSSARY.
