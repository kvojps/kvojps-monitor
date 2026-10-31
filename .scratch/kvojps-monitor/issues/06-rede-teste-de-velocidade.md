# 06: Teste de velocidade

**What to build:** no Módulo Rede posso rodar um teste de velocidade completo sob demanda; um teste leve roda sozinho ao conectar e a cada 6 h, nunca em conexão limitada (metered), e pode ser desligado nas Configurações → Rede. A última velocidade aparece sempre com a idade e passa a pesar na Qualidade. Ver spec, "Módulo Rede".

**Blocked by:** 04 (Qualidade da rede e tráfego total), 09 (Coleta resiliente)

**Status:** feito

- [x] Medição pela Cloudflare (speed.cloudflare.com, HTTPS) via porta HTTP
- [x] Completo sob demanda (botão); leve (~25 MB ↓, ~10 MB ↑) ao conectar e a cada 6 h; nunca em metered
- [x] Interruptor "teste de velocidade automático" nas Configurações → Rede, ligado por padrão
- [x] Velocidade pesa na Qualidade só se medida há < 6 h na rede atual: Ruim < 2 Mbps de descida, Regular < 10 Mbps
- [x] Último resultado no cache em disco e exibido com idade, inclusive sem conexão
- [x] Testes com relógio falso: agendamento de 6 h, metered bloqueia, resultado antigo não pesa

## Integração com a persistência (ticket 09)

Conforme escopo confirmado pelo usuário no ticket 09, integrar o snapshot deste coletor ao cache em disco quando seu contrato existir. Reutilizar as portas de persistência e adicionar projeção tipada sanitizada: nunca persistir credenciais nem conteúdo de commits/cards/prompts/respostas. Restaurar o último dado na inicialização; para Rede, persistir apenas o último teste de velocidade.


## Implementação e validação

- Transferências HTTPS contra `speed.cloudflare.com/__down` e `__up`, pela porta HTTP assíncrona. Teste leve usa 25 MB de descida e 10 MB de subida; completo manual usa 100 MB e 50 MB. Taxas em Mbps calculadas a partir dos bytes efetivos e da duração. Respostas são descartadas, sem cache de conteúdo nem credenciais. Fixture real de 10 KB por direção documentada em `src-tauri/fixtures/network/README.md`.
- O worker mantém a coleta de rede livre durante as transferências. Pedidos duplicados não sobrepõem testes. Desconexão, mudança de rede/rota ou conexão que passe a ser limitada abortam a tarefa HTTP, inclusive enquanto aguarda resposta. Estados de medição limitada desconhecidos bloqueiam o início por precaução.
- Agendamento pelo relógio injetado: ao conectar e a cada seis horas, se a preferência estiver ligada (padrão). Configurações → Rede altera a preferência, persistida pela porta de armazenamento em `network-preferences.json`.
- Qualidade considera a descida apenas por menos de seis horas na rede atual, com limites estritos de 2 e 10 Mbps, sem melhorar uma avaliação já pior por latência/perda/jitter. Identidade composta pelos UUIDs do perfil físico e dos perfis VPN, interface e rota VPN primária; recriação dos caminhos D-Bus preserva a validade, troca de VPN invalida e agenda novo teste.
- Projeção tipada do último teste reutiliza `Persistence`/`Cache` do ticket 09. Cache de Rede guarda somente velocidades, data e identificação da rede; campos extras são descartados. Restauração e publicação ocorrem antes das sondas ao iniciar. Último resultado continua visível offline, com idade atualizada pela UI.
- Testes no seam do núcleo com relógio falso cobrem agendamento, metered, preferência, limiares, validade temporal e por rede, cancelamento da HTTP pendente, volumes e conversão em Mbps, sanitização/restauração do cache. Snapshots publicados pelo núcleo cobrem resultado, progresso, falha e desconexão; a UI verifica renderização e comandos.
- Validação final: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (98 testes), `npm run check`, `npm test` (35 testes) e `npm run build`: aprovados. Validação visual da janela no GNOME permanece manual, conforme spec.

## Revisão

Base confirmada pelo usuário: `db9d5889b413a52ec35d12365ec8e996876c5a66`.

### Standards

Nenhuma violação comprovada das regras documentadas e nenhum smell acionável. E/S passa por portas, fixtures registram transferências reais e a persistência tem projeção sanitizada e teste contra vazamentos. A ressalva inicial sobre operações HTTP bloqueantes foi resolvida com tarefa assíncrona abortável e regressão que observa o descarte da future pendente pelo seam do núcleo.

### Spec

O achado inicial sobre reutilizar velocidade após trocar de VPN foi corrigido: a identidade inclui os perfis VPN e a rota primária. A regressão exercita o adapter D-Bus, preserva validade quando apenas os object paths mudam e invalida/agenda teste ao trocar o perfil VPN. Nenhum achado adicional na segunda revisão.

Resultado final: Standards 0 achados pendentes; Spec 0 achados pendentes (1 corrigido).
