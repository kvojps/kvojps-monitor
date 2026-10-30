# 21: Boas-vindas e Configurações Gerais

**What to build:** na primeira abertura vejo uma tela de boas-vindas única listando os Módulos com status (detectado / precisa configurar / não precisa) e atalhos para configurar cada um, incluindo "Conectar ao Claude Code". Nas Configurações → Geral posso desligar o início com o login e a checagem de nova versão, abrir a pasta de logs e ver uma linha discreta com link para o release quando houver versão nova. Ver spec, "Configurações e primeira execução" e "Atualização do app".

**Blocked by:** 11 (Token clássico do GitHub no keyring), 12 (Trello: conectar e ver o quadro), 15 (Claude via status line), 18 (Sessões do Codex)

**Status:** ready-for-agent

- [ ] Boas-vindas só na primeira abertura, com status por Módulo e atalhos
- [ ] Interruptor de início com o login (remove/recria o `.desktop`)
- [ ] Checagem diária da API de releases do GitHub, ligada por padrão, desligável; linha discreta com link; sem notificação nem auto-update
- [ ] Botão "Abrir pasta de logs"
- [ ] Testes de UI por fixtures (primeira execução, versão nova disponível)
