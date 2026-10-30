# kvojps-monitor

Aplicativo pessoal de bandeja que reúne num só lugar, de forma enxuta, dados que hoje exigem abrir vários apps e abas: uso de IA, sessões de agentes, atividade no GitHub, quadro do Trello e saúde da rede.

## Language

**Indicador**:
O ícone do app na barra superior do sistema; ponto de entrada sempre visível.
_Avoid_: Doca, dock, ícone de bandeja

**Painel**:
A janela compacta que abre a partir do Indicador e exibe os Módulos.
_Avoid_: Doca, popup, dashboard

**Módulo**:
Uma área do Painel dedicada a uma fonte de dados (IA, Agentes, GitHub, Trello, Rede).
_Avoid_: Widget, aba, card

**Configurações**:
A área do Painel onde o usuário informa credenciais e preferências de cada Módulo.
_Avoid_: Settings, preferências

**Provedor de IA**:
Um serviço de IA pago por assinatura cujo uso é acompanhado (ex.: Claude, ChatGPT/Codex).
_Avoid_: Modelo, IA paga

**Limite de uso**:
A cota de um Provedor de IA dentro de uma janela de tempo (ex.: janela de sessão, semanal), com quanto já foi consumido e quando reinicia.
_Avoid_: Limite de tokens, quota

**Sessão de agente**:
Uma execução local de um CLI de agente (Claude Code, Codex) nesta máquina, que está em andamento ou finalizada.
_Avoid_: Sessão, agente, run

**Segurança da rede**:
A avaliação de riscos da conexão atual (criptografia, DNS, portal cativo, gateway), expressa como Sem alertas, Atenção ou Risco; nunca afirma que a rede é segura.
_Avoid_: Saúde da rede, rede segura

**Qualidade da rede**:
O desempenho da conexão atual (latência, jitter, perda, velocidade), expresso como Boa, Regular ou Ruim junto dos números medidos.
_Avoid_: Saúde da rede, status da rede
