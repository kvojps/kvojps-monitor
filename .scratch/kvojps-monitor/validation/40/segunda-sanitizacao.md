# Segunda sanitização do histórico

Decisão posterior do mantenedor (02/11/2026): não enviar solicitação ao Support. O texto privado abaixo fica somente como registro, sem envio previsto nem retorno aguardado. O acesso residual por SHA continua documentado; o expurgo completo do servidor não foi comprovado. O fechamento restante do ticket depende dos documentos e capturas finais da v1 e da respectiva revisão.

Em 02/11/2026, o mantenedor autorizou executar a limpeza consolidada dos achados da [revisão textual](revisao-textual.md) e da [revisão histórica](revisao-historica.md), incluindo expurgo histórico, atualização da única referência pública e verificação por clone novo.

## Correções preparadas

- Resultados privados na pesquisa do GitHub e exemplos derivados em tickets/spec substituídos por contagens e horários sintéticos declarados. A transformação histórica inclui os dois tickets removidos da árvore atual. Não altera o comportamento especificado.
- Metadados extras de branch omitidos de `src-tauri/fixtures/github/viewer.json`, pois o adaptador usa dessa resposta somente a identidade, o calendário e os nomes dos repositórios. Preserva-se a estrutura necessária aos testes; o README da fixture registra a omissão e a origem dos dados restantes.
- Expurgo dos caminhos históricos `.scratch/compact-panel-implementation/validation/ai-dark.png` e `ai-light.png`. Já ausentes da árvore atual; as demais evidências de validação são preservadas.
- Os inventários da revisão representam a base anterior à limpeza, não uma aprovação nem o inventário da base reescrita. Resultados e mapas privados de substituição não são publicados neste registro.

## Validação da árvore preparada

`npm run check`: zero erros e avisos. Testes UI de GitHub, configurações GitHub e integração do Painel: 98 aprovados em três arquivos. Testes Rust do GitHub: 36 aprovados. `git diff --check` aprovado. Não há mudança de código executável nem testes novos; foram executados os testes existentes afetados pela alteração dos dados.

## Execução e limites

A preparação da reescrita usa clone bare isolado sem hardlinks, transformação dos blobs nos seis caminhos textuais e remoção dos dois caminhos de imagens em todas as referências locais. Branch/tag de protótipo permanece somente local. A ponta reescrita deve conservar exatamente a árvore já testada. A atualização remota usa somente `refs/heads/main` e lease sobre o SHA remoto inventariado, sem enviar outras branches ou tags.

Estado inicial do servidor: somente `main`, coincidente com a base revisada. A remoção de objetos antigos dos caches do GitHub continua sendo uma pendência separada: reescrita e clone novo não comprovam garbage collection do servidor. Nenhum contato com terceiros foi feito.

## Resultado da execução

- Reescritos 19 blobs textuais originais; removidas as duas imagens históricas. Todas as referências locais de branches/tags foram tratadas para evitar reintrodução acidental, preservando nomes, modos e symlinks dos demais arquivos.
- Verificação de todas as referências locais: 180 commits e 1.748 blobs únicos; nenhum blob original alterado nem imagem expurgada alcançável. Zero ocorrências dos horários privados identificados nos blobs, e nenhum par de contagens privado nos caminhos textuais afetados. Todas as versões da fixture omitem os metadados extras de branch.
- A árvore da ponta reescrita ficou idêntica à do commit de sanitização já testado. Não foi necessário executar novamente as suítes sobre bytes idênticos.
- `main` pública atualizada para `cbe706922495ea68d27c51ab5d4b2bd9c2621f86`, usando lease explícito sobre a base remota inventariada. Nenhuma branch/tag de protótipo foi publicada.
- Clone mirror novo do GitHub: somente `refs/heads/main`, 176 commits e 1.732 blobs únicos. Repetidas as verificações dos blobs originais, horários privados, pares de contagens nos caminhos afetados, fixture e ausência dos dois caminhos de imagens em todas as árvores: aprovadas.
- Reflogs locais expirados e garbage collection executado. Inspeção de todos os objetos físicos locais, inclusive os não alcançáveis: nenhum dos 19 blobs textuais originais alterados nem das duas imagens permanece. Não há ponteiro `ORIG_HEAD` antigo.

O commit deste resultado acrescenta somente documentação e será publicado por atualização normal da `main`. Os inventários anteriores continuam como evidência da revisão da base antiga; não devem ser usados como lista de objetos da base atual.

### Pendência do servidor confirmada novamente

Após o force-push, uma consulta silenciosa à API Git retornou sucesso para o commit da base anterior à segunda limpeza. Portanto, o acesso residual por SHA continua confirmado, mesmo que o clone novo não transfira esses objetos. Não se afirma remoção completa dos dados do GitHub.

Complemento preparado para o relato **privado** ao Support, junto do texto já registrado em [sanitizacao.md](sanitizacao.md):

> Houve uma segunda limpeza autorizada em `Kvojps/kvojps-monitor`: foram removidas duas imagens históricas com identificadores de sessão/projeto e substituídos resultados privados de atividade GitHub em pesquisa, fixture e exemplos derivados. A única referência pública continua sendo `main`; nenhuma tag ou branch de protótipo foi enviada. Um clone novo confirmou a limpeza das referências. A API ainda responde ao commit da base anterior à segunda limpeza, `a02568bef300b3820742fd7d125b3d16cab8b6f2`. Solicito incluir os objetos órfãos e caches dessa segunda história na avaliação de expurgo. Não foi identificada credencial utilizável nesses novos achados.

Esse texto não foi enviado. O acompanhamento depende do canal de suporte e da avaliação do provedor; a revisão final da v1 permanece separada.

Esta limpeza não encerra o ticket 40: documentos e capturas finais dos tickets 22/23/41/42 ainda exigem revisão, e o retorno de expurgo do servidor permanece pendente.
