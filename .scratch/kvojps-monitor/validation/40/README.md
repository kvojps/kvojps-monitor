# Revisão antes da abertura pública

Revisão inicial em 02/11/2026. **Este documento não é uma aprovação de publicação.**

Atualização: o mantenedor autorizou a limpeza e a consulta ao GitHub revelou que o repositório já estava público. O registro abaixo preserva as constatações iniciais; o estado posterior e a sanitização ficam em [sanitizacao.md](sanitizacao.md).

## Escopo de publicação

Base examinada: `447c220e1daace999aaad910a919e4762231909e` da `main`, incluindo os 169 commits alcançáveis e arquivos removidos ou substituídos nesse histórico. O commit desta revisão acrescenta documentação ao escopo; mudanças posteriores exigem nova revisão.

A referência prevista para publicação é somente `refs/heads/main`. Nenhuma tag está aprovada. Uma futura tag de release exige revisão própria. Não usar push com `--all`, `--mirror`, `--tags` ou envio automático de tags. Branches `research/*` e `prototype/*` permanecem exclusivamente locais. A branch local `prototype/panel-opening` e as tags `archive/prototype/panel-identity`, `archive/prototype/panel-motion` e `archive/prototype/panel-operations` ficam excluídas. Essa exclusão não retira objetos já alcançáveis pela `main`.

`refs/remotes/origin/main` e `refs/remotes/origin/HEAD` apontavam para a mesma base. Não foi consultado o servidor: inventário remoto, referências adicionais, releases, artefatos, issues e anexos precisam ser conferidos antes de mudar a visibilidade. Tornar um repositório existente público expõe também material já presente no servidor, independentemente do próximo push.

## Método e cobertura

- Inventário local com `git status --short`, `git for-each-ref`, `git ls-files` e `git rev-list --objects main`. Árvore inicialmente limpa; 586 arquivos versionados.
- Leitura de cada um dos 1.701 blobs únicos alcançáveis com `git cat-file`, incluindo versões históricas. Busca de assinaturas de tokens GitHub/Anthropic e chaves privadas PEM; identificação de candidatos por campos de credencial/conteúdo, e-mail, caminhos de usuário, IPv4 e MAC. Foram encontrados 93 blobs com bytes nulos, que exigem interpretação específica; a busca textual não os valida.
- Busca nos corpos das mensagens e identidades de autor/committer dos 169 commits. O relatório conserva somente categorias, nunca valores encontrados. Não foram exportados dumps nem trechos dos candidatos para este diretório.
- Leitura das declarações de origem das fixtures e dos guias de captura; inspeção do schema SQLite em modo somente leitura. Os candidatos não recebem aprovação automática por estarem em testes.
- Escopos atuais incluídos na busca: 86 arquivos em `src-tauri/fixtures/`, seis em `docs/research/`, 136 em `.scratch/`, um em `docs/assets/` e 103 em `.agents/skills/`. Código, configurações, snapshots de UI e demais arquivos também entraram na varredura dos blobs.
- Inspeção visual individual de `docs/assets/painel-b.png`: mostra demonstração com nomes de exemplo e métricas; não mostra credenciais nem conversa. É protótipo, não captura da versão distribuída, e não satisfaz o ticket 23. As futuras capturas do README ainda não existem.

Não houve correspondência às assinaturas de credenciais pesquisadas. Isso não prova ausência de credenciais de outros formatos, dados codificados ou conteúdo privado sem marcadores. A leitura semântica integral de todas as versões textuais e a revisão visual de todas as imagens históricas não foram concluídas. Há 88 caminhos de imagens no histórico; capturas em `.scratch/` também serão expostas pela `main` e precisam dessa revisão.

## Origem dos dados e achados

| Caminhos / categoria | Avaliação e ação necessária |
| --- | --- |
| `src-tauri/fixtures/agents/recorded.json` | Captura real com caminho pessoal, identificadores e nome de sessão/projeto. Sanitizar mantendo o contrato; verificar expectativas nos testes e todas as versões históricas. Conteúdo de metadados também pode identificar trabalho privado. |
| `src-tauri/fixtures/network/address-data.json`, `networkmanager.json`, `neighbors.json`; versões antigas em `src-tauri/fixtures/rede/`; `docs/research/rede-linux.md` | Capturas reais com endereçamento e identificadores de rede. Tratar como dados pessoais pendentes de sanitização; revisar também snapshots e testes derivados em `src/lib/network/fixtures/` e `src-tauri/src/network/`. |
| `src-tauri/fixtures/system/notebook/proc/self/mountinfo` | Captura real de topologia e identificadores de armazenamento. Revisar e substituir identificadores do host por exemplos consistentes. |
| Metadados de autor/committer do histórico Git | E-mails fora do domínio de anonimização do GitHub. Confirmar consentimento específico para exposição; não copiar os endereços para a validação. Trocar a configuração futura não altera commits anteriores. |
| `src-tauri/fixtures/codex/` | Capturas declaradas sanitizadas: conta e diretório substituídos; rollouts reduzidos a tipos e contagens. Permanecem horários, métricas e identificadores de sessões nos nomes de arquivo; revisar necessidade/consentimento para esses metadados. |
| `src-tauri/fixtures/agents/codex/` | Captura declarada anonimizada. SQLite contém uma tabela de sessões, uma linha e apenas colunas de metadados; essa inspeção de schema não comprova ausência de dados residuais nas páginas do arquivo. Exigir inspeção binária ou reconstrução sanitizada antes da aprovação. |
| `src-tauri/fixtures/claude/` | Credenciais com valores explícitos de exemplo; payload de status line transcrito da documentação; resposta de uso transcrita de teste real e configurações locais encurtadas. Não classificar os valores de exemplo como vazamento de token. |
| `src-tauri/fixtures/github/`, `src-tauri/fixtures/trello/` | Capturas declaradas sanitizadas, com nomes/IDs/títulos substituídos; preservar essa distinção em futuras recapturas. |
| `src-tauri/fixtures/network/tcp/` | Captura binária real de loopback, declarada sem conteúdo de aplicação; contém metadados de processo/socket. Falta validação binária independente de todas as versões. |
| `src-tauri/fixtures/releases/`, `session/`, `appearance/`, `system/` | Metadados reais de API pública, D-Bus e sistema, com variantes controladas. Não são todos dados sintéticos. |
| `.scratch/`, `docs/research/`, `.agents/skills/`, snapshots da UI | Busca incluiu planejamento, scripts, validações, instruções e exemplos. Campos de prompt, resposta, commit e card podem ser sentinelas sintéticas; busca por chave sozinha não distingue conteúdo privado. Revisão semântica e visual histórica restante é requisito de liberação. |

Os achados de dados reais acima bloqueiam a abertura até sanitização e decisão sobre suas versões históricas. O ticket autoriza registrar a correção necessária, não reescrever o histórico nem revogar credenciais. Nenhum desses atos foi realizado. Sanitizar somente a árvore atual não resolve versões anteriores; qualquer expurgo histórico precisa de autorização específica e nova revisão das referências remotas e locais.

## Dependências antes da abertura

| Requisito | Estado observado |
| --- | --- |
| `LICENSE` MIT — ticket 22 | Ausente; declaração MIT no Cargo não substitui o arquivo. |
| `README.md` e capturas reais sanitizadas — ticket 23 | Ausentes. |
| `SECURITY.md` e canal privado verificado — ticket 41 | Documento ausente; habilitação do canal não verificada nesta atividade. |
| `docs/desenvolvimento.md` — ticket 42 | Presente, restrito a protótipo, fixtures e captura; guia de clone limpo, build e forks ainda pendente. |

## Fechamento pendente

1. Sanitizar os caminhos e categorias acima e conferir testes derivados; decidir explicitamente sobre exposição de identidades e metadados reais.
2. Obter autorização específica caso seja necessário expurgar histórico; revisar novamente todos os objetos ainda alcançáveis após a correção.
3. Concluir a revisão semântica dos textos e a inspeção dos binários/imagens históricos, incluindo `.scratch/` e objetos removidos da árvore atual.
4. Concluir tickets 22, 23, 41 e 42; revisar visualmente cada captura final do README.
5. Inventariar o servidor e registrar aqui o SHA final, todas as referências e superfícies realmente expostas, método, resultados, limitações e ausência de pendências bloqueantes.

A revisão final permanece pendente até esses passos. Nenhum push ou alteração de visibilidade foi realizado.

## Verificação desta entrega

`npm run check`: zero erros e avisos. `npm test`: 376 testes em 16 arquivos passaram. `cargo test --manifest-path src-tauri/Cargo.toml --locked --quiet`: 305 testes unitários e três de integração passaram. A primeira execução no sandbox impediu dois testes de socket; a repetição autorizada fora dele passou. `git diff --check` passou. Nenhum teste novo foi criado para esta alteração documental.

Revisão pela skill `code-review`, comparando `447c220...cf8c335`:

- **Standards:** zero achados. Documentação em pt-BR, status canônico, registro por caminhos/categorias e restrição de branches preservados.
- **Spec:** dois requisitos parciais. A revisão semântica/visual histórica continua incompleta e pode avançar com os arquivos locais, independentemente das dependências externas. O checklist de inspeção das capturas finais estava marcado prematuramente; foi reaberto. A entrega é inicial e não encerra o ticket.

A inspeção do histórico restante não depende de autorização para leitura. A autorização específica só será necessária para eventual expurgo; a revisão final também depende dos documentos e capturas ainda ausentes.
