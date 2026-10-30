# Modo do tráfego por processo

Type: grilling
Status: resolved
Blocked by: 05
Map: [kvojps-monitor v1](../map.md)

## Question

O tráfego por processo da v1 usa só o modo sem privilégio (contadores TCP por conexão, só processos do usuário, sem UDP), ou também entrega o serviço eBPF opcional no `.deb` (exato, com `CAP_BPF`+`CAP_PERFMON` num usuário dedicado, sem equivalente no AppImage)? Como a instalação pede esse consentimento?

## Answer

- **Modo**: só o **sem privilégio** na v1 — contadores TCP por conexão do kernel (`sock_diag`/`tcp_info`), só processos do próprio usuário. Igual no `.deb` e no AppImage; o app segue um processo único sem privilégio. Serviço eBPF fica fora da v1 (ver Out of scope do mapa), então não há consentimento na instalação, `postinst` com capabilities nem usuário de sistema.
- **Lista**: top 5 consumidores **agrupados por nome do app** (vários processos do Firefox = uma linha), com **taxa atual ↓/↑** (média dos últimos ~5 s). Sem acumulado.
- **"Outros"**: linha final com o total da interface (`/proc/net/dev`) menos a soma atribuída; dica "UDP/QUIC, processos do sistema e conexões curtas".
- **Coleta**: por processo só enquanto o Módulo Rede está visível no Painel, a cada 2 s; para quando o Painel fecha ou troca de Módulo. Não notifica nem pesa no nível de Qualidade da rede. Tráfego total segue sempre ativo.
