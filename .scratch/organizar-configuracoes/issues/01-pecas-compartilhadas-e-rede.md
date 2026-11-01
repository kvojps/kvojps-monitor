# 01: Peças compartilhadas e Rede como seção própria

**What to build:** as seções das Configurações ganham peças comuns: linha de estado (ponto colorido por tom: ok, atenção, erro, neutro + texto), interruptor com erro associado e mensagem de erro logo abaixo do controle que falhou. Essas peças substituem o CSS global montado à mão no container. A seção Rede vira um componente próprio, sempre presente: o interruptor de teste de velocidade automático fica desabilitado até chegar o primeiro snapshot, em vez de sumir. Ver `../spec.md`, histórias 14–17 e 33.

**Blocked by:** None (can start immediately)

**Status:** done

- [x] Peças de linha de estado, interruptor e erro disponíveis e usadas pela seção Rede
- [x] Rede aparece sem snapshot, com o interruptor desabilitado; com snapshot, o interruptor reflete e altera o teste automático como hoje
- [x] Nenhuma seção existente perde comportamento (os testes atuais continuam passando)
- [x] Testes de UI por fixture para Rede com e sem snapshot

**Notas da implementação:**

- Geral e Aparência já passaram a usar `Switch` e `FieldError`, o que permitiu remover a regra global `label.switch` e `.error`. A regra global `label:has(> input[type="checkbox"])` e a cor de `[role="alert"]` ficam até IA, Agentes, GitHub e Trello migrarem (tickets 04–07).
- O interruptor da Rede continua sem caminho de erro: `setAutomaticSpeed` não devolve resultado e a spec não muda o núcleo. Ele mostra a escolha na hora, como antes, e o próximo snapshot confirma.
