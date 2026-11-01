# 04: IA em blocos Claude e Codex

**What to build:** a seção IA passa a ter um bloco por Provedor de IA, cada um com sua linha de estado.
- **Claude:** estado da status line (conectada, trocada por terceiros, desconectada ou ausente), com Conectar, Desconectar ou Reconectar conforme o caso. Abaixo, o opt-in `oauth/usage` (desligado, aviso aberto, ligado, expirado), com o mesmo aviso de hoje.
- **Codex:** só a linha de estado (detectado · plano, ou não encontrado).

Ver `../spec.md`, histórias 18–21.

**Blocked by:** 01 (Peças compartilhadas e Rede como seção própria)

**Status:** ready-for-agent

- [ ] Blocos Claude e Codex com linha de estado própria
- [ ] Todos os estados da status line e do opt-in preservados, com o erro junto da ação que falhou
- [ ] `settings.json` só muda após clique explícito (guardrail inalterado)
- [ ] Testes de seção por fixture cobrindo cada estado
