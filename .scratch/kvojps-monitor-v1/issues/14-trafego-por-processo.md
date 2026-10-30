# Modo do tráfego por processo

Type: grilling
Status: open
Blocked by: 05
Map: [kvojps-monitor v1](../map.md)

## Question

O tráfego por processo da v1 usa só o modo sem privilégio (contadores TCP por conexão, só processos do usuário, sem UDP), ou também entrega o serviço eBPF opcional no `.deb` (exato, com `CAP_BPF`+`CAP_PERFMON` num usuário dedicado, sem equivalente no AppImage)? Como a instalação pede esse consentimento?
