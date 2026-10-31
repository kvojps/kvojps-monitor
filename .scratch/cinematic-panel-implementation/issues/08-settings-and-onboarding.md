# 08: Configurações completas, temas e primeira execução

**What to build:** configurar o app e suas fontes na única área separada do Painel, com formulários legíveis, primeira execução completa e preferências visuais persistentes.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** ready-for-agent

- [ ] Aplicar a linguagem visual às Configurações, mantendo “Voltar ao Painel”, contexto anterior e Escape para ocultar. Primeira execução usa essa mesma superfície, com status por Módulo e atalhos de configuração.
- [ ] Preservar Claude (status line, consentimento oauth/usage e diagnóstico), hooks com passos de confiança, notificações, fonte/token GitHub, Trello (key/token/autorização/quadro/desconexão) e velocidade automática.
- [ ] Completar requisitos da v1 ainda ausentes: autostart ligado inicialmente e desligável, sincronização do caminho no AppImage, checagem diária de versão ligada por padrão/desligável e link discreto sem auto-update, e abertura da pasta de logs.
- [ ] Oferecer escuro/claro/sistema com tokens completos de geometria/hierarquia e acompanhar mudanças do ambiente. Persistir tema e redução manual de animações; redução do sistema prevalece.
- [ ] Validar antes de salvar; erro junto do campo e sucesso identifica conexão. Campos e ações são acessíveis por teclado, sem explicações de implementação desnecessárias.
- [ ] Testar explicitamente: credenciais só no GNOME Keyring e recusa sem keyring; logs/cache/notificações só com metadados permitidos; hooks/status line alterados só após clique, aditiva e reversivelmente preservando terceiros; token Claude apenas em memória e sem renovação. Usar exclusivamente o seam de E/S existente.
- [ ] Testar fixtures e comandos de formulários, primeira execução, persistência de preferências, mudança de tema do sistema e falhas de validação/portas; não substituir comportamento funcional por simulação do protótipo.
