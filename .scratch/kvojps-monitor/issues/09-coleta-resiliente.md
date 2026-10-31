# 09: Coleta resiliente: backoff, credencial inválida, cache em disco e logs

**What to build:** falhas transitórias não me incomodam: o app tenta de novo com backoff e só mostra "Falha ao atualizar há X" após 3 falhas seguidas, sem notificação. Uma credencial recusada (401/403) deixa o último dado esmaecido com a idade e uma faixa "Credencial inválida · Reconectar", e o app para de tentar até a credencial mudar. Logo após o login vejo o último dado conhecido, vindo de um cache em disco sem credenciais. Há um arquivo de log só com metadados. Aplicado ao GitHub nesta fatia; IA e Trello entram nesse mecanismo depois. Ver spec, "Ritmo de coleta" e "Logs".

**Blocked by:** 08 (GitHub via gh)

**Status:** feito

- [x] Backoff em timeout, 5xx e 429: 30 s → 1 → 2 → 4 → 8 → 15 min, respeitando `Retry-After`
- [x] "Falha ao atualizar há X" após 3 falhas seguidas; sem notificação
- [x] 401/403: para de tentar até a credencial mudar; faixa "Credencial inválida · Reconectar"; dado esmaecido com idade (teste explícito)
- [x] Cache sanitizado do GitHub em `~/.cache/kvojps-monitor/`, carregado na inicialização e sem credenciais (teste explícito). Integração de IA, Trello e último teste de velocidade transferida para os tickets 06, 12, 14 e 15 com autorização do usuário, conforme decisões abaixo
- [x] Logs em `~/.local/state/kvojps-monitor/`, rotação de 3 × 1 MB, nível `info`, só fonte, evento, status HTTP e duração; nunca credenciais, prompts, respostas nem títulos (teste explícito)
- [x] Testes de UI por fixtures (desatualizado, falha, credencial inválida)

## Decisões confirmadas nesta implementação

- O usuário autorizou cache sanitizado: contagens, datas e identificação de repositórios podem persistir; credenciais e conteúdo de commits/cards/prompts/respostas não. O título do commit existe apenas em memória e volta após atualização; não é persistido nem registrado em log.
- IA, Trello e teste de velocidade ainda não possuem coletores. O usuário confirmou integrar o cache nesta fatia somente ao GitHub, deixando a integração dos outros snapshots para os tickets 06, 12, 14 e 15, quando seus contratos existirem. A infraestrutura de persistência usa portas injetáveis e pode ser reutilizada por essas fatias.
- A revisão usa o HEAD anterior ao trabalho: `caea54cd612c56873d3e1a282de4c9fd944ae6de`.

## Implementação e validação

- Backoff contado após a falha da requisição; `Retry-After` suporta segundos e data HTTP. O relógio injetado governa todos os prazos, inclusive requisições que consomem tempo. Uma atualização bem-sucedida limpa a sequência de falhas.
- 401 e 403 preservam o snapshot, exibem a faixa de reconexão e esmaecem o dado com sua idade. O app consulta a credencial do `gh` para detectar mudanças, sem novas requisições HTTP enquanto ela continuar recusada; nenhuma credencial é persistida.
- Cache gravado por projeção tipada e substituição por rename. Falhas de leitura/gravação não interrompem a coleta. O snapshot restaurado omite títulos e já está disponível antes de HTTP.
- Log `info.log`, `info.1.log`, `info.2.log`, limitados a 1 MB cada. Eventos fechados e campos tipados impedem conteúdo livre no log.
- Fixtures de UI de dado desatualizado, falha persistente e credencial inválida são verificadas contra snapshots publicados pelo núcleo.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (87 testes), `npm run check`, `npm test` (31 testes) e `npm run build`: aprovados.
- Revisão em dois agentes: Standards e Spec sem achados restantes após corrigir o início do backoff e eliminar o relógio real na interpretação de `Retry-After`.
- Verificação visual no GNOME não realizada nesta execução.
