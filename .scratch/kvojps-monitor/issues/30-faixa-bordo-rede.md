# 30: Comunicar mudanças de Rede e teste de velocidade

**What to build:** comunicar na Faixa de bordo mudanças relevantes da Rede e o andamento real do teste de velocidade, sem transformar amostras contínuas em mensagens ou resultados descartados em sucesso.

**Blocked by:** 27 — Integrar assinatura e Faixa de bordo às atualizações do GitHub.

**Status:** done

- [x] Comunicar mudanças de conexão, VPN e categoria de Qualidade da rede e Segurança da rede; não gerar mensagens por sondagem, latência, tráfego ou snapshot repetido. Permanecer conectado não comprova troca de rede.
- [x] Correlacionar início e resultado real de teste de velocidade com identidade de operação e motivo explícito de cancelamento, suprindo apenas as lacunas necessárias do contrato.
- [x] Diferenciar sucesso, falha e cancelamento; descartar corretamente resultados após mudança de conexão ou suspensão, sem conclusão inferida por tempo.
- [x] Aplicar agrupamento, prioridade, permanência e anúncios não interruptivos da faixa. Reabertura estabelece referência sem reapresentar mudanças antigas; inícios automáticos são silenciosos para leitores de tela.
- [x] Preservar medições, coleta, cadências, limites de privilégio, notificações e ações existentes; indisponibilidade nunca vira valor zero ou sucesso e explicações permanecem no Módulo.
- [x] Testar transições de categoria, VPN e conexão, repetição, reabertura, teste rápido/longo, falha, cancelamento, suspensão e resultado obsoleto com portas existentes, fixtures reais, relógio falso e fixtures de snapshots/transições na UI.
- [x] Testar sanitização com sentinelas: apenas fonte, evento e estado operacional nas mensagens; sem conteúdo sensível em logs, cache ou notificações e sem histórico operacional persistido. Preservar demais guardrails de privacidade.


## Notas da implementação

- O snapshot publica `speedOperation` transitório com identidade incremental, origem e resultado real. Cancelamentos têm motivo fechado: mudança de conexão, desconexão, conexão limitada, internet indisponível ou suspensão. O worker abortado descarta seu receptor; resultados obsoletos não atualizam a medição nem viram sucesso. O resultado terminal é publicado antes de iniciar outro teste.
- O núcleo publica `connectionRevision` opaca quando a identidade de uma conexão já estabelecida realmente muda. A faixa compara esse sinal e os estados de conexão, VPN, Qualidade e Segurança; latência, tráfego, sondagens e timestamps repetidos não geram mensagens.
- A faixa agrupa os sinais de Rede, conserva a prioridade e os quatro segundos dos resultados, e mantém testes longos ativos até o resultado real. Inícios automáticos e sucessos automáticos de velocidade são silenciosos para leitores de tela; início e resultado permanecem visuais. Mensagens usam somente vocabulário fechado.
- Ocultar e reabrir o Painel estabelece uma referência atual mesmo com a UI montada. O Módulo mantém a explicação sanitizada do cancelamento até a próxima operação, com a ação de teste e as restrições existentes. Não foram acrescentadas notificações do sistema.
- Fixtures de transições de velocidade são geradas e conferidas pelo núcleo. Testes com portas existentes e relógio falso cobrem sucesso, falha, resultado numérico inválido, duração longa, identidade de operação, todos os motivos de cancelamento, suspensão e resultado obsoleto. A UI cobre transições, repetição, agrupamento, reabertura e explicação persistente.
- Testes com sentinelas verificam sanitização das mensagens e ausência de conteúdo sensível e histórico operacional no cache/logs. Restaurar uma medição não restaura operação antiga. Guardrails existentes continuam cobertos pela suíte completa.
- Validação: 346 testes de UI; 305 testes unitários Rust e 3 de integração; `npm run check` sem erros ou avisos; `npm run build`; `git diff --check`. Integrações de hooks executadas fora do sandbox para permitir sockets locais.

## Revisão

### Standards

Sem violações documentadas nem observações heurísticas restantes. Resolvida a observação inicial de parâmetros posicionais agrupados: a descrição das operações recebe uma fonte tipada com categoria discriminada.

### Spec

Sem achados restantes. Corrigidos e testados: sinal explícito de troca real de conexão, explicação persistente do cancelamento no Módulo e silêncio acessível de início e sucesso automáticos de velocidade.
