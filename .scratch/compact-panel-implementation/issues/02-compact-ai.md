# 02: Instrumentos e ações compactas de IA

**What to build:** consultar os Limites de uso de Claude e Codex no resumo compacto de B, com leitura e recuperação independentes por Provedor de IA e ações locais acessíveis.

**Blocked by:** 01 — Composição compacta e janela adaptada à área útil.

**Status:** resolved

- [x] Mostrar instrumentos de uso em 5 h, reinício, barras semanais e idade/horário da leitura nas proporções da referência B, utilizando snapshots reais.
- [x] Oferecer atualizar/configurar por ícones junto ao título, com nome acessível e descrição por ponteiro e foco; cada ação alcança a fonte ou seção correta.
- [x] Preservar CLI ausente, status line desconectada, consentimento de uso, token expirado e demais caminhos de recuperação; reconectar somente por ação explícita.
- [x] Distinguir carregamento/sem leitura, antigo, offline e falha. Preservar cache utilizável com idade e sinalização pertinente; falha de um provedor não apaga o outro.
- [x] Ao reiniciar uma janela sem leitura nova, remover apenas arco/barra afetados e explicar o reinício, sem converter ausência em zero.
- [x] Cobrir com fixtures de snapshot os estados acima e testar comandos, foco, descrições e resumos acessíveis dos gráficos, nos temas e redução de movimento existentes.
- [x] Não alterar coleta, credenciais ou guardrails; token Claude permanece apenas em memória e nunca é renovado pelo app.

## Comments

Origem: especificação aprovada do Painel compacto e requisitos herdados do complemento visual cinematográfico/spec da v1. Divisão aprovada em 2026-10-31.


## Answer

Implementado em 2026-10-31:

- Instrumentos de 94 px conforme B, barras semanais compactas, reinício e fonte/idade/horário por Provedor de IA, alimentados pelos snapshots existentes.
- Atualizar/configurar por ícones no título de IA; descrições por foco e ponteiro, alcançáveis pelo ponteiro e dispensáveis por Escape antes de fechar o Painel. Integração com comandos e retorno de Configurações preservada.
- Recuperações anteriores mantidas: CLI ausente, conexão/consentimentos pelas Configurações, status line substituída com reconexão explícita e token expirado. Nenhuma alteração no núcleo, coleta ou credenciais.
- Cache utilizável permanece com idade, inclusive em falha/pausa. O contrato de pausa inclui offline e espera após reconexão; a UI explica ambas as possibilidades, sem inferir a causa.
- Reinícios removem apenas o instrumento afetado, também quando o relógio cruza o reinício antes do próximo snapshot. Valor ausente sem indicação de reinício é explicado como sem leitura.
- Fixtures adicionais derivadas dos snapshots reais cobrem carregamento, pausa/offline e janela semanal reiniciada/indisponível. Testes cobrem instrumentos acessíveis, comandos, foco, Escape e independência dos provedores.
- Validação: 216 testes UI, 271 testes unitários Rust e 3 testes de integração de hooks aprovados; `npm run check` sem erros/avisos; build aprovado. Hooks executados fora do sandbox por exigirem sockets Unix.
- Revisão code-review desde `c4c18cbd`: Standards 0 achados restantes; Spec 0. Corrigidos durante a revisão: alcance/dismissão das dicas e associação implícita da pausa por posição.

Evidências e limites: [validação focada](../validation/README.md), com 66 cenários de navegador, capturas e relatório. Integração visual dos seis resumos, medições de desempenho e validação GNOME/Wayland permanecem no ticket 07; esta etapa não comprova ausência de rolagem do Painel completo.

**Reaberto em 2026-10-31 por fidelidade ao protótipo:** no app nativo, o resultado diverge do design e do conteúdo do protótipo B. A correção segue em [compact-panel-fidelity](../../compact-panel-fidelity/map.md).
