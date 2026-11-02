# 05: GitHub recolhido quando conectado

**What to build:** a seção GitHub mostra uma única linha de estado: "Conectado via gh · login" ou "Token próprio · login". O formulário de token fica recolhido e abre por "Usar token próprio" (fonte `gh`) ou "Trocar token" (fonte token). Ele já vem aberto, junto da instrução para usar o `gh`, quando o estado é sem `gh` ou credencial inválida. Depois de salvar com sucesso, recolhe, e a linha de estado mostra o novo login. Sem comando novo de remover token. Ver `../spec.md`, histórias 22–25.

**Blocked by:** 01 (Peças compartilhadas e Rede como seção própria)

**Status:** done

- [x] Via `gh`: só a linha de estado e "Usar token próprio"; sem campo de token visível
- [x] Token próprio: estado exibido uma única vez e "Trocar token"
- [x] Sem credencial: formulário aberto com a instrução
- [x] Salvar valida antes de gravar; falha não salva e mostra o erro junto do campo; sucesso recolhe
- [x] Testes de seção por fixture para os três estados e para o recolhimento após salvar

**Notas da implementação:**
- Sem snapshot, a linha mostra "Verificando…" e o formulário fica recolhido.
- Sem credencial no snapshot, a linha mostra "Não conectado" (tom de atenção); com credencial inválida, "Credencial do GitHub inválida" (tom de atenção, como no Painel). Nos dois, o formulário vem aberto com a instrução do `gh` e sem botão para abri-lo.
- Depois de salvar, a seção mostra "Token próprio · login" com o login devolvido pela validação até chegar o próximo snapshot, que passa a valer.
- A mensagem "Conectado como login" e o texto "Rodar `gh auth login`" saíram: o estado aparece só na linha de estado e a instrução já explica o `gh`.
