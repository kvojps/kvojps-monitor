# Sanitização autorizada

Em 02/11/2026, após a revisão inicial, o mantenedor autorizou sanitizar os dados identificados e removê-los também do histórico Git. Este registro complementa o [relatório inicial](README.md); não aprova a revisão final da v1.

## Escopo e correções

- Caminho pessoal, projeto privado, nome e identificadores da sessão Claude substituídos por exemplos; mesmas substituições nas pesquisas, snapshots da UI e testes derivados.
- UUIDs reais dos nomes de rollouts substituídos por UUIDs sintéticos, com atualização das referências nos testes. Contadores e horários necessários aos cenários foram preservados.
- IPv4 da captura usa faixa de documentação; IPv6 global usa `2001:db8::/32`, link-local usa aliases e MAC usa endereço administrado localmente. As substituições incluem versões anteriores em `fixtures/rede/` e pesquisas.
- Montagens mantêm a estrutura de captura e os tipos de sistema de arquivos, com aliases para dispositivos, IDs de montagem, pacotes snap e namespaces. Os testes de deduplicação usam o mesmo dispositivo de exemplo da fixture.
- SQLite reconstruído em um arquivo novo a partir do schema e das linhas sanitizadas, evitando copiar páginas livres ou bytes residuais.
- Netlink de loopback mantém formato, atributos e contadores; portas, PID, interface, cookie, proprietário, cgroup e inode foram substituídos. Arquivos auxiliares de processo/proprietário usam apenas exemplos mínimos.
- E-mails de autor e committer substituídos por endereço noreply, preservando nome público, datas, mensagens e sequência dos commits, salvo substituições de dados identificados. Configuração local de novos commits ajustada para usar noreply.
- Helpers de captura de endereços e TCP sanitizam os campos antes de gravar. Nenhuma credencial foi revogada: não foi identificada credencial válida pelos métodos empregados.

Os mapas com valores de origem foram usados somente em diretório temporário privado. Não são parte do repositório nem do relatório.

## Método de reescrita e verificação

Preparação em clone bare isolado, sem hardlinks com o original. Transformação dos blobs e caminhos, com reconstrução específica para SQLite e mensagens netlink. Reescrita das referências locais por `git filter-branch` com filtros de índice, mensagem e identidade; o filtro de índice preserva os demais arquivos, modos e symlinks. `git-filter-repo` não estava instalado.

A busca dos literais originais, antes da transformação, detectou correspondências em 51 blobs, três caminhos e 175 commits alcançáveis pelas referências locais. Essa é a verificação negativa de controle. Depois da transformação, a mesma busca deve retornar zero em conteúdo, caminhos, mensagens e identidades. Não procura apenas nos arquivos da ponta da branch. Os blobs originais alterados também precisam deixar de ser alcançáveis.

As branches e tags de protótipo são sanitizadas localmente para evitar reintrodução acidental; continuam proibidas de publicação. No GitHub, somente `refs/heads/main` será atualizada, usando `--force-with-lease` com o SHA remoto inventariado. Nenhum `--mirror`, envio de tags ou alteração de visibilidade faz parte da operação.

## Inventário remoto observado

A consulta autenticada ao GitHub mostrou que o repositório **já estava público**, ao contrário da hipótese inicial de preparação para abertura. Isso não foi alterado por esta atividade.

- Uma referência publicada: `refs/heads/main`.
- Zero tags, releases, pull requests, issues abertas, forks e artefatos do Actions nas consultas realizadas.
- Discussões desabilitadas. Wiki habilitada na configuração; essa configuração não comprova a existência nem a revisão de páginas.
- Logs de execuções do Actions, caches do servidor e cópias externas não recebem garantia de expurgo nesta revisão.

## Validação das alterações

A verificação da árvore atual retornou zero ocorrências dos literais identificados. Os testes existentes foram mantidos nos seams previstos pela spec; as expectativas que identificavam trabalho privado agora usam exemplos.

A validação dos helpers em diretório temporário confirmou a sanitização dos endereços tanto na lista quanto na saída D-Bus textual, e a sanitização de quatro mensagens TCP reais de loopback, incluindo atributos de cgroup e consistência com o proprietário sintético. Nenhum arquivo real de configuração de agentes foi alterado.

Resultados: `npm run check` sem erros/avisos; `npm test` com 376 testes aprovados; suíte Rust com 305 testes unitários e três de integração aprovados; `cargo fmt --check` e `git diff --check` aprovados.

Revisão `code-review` nos dois eixos, sobre o diff da sanitização antes da reescrita: Standards sem violações documentadas, Spec sem achados. Uma observação adicional identificou que IPv4 link-local poderia receber alias IPv6 no helper; corrigido e validado com entrada IPv4 link-local.

Reescrita local concluída: 176 commits processados nas referências locais; 172 commits na `main`, preservando sua quantidade. Foram transformados 58 blobs originais em 28 caminhos históricos e renomeados três caminhos de rollout. A busca posterior nos 1.735 blobs únicos alcançáveis, caminhos e objetos de commit/tag retornou zero correspondências aos literais identificados. Nenhum dos 58 blobs originais alterados permaneceu alcançável. O tree da ponta da `main` foi preservado exatamente em relação ao commit de sanitização já testado.

Ajustes de documentação e da validação IPv4 link-local posteriores à reescrita não contêm valores de origem. A conferência do GitHub e a remoção dos objetos locais não referenciados serão registradas a seguir.

## Limites que permanecem

A limpeza cobre os dados identificados; não substitui a revisão semântica e visual histórica completa nem os documentos e capturas pendentes nos tickets 22/23/41/42.

Reescrever referências não apaga cópias feitas por terceiros. O GitHub também pode manter commits antigos acessíveis por SHA ou em caches; a remoção desses objetos do servidor pode exigir avaliação pelo [GitHub Support](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/removing-sensitive-data-from-a-repository). Não foi enviado relato a terceiros nesta atividade. Não reutilizar branches/clones antigos com merge ou push: eles podem reintroduzir o histórico removido.
