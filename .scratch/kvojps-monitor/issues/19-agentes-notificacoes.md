# 19: Notificações de Agentes

**What to build:** recebo uma notificação nativa quando uma sessão entra em Aguardando você ("Aguardando aprovação: <ferramenta>" ou "Aguardando sua resposta"), quando um turno termina Com falha, e quando um Turno concluído durou ≥ 30 s ("Turno concluído em 4 min 12 s"). Interrupção e Encerrada nunca notificam. No máximo uma notificação viva por sessão; 3+ eventos em 10 s viram uma agrupada ("3 sessões aguardando você"). Clicar abre o Painel no Módulo Agentes com a sessão destacada. Três interruptores nas Configurações → Agentes, todos ligados por padrão. Ver spec, "Módulo Agentes" e ticket 09 do mapa.

**Blocked by:** 05 (Segurança da rede, notificação e badge de Risco), 17 (Sessões do Claude Code por polling)

**Status:** done

- [x] Título `<Claude Code | Codex> · <projeto>` [`· <nome>`]; corpo sem nenhum trecho de prompt ou resposta (teste explícito)
- [x] Turno concluído < 30 s não notifica (teste explícito); interrupção e Encerrada nunca notificam
- [x] Nova notificação da mesma sessão substitui a anterior
- [x] ≥ 3 eventos em 10 s → uma notificação agrupada (teste explícito com relógio falso)
- [x] Clique abre o Painel no Módulo Agentes com a sessão destacada (exceção ao "último Módulo visitado")
- [x] Três interruptores (Aguardando, Turno concluído, Falha), sem separação por provedor; Não perturbe do GNOME sem lógica própria


## Implementação

Política de notificações integrada ao polling de Claude Code e Codex, com relógio e
notificador injetáveis. Notificações nativas via D-Bus substituem o ID anterior da
sessão; três sessões com eventos em até 10 s fecham as individuais e atualizam uma
agrupada. O clique abre Agentes e destaca a sessão, inclusive com o Painel já aberto;
a abertura comum continua lembrando o último Módulo.

Os três interruptores são compartilhados pelos provedores, ligados por padrão e
persistidos pela porta de armazenamento. Interrupções ficam silenciosas. A duração
observada do turno Claude preserva pausas para aprovação; o Codex usa `duration_ms`
do rollout quando disponível. Conclusões e falhas já presentes na primeira coleta
não geram notificações retroativas. O corpo usa apenas textos fixos, duração e nomes
conhecidos de ferramentas; detalhes livres de espera não são copiados.

Revisão em dois eixos concluída, com os achados corrigidos e confirmados pelos
revisores. Testes cobrem privacidade, duração mínima, interrupção, substituição,
agrupamento, preferências e navegação/destaque. A entrega e o clique no GNOME,
incluindo Não perturbe, ainda precisam de verificação manual no ambiente gráfico,
conforme a spec (sem E2E da janela/Indicador).
