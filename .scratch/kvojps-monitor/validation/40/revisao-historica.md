# Revisão do histórico após a sanitização

Atualização posterior: os achados deste diagnóstico foram tratados na [segunda sanitização](segunda-sanitizacao.md), com atualização da `main` pública e verificação por clone novo. Este relatório e o inventário representam a base anterior. O acesso residual a objetos no servidor e a revisão final da v1 continuam pendentes.

Revisão local em 02/11/2026, sobre `main` em `a02568bef300b3820742fd7d125b3d16cab8b6f2`. **Resultado: há bloqueios de privacidade em imagens e textos; o histórico não está aprovado.** Este registro complementa o relatório inicial e a sanitização, sem substituí-los. Não houve consulta ao servidor, push, reescrita ou contato com terceiros nesta etapa.

## Escopo e método

- Enumerados os 175 commits alcançáveis pela `main`, com `git rev-list main`; os objetos foram lidos com `git cat-file`, sem checkout de versões antigas nem execução de scripts históricos.
- Todas as árvores foram enumeradas com `git ls-tree -rz`, incluindo arquivos removidos e renomeados. São 783 caminhos históricos, 1.726 blobs únicos e 1.764 associações entre blob e caminho. A enumeração dos nomes feita somente por `git rev-list --objects` não representa todos os aliases; por isso foi complementada pelas árvores.
- Classificação por assinatura e conteúdo: 1.633 blobs UTF-8, 88 PNG, um SQLite e quatro mensagens netlink TCP. Os 38 symlinks históricos apontam para diretórios relativos de skills dentro do repositório; os alvos foram tratados como texto, sem seguir links para o host. Não há submódulos nas árvores examinadas.
- Os textos totalizam 13.692.617 bytes. A busca completa cobriu assinaturas de chaves privadas, GitHub, Anthropic/OpenAI, AWS, Google, Slack e JWT, além de caminhos pessoais, e-mails, IPv4, MAC, UUID, autenticação em URLs e literais opacos. Nenhuma assinatura de credencial pesquisada foi encontrada. Isso não comprova ausência de credenciais de outros formatos.
- Revisão semântica dirigida aos dados: extração e leitura dos 355 valores textuais distintos dos JSON/JSONL históricos de fixtures, com 244 documentos/registros válidos; leitura das declarações de origem e dos candidatos de credenciais/conteúdo nos testes, pesquisas, tickets e skills. A busca por campos de credencial/conteúdo produziu 1.491 linhas, reduzidas a 449 linhas distintas para inspeção. As ocorrências verificadas correspondem a contratos de API, instruções, exemplos e sentinelas de teste, não a conversas gravadas.
- Inspeção dos assuntos dos 175 commits e varredura dos corpos e identidades. Autor e committer usam um único endereço noreply. Assuntos descrevem trabalho deste projeto; não houve novo achado por esses controles. Não são reproduzidos endereços nem conteúdos privados neste relatório.

**Complemento textual:** a revisão inicial dirigida acima foi ampliada para todas as 1.633 versões textuais, com leitura organizada por prosa, literais/comentários/templates, dados estruturados e recursos. Método, limites, inventário por blob e novo achado de atividade privada do GitHub estão em [revisao-textual.md](revisao-textual.md). Trata-se de revisão de privacidade por conteúdo distinto, não de revisão funcional linha a linha do código. Cobertura da revisão não significa aprovação do material: os achados permanecem bloqueantes.

## Origem dos dados

- Claude Agents, rollouts e app-server Codex, uso Claude, rede, sistema, aparência, sessão e respostas GitHub/Trello incluem dados derivados de capturas reais. Sanitização não os torna sintéticos. Contagens, horários e estrutura necessários aos cenários permanecem conforme as declarações de origem.
- Nomes, caminhos, IDs e conteúdo de cards/commits nos JSON inspecionados usam exemplos; hooks e operações usam sentinelas explícitas de privacidade. O payload da status line Claude é transcrito de documentação. Variantes de estado são controladas, conforme os READMEs e testes.
- Respostas de releases contêm metadados de API pública. Nomes de processos e sensores são categorias necessárias ao cenário, não conteúdo de aplicação.
- Endereços de fixtures usam faixas de documentação/aliases e loopback. Destinos públicos de conectividade e referências a versões de software foram distinguidos de endereços de rede capturados. UUIDs examinados são exemplos consistentes com os helpers/testes. Uma correspondência de e-mail na configuração Tauri é nome de arquivo de ícone, não identidade pessoal.
- Capturas de validação e protótipos são imagens da UI alimentada por fixtures; isso, por si só, não garante sanitização: duas imagens conservam identificadores anteriores à limpeza textual.

## Inspeção visual e metadados de imagens

As 88 versões PNG únicas foram abertas e inspecionadas em 11 pranchas, com identificação por blob. A cobertura inclui as validações 33/38/39, a validação histórica de implementação compacta, os protótipos compactos, `docs/assets/painel-b.png` e os ícones. Cada versão e seus caminhos estão no [inventário de binários](binarios-historicos.tsv).

Conferidos CRCs e limites de todos os chunks, além da ausência de bytes após `IEND`. Os únicos tipos de chunk são `IHDR`, `bKGD`, `IDAT` e `IEND`: não há chunks textuais, EXIF ou outros metadados auxiliares. Isso não é análise de esteganografia.

### Achado bloqueante

| Caminho histórico | Categoria | Evidência segura |
| --- | --- | --- |
| `.scratch/compact-panel-implementation/validation/ai-dark.png` | Nome de sessão e projeto sem sanitização visual | Texto identificável na lista de Agentes; blob `6fe55697ff` no inventário. |
| `.scratch/compact-panel-implementation/validation/ai-light.png` | Nome de sessão e projeto sem sanitização visual | Mesmo cenário no tema claro; blob `768d740b39` no inventário. |

As imagens foram introduzidas no commit `73d7706`, continuam alcançáveis pela `main` e já não existem na árvore da ponta. Os nomes visíveis não aparecem nos textos históricos atuais; portanto, a busca de literais em texto e a conferência da árvore atual não detectam esse resíduo. Não há credencial utilizável identificada nessas imagens. O achado é de metadados de sessão/projeto, não de conteúdo de conversa.

**Correção necessária:** substituir as duas imagens históricas por versões visualmente sanitizadas, ou expurgar esses caminhos do histórico publicado, preservando o restante da validação. Conferir novamente todas as referências e um clone novo depois da correção. Nenhuma dessas alterações foi executada nesta revisão. A autorização anterior de sanitização está registrada no ticket; a execução de outra reescrita/atualização remota deve ter escopo explícito para este novo achado. Acrescentar esta categoria ao eventual acompanhamento privado do expurgo no servidor, sem copiar os valores.

As demais 86 versões não produziram novo achado visual de privacidade. O protótipo B continua identificado como protótipo; a inspeção não substitui as capturas reais finais exigidas pelo ticket 23.

## SQLite e capturas TCP

- SQLite histórico: uma única versão, 8.192 bytes, duas páginas, uma tabela e uma linha de metadados. `PRAGMA integrity_check` retornou `ok`; freelist vazia. As duas páginas folha não têm freeblocks nem fragmentos, e o espaço não alocado antes das células está zerado. Colunas de identificação/caminho/título usam exemplos; a reconstrução já documentada evita transportar páginas livres do original. Não foi aberto banco real do usuário.
- Quatro blobs TCP de 400 bytes: comprimento netlink, tipo, sequência/PID, portas, endereços IPv4 de loopback, índice de interface, cookie, UID, inode e limites/alinhamento dos atributos conferidos contra o contrato sanitizado. Cgroup zerado nas quatro mensagens. Atributos presentes: 8, 21, 22 e 2; os contadores TCP permanecem derivados da captura. Não são pacotes de aplicação nem carregam prompts/respostas. Nenhuma nova captura de rede foi realizada.
- O inventário contém 94 associações blob/caminho para os 93 blobs não textuais; um blob aparece em mais de um caminho histórico.

## Estado de fechamento

- [x] Inventariar todas as árvores e versões alcançáveis pela `main` na base indicada.
- [x] Inspecionar visualmente as 88 versões PNG e verificar seus chunks/metadados.
- [x] Inspecionar o SQLite e as quatro capturas TCP históricas por formato.
- [x] Distinguir capturas derivadas de dados reais, exemplos e sentinelas nos dados inspecionados.
- [x] Registrar os achados sem reproduzir os identificadores ou valores privados.
- [x] Ampliar a revisão semântica para todos os textos históricos, além da revisão dirigida inicial, com método e limites no complemento textual.
- [ ] Corrigir a atividade privada do GitHub na pesquisa, fixture e exemplos derivados, incluindo suas versões históricas.
- [ ] Resolver as duas imagens históricas e verificar novamente o histórico resultante.

A revisão final da v1, os tickets 22/23/41/42 e a pendência de objetos antigos no servidor continuam separados. Nenhuma aprovação de publicação decorre desta etapa.

## Verificação documental

Alterações limitadas à documentação do ticket e aos inventários por caminhos/categorias. `git diff --check` passou; os arquivos novos também foram conferidos quanto a espaços no fim das linhas. Não há mudança de código/fixtures e não foi necessária repetição das suítes UI/Rust. Os arquivos extraídos e as pranchas foram mantidos temporariamente fora da árvore Git e removidos ao concluir.
