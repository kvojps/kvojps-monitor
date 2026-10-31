# 07: Tráfego por processo

**What to build:** com o Módulo Rede visível vejo os 5 apps que mais usam rede agora (↓/↑, agrupados por nome) e uma linha "Outros" explicando o que não é atribuível. Ver spec, "Módulo Rede" e o ticket 14 do mapa.

**Blocked by:** 04 (Qualidade da rede e tráfego total)

**Status:** feito

- [x] Contadores TCP por conexão do kernel (`sock_diag`/`tcp_info`), só processos do próprio usuário, sem privilégio
- [x] Top 5 agrupados por nome do app, taxa ↓/↑ média dos últimos ~5 s, a cada 2 s
- [x] Linha "Outros" = total da interface − atribuído, com dica "UDP/QUIC, processos do sistema e conexões curtas"
- [x] Coleta só com o Módulo Rede visível; não pesa na Qualidade
- [x] Testes do núcleo a partir de respostas `sock_diag` gravadas; testes de UI por fixture

## Implementação e validação

- Porta injetável de diagnóstico TCP com adapter Linux `NETLINK_SOCK_DIAG`: dumps IPv4 e IPv6 com `INET_DIAG_INFO`, usando `tcpi_bytes_received` e `tcpi_bytes_sent`. Sem privilégios, subprocessos ou leitura do conteúdo trafegado. Socket com timeout, detecção de dumps interrompidos e limite de memória.
- Associação dos sockets aos FDs em `/proc` pela porta de arquivos; tanto UID do socket quanto UID real/efetivo do processo precisam corresponder ao usuário. Agrupamento pelo nome do executável, com fallback para `comm`; FDs compartilhados não duplicam contadores. Loopback não entra na atribuição, inclusive IPv4 mapeado em IPv6. A origem deve pertencer aos endereços IPv4/IPv6 da interface física selecionada ou de VPN ativa, obtidos por `AddressData` no NetworkManager; uma segunda interface física não se mistura ao total.
- Atualização a cada 2 s enquanto Rede está visível; média móvel de três intervalos (6 s, aproximadamente os 5 s da spec). Abertura, fechamento, troca de Módulo e mudança de conexão descartam a janela anterior. Primeira amostra estabelece a linha de base; falhas ficam sem atribuição e preservam a Qualidade.
- Top 5 ordenado pela soma ↓/↑. “Outros” desconta todos os apps atribuídos, inclusive os que ficaram fora do top 5, do total da interface na mesma janela, com piso zero e a dica pedida. Contadores de sockets novos/reutilizados não geram picos artificiais.
- Dados de processos e sockets permanecem somente em memória; o cache de Rede continua restrito ao último teste de velocidade. Nenhuma notificação ou log foi acrescentado.
- Fixtures reais de respostas binárias do kernel capturadas por conexão local temporária; origem, recaptura e variações controladas documentadas em `src-tauri/fixtures/network/tcp/README.md`. Snapshots cheio e vazio da UI são produzidos pelo núcleo.
- Validação: `cargo fmt --check`, `cargo clippy --offline --all-targets -- -D warnings`, `cargo test --offline` (107 testes), `npm run check`, `npm test` (38 testes) e `npm run build`: aprovados. A janela Tauri no GNOME permanece sujeita à verificação manual prevista na spec.

## Revisão

Base: `f1cb64e0c5fce5869df6deb232ff100c0703dd8b`, HEAD anterior à implementação.

### Standards

Sem achados pendentes. Corrigida a ausência de fixtures reais de `/proc` para
associação ao proprietário: status, comm e destinos de FD/executável foram
capturados e as variações controladas partem dessa base. O julgamento de
Primitive Obsession / Data Clumps foi resolvido com `ByteCounters`, campos
`received`/`sent` e operações de soma e delta. Leituras de `AddressData` seguem
atrás da porta do NetworkManager, com captura real documentada.

### Spec

Sem achados bloqueantes remanescentes nos cenários revisados. Corrigidos a mistura
de sockets de outra interface física e o loopback IPv4 mapeado em IPv6. Regressões
cobrem interface primária com VPN, segunda interface, IPv6 nativo, IPv4 externo
mapeado e loopback mapeado. Não foram identificados requisitos faltantes nem
escopo adicional. A seleção por endereço atende ao modo aproximado sem privilégio;
não demonstra roteamento exato por socket com múltiplas VPNs ou policy routing.

Resultado final: Standards 0 pendências (1 violação e 1 julgamento corrigidos);
Spec 0 pendências nos cenários revisados (2 achados corrigidos).
