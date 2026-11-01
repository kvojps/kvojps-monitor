# 02: Instrumentos e ações compactas de IA

**What to build:** consultar os Limites de uso de Claude e Codex no resumo compacto de B, com leitura e recuperação independentes por Provedor de IA e ações locais acessíveis.

**Blocked by:** 01 — Composição compacta e janela adaptada à área útil.

**Status:** ready-for-agent

- [ ] Mostrar instrumentos de uso em 5 h, reinício, barras semanais e idade/horário da leitura nas proporções da referência B, utilizando snapshots reais.
- [ ] Oferecer atualizar/configurar por ícones junto ao título, com nome acessível e descrição por ponteiro e foco; cada ação alcança a fonte ou seção correta.
- [ ] Preservar CLI ausente, status line desconectada, consentimento de uso, token expirado e demais caminhos de recuperação; reconectar somente por ação explícita.
- [ ] Distinguir carregamento/sem leitura, antigo, offline e falha. Preservar cache utilizável com idade e sinalização pertinente; falha de um provedor não apaga o outro.
- [ ] Ao reiniciar uma janela sem leitura nova, remover apenas arco/barra afetados e explicar o reinício, sem converter ausência em zero.
- [ ] Cobrir com fixtures de snapshot os estados acima e testar comandos, foco, descrições e resumos acessíveis dos gráficos, nos temas e redução de movimento existentes.
- [ ] Não alterar coleta, credenciais ou guardrails; token Claude permanece apenas em memória e nunca é renovado pelo app.

## Comments

Origem: especificação aprovada do Painel compacto e requisitos herdados do complemento visual cinematográfico/spec da v1. Divisão aprovada em 2026-10-31.
