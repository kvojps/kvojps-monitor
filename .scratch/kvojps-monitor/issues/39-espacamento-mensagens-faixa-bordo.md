# 39: Exibir mensagens consecutivas na Faixa de bordo

**What to build:** corrigir a distribuição horizontal das mensagens da Faixa de bordo para que apareçam consecutivamente, a partir da esquerda, com espaçamento fixo entre elas e sem grandes espaços vazios.

**Blocked by:** nenhum.

**Status:** ready-for-agent

## Problema observado

Com mais de uma mensagem, a primeira aparece no começo da faixa e outra pode começar depois do meio, mesmo havendo espaço vazio entre elas. Em `src/lib/OperationsStrip.svelte`, `.summaries span` usa `flex: 1`, distribuindo os resumos em colunas de igual largura; conferir essa distribuição ao reproduzir o defeito.

## Critérios de aceite

- [ ] Exibir dois ou mais resumos lado a lado, ocupando somente a largura necessária e separados por um espaçamento fixo consistente; não distribuir espaço restante entre mensagens.
- [ ] Preservar agrupamento por Módulo, ordenação por prioridade e permanência dos eventos conforme a spec.
- [ ] Respeitar o espaço disponível e preservar o resumo de excedentes, como “+3 Módulos”, sem expandir a grade, sobrepor mensagens ou provocar transbordamento horizontal.
- [ ] Verificar uma, duas e várias mensagens, textos de comprimentos diferentes, entrada e saída de resumos, largura reduzida, texto ampliado e temas claro e escuro.
- [ ] Validar com fixtures de snapshots/transições da UI e inspeção visual; registrar evidência de mensagens curtas consecutivas e de excedentes. Manter anúncios acessíveis e conteúdo limitado aos metadados operacionais já permitidos.
- [ ] Preservar o movimento decorativo Órbita contínua e a redução de movimento existentes, mantendo os textos estáticos e sem carrossel, conforme a spec.

## Referências

- [Spec — Faixa de bordo e movimento](../spec.md#faixa-de-bordo).
- [Faixa de bordo do GitHub](27-faixa-bordo-github.md) e [Órbita contínua](32-orbita-continua.md).
- Implementação: `src/lib/OperationsStrip.svelte`.

## Comments

### 02/11/2026 — escopo aprovado

O usuário confirmou mensagens consecutivas com espaçamento fixo, eliminando grandes espaços vazios e preservando a ordem atual. A preservação do movimento refere-se à decoração existente; a spec mantém os textos estáticos, sem carrossel. Este ticket trata da distribuição visual das mensagens.
