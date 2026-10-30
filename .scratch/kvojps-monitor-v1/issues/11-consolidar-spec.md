# Consolidar a spec da v1

Type: grilling
Status: resolved
Assignee: Kvojps
Blocked by: 07, 08, 09, 10, 12, 13, 14, 15, 16, 17, 18
Map: [kvojps-monitor v1](../map.md)

## Question

Com todas as decisões tomadas, qual é a spec da v1 (Módulos, fontes de dados, Configurações, stack, instalação, release) pronta para entregar a um agente implementador?

## Answer

Spec da v1 escrita em [Spec: kvojps-monitor v1](../../kvojps-monitor/spec.md) (`Status: ready-for-agent`), sintetizando todas as decisões do mapa. Decisões novas tomadas ao consolidar:

- **Forma e local**: uma spec única no tracker local, em `.scratch/kvojps-monitor/spec.md`. A `.scratch/` **fica versionada** e vai ao repo público como acompanhamento do planejamento; o usuário a apaga quando a v1 estiver finalizada (revisa o ponto sobre `.scratch/` em [Pipeline de release e repo público](18-release-e-repo-publico.md)).
- **Seam de teste**: um seam no núcleo Rust — portas de E/S injetáveis (processos, HTTP, sistema de arquivos, D-Bus, keyring, notificador, socket de hooks, relógio) alimentadas com saídas reais gravadas; afirma sobre snapshots, notificações e efeitos de comandos. UI testada com Vitest + Testing Library a partir de fixtures de snapshot. Sem E2E.
- **Encerradas no Módulo Agentes**: seção recolhida por 2 h após encerrar, no máximo 5, só sessões vistas vivas, não persistidas.
- **Hooks → núcleo**: o binário em modo `hook <fonte> <evento>` extrai só metadados do stdin e envia a um socket Unix em `$XDG_RUNTIME_DIR`; app fora do ar → descarta; sempre sai 0 em < 200 ms.
- **Painel**: fecha com Esc e ao perder o foco; abre no último Módulo visitado; instância única (segundo lançamento abre o Painel).
- **Card do Trello**: título (2 linhas), etiquetas como pontos, data de entrega (atenção se vencida); duplo clique abre no navegador.
- **Logs**: `~/.local/state/kvojps-monitor/`, 3 × 1 MB, só metadados (nunca credenciais nem conteúdo); botão "Abrir pasta de logs".
- **Ordem de fatias** sugerida na spec: esqueleto → Sistema → Rede → IA → Agentes → GitHub → Trello → primeira execução/Configurações → CI/release.
