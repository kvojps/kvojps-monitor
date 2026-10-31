# 01: Painel em estações com navegação e coleta por visibilidade

**What to build:** abrir pelo Indicador um Painel responsivo com os seis Módulos existentes na composição Estações de comando, mantendo navegação, contexto e coletas coerentes com o que está efetivamente visível.

**Blocked by:** None (can start immediately).

**Status:** ready-for-agent

- [ ] Aplicar a base estática da linguagem visual aprovada, com tokens e tipografia legíveis, sem importar fixtures ou ações simuladas do protótipo. Preservar os componentes funcionais existentes até suas respectivas fatias.
- [ ] Exibir Capacidade (IA/Sistema), Execução (Agentes/Trello) e Observação (GitHub/Rede), com três colunas iguais acima de 850 px, estações empilhadas com dois Módulos lado a lado entre 651–850 px e uma coluna até 650 px. Desfazer alinhamento de alturas ao empilhar e permitir alcançar todos os Módulos em 560 px.
- [ ] Abrir janela sem decoração em 95% da largura/altura da área útil respeitando escala e barras; preservar instância única, gestos do Indicador e posicionamento pelo compositor conforme ADR 0001. Registrar verificação nativa inicial e eventuais limitações.
- [ ] Navegação por nome rola e permite foco na seção; última âncora escolhida persiste entre execuções. Configurações é a única área separada, com “Voltar ao Painel” restaurando contexto. Escape e perda de foco ocultam em ambas as áreas.
- [ ] Cada Módulo identifica sua fonte e atualização; cache aparece prontamente sem bloqueio global. Atualizações preservam foco, seleção e rolagem.
- [ ] Migrar o contrato de visibilidade entre UI e núcleo para o conjunto de Módulos que intersectam a área efetivamente visível. Painel oculto ou Configurações reportam conjunto vazio; fora da rolagem não conta. Separar essa informação da última âncora escolhida.
- [ ] Testar pelo seam existente múltiplos visíveis, rolagem, fechamento/reabertura e Configurações, preservando cadências, pausa offline, retomada escalonada e evitando coletas duplicadas por eventos de interseção. Preservar encaminhamento de notificação a Agentes para a fatia 03.
- [ ] Testar navegação, restauração e comandos com fixtures da UI; manter ações acessíveis por teclado e evitar expansores, cortes de informação e redução de texto para forçar simetria.
- [ ] Medir memória em repouso nesta primeira composição; se superar aproximadamente 150 MB, aplicar a regra da v1 de destruir a webview ao fechar, preservando restauração. Registrar condições para revalidação final.
