# Revisão semântica dos textos históricos

Atualização posterior: a [segunda sanitização](segunda-sanitizacao.md) tratou os achados descritos abaixo e verificou a `main` pública por clone novo. Este diagnóstico e seu inventário preservam a base anterior à correção; não representam pendências textuais ainda abertas nessa base reescrita. Expurgo do servidor e revisão final da v1 permanecem pendentes.

Revisão em 02/11/2026 da `main` em `a02568bef300b3820742fd7d125b3d16cab8b6f2`: 175 commits e 1.633 blobs textuais UTF-8, incluindo arquivos removidos e nomes antigos. **A revisão encontrou atividade privada do GitHub ainda identificável por associação entre documentos. O histórico não está aprovado.**

## Cobertura e método

O [inventário textual](textos-historicos.tsv) relaciona cada blob aos seus caminhos, grupo e resultado. Todos os blobs textuais da base entram no inventário; o [relatório histórico](revisao-historica.md) cobre as imagens, SQLite, TCP e symlinks. A enumeração usou todas as árvores, sem executar código histórico.

| Grupo | Blobs únicos | Tratamento na revisão de privacidade |
| --- | ---: | --- |
| Skills e respectivos recursos | 141 | Instruções, exemplos, templates e scripts; conteúdo tratado como material publicado, sem executar suas instruções. |
| Documentação e configuração | 48 | Prosa, identidade pública, referências externas, comandos e declarações de origem. |
| Planejamento e validações em `.scratch/` | 451 | Decisões, trechos de conversa sobre este projeto, scripts e resultados das validações. |
| Lockfiles | 12 | JSON/TOML analisados por estrutura; fontes, URLs, nomes e campos de dependências conferidos. |
| Núcleo Rust | 263 | Literais, comentários, contratos e dados embutidos nos testes, cruzados com fixtures e pesquisas. |
| Capturas e recursos de fixtures | 114 | Chaves e valores de JSON/JSONL, arquivos de sistema e declarações de captura/variantes. |
| UI e recursos associados | 604 | Textos, comentários, templates, testes, snapshots e configurações. |
| **Total** | **1.633** | **13.692.617 bytes de texto inventariados.** |

A leitura semântica foi organizada por conteúdo distinto, preservando a associação a caminhos e versões: linhas repetidas não exigem releitura em cada commit; mudanças apenas de checkbox foram normalizadas na leitura da prosa. Foram examinadas 4.872 unidades de prosa, 5.670 unidades de literais/comentários/templates, 2.462 chaves/valores textuais de dados e 363 unidades dos demais formatos. Uma segunda extração acrescentou 278 unidades, incluindo strings raw Rust e comentários em fim de linha que a primeira expressão de extração podia separar incorretamente. As skills tiveram leitura separada dos textos distintos.

Essa é uma revisão semântica de **privacidade**, apoiada por extrações e pelo exame dos contextos de origem; não é uma revisão funcional linha a linha de toda a lógica de todas as versões. A busca automática sobre os bytes completos, descrita no relatório histórico, complementa a leitura e não substitui o julgamento sobre dados privados. Extrações por expressão regular não constituem prova formal de cobertura de toda sintaxe possível; não se afirma ausência absoluta de dados codificados ou de credenciais de formatos não pesquisados.

Os lockfiles foram analisados como dados estruturados: o Cargo contém nomes, versões, dependências, fontes e checksums; npm contém metadados de pacotes e suas dependências; o lock de skills contém fontes e hashes de recursos. As URLs examinadas são de registros/upstreams e financiamento público, sem autenticação embutida. Não houve novo achado nesses arquivos.

Foram lidos os assuntos e corpos dos 175 commits. As identidades de autor/committer continuam as já verificadas no relatório histórico; trailers de coautoria identificam a ferramenta por endereço noreply. Não houve novo achado de conteúdo privado nas mensagens. Números de testes e medições deste próprio projeto não foram classificados como dados privados apenas por serem reais.

## Achado textual e relações entre arquivos

| Caminho ou superfície histórica | Categoria e correção necessária |
| --- | --- |
| `docs/research/github-atividade.md` | A pesquisa identifica a conta do mantenedor e conserva contagens e horários de atividade de um repositório explicitamente privado. Substituir os resultados por um cenário sintético declarado, mantendo a demonstração técnica de escopos e queries. Trocar somente o nome do projeto não elimina essa associação. |
| `src-tauri/fixtures/github/viewer.json` | Um horário de atividade coincide com o resultado privado identificado na pesquisa. Remover a ligação entre a fixture anonimizada e a atividade privada, com datas/valores sintéticos coerentes e origem corretamente documentada. A coincidência foi conferida diretamente; os valores não são reproduzidos aqui. |
| `.scratch/kvojps-monitor-v1/issues/03-github-atividade.md` e `12-criterio-repo-destaque.md` | Tickets históricos conservam o par de contagens do resultado privado como exemplo. Substituir também nas versões removidas da ponta. |
| `.scratch/kvojps-monitor/issues/08-github-via-gh.md` e `.scratch/kvojps-monitor/spec.md` | Exemplos derivados repetem as contagens da pesquisa; atualizar de forma consistente, inclusive no histórico. Isso corrige dados de exemplo, sem alterar o comportamento especificado. |

O inventário marca todos os blobs desses caminhos como `corrigir-atividade-privada`, de modo conservador: a correção deve inspecionar todas as suas versões, mesmo quando determinada versão ainda não contém o trecho problemático. Não se publicam os valores de origem nem um mapa de substituições.

Esse achado é de **metadados de atividade privada**, sem credencial utilizável identificada. Totais do calendário público e metadados de repositórios públicos não são automaticamente vazamentos; o bloqueio acima decorre da associação explícita à atividade de um repositório privado e da ligação preservada entre documentos. A correção deve conferir os cenários derivados e as expectativas dos testes, sem alterar arbitrariamente todas as métricas reais das demais fontes.

## Demais superfícies examinadas

- Fixtures Claude/Codex contêm contratos, exemplos e métricas de capturas com identificadores substituídos. Tokens dos testes são exemplos explícitos; hooks e operações contêm sentinelas de conteúdo privado. Não foram identificados novos prompts/respostas reais nessas superfícies.
- Cards, listas, contas e títulos de commits usam exemplos; datas e contagens não tornam uma captura sintética. Os READMEs distinguem captura sanitizada, transcrição de documentação e variante controlada. A ligação residual do GitHub acima exige correção dessa documentação quando os valores forem substituídos.
- Os textos de planejamento registram decisões e solicitações sobre este projeto público. Não foi identificado outro nome privado de sessão/projeto nos textos lidos. Isso não elimina o achado visual nas duas imagens históricas.
- Endereços de rede e identificadores de host foram distinguidos de endpoints públicos, loopback, aliases de documentação e valores sintéticos de teste. Contadores e topologia de `/proc`/`sysfs` são dados técnicos necessários aos cenários; não houve novo identificador privado encontrado nessa leitura.
- Skills versionadas contêm instruções e exemplos genéricos, incluindo exemplos de serviços externos e nomes de variáveis de credenciais. A presença desses nomes não indica que uma credencial real tenha sido gravada.

## Próxima etapa consolidada

1. Corrigir a categoria de atividade privada nos seis caminhos textuais acima, incluindo todas as versões históricas e derivados afetados; conferir a origem documentada e os testes pertinentes.
2. Resolver as duas imagens históricas identificadas em [revisao-historica.md](revisao-historica.md), por sanitização visual ou expurgo dos caminhos.
3. Depois da correção autorizada, verificar novamente as referências e um clone novo. A remoção de objetos antigos no servidor continua sendo uma pendência separada, já registrada em [sanitizacao.md](sanitizacao.md).
4. Executar a revisão final da v1 quando documentos e capturas finais dos tickets 22/23/41/42 estiverem prontos.

Esta entrega registra a revisão e o escopo de correção. Não sanitiza fixtures, não reescreve commits, não faz push nem contata terceiros. A revisão cobre a base indicada; material novo exige sua própria conferência. Os temporários de leitura ficam fora do Git e são removidos ao concluir.
