# 10: Pausa sem rede, retomada e login escalonado

**What to build:** sem conexão o app pausa as chamadas de rede e mostra o último dado com idade, sem faixa de erro; ao voltar, atualiza tudo uma vez, escalonado. Após suspender/retomar, espera a conectividade, atualiza tudo escalonado, zera os relógios e reinicia subprocessos mortos. Após o login, espera ~10 s e a conectividade e atualiza uma fonte a cada ~3 s. Ver spec, "Ritmo de coleta".

**Blocked by:** 04 (Qualidade da rede e tráfego total), 09 (Coleta resiliente)

**Status:** feito

- [x] Conectividade do NetworkManager pausa GitHub; sem faixa de erro durante a pausa. Trello, `oauth/usage` e Codex transferidos para os tickets correspondentes por autorização abaixo
- [x] Volta da conexão dispara uma atualização escalonada de tudo
- [x] Sinal do logind (PrepareForSleep) na retomada: aguarda conectividade, atualiza escalonado e zera relógios. Reinício de subprocessos transferido para os tickets dos coletores que os manterão
- [x] Inicialização: ~10 s, aguarda conectividade, uma fonte a cada ~3 s
- [x] Testes com relógio falso e NM/logind falsos para cada cenário

## Decisões confirmadas nesta implementação

- O usuário autorizou implementar esta fatia nos coletores existentes (GitHub, Rede e Sistema). IA e Trello ainda não têm coletores: a integração da pausa, escalonamento e reinício de subprocessos fica nos tickets 12, 14, 16, 17 e 18.
- O seam já acordado na spec e no AGENTS.md permanece: núcleo com portas de E/S e relógio falso; UI por fixtures de snapshots.
- Revisão contra o HEAD anterior: `c31e5c8d045be45cd5adb4be6dea6a99b45efe81`.

## Implementação

- Porta de sessão com adapter do logind que escuta `PrepareForSleep`, preservando a sequência dos sinais. Conectividade plena do NetworkManager libera fontes remotas; indisponibilidade de D-Bus mantém a pausa.
- Login espera 10 segundos e conectividade. Posições estáveis de fontes remotas têm prazos separados por 3 segundos; GitHub ocupa a primeira posição, sendo a única fonte remota implementada nesta fatia.
- GitHub pausa inclusive atualização manual; mantém snapshot e idade, sem faixa de erro. Reconexão/retomada força uma atualização completa uma vez, preservando bloqueio de credenciais recusadas. Requisições já em andamento podem terminar.
- Rede reinicia os relógios de coleta, DNS/portal, ping e teste leve; descarta bases de taxas anteriores à suspensão e cancela teste de velocidade interrompido. Sistema reinicia leituras e histórico enquanto visível.
- Reinício de processos persistentes pertence aos tickets dos agentes/Codex: os coletores existentes não mantêm esses subprocessos.

## Validação e revisão

- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `npm run check`, `npm test` e `npm run build` aprovados (114 testes Rust e 39 testes de UI).
- Standards: 0 achados restantes. Corrigida cobertura do adapter do logind, incluindo ordem dos sinais e ausência de eventos; leitura real capturada com variações controladas documentadas.
- Spec: 0 achados restantes. Corrigido reset antes da conectividade, que antes podia se repetir na reconexão; a geração e o despacho agora respeitam rede e prazo de login.
- Teste visual de suspensão/login no GNOME não realizado nesta execução.
