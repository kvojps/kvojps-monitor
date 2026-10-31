# 05: Trello em quadro interativo

**What to build:** consultar e mover cards no quadro integrado à estação, preservando acesso às listas reais e recuperação de falhas.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** ready-for-agent

- [ ] Mostrar colunas e cards com título, etiquetas e vencimento conforme a composição aprovada; listas extensas têm rolagem, sem esconder cards para obter simetria.
- [ ] Selecionar card/lista de destino e mover com resultado otimista; falha reverte o movimento e informa o ocorrido. Duplo clique abre no navegador, com alternativa acessível por teclado.
- [ ] Atualizar/configurar permanecem acessíveis; respeitar intervalo manual mínimo de 10 s e preservar seleção/foco durante atualização.
- [ ] Cobrir cheio, listas vazias, sem quadro, sem credencial, credencial inválida, antigo, offline e falha com cache utilizável.
- [ ] Testar fixtures, comandos de abertura/movimento e reversão, incluindo listas densas sem sobreposição de ações ou corte de informação indispensável.
- [ ] Preservar coleta/armazenamento; testar que credenciais e conteúdo de cards ficam fora de logs/cache/notificações.
