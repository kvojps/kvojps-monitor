# Rede no Linux: segurança, latência, velocidade, tráfego e permissões

Type: research
Status: resolved
Blocked by: 
Map: [kvojps-monitor v1](../map.md)

## Question

No Ubuntu 26.04/GNOME, como obter sem custo: (a) segurança do Wi‑Fi conectado (aberta/WEP/WPA2/WPA3) via NetworkManager; (b) latência e perda até gateway e internet; (c) checagem de DNS e portal cativo; (d) teste de velocidade gratuito (Cloudflare speed, LibreSpeed, M-Lab/NDT, Ookla CLI — licenças e termos para uso automatizado); (e) tráfego total em tempo real; (f) tráfego por processo (eBPF, nethogs, /proc, libpcap) e qual permissão mínima isso exige (capabilities, polkit, serviço auxiliar). Que sinais de "risco" são confiáveis de detectar?

## Answer

Validado em Ubuntu 26.04.1 / NetworkManager 1.54.3 / systemd 259, sem sudo.

- **(a) Segurança do Wi‑Fi**: NetworkManager (D-Bus) expõe flags do AP (aberta/WEP/WPA1/WPA2/WPA3/OWE/Enterprise) e o perfil ativo mostra o que a conexão usa — sem privilégio.
- **(b) Latência/perda**: ICMP sem root via ping socket (systemd libera `ping_group_range`; testado). Fallback: tempo de conexão TCP 443.
- **(c) DNS/portal cativo**: connectivity check do NM (estado "portal") ativo por padrão no Ubuntu, legível e re-disparável sem privilégio. DNS sequestrado: resolver nome aleatório inexistente (NXDOMAIN esperado).
- **(d) Velocidade**: Cloudflare (speed.cloudflare.com, MIT, HTTPS puro), só sob demanda (não há termos publicados para automação). M-Lab NDT7 publica IP/horário e limita a 4 testes/dia para clientes automatizados; LibreSpeed só faz sentido com servidor próprio; licença da Ookla proíbe embarcar o CLI.
- **(e) Tráfego total**: delta de `/proc/net/dev` por segundo, sem privilégio.
- **(f) Tráfego por processo**: sem privilégio, contadores por conexão TCP do kernel (estilo `ss -tip`) para processos do próprio usuário (testado: 55/55 conexões mapeadas). Exato (eBPF) exige `CAP_BPF`+`CAP_PERFMON` → só como serviço systemd opcional no `.deb`, com usuário dedicado; AppImage não carrega capabilities → só modo sem privilégio. nethogs exige capabilities amplas; nunca dar capability ao interpretador Python.
- **Sinais de risco**: confiáveis para alerta — rede aberta, WEP/TKIP, portal cativo, DNS respondendo a nomes inexistentes. Só aviso contextual — MAC do gateway mudando (possível ARP spoofing), SSID parecido. Nunca declarar a rede "segura".

Achados completos: branch local `research/rede-linux` (commit `533591f`), arquivo `research/rede-linux.md`.
