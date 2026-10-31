# 02: IA com instrumentos de Limite de uso

**What to build:** acompanhar Codex e Claude pelos instrumentos circulares de uso de 5 h e barras semanais aprovados, com leitura confiável e caminhos de recuperação no próprio Painel.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** ready-for-agent

- [ ] Exibir dois instrumentos de 5 h com percentual/reinício e duas barras semanais, identificação e idade da leitura; não adicionar buckets, planos, calendário ou rolagem de detalhes.
- [ ] Atualizar/configurar e reconectar continuam funcionais; atualização manual respeita intervalo mínimo de 10 s. Reconexão ocorre somente por ação explícita.
- [ ] Preservar estados de CLI ausente, sem leitura, status line desconectada, token expirado e consentimento para oauth/usage, mantendo independência entre Provedores de IA.
- [ ] Ao reiniciar uma janela, remover seu arco/barra e mostrar “Reiniciado às HH:MM · sem leitura nova”; preservar dado válido antigo com idade, sem inferir 0%.
- [ ] Testar fixtures cheio, vazio, antigo, credencial inválida e erro, estados específicos por provedor, comandos e recuperação; instrumentos têm nomes/valores acessíveis e estados não dependem apenas da cor.
- [ ] Preservar aquisição existente; testar que o token Claude fica só em memória, nunca é renovado e não aparece em logs/cache/notificações, usando o seam de E/S existente.
