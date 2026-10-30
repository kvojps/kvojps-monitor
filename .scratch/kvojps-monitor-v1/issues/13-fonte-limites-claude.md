# Fonte dos Limites de uso do Claude

Type: grilling
Status: resolved
Blocked by: 01
Map: [kvojps-monitor v1](../map.md)

## Question

Para o Claude, o app usa só a via documentada (instalar/encadear um script de status line no Claude Code, que só atualiza quando há sessão ativa e não respeita status line já configurada pelo usuário sem encadeamento), o endpoint não documentado `oauth/usage` (sempre atualizado, mas zona cinzenta nos termos da Anthropic), ou ambos com o endpoint como opt-in? Como o Painel mostra dado desatualizado?

## Answer

**Fontes**: status line do Claude Code é a fonte padrão; `GET /api/oauth/usage` é opt-in, **desligado por padrão**, com aviso de API não documentada e de zona cinzenta nos termos da Anthropic.

**Status line** (consentimento explícito):
- Botão "Conectar ao Claude Code" em Configurações → IA (também oferecido no onboarding); o app nunca altera `~/.claude/settings.json` sem esse clique.
- O app grava um wrapper em `~/.local/share/kvojps-monitor/claude-statusline` e aponta `statusLine.command` para ele; se já havia comando, o wrapper o encadeia (repassa stdin, devolve a saída dele) e só copia `rate_limits` para o cache. O comando original fica guardado na configuração do app.
- "Desconectar" restaura o original (ou remove a chave se não havia).
- Se o `statusLine` for trocado depois, o app mostra "Status line desconectada" com botão de reconectar; não reescreve sozinho.
- Só o `settings.json` do usuário; projetos com `statusLine` próprio não alimentam o cache (aceito).

**Opt-in `oauth/usage`**:
- Consulta a cada 5 min enquanto o app roda, com backoff em erro.
- Token lido de `~/.claude/.credentials.json` só em memória; **nunca faz refresh**. Token expirado/401 → volta ao cache da status line e mostra "Token do Claude expirado · abra o Claude Code".
- Com as duas fontes ativas, vale a leitura mais recente de cada janela.
- Semanais por modelo só aparecem quando vierem não-nulos; campos desconhecidos são ignorados.
- Desligar volta à status line na hora.

**Dado desatualizado** (vale também para o fallback por rollouts do Codex):
1. Leitura antiga com janela ainda válida (`resets_at` no futuro): mostra o % normal com "atualizado há X" discreto, sem alerta.
2. Janela já reiniciou desde a leitura: "Reiniciado às HH:MM · sem leitura nova", sem barra; **nunca assume 0%** (o consumo pode vir de claude.ai).
3. Nenhuma leitura ainda: estado vazio "Abra uma sessão do Claude Code para ler os limites".
