# Fluxo de credenciais nas Configurações

Type: grilling
Status: open
Blocked by: 07
Map: [kvojps-monitor v1](../map.md)

## Question

Como cada Módulo obtém e valida suas credenciais nas Configurações (armazenamento já decidido: GNOME Keyring, nunca texto puro)? GitHub reaproveita o token do `gh` ou pede um token próprio; Trello guia o usuário a criar key + token; IA detecta Claude/Codex sozinho? Como é a primeira execução (onboarding) com Módulos sem credencial, como o token é validado ao salvar, e como o Painel mostra credencial inválida ou expirada?
