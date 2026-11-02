# Sanitização antes de versionar

Estas regras valem para qualquer material que possa entrar no Git: código e testes, fixtures, pesquisas, tickets, validações em `.scratch/`, imagens, scripts e skills. Valem desde o primeiro commit local, mesmo quando não há intenção imediata de push. Um arquivo ignorado pelo Git ainda pode aparecer em saídas de ferramentas ou capturas de tela.

## Ao coletar evidências

1. Selecione somente os campos necessários ao comportamento investigado. Credenciais ficam fora da captura; aplique a projeção e a sanitização em memória, antes de escrever ou imprimir. Para revisar material bruto sem credenciais, use somente um diretório temporário privado fora do repositório e remova-o ao concluir.
2. Substitua dados identificáveis de forma consistente usando a tabela abaixo. Preserve estrutura, tipos e relações entre registros. Atualize também referências em nomes de arquivo, testes, snapshots e pesquisas. Se a substituição mudar a semântica do cenário, ajuste os exemplos e confira o comportamento nos seams existentes.
3. Registre junto da fixture sua origem, os campos sanitizados e quais variantes são sintéticas. Captura real sanitizada continua sendo derivada de uma leitura real; uma variante inventada deve ser identificada como tal. Relatórios registram caminhos, categorias e resultados, sem os valores privados de origem ou mapas de substituição.
4. Nos helpers de captura, sanitize antes da gravação. Confira a saída completa, inclusive cópias textuais da resposta bruta, mensagens de erro e arquivos auxiliares; a sanitização não pode depender de uma edição manual posterior.

| Categoria | Tratamento para material versionado |
| --- | --- |
| Tokens, senhas, chaves privadas, cookies e cabeçalhos de autenticação | Excluir da captura. Quando o parser exigir o campo, usar um valor explicitamente fictício. |
| Prompts, respostas, comandos privados, títulos de commits e conteúdo de cards | Omitir ou substituir por exemplos sintéticos mínimos. Sentinelas de privacidade em testes são sintéticas e identificadas como tal. |
| Caminhos pessoais, nomes de projetos privados, sessões e contas; e-mails e IDs de conta/sessão | Usar aliases estáveis de exemplo, inclusive nos nomes de arquivos, URLs e campos derivados. Preservar somente identidades públicas necessárias e explicitamente autorizadas. |
| IPv4 de host/gateway ou serviço privado | Usar faixas de documentação, como `192.0.2.0/24`, preservando distinções entre hosts. Endereços privados reais também precisam ser substituídos. |
| IPv6 global e link-local | Usar `2001:db8::/32` para globais e aliases como `fe80::1` para link-local. Preservar família, escopo e distinções entre hosts necessários ao teste. |
| MAC, SSID, perfil de rede, hostname, UUID e identificadores de dispositivo | Omitir quando dispensáveis; nos demais casos usar exemplos sintéticos consistentes. Para MAC, usar endereços unicast administrados localmente, como `02:00:00:00:00:01`. |
| Montagens, processos, sockets e armazenamento | Substituir caminhos e identificadores do host; preservar apenas a topologia e os contadores necessários ao cenário. |
| Identidade dos commits | Conferir autor e committer antes de criar commits; usar o endereço noreply do mantenedor ou outra identidade explicitamente autorizada. Configurar noreply depois não corrige commits anteriores. |

Destinos públicos documentados do produto e loopback podem ser preservados quando necessários ao teste. Métricas e horários reais só permanecem quando necessários, sem associação a identificadores privados; se o próprio valor revelar atividade privada, use uma variante declaradamente sintética.

## Antes do commit e do push

1. Confira a lista completa de arquivos selecionados e o diff do índice, incluindo arquivos novos e renomeados. Procure credenciais e as categorias acima; um scanner de segredos sozinho não detecta todos os dados pessoais. Revise metadados de imagens e outros binários, além do texto visível.
2. Abra e inspecione visualmente cada imagem nova ou alterada. Para SQLite, publique um banco reconstruído a partir de schema e registros sanitizados; editar linhas não garante remover páginas antigas. Para capturas binárias, confira os campos pelo formato do protocolo. Material sem revisão suficiente fica fora do commit até essa revisão.
3. Execute os testes existentes afetados pelas substituições. Registre os caminhos revisados, método, resultados e limitações na validação do ticket, sem reproduzir dados de origem. Não marque uma revisão pendente como concluída.
4. Antes do push, confira as referências de destino e todos os commits que serão enviados, incluindo mensagens e identidades de autor/committer. A árvore atual limpa não comprova que o intervalo de commits está limpo. Preserve a restrição de branches locais descrita no `AGENTS.md`.

## Se encontrar exposição

Interrompa a publicação do material afetado e registre somente caminhos e categorias. Distinga credenciais utilizáveis, conteúdo privado e metadados pessoais; descreva o risco com base no que foi verificado, sem presumir invasão nem ausência de outros dados.

Sanitize a árvore e identifique em quais commits e referências o dado entrou. Reescrita de histórico, force-push, exclusão de repositório e revogação de credenciais dependem de autorização explícita para a operação. Uma correção na ponta da branch não remove versões anteriores.

Diferencie a limpeza de referências da remoção de objetos em caches ou cópias externas. Acionar suporte é uma decisão proporcional aos dados encontrados e ao objetivo de expurgo; não é automaticamente necessário para todo metadado. Registre separadamente o que foi confirmado e o que depende do provedor ou de terceiros.

Estas instruções orientam a revisão; não constituem bloqueio automatizado de commit ou garantia de detecção completa.
