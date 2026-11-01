# 03: Boas-vindas no lado direito e marcas "!"

**What to build:** na primeira abertura, as Boas-vindas ocupam o lado direito, com o índice visível. Escolher qualquer seção pelo índice, ou "Configurar X" nas Boas-vindas (que seleciona X), as encerra nesta abertura; a persistência de "já vistas" continua como hoje. O atalho "Conectar ao Claude Code" continua. No índice, cada Módulo que precisa configurar ganha uma marca discreta "!" com texto acessível ("precisa configurar"), usando a mesma regra de status das Boas-vindas. Detectado, não precisa e verificando não mostram marca. Ver `../spec.md`, histórias 7–12.

**Blocked by:** 02 (Índice + uma seção)

**Status:** done

- [x] Primeira abertura mostra as Boas-vindas no lado direito
- [x] Escolher uma seção ou "Configurar X" encerra as Boas-vindas e seleciona a seção certa
- [x] "Conectar ao Claude Code" continua funcionando a partir das Boas-vindas
- [x] Marca "!" só nos Módulos que precisam configurar, com uma única regra compartilhada com as Boas-vindas
- [x] Testes via Painel com fixtures (IA desconectada, GitHub sem `gh`, Trello vazio e escolhendo quadro)

**Notas da implementação:**

- Sem seção escolhida, o painel direito mostra só as Boas-vindas e o foco vai para o título delas; nenhum item do índice fica com `aria-current`.
- A marca "!" usa `moduleStatus` (`settings/status.ts`), a mesma regra das Boas-vindas. O texto acessível "precisa configurar" entra como descrição (`aria-describedby`), mantendo o nome do item igual ao da seção.
