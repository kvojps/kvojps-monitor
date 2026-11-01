# 05: GitHub recolhido quando conectado

**What to build:** a seção GitHub mostra uma única linha de estado: "Conectado via gh · login" ou "Token próprio · login". O formulário de token fica recolhido e abre por "Usar token próprio" (fonte `gh`) ou "Trocar token" (fonte token). Ele já vem aberto, junto da instrução para usar o `gh`, quando o estado é sem `gh` ou credencial inválida. Depois de salvar com sucesso, recolhe, e a linha de estado mostra o novo login. Sem comando novo de remover token. Ver `../spec.md`, histórias 22–25.

**Blocked by:** 01 (Peças compartilhadas e Rede como seção própria)

**Status:** ready-for-agent

- [ ] Via `gh`: só a linha de estado e "Usar token próprio"; sem campo de token visível
- [ ] Token próprio: estado exibido uma única vez e "Trocar token"
- [ ] Sem credencial: formulário aberto com a instrução
- [ ] Salvar valida antes de gravar; falha não salva e mostra o erro junto do campo; sucesso recolhe
- [ ] Testes de seção por fixture para os três estados e para o recolhimento após salvar
