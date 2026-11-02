# 21: Boas-vindas e Configurações Gerais

**What to build:** na primeira abertura vejo uma tela de boas-vindas única listando os Módulos com status (detectado / precisa configurar / não precisa) e atalhos para configurar cada um, incluindo "Conectar ao Claude Code". Nas Configurações → Geral posso desligar o início com o login e a checagem de nova versão, abrir a pasta de logs e ver uma linha discreta com link para o release quando houver versão nova. Ver spec, "Configurações e primeira execução" e "Atualização do app".

**Blocked by:** 11 (Token clássico do GitHub no keyring), 12 (Trello: conectar e ver o quadro), 15 (Claude via status line), 18 (Sessões do Codex)

**Status:** done

- [x] Boas-vindas só na primeira abertura, com status por Módulo e atalhos
- [x] Interruptor de início com o login (remove/recria o `.desktop`)
- [x] Checagem diária da API de releases do GitHub, ligada por padrão, desligável; linha discreta com link; sem notificação nem auto-update
- [x] Botão "Abrir pasta de logs"
- [x] Testes de UI por fixtures (primeira execução, versão nova disponível)

## Implementação

Implementado em `d86fbd9`, com recuperação de erros e atualizações ao vivo em
`418065c`. Conclusão conferida em 02/11/2026: a tela de boas-vindas usa os
snapshots dos Módulos e registra sua apresentação nas preferências; os comandos
de Geral alteram a entrada de autostart, persistem a preferência de checagem e
abrem a pasta de logs. O coletor de releases respeita a cadência diária, pausa
sem rede e publica apenas os metadados da versão disponível, sem notificar nem
instalar atualizações.

Validação: `npm run check` sem erros ou avisos; os 35 testes de
`src/lib/settings/Settings.test.ts` passaram; suíte completa com 376 testes de UI
e 308 testes Rust aprovados. Os testes de integração de hooks precisaram de
execução fora do sandbox para criar sockets Unix locais. A janela nativa no
GNOME não foi verificada manualmente nesta conferência.

Revisão em dois eixos: nenhuma violação documentada de padrões e nenhuma lacuna
da spec. Observação opcional de desenho: o agendador de releases monta a resposta
completa de Geral, acoplando a publicação da versão à consulta do autostart; não
impede a conclusão deste ticket.
