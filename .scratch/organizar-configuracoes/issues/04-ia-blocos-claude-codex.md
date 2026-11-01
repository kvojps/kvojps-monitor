# 04: IA em blocos Claude e Codex

**What to build:** a seção IA passa a ter um bloco por Provedor de IA, cada um com sua linha de estado.
- **Claude:** estado da status line (conectada, trocada por terceiros, desconectada ou ausente), com Conectar, Desconectar ou Reconectar conforme o caso. Abaixo, o opt-in `oauth/usage` (desligado, aviso aberto, ligado, expirado), com o mesmo aviso de hoje.
- **Codex:** só a linha de estado (detectado · plano, ou não encontrado).

Ver `../spec.md`, histórias 18–21.

**Blocked by:** 01 (Peças compartilhadas e Rede como seção própria)

**Status:** done

- [x] Blocos Claude e Codex com linha de estado própria
- [x] Todos os estados da status line e do opt-in preservados, com o erro junto da ação que falhou
- [x] `settings.json` só muda após clique explícito (guardrail inalterado)
- [x] Testes de seção por fixture cobrindo cada estado

**Notas da implementação:**
- Sem snapshot, os dois blocos mostram "Verificando…"; as ações do Claude continuam disponíveis, como antes.
- Com o Claude Code ausente e a status line ainda conectada ou trocada, o bloco mostra "não encontrado" e, logo abaixo, o estado da status line.
- A linha de estado compartilhada passa a ser `role="status"`, mantendo o anúncio que a mensagem de status line trocada tinha.
- Status line desligada ganha o texto "Status line não conectada"; Claude ou Codex ausentes usam o tom de atenção, como no Painel.
- O erro da status line fica sob Conectar/Desconectar/Reconectar e o do opt-in sob o próprio opt-in, cada um ligado ao botão por `aria-describedby`; uma ação não apaga o erro da outra.
