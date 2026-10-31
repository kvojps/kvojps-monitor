# 06: Rede com instrumentos e tráfego real

**What to build:** interpretar Qualidade e Segurança da rede, latência, velocidade e consumo por aplicativo na revisão visual aprovada, acionando testes reais quando permitido.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** resolved

- [x] Separar Qualidade e Segurança com nomes canônicos, conexão cabeada/sem fio, VPN, desconexão e motivos de Atenção/Risco; nunca afirmar “Rede segura”.
- [x] Destacar latência de Internet e instrumento menor acionável de download/upload do último teste em Mbps. Mostrar jitter/perda de Internet e latência do gateway em indicadores menores; jitter/perda do gateway em faixa secundária, sem tabela ou cabeçalhos repetidos.
- [x] Mostrar gráfico com duas séries distintas, rótulos, resumo textual e taxas atuais; escala deriva das amostras disponíveis, sem inventar tendência, preencher lacunas ou animar números.
- [x] Exibir até cinco aplicativos mais Outros em barras pareadas de download/upload, com nomes e valores em escala comum local; explicar em Outros UDP/QUIC, processos do sistema e conexões curtas. As barras não representam capacidade do link.
- [x] Diferenciar Mbps de capacidade medida e KB/s ou MB/s de tráfego. Idade/data do teste fica acessível por foco e ponteiro e aparece diretamente quando antigo/indisponível.
- [x] Estados indisponível/em andamento/falha e desabilitação do teste têm rótulo/ícone; respeitar limites e regras da v1. Sem medição significa ausência explícita, nunca 0.
- [x] Testar snapshots coerentes de cheio, vazio, antigo, erro, VPN, desconexão, alertas, teste em andamento/falha e cinco apps, além de comandos/teclado. Preservar coletas e regras de notificação de Risco, sem novos dados para decoração.

## Registro da implementação (2026-10-31)

- **Módulo Rede no Painel:** `NetworkModule` volta à estação Observação seguindo a revisão `NetworkCompact` aprovada. Antes do primeiro snapshot: “Sem leitura da rede ainda”.
- **Qualidade e Segurança:** dois blocos separados com os nomes canônicos (“Qualidade da rede”, “Segurança da rede”), ícone e texto por estado (✓/◐/✕, ✓/⚠/✕, ⊘ sem conexão, … aguardando), nunca “segura” (há teste). Conexão “Sem fio”/“Cabeada” e “VPN ativa”/“VPN inativa”; motivos de Atenção/Risco listados com ⚠. Os blocos são `group`, não `status`, para não anunciar cada ping.
- **Instrumentos:** latência da internet em destaque (46 px). Ao lado, o botão de teste com ↓/↑ em Mbps (“Mbps · capacidade”), cujo nome acessível é “Testar velocidade” / “Testando velocidade…”. Jitter, perda e latência do gateway ficam em indicadores pequenos; jitter e perda do gateway numa faixa “Gateway · jitter … · perda …”.
- **Idade do teste:** “Capacidade medida há … · dd/mm, hh:mm” como descrição do botão (`aria-describedby`), visível no hover e no foco. Aparece direto quando o teste é antigo (≥ 6 h, “Teste antigo · …”, em tom de atenção), quando não pode ser repetido (sem internet ou conexão limitada), quando está em andamento ou sem conexão.
- **Gráfico:** duas séries, com descida contínua e subida tracejada, e legenda com amostra do traço ao lado de “↓ Descida”/“↑ Subida” e das taxas atuais em KB/s ou MB/s. A escala vem do maior valor das amostras. Cada amostra ocupa um espaço fixo de 1 s ancorado à direita, sem esticar a série nem preencher lacunas; com uma amostra só, desenha marcas em vez de linha. O resumo textual (amostras, atual e pico por série) fica no nome acessível.
- **Aplicativos:** até cinco mais “Outros” em barras pareadas ↓/↑ numa escala comum local (maior valor entre as linhas). A legenda diz que as barras não indicam a capacidade do link, e a explicação de Outros fica visível (também no `title`).
- **Ausência explícita:** valor sem medição aparece como “sem medição”, teste ausente como “Sem teste de velocidade”, sem amostras como “Aguardando amostras de tráfego”. Sem conexão, a latência e os aplicativos dizem “Sem conexão”.
- **Fixture `full`:** gerada pelo núcleo em `full_ui_fixture_is_a_snapshot_published_by_core`, com teste medido, cinco amostras de tráfego e seis apps (o corte de cinco do núcleo se aplica). Estados de teste antigo e em andamento saem das fixtures existentes, com relógio controlado.
- **Preservado:** a coleta, as regras de Qualidade/Segurança e a notificação de Risco não mudaram, e não há dado novo. O botão segue desabilitado sem internet ou em conexão limitada e durante o teste. Rede continua sem ↻.
- **Limitação:** o “antigo” da UI considera só a idade. A regra “na rede atual” continua a cargo do núcleo, que é quem decide se o teste pesa na Qualidade, e a UI não conhece o id da rede atual.
- **Pendente (usuário):** conferência visual nativa nas três larguras e nos temas escuro/claro, e conferência de contraste das cores das séries.

## Comments

**2026-10-31 — resolvido a pedido do usuário.** Qualidade/Segurança, latência, teste de velocidade, gráfico de tráfego e aplicativos entregues em `2bd7b79`. A conferência visual nativa listada em “Pendente (usuário)” segue para o ticket 10, junto com a validação de composição e acessibilidade no GNOME/Wayland.
