# Mapa: Painel cinematográfico para trabalho

Label: wayfinder:map
Status: resolved

## Destination

Uma especificação visual do Painel inteiro, validada pelo usuário por meio de protótipo, pronta para implementação: estética cinematográfica futurista de intensidade máxima, preservando as funções e a utilidade do app atual.

## Notes

- Planejamento visual; a implementação no app fica para depois da validação da especificação.
- Skills: `grilling` e `domain-modeling` nas decisões; `prototype` na exploração visual.
- Vocabulário: [GLOSSARY.md](../../GLOSSARY.md). Trata-se do Painel existente, não de criar um assistente conversacional.
- Acordos de abertura confirmados pelo usuário: preto profundo, ciano e azul elétrico; geometria técnica, anéis de telemetria e movimento contínuo enquanto aberto; opção de reduzir animações. Pode ampliar o Painel e reorganizar os Módulos, mantendo acesso pelo Indicador.
- Restrição central: continua sendo ferramenta de trabalho. Preservar funções, dados reais, ações acessíveis, legibilidade, identificação de alertas e fluidez. Instrumentos reais distinguíveis de decoração; sem números inventados apresentados como medições.
- Referência de comportamento: [spec da v1](../kvojps-monitor/spec.md) e implementação atual em `src/lib/Panel.svelte`. IA, Agentes, GitHub, Trello, Rede, Sistema e Configurações permanecem no escopo.
- A direção visual reabre [Layout e visual do Painel](../kvojps-monitor-v1/issues/10-visual-do-painel.md), conforme autorização registrada ali. As substituições visuais aprovadas estão delimitadas no complemento visual; a spec da v1 permanece referência funcional.
- Tracker local: tickets irmãos em `issues/`, dependências em `Blocked by`, resolução em `Answer`. Tickets sem Status estão abertos e não reivindicados.
- Branches de pesquisa e protótipo permanecem locais, conforme AGENTS.md.
- Consolidação aprovada: [Especificação visual — Painel cinematográfico](spec.md), vinculada a [Consolidar a especificação visual validada do Painel](issues/03-visual-specification.md). O documento é o complemento visual aprovado da spec da v1.
- A exploração adicional de movimento foi aprovada em [Intensificar o movimento visual do Painel](issues/04-more-panel-motion.md); a consolidação final incorpora a Coreografia combinada.

## Decisions so far

- [Definir a composição e a navegação do Painel cinematográfico](issues/01-panel-composition.md): Estações de comando aprovadas, três colunas simétricas, Painel completo sem expansores e apenas Configurações separada.

- [Validar linguagem visual, movimento e leitura no trabalho](issues/02-motion-and-readability.md): movimento apenas decorativo, redução e pausa quando oculto, temas escuro/claro/sistema e leitura dos estados aprovados.

- [Intensificar o movimento visual do Painel](issues/04-more-panel-motion.md): Coreografia combinada aprovada, com fluxos nas faixas, varreduras nas bordas, deriva nos títulos e resposta da navegação ao apontador; restrições e orçamento de desempenho preservados.

- [Consolidar a especificação visual validada do Painel](issues/03-visual-specification.md): complemento visual aprovado integralmente, com memória da última âncora e critérios de acessibilidade/desempenho; planejamento concluído.

## Not yet specified

- Nenhuma. Destino alcançado; todos os tickets resolvidos.

## Out of scope

- Implementar o redesign no app nesta etapa de planejamento.
- Adicionar conversa, voz, novas integrações ou novas capacidades de assistente.
- Alterar regras de negócio, fontes de dados, credenciais, privacidade ou coleta para sustentar efeitos decorativos.
