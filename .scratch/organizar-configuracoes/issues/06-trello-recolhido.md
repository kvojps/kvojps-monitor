# 06: Trello recolhido quando conectado

**What to build:** a seção Trello mostra:
- **Conectado:** só "Conta X · quadro Y", com "Trocar quadro" (abre a lista de quadros) e "Desconectar".
- **Sem credencial:** o passo a passo numerado (1. chave, 2. token).
- **Depois de validar, ou quando falta escolher o quadro:** a lista de quadros já aberta.

Escolher um quadro recolhe a lista. Continuam os rádios em vez de `<select>` e a chave preservada quando o Painel fecha durante a geração do token. Ver `../spec.md`, histórias 26–30.

**Blocked by:** 01 (Peças compartilhadas e Rede como seção própria)

**Status:** done

- [x] Conectado: resumo + Trocar quadro + Desconectar, sem passo a passo
- [x] Sem credencial: passo a passo numerado
- [x] Escolher quadro (estado ou após validar): lista aberta; escolher recolhe
- [x] Chave pendente preservada; erros junto do controle
- [x] Testes de seção por fixture (vazio, escolhendo quadro, completo, Trocar quadro)

**Notas da implementação:**
- Linha de estado: "Conta X · quadro Y" (conectado), "Conta X · escolha um quadro" (falta quadro), "Não conectado" (sem credencial), "Credencial inválida" (passo a passo aberto, com "Desconectar") e "Verificando…" sem snapshot.
- "Trocar quadro" alterna a lista (`aria-expanded`); escolher um quadro recolhe e mostra o novo quadro até chegar o próximo snapshot. Depois de validar, a lista abre mesmo se a conta mantiver o quadro anterior.
- Ações ("Trocar quadro", "Desconectar") logo abaixo da linha de estado, antes do formulário ou da lista. "Trocar quadro" some quando não há quadros listados; o estado `error` usa tom de atenção.
- Depois de validar, a lista espera o próximo snapshot (mostra "Verificando…") em vez de exibir os quadros da credencial anterior. Se escolher um quadro falha, a marca volta para o quadro atual.
- A chave pendente é restaurada do primeiro snapshot, mesmo se a seção abrir antes dele.
- Erros ficam junto do controle: credencial sob o campo de token, quadro sob a lista, desconexão sob o botão.
- O passo do quadro perdeu o número ("Escolha o quadro exibido no Painel."), já que aparece sozinho; a mensagem "Conectado como X" saiu, o estado fica só na linha de estado.
