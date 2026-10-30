# Pipeline de release e repo público

Type: grilling
Status: open
Blocked by: 07, 15
Map: [kvojps-monitor v1](../map.md)

## Question

Como a v1 é publicada no GitHub: o que dispara um release (tag `v*`, manual), versionamento, o que o workflow do GitHub Actions faz (`tauri-action` em `ubuntu-24.04`, `.deb` + AppImage, checksums, assinatura?), se há atualização automática ou só aviso de nova versão, licença do projeto, e o que o README precisa conter (instalação, extensão AppIndicator, guia de key + token do Trello, `gh auth login`, aviso do opt-in `oauth/usage`)? Stack e empacotamento já decididos em [Escolher a stack](07-escolher-stack.md); credenciais em [Fluxo de credenciais nas Configurações](15-fluxo-de-credenciais.md).
