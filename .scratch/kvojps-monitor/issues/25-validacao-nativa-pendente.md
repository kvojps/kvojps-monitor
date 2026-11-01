# 25: Validação nativa pendente do Painel

**What to build:** executar no app nativo (GNOME/Wayland, WebKitGTK) as verificações que ficaram sem evidência quando o Painel compacto foi aprovado visualmente, e corrigir os desvios comprovados. Ver spec, "Visual do Painel", e [`docs/desempenho.md`](../../../docs/desempenho.md).

**Blocked by:** None (can start immediately)

**Status:** ready-for-human

- [ ] Janela real: dimensões lógicas/escala, monitor, área útil, detalhes contidos na janela; abertura pelo Indicador (menu, duplo clique, clique do meio); ocultação por Esc e perda de foco; segunda instância abrindo o Painel existente; troca de tema pelo portal; texto ampliado do GNOME
- [ ] Trello real: mover card, reversão na falha, resumo sincronizado, seleção preservada, arraste no WebKitGTK, alternativa por teclado, listas fora da área visível, Esc com o seletor nativo aberto, abertura externa
- [ ] Notificação real de Agentes abrindo o detalhe com a sessão direcionada, incluindo alvo encerrado retido e alvo indisponível
- [ ] Resposta ≤ 100 ms e 60 fps com traces de quadros, incluindo listas densas e ações
- [ ] A/B de CPU da decoração (três pares de 60 s, ≤ 2 p.p. de um núcleo)
- [ ] Painel oculto sem trabalho de animação; redução manual e do sistema; PSS aberto/fechado e reabertura

## Comments

Origem: pendências registradas na validação integrada do Painel compacto e na medição de desempenho do Painel cinematográfico (2026-10-31). Os dois tickets foram encerrados por decisão do usuário sem essas evidências; os mapas foram apagados em 2026-11-01 e as pendências consolidadas aqui. Validação de navegador já feita: sem rolagem em 1100 × 700 nos três temas e escalas 1,25/1,5/2, 130 estados, teclado 18/18, contraste AA auditado; nada disso comprova comportamento Tauri.
