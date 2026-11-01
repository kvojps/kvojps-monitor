# 07: Agentes, Geral e Aparência na nova estrutura

**What to build:** as três seções restantes seguem a mesma estrutura: título → estado → ações → preferências → erro.
- **Agentes:** Detecção instantânea primeiro, com a nota de confiança do `/hooks` do Codex quando ligada, e os três interruptores agrupados sob "Notificações".
- **Geral:** início com o login, checagem de nova versão e abrir pasta de logs; no fim, a versão e a linha discreta de nova versão com link.
- **Aparência:** tema e redução de animações.

Ver `../spec.md`, histórias 31, 32, 35 e 36.

**Blocked by:** 01 (Peças compartilhadas e Rede como seção própria)

**Status:** ready-for-agent

- [ ] Agentes com Detecção instantânea e grupo "Notificações"
- [ ] Geral com versão e release no fim; erros junto de cada interruptor
- [ ] Aparência na mesma estrutura, mantendo os rádios em vez de `<select>`
- [ ] `hooks.json` e `settings.json` só mudam após clique explícito (guardrail inalterado)
- [ ] Testes existentes adaptados e passando
