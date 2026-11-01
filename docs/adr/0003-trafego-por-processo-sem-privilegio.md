# Tráfego por processo sem privilégio

O Módulo Rede mostra os aplicativos que mais consomem. Atribuição exata por processo (incluindo UDP/QUIC e processos do sistema) exige eBPF ou captura de pacotes, ou seja, um serviço privilegiado. Decidimos usar só o modo **sem privilégio**: contadores TCP por conexão do kernel (`sock_diag`/`tcp_info`) dos processos do próprio usuário, top 5 agrupado por nome do app com taxa ↓/↑, e uma linha **"Outros"** = total da interface (`/proc/net/dev`) − atribuído, com a dica "UDP/QUIC, processos do sistema e conexões curtas". Coleta a cada 2 s só com o Módulo Rede visível. Motivo: um app pessoal de bandeja não deve instalar componente com capabilities, usuário de sistema e `postinst` só para ganhar precisão; o top consumidores já responde à pergunta, e o comportamento fica igual no `.deb` e no AppImage.

## Considered Options

- **Serviço eBPF opcional só no `.deb`**: preciso, mas exige capabilities, usuário dedicado, consentimento na instalação e não existe no AppImage.
- **`nethogs`/libpcap**: precisa de `CAP_NET_RAW`/root, mesmo problema.
