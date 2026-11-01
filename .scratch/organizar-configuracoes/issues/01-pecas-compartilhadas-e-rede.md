# 01: Peças compartilhadas e Rede como seção própria

**What to build:** as seções das Configurações ganham peças comuns: linha de estado (ponto colorido por tom: ok, atenção, erro, neutro + texto), interruptor com erro associado e mensagem de erro logo abaixo do controle que falhou. Essas peças substituem o CSS global montado à mão no container. A seção Rede vira um componente próprio, sempre presente: o interruptor de teste de velocidade automático fica desabilitado até chegar o primeiro snapshot, em vez de sumir. Ver `../spec.md`, histórias 14–17 e 33.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] Peças de linha de estado, interruptor e erro disponíveis e usadas pela seção Rede
- [ ] Rede aparece sem snapshot, com o interruptor desabilitado; com snapshot, o interruptor reflete e altera o teste automático como hoje
- [ ] Nenhuma seção existente perde comportamento (os testes atuais continuam passando)
- [ ] Testes de UI por fixture para Rede com e sem snapshot
