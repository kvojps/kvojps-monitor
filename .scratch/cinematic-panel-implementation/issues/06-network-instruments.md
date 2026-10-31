# 06: Rede com instrumentos e tráfego real

**What to build:** interpretar Qualidade e Segurança da rede, latência, velocidade e consumo por aplicativo na revisão visual aprovada, acionando testes reais quando permitido.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** ready-for-agent

- [ ] Separar Qualidade e Segurança com nomes canônicos, conexão cabeada/sem fio, VPN, desconexão e motivos de Atenção/Risco; nunca afirmar “Rede segura”.
- [ ] Destacar latência de Internet e instrumento menor acionável de download/upload do último teste em Mbps. Mostrar jitter/perda de Internet e latência do gateway em indicadores menores; jitter/perda do gateway em faixa secundária, sem tabela ou cabeçalhos repetidos.
- [ ] Mostrar gráfico com duas séries distintas, rótulos, resumo textual e taxas atuais; escala deriva das amostras disponíveis, sem inventar tendência, preencher lacunas ou animar números.
- [ ] Exibir até cinco aplicativos mais Outros em barras pareadas de download/upload, com nomes e valores em escala comum local; explicar em Outros UDP/QUIC, processos do sistema e conexões curtas. As barras não representam capacidade do link.
- [ ] Diferenciar Mbps de capacidade medida e KB/s ou MB/s de tráfego. Idade/data do teste fica acessível por foco e ponteiro e aparece diretamente quando antigo/indisponível.
- [ ] Estados indisponível/em andamento/falha e desabilitação do teste têm rótulo/ícone; respeitar limites e regras da v1. Sem medição significa ausência explícita, nunca 0.
- [ ] Testar snapshots coerentes de cheio, vazio, antigo, erro, VPN, desconexão, alertas, teste em andamento/falha e cinco apps, além de comandos/teclado. Preservar coletas e regras de notificação de Risco, sem novos dados para decoração.
