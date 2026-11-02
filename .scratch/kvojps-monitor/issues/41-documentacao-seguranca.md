# 41: Documentação e canal privado de segurança

**What to build:** `SECURITY.md` em pt-BR com instruções para relatar vulnerabilidades em privado pelo recurso de relatos privados de vulnerabilidades do GitHub.

**Status:** resolved

- [x] Verificar a disponibilidade do recurso para o repositório e documentar qualquer impedimento; não afirmar que está habilitado sem conferir
- [x] Documentar como encontrar o canal privado no GitHub, com link para o repositório correto
- [x] Orientar a não publicar credenciais, conteúdo de prompts/respostas, commits, cards ou outros dados pessoais em relatos públicos
- [x] Solicitar descrição, versão e passos mínimos de reprodução com dados sanitizados
- [x] Explicar que relatos privados de segurança são aceitos mesmo que o projeto não receba contribuições externas; não prometer prazo de resposta ou correção
- [x] Registrar a habilitação verificada como requisito antes da abertura pública; se exigir ação do mantenedor, deixar o passo concreto documentado e pendente
- [ ] Vincular `SECURITY.md` no README por meio do ticket 23 (integração pendente naquele ticket)

## Comments

- 2026-11-02 — Canal privado do GitHub aprovado pelo mantenedor. A disponibilidade e a ativação ainda precisam ser verificadas na execução; esta atividade não inclui mudar a visibilidade do repositório.

- 2026-11-02 — Implementado `SECURITY.md` em pt-BR. Consultas autenticadas à API confirmaram que o repositório já era público e que o canal estava desabilitado. Habilitado via `PUT /repos/kvojps/kvojps-monitor/private-vulnerability-reporting`; nova consulta retornou `enabled: true`. Nenhuma alteração de visibilidade. A habilitação não estava verificada antes da abertura já ocorrida; requisito mantido para a preparação da v1.
- 2026-11-02 — O README ainda não existe. Link e canal privado seguem como pendência explícita do ticket 23, sem criar um README pré-v1.

## Validação

- Revisão textual de `SECURITY.md` e dos tickets 23 e 41: somente política, URLs públicas do projeto, estado do recurso e procedimentos; sem capturas, credenciais ou conteúdo pessoal. Autor e committer configurados com endereço noreply do mantenedor.
- Referência: [documentação oficial do GitHub](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configure-for-a-repository). API consultada somente com projeção de visibilidade/permissões e estado do canal.

- Checagens: `npm run check` sem erros ou avisos; `npm test` com 376 testes aprovados; `cargo test --manifest-path src-tauri/Cargo.toml` com 305 testes unitários e 3 de integração aprovados. A execução inicial no sandbox negou sockets locais em dois testes; a suíte completa passou fora dele. `git diff --check` sem problemas.
- Revisão independente com a skill `code-review`, base `81060715091dfd718f4f68ef6246e1faacedb59d` e inclusão explícita do novo `SECURITY.md`: Standards, 0 achados; Spec, 0 achados. Integração futura ao README permanece no ticket 23.
