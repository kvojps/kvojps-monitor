# Segunda sanitização do histórico

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

Estado inicial do servidor: somente `main`, coincidente com a base revisada. Execução, conferência por clone novo e expurgo local serão registrados após a validação de cada etapa. A remoção de objetos antigos dos caches do GitHub continua sendo uma pendência separada: reescrita e clone novo não comprovam garbage collection do servidor. Nenhum contato com terceiros foi feito.

Esta limpeza não encerra o ticket 40: documentos e capturas finais dos tickets 22/23/41/42 ainda exigem revisão, e o retorno de expurgo do servidor permanece pendente.
