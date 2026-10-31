# 15: Claude via status line

**What to build:** nas Configurações → IA clico "Conectar ao Claude Code"; o app instala um wrapper de status line que encadeia minha status line existente e passa a ler os limites de 5 h e semanal do Claude, com o plano detectado. "Desconectar" restaura exatamente a status line original (ou remove a chave). Se outra ferramenta trocar a status line, vejo "Status line desconectada · Reconectar" e o app não reescreve sozinho. Sem leitura: "Abra uma sessão do Claude Code para ler os limites". Ver spec, "Módulo IA" e ticket 13 do mapa.

**Blocked by:** 14 (Limites do Codex)

**Status:** feito

- [x] Wrapper em `~/.local/share/kvojps-monitor/claude-statusline`; `statusLine.command` do `~/.claude/settings.json` só muda após o clique
- [x] Wrapper repassa stdin ao comando original, devolve a saída dele e copia só `rate_limits` (`five_hour`, `seven_day`) para o cache
- [x] Comando original guardado na configuração do app; Desconectar restaura ou remove a chave, preservando o resto do arquivo (teste explícito de encadeamento e restauração)
- [x] Troca por terceiros detectada → aviso com Reconectar, sem reescrever
- [x] inotify no cache da status line atualiza o snapshot
- [x] Plano por `subscriptionType` de `~/.claude/.credentials.json` (Pro/Max); detecção = `claude` no PATH + credenciais
- [x] Regras de dado desatualizado do ticket 14 aplicadas ao Claude
- [x] Testes de UI por fixtures (conectado, desconectado por terceiros, sem leitura)

## Integração com a persistência (ticket 09)

Conforme escopo confirmado pelo usuário no ticket 09, integrar o snapshot deste coletor ao cache em disco quando seu contrato existir. Reutilizar as portas de persistência e adicionar projeção tipada sanitizada: nunca persistir credenciais nem conteúdo de commits/cards/prompts/respostas. Restaurar o último dado na inicialização; para Rede, persistir apenas o último teste de velocidade.

## Decisões e desvios desta implementação

- **Wrapper:** script `sh` em `~/.local/share/kvojps-monitor/claude-statusline`. `statusLine.command` vira `sh '<caminho>'` (dispensa bit de execução). O wrapper repassa o stdin ao comando original via `sh -c`, devolvendo a saída e o código de saída dele. Em paralelo, e em segundo plano para não atrasar a status line, roda o próprio binário do app em modo `statusline <cache>`, que copia só `five_hour` e `seven_day` (`used_percentage`, `resets_at`) com o horário da leitura para `~/.cache/kvojps-monitor/claude-statusline.json`. Esse modo leva ~50 ms e não inicia o app. Num AppImage, aponta para o arquivo do AppImage. Com o app conectado, o wrapper é regravado a cada inicialização, porque o binário pode ter mudado de lugar. O `settings.json` nunca é regravado assim.
- **Configuração do app:** o `statusLine` original inteiro (inclusive `padding` e `refreshInterval`) fica em `~/.config/kvojps-monitor/claude-statusline.json`. Ao conectar, o app mantém as outras chaves do original e troca só `command`. Desconectar devolve o objeto original ou remove a chave. O teste compara o arquivo byte a byte com o de antes. Para isso, `serde_json` passou a usar `preserve_order`; como efeito colateral, as fixtures de Rede e Sistema, comparadas como texto, foram regeradas só com outra ordem de chaves (mesmos valores).
- **Gravação segura:** `settings.json` inválido ou que não seja objeto → "não é um JSON válido; nada foi alterado", sem gravar nada. Gravações por arquivo temporário + rename. Se gravar o `settings.json` falhar, a configuração do app é apagada.
- **Troca por terceiros:** checada a cada 5 s (com detecção e plano). Reconectar guarda a status line nova como original. Desconectar nesse estado só esquece a conexão e não toca no `settings.json`.
- **Janela descartada:** o Claude Code tira do payload a janela cujo `resets_at` passou. O app mantém a janela anterior já reiniciada, que aparece como "Reiniciado às HH:MM · sem leitura nova", em vez de sumir. Uma cópia mais antiga nunca substitui uma leitura mais nova.
- **Plano/detecção:** só `subscriptionType` é desserializado de `~/.claude/.credentials.json`; o token nunca é mantido. Sem `claude` no PATH ou sem `claudeAiOauth`: "Claude Code não encontrado · instale e faça login no CLI".
- **Sem conexão:** o Módulo mostra "Conecte o app ao Claude Code para ler os limites · Configurar", que leva a Configurações → IA. "Reconectar" no aviso de troca é um clique explícito e conecta direto.
- **Cache em disco:** Codex e Claude dividem `ai.json` por meio de `SharedCache`, e cada coletor grava só a sua chave. Só vão para o disco plano e janelas.
- **Fixture do payload:** transcrita do exemplo da documentação oficial, sem captura real; ver `src-tauri/fixtures/claude/README.md`. Credenciais e `settings.json` seguem a forma real desta máquina, com valores de exemplo.
- **Formato do `settings.json`:** ao conectar, o arquivo é regravado no formato do próprio Claude Code (JSON com 2 espaços, chaves na ordem original, quebra de linha final). Desconectar restaura exatamente o objeto `statusLine` original e os demais valores; outra indentação vinda de edição manual não é preservada.
- **Reinício sem leitura nova:** o coletor do Claude publica sozinho no instante em que uma janela reinicia, sem depender do Codex.
- Verificação visual no GNOME e conexão real ao `~/.claude/settings.json` desta máquina não realizadas.
