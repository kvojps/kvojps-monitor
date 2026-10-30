# 10: Pausa sem rede, retomada e login escalonado

**What to build:** sem conexão o app pausa as chamadas de rede e mostra o último dado com idade, sem faixa de erro; ao voltar, atualiza tudo uma vez, escalonado. Após suspender/retomar, espera a conectividade, atualiza tudo escalonado, zera os relógios e reinicia subprocessos mortos. Após o login, espera ~10 s e a conectividade e atualiza uma fonte a cada ~3 s. Ver spec, "Ritmo de coleta".

**Blocked by:** 04 (Qualidade da rede e tráfego total), 09 (Coleta resiliente)

**Status:** ready-for-agent

- [ ] Conectividade do NetworkManager pausa GitHub, Trello, `oauth/usage` e consultas do Codex; sem faixa de erro durante a pausa
- [ ] Volta da conexão dispara uma atualização escalonada de tudo
- [ ] Sinal do logind (PrepareForSleep) na retomada: aguarda conectividade, atualiza escalonado, zera relógios, reinicia subprocessos mortos
- [ ] Inicialização: ~10 s, aguarda conectividade, uma fonte a cada ~3 s
- [ ] Testes com relógio falso e NM/logind falsos para cada cenário
