# 07: Agentes, Geral e Aparência na nova estrutura

**What to build:** as três seções restantes seguem a mesma estrutura: título → estado → ações → preferências → erro.
- **Agentes:** Detecção instantânea primeiro, com a nota de confiança do `/hooks` do Codex quando ligada, e os três interruptores agrupados sob "Notificações".
- **Geral:** início com o login, checagem de nova versão e abrir pasta de logs; no fim, a versão e a linha discreta de nova versão com link.
- **Aparência:** tema e redução de animações.

Ver `../spec.md`, histórias 31, 32, 35 e 36.

**Blocked by:** 01 (Peças compartilhadas e Rede como seção própria)

**Status:** done

- [x] Agentes com Detecção instantânea e grupo "Notificações"
- [x] Geral com versão e release no fim; erros junto de cada interruptor
- [x] Aparência na mesma estrutura, mantendo os rádios em vez de `<select>`
- [x] `hooks.json` e `settings.json` só mudam após clique explícito (guardrail inalterado)
- [x] Testes existentes adaptados e passando

**Notas da implementação:**

- Agentes ganhou linha de estado (história 14): "Detecção instantânea ligada" (ok), "Aguardando indisponível sem Detecção instantânea" (atenção, quando o Codex não informa "aguardando") ou "Detecção instantânea desligada" (neutro); "Verificando…" sem snapshot.
- A nota do `/hooks` virou a dica do interruptor de Detecção instantânea, e o erro de configuração fica logo abaixo dele.
- Geral: o botão de logs continua visível mesmo se as configurações não carregam; versão e linha de release só aparecem com as configurações carregadas.
- Aparência: o erro de salvar aparece sob o controle que falhou (rádios do tema ou "Reduzir animações").
- Com todas as seções nas peças compartilhadas, saíram do container as regras globais de checkbox, `[role="alert"]` e `[role="status"]`. A última pintava de verde a linha de estado em qualquer tom. A regra global de rádio fica por causa da lista de quadros do Trello.
- Geral e Aparência ficam sem linha de estado: a história 14 fala da situação de um Módulo, e as seções do App não têm estado próprio.
- O tom de atenção da linha de Agentes não acende a marca "!" no índice: a regra única das Boas-vindas segue tratando Agentes como "não precisa configurar", e a Detecção instantânea é opcional.
