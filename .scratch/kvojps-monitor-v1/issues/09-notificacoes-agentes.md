# Quais eventos de Sessão de agente geram notificação

Type: grilling
Status: resolved
Blocked by: 02
Map: [kvojps-monitor v1](../map.md)

## Question

Quais eventos de Sessão de agente disparam notificação nativa (finalizada, aguardando resposta, erro), com que conteúdo, e como evitar ruído (sessões curtas, várias sessões simultâneas)?

## Answer

Vocabulário: os **Estados da sessão** são Em andamento, Aguardando você, Turno concluído, Com falha e Encerrada (no [GLOSSARY](../../../GLOSSARY.md)). "Finalizada" sai do vocabulário.

- **O que notifica**: **Aguardando você** e **Com falha** sempre. **Turno concluído** só se o turno durou **≥ 30 s** (limiar fixo na v1). **Encerrada** nunca notifica, e interrupção pelo usuário (Esc/Ctrl‑C) conta como Turno concluído sem notificação.
- **Conteúdo**: título `<Claude Code | Codex> · <projeto>` (pasta do `cwd`), mais `· <nome da sessão>` quando houver. Corpo: "Aguardando aprovação: <ferramenta>", "Aguardando sua resposta", "Turno concluído em 4 min 12 s" ou "Turno terminou com erro". Nunca inclui trecho de prompt nem de resposta.
- **Anti-ruído**: uma notificação viva por sessão (um evento novo substitui o anterior). Se 3 ou mais sessões dispararem em 10 s, viram uma notificação agrupada ("3 sessões aguardando você"). O Não perturbe do GNOME silencia tudo sem lógica própria.
- **Clique**: abre o Painel no Módulo Agentes com a sessão destacada (focar o terminal não é confiável no Wayland).
- **Indicador**: ganha um ponto de acento enquanto alguma sessão estiver Aguardando você. Turno concluído e Com falha não marcam.
- **Detecção**: os hooks são opt-in nas Configurações ("Detecção instantânea"), instalados de forma aditiva sem tocar nos hooks existentes (ex.: Orca), com o passo de confiança em `/hooks` mostrado para o Codex. Sem hooks: Claude via polling de `claude agents --json` a cada ~2 s (cobre todos os estados); Codex só detecta Turno concluído e Com falha pelo rollout, e o Módulo Agentes indica que "Aguardando" do Codex está indisponível.
- **Configurações**: três interruptores, um por tipo (Aguardando / Turno concluído / Falha), todos ligados por padrão, sem separação por provedor.
