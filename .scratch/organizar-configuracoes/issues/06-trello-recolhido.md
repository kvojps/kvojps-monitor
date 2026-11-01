# 06: Trello recolhido quando conectado

**What to build:** a seção Trello mostra:
- **Conectado:** só "Conta X · quadro Y", com "Trocar quadro" (abre a lista de quadros) e "Desconectar".
- **Sem credencial:** o passo a passo numerado (1. chave, 2. token).
- **Depois de validar, ou quando falta escolher o quadro:** a lista de quadros já aberta.

Escolher um quadro recolhe a lista. Continuam os rádios em vez de `<select>` e a chave preservada quando o Painel fecha durante a geração do token. Ver `../spec.md`, histórias 26–30.

**Blocked by:** 01 (Peças compartilhadas e Rede como seção própria)

**Status:** ready-for-agent

- [ ] Conectado: resumo + Trocar quadro + Desconectar, sem passo a passo
- [ ] Sem credencial: passo a passo numerado
- [ ] Escolher quadro (estado ou após validar): lista aberta; escolher recolhe
- [ ] Chave pendente preservada; erros junto do controle
- [ ] Testes de seção por fixture (vazio, escolhendo quadro, completo, Trocar quadro)
