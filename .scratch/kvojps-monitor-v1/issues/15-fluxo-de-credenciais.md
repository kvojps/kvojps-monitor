# Fluxo de credenciais nas Configurações

Type: grilling
Status: resolved
Blocked by: 07
Map: [kvojps-monitor v1](../map.md)

## Question

Como cada Módulo obtém e valida suas credenciais (no Claude, a conexão da status line por botão já foi decidida em [Fonte dos Limites de uso do Claude](13-fonte-limites-claude.md)) nas Configurações (armazenamento já decidido: GNOME Keyring, nunca texto puro)? GitHub reaproveita o token do `gh` ou pede um token próprio; Trello guia o usuário a criar key + token; IA detecta Claude/Codex sozinho? Como é a primeira execução (onboarding) com Módulos sem credencial, como o token é validado ao salvar, e como o Painel mostra credencial inválida ou expirada?

## Answer

- **GitHub**: usa o `gh` em tempo de execução (`gh auth token` a cada atualização, nada copiado para o keyring; acompanha rotação do token). Sem `gh` instalado/logado: oferece "Rodar `gh auth login`" ou "Colar token clássico" (escopo `repo`, guardado no keyring). Configurações mostram a fonte: "via gh · <login>" ou "token próprio".
- **Trello**: passo a passo numa tela só em Configurações → Trello: (1) link para trello.com/apps/admin + campo da key; (2) botão "Gerar token" abre a página de autorização com a key preenchida (`scope=read,write`, `expiration=never`) e o usuário cola o token; (3) seletor de quadro com os quadros abertos. Sem captura automática via `return_url`. "Desconectar" apaga key e token do keyring.
- **IA**: detecção automática, nenhuma credencial do app. Codex = `codex` no PATH + `account/read` do app-server responde; Claude = `claude` no PATH + `~/.claude/.credentials.json` (plano); limites só após "Conectar ao Claude Code" (já decidido em [Fonte dos Limites de uso do Claude](13-fonte-limites-claude.md)). Ausente → "Codex não encontrado · instale e faça login no CLI", sem botão de configurar.
- **Primeira execução**: tela única de boas-vindas só na primeira abertura, listando os Módulos com status (detectado / precisa configurar / não precisa) e atalhos para configurar cada um, incluindo "Conectar ao Claude Code". No uso normal, Módulo sem credencial segue visível no trilho com estado vazio e botão "Configurar" que leva à seção dele nas Configurações.
- **Validação ao salvar**: chamada de teste antes de gravar no keyring — GitHub `viewer { login }` + escopo `repo`; Trello `GET /1/members/me` + listagem de quadros. Falhou → nada é salvo, erro junto do campo ("Token inválido", "Falta o escopo repo", "Sem conexão, tente de novo"). Passou → "Conectado como <login>".
- **Credencial inválida em uso** (401/403 ou `gh` deslogado): o Módulo mantém o último dado com a idade, em tom apagado; faixa no cabeçalho "Credencial inválida · Reconectar" que leva às Configurações; para de tentar até a credencial mudar; sem notificação nem marca no Indicador. Mesmo padrão do "Token do Claude expirado" do opt-in `oauth/usage`.
