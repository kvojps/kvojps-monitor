# Rede no Linux: segurança, latência, velocidade, tráfego e permissões

Contexto: app de bandeja pessoal (kvojps-monitor) em Ubuntu 26.04 / GNOME Wayland, custo zero, tudo local, instalável por terceiros via `.deb`/AppImage.
Data da pesquisa: 2026-10-30. Validações locais feitas na máquina de desenvolvimento (Ubuntu 26.04.1 LTS, kernel 7.0, NetworkManager 1.54.3, systemd 259, iputils-ping 20250605), apenas com comandos sem `sudo`.

## (a) Segurança do Wi‑Fi conectado via NetworkManager

Fonte primária: tipos D-Bus do NM — https://networkmanager.dev/docs/api/latest/nm-dbus-types.html

- Caminho D-Bus: `org.freedesktop.NetworkManager` → `Device` do tipo Wi‑Fi → propriedade `ActiveAccessPoint` (interface `org.freedesktop.NetworkManager.Device.Wireless`) → objeto `AccessPoint` com `Flags`, `WpaFlags`, `RsnFlags`. A leitura não exige privilégio (é o mesmo que o `nmcli` faz como usuário comum).
- `Flags` (`NM80211ApFlags`): `PRIVACY = 0x1` indica que o AP exige criptografia (WEP ou WPA). Também `WPS = 0x2`, `WPS_PBC = 0x4`, `WPS_PIN = 0x8`.
- `WpaFlags` (IE WPA1) e `RsnFlags` (IE RSN, isto é, WPA2/WPA3) são `NM80211ApSecurityFlags`: `PAIR_WEP40 0x1`, `PAIR_WEP104 0x2`, `PAIR_TKIP 0x4`, `PAIR_CCMP 0x8`, `GROUP_WEP40 0x10`, `GROUP_WEP104 0x20`, `GROUP_TKIP 0x40`, `GROUP_CCMP 0x80`, `KEY_MGMT_PSK 0x100`, `KEY_MGMT_802_1X 0x200`, `KEY_MGMT_SAE 0x400`, `KEY_MGMT_OWE 0x800`, `KEY_MGMT_OWE_TM 0x1000`, `KEY_MGMT_EAP_SUITE_B_192 0x2000`.
- Classificação proposta a partir desses bits:
  - Aberta: `PRIVACY` ausente e `WpaFlags == RsnFlags == 0`.
  - OWE ("Enhanced Open"): `RsnFlags & KEY_MGMT_OWE`. É cifrada, mas sem autenticação: qualquer um entra.
  - WEP: `PRIVACY` presente com `WpaFlags == RsnFlags == 0` (ou bits `*_WEP*`).
  - WPA1: apenas `WpaFlags` diferente de zero. TKIP como cifra par indica configuração antiga e fraca.
  - WPA2-Personal: `RsnFlags & KEY_MGMT_PSK`. WPA3-Personal: `RsnFlags & KEY_MGMT_SAE`. Os dois juntos indicam modo de transição WPA2/WPA3.
  - Enterprise: `KEY_MGMT_802_1X` (WPA2/3-Enterprise) ou `EAP_SUITE_B_192` (WPA3-Enterprise 192-bit).
- O que o AP anuncia não é necessariamente o que a conexão usa. Para o modo efetivo, ler o perfil ativo, setting `802-11-wireless-security.key-mgmt`: `none` (WEP estático ou sem senha), `ieee8021x` (WEP dinâmico), `owe`, `wpa-psk` (WPA2 e WPA3 personal), `sae` (somente WPA3), `wpa-eap`, `wpa-eap-suite-b-192` — https://networkmanager.dev/docs/api/latest/settings-802-11-wireless-security.html. Perfil sem setting de segurança = rede aberta.
- Validação local: `nmcli -f IN-USE,SSID,SECURITY,WPA-FLAGS,RSN-FLAGS dev wifi list --rescan no` retorna, sem root, `SECURITY=WPA2` e `RSN-FLAGS=pair_ccmp group_ccmp psk`.
- Em Python, o caminho mais simples é falar D-Bus direto (GIO `Gio.DBusProxy`, `jeepney` ou `dbus-next`), ou usar `libnm` via GObject Introspection (`gi.repository.NM`), que é a biblioteca cliente oficial do NM.

## (b) Latência e perda até gateway e internet (ICMP sem root)

- Kernel: `net.ipv4.ping_group_range` restringe sockets ICMP do tipo datagrama (`socket(AF_INET, SOCK_DGRAM, IPPROTO_ICMP)`) aos grupos dentro do intervalo. O padrão do kernel é `"1 0"`: "nobody (not even root) may create ping sockets" — https://docs.kernel.org/networking/ip-sysctl.html
- O systemd sobrescreve esse padrão: `sysctl.d/50-default.conf` define `-net.ipv4.ping_group_range = 0 2147483647` com o comentário "ping(8) without CAP_NET_ADMIN and CAP_NET_RAW" — https://github.com/systemd/systemd/blob/main/sysctl.d/50-default.conf. No Ubuntu 26.04 o arquivo fica em `/usr/lib/sysctl.d/50-default.conf` e `/proc/sys/net/ipv4/ping_group_range` vale `0 2147483647` (validado).
- Validação local: `python3 -c "import socket; socket.socket(socket.AF_INET, socket.SOCK_DGRAM, socket.IPPROTO_ICMP)"` funciona como usuário comum. Portanto **o app pode fazer ICMP echo sem root e sem capabilities**. O kernel cuida do identificador e do checksum; o app só monta o payload. O mesmo sysctl vale para IPv6 (`IPPROTO_ICMPV6`).
- O binário `/usr/bin/ping` ainda tem `cap_net_raw=ep` (o `postinst` do `iputils-ping` executa `setcap cap_net_raw+ep`), mas usar o socket datagrama diretamente evita depender disso e de interpretar saída traduzida.
- Alvos: o gateway é a rota default (`ip route`/rtnetlink, ou `Ip4Config.Gateway` do NM; local: `192.0.2.101`); a internet é um IP anycast estável (ex. 1.1.1.1, 8.8.8.8, 9.9.9.9). O diagnóstico mais útil é separar "gateway ok, internet ruim" de "Wi‑Fi ruim".
- Métricas: RTT mín/méd/máx; jitter (média da diferença absoluta entre RTTs consecutivos, a mesma definição usada pelo Cloudflare speedtest, ver d); perda = respostas faltantes / pacotes enviados em janela deslizante. Fallback quando ICMP é bloqueado: tempo de handshake TCP (`connect`) na porta 443.
- Ressalva: em ambientes sem esse sysctl, o socket falha com `EACCES`/`EPERM`. Tratar o erro e cair para TCP-connect.

## (c) Checagem de DNS e portal cativo (connectivity check do NM)

- Configuração: seção `[connectivity]` do `NetworkManager.conf` (`uri`; `interval`, padrão 300 s; `response`, padrão "NetworkManager is online"; `enabled`). Sem `uri` a checagem fica desligada. A doc diz: "Gnome's portal helper uses this as signal to show a captive portal login page" — https://networkmanager.dev/docs/api/latest/NetworkManager.conf.html
- O Ubuntu ativa por padrão: `/usr/lib/NetworkManager/conf.d/20-connectivity-ubuntu.conf` contém `uri=http://connectivity-check.ubuntu.com/` (validado).
- Estado no D-Bus, sem privilégio: propriedade `Connectivity` do objeto raiz (`NMConnectivityState`: `0 UNKNOWN`, `1 NONE`, `2 PORTAL`, `3 LIMITED`, `4 FULL`, conforme nm-dbus-types acima), além de `ConnectivityCheckAvailable`, `ConnectivityCheckEnabled`, `ConnectivityCheckUri` e o método `CheckConnectivity() → u`, que força uma checagem. Validado com `busctl introspect org.freedesktop.NetworkManager /org/freedesktop/NetworkManager` (valor local: `4`). O sinal `PropertiesChanged` permite reagir sem polling.
- Portanto **a detecção de portal cativo já vem pronta**: `Connectivity == 2` (PORTAL). O valor `3` (LIMITED) indica internet quebrada ou DNS falhando.
- Checagens de DNS próprias, complementares e sem root:
  - Servidores em uso: `IP4Config.NameserverData` do NM ou `org.freedesktop.resolve1` (systemd-resolved; `resolvectl dns` mostra localmente `192.0.2.101`).
  - Sequestro de NXDOMAIN: resolver um nome aleatório inexistente. A RFC 6761 §6.4 reserva `.invalid` para sempre retornar NXDOMAIN — https://www.rfc-editor.org/rfc/rfc6761#section-6.4 (um subdomínio aleatório de um domínio real também serve e é mais difícil de "acertar" por listas). Se vier um IP, o resolvedor está reescrevendo respostas (ISP com página de busca, portal ou ataque).
  - Comparar o resolvedor local com um resolvedor confiável via DoH (RFC 8484 — https://www.rfc-editor.org/rfc/rfc8484; ex. `https://cloudflare-dns.com/dns-query`) para alguns domínios estáveis. Divergência persistente combinada com certificado TLS inválido no destino é sinal forte. Divergência isolada não é, porque CDNs e GeoDNS respondem IPs diferentes.
- Portais modernos podem anunciar a URL via DHCP/RA (RFC 8910, opção Captive-Portal — https://www.rfc-editor.org/rfc/rfc8910) e via API (RFC 8908 — https://www.rfc-editor.org/rfc/rfc8908), mas o estado do NM já cobre o caso comum.

## (d) Teste de velocidade gratuito

| Opção | Licença do cliente | Servidor | Termos para uso automatizado | Observações |
|---|---|---|---|---|
| Cloudflare (`speed.cloudflare.com`, lib `@cloudflare/speedtest`) | MIT — https://github.com/cloudflare/speedtest | `GET https://speed.cloudflare.com/__down?bytes=N`, `POST https://speed.cloudflare.com/__up` | O projeto não publica limites nem termos específicos para uso automatizado. Os resultados são enviados à Cloudflare para estatísticas agregadas. | Mede download, upload, latência com e sem carga, jitter; perda só com servidor TURN próprio. Como os endpoints são HTTP simples, é fácil reimplementar em Python. O teste padrão transfere até ~250 MB. |
| LibreSpeed (`librespeed-cli`) | LGPL-3.0 — https://github.com/librespeed/speedtest-cli | Lista pública de servidores de voluntários, ou servidor próprio (`--local-json`, `--server-json`) | Sem política de uso dos servidores públicos documentada. A telemetria vai para librespeed.org por padrão (desligável com `--telemetry-level disabled`). | Saída `--json`/`--csv`. Ideal quando se quer autohospedar. |
| M-Lab NDT7 (`ndt7-client-go`, https://github.com/m-lab/ndt7-client-go) | Apache-2.0 | Plataforma M-Lab | AUP: clientes automatizados "should not run more than 4 tests per day", com horários aleatorizados; uso interativo até 40/dia; integrações comerciais devem contribuir; cliente de terceiros precisa publicar política de privacidade compatível — https://www.measurementlab.net/aup/ | **O IP do usuário, a data e a hora são publicados** em dados abertos (domínio público) — https://www.measurementlab.net/privacy/. Exige aviso e consentimento explícitos no app. |
| Ookla Speedtest CLI | Proprietária (EULA) | Ookla | Licença "for your personal, non-commercial use on a single personal computer"; proibido copiar "except as expressly permitted"; proibido uso em roteador/modem. EULA oficial: https://www.speedtest.net/about/eula (não acessível pela ferramenta de busca; texto conferido na cópia do Gentoo: https://mirrors.sjtug.sjtu.edu.cn/gentoo/licenses/Ookla) | **Não pode ser empacotado nem redistribuído** no `.deb`/AppImage de um app open-source. No máximo, usar se o próprio usuário já o tiver instalado. |

Conclusão de (d): Cloudflare é o caminho mais simples (MIT, só HTTPS, sem binário extra), desde que rode **sob demanda** (clique do usuário), e não em agendamento. NDT7 é alternativa com aviso de privacidade e no máximo 4 testes/dia se automatizado. Ookla fica de fora.

## (e) Tráfego total em tempo real

- `/proc/net/dev`: "The historical /proc/net/dev text interface"; alguns contadores são agregados (ex. `rx_missed_errors` é somado em `drop`) — https://docs.kernel.org/networking/statistics.html
- `/sys/class/net/<if>/statistics/{rx,tx}_bytes`: um arquivo por contador (ABI `sysfs-class-net-statistics`), mas "inefficient when reading multiple stats as it internally performs a full dump" (mesma página).
- Segundo a mesma página, rtnetlink (`RTM_GETLINK`/`RTM_GETSTATS`) é o método preferido. Em Python, `pyroute2` faz isso.
- Na prática, ler `/proc/net/dev` a cada 1 s e calcular deltas (bytes/s) é barato, não exige privilégio e cobre todas as interfaces (validado). Ignorar `lo` e, conforme a preferência, interfaces virtuais (docker, veth, tun). Os contadores são de 64 bits, mas é preciso tratar o reset quando uma interface some e volta.

## (f) Tráfego por processo e permissões mínimas

### Opções

1. **Sem privilégio, via `sock_diag`/`tcp_info` (o que `ss -tip` faz)**. O kernel expõe, por socket TCP, `bytes_sent`, `bytes_acked` e `bytes_received` (`struct tcp_info` em `include/uapi/linux/tcp.h` — https://github.com/torvalds/linux/blob/master/include/uapi/linux/tcp.h; interface `sock_diag(7)` — https://man7.org/linux/man-pages/man7/sock_diag.7.html). O mapeamento socket → processo é feito pelos inodes em `/proc/<pid>/fd/`, que o usuário consegue ler **para os próprios processos** sem privilégio. Validado: `ss -tinpH state established` como usuário comum mostrou 55 de 55 conexões com `users:(("firefox",pid=...))` e os contadores de bytes.
   Limitações: só TCP (QUIC/UDP não tem contadores de bytes no sock_diag); sockets que abrem e fecham entre duas leituras se perdem; processos de outros usuários ou do sistema (ex. `apt`, `snapd`) aparecem sem dono. Mesmo assim, é uma **aproximação boa o bastante para um "top consumidores"** num app de bandeja.
2. **libpcap/nethogs**: captura pacotes e cruza com `/proc/net/tcp*` e `/proc/<pid>/fd`. Segundo o README do nethogs, sem root ele precisa de `cap_net_admin,cap_net_raw` (captura) **e** `cap_dac_read_search,cap_sys_ptrace` (ler processos de outros usuários) — https://github.com/raboof/nethogs (GPL-2.0+). É um conjunto de capabilities grande e sensível.
3. **eBPF** (kprobes/tracepoints em `tcp_sendmsg`/`tcp_cleanup_rbuf`/`udp_*`, ou programas cgroup-skb): preciso, cobre UDP/QUIC e processos de vida curta. Exige `CAP_BPF` + `CAP_PERFMON`, separados de `CAP_SYS_ADMIN` no Linux 5.8 (https://man7.org/linux/man-pages/man7/capabilities.7.html), e às vezes `CAP_NET_ADMIN` para anexar a um cgroup. No Ubuntu 26.04, `kernel.unprivileged_bpf_disabled = 2` e `kernel.perf_event_paranoid = 4` (validado), ou seja, bpf sem privilégio está desativado — https://docs.kernel.org/admin-guide/sysctl/kernel.html. Em Python, o BCC (`python3-bpfcc`) é pesado (compila em tempo de execução e precisa dos headers do kernel); um programa libbpf CO-RE pré-compilado é o mais portável.
4. **systemd `IPAccounting=yes`**: contabiliza IPv4/IPv6 de todos os sockets da unit (via BPF), mas, segundo `systemd.resource-control(5)`, "this functionality is currently only available for system services, not for per-user services" (conferido no man local; online: https://www.freedesktop.org/software/systemd/man/latest/systemd.resource-control.html). Não serve para apps GNOME, que rodam em scopes de usuário.

### Permissão mínima e empacotamento

- Capabilities de arquivo (`setcap`) ficam no xattr `security.capability` (capabilities(7)). O padrão Debian/Ubuntu é aplicá-las no `postinst` do pacote. Exemplo real: `/var/lib/dpkg/info/iputils-ping.postinst` executa `setcap cap_net_raw+ep` e só remove o setuid se o `setcap` funcionar (validado).
- **Nunca dar capabilities ao interpretador Python**, porque elas valeriam para qualquer script. Se for preciso privilégio, isolá-lo num helper pequeno.
- **AppImage não consegue carregar capabilities nem setuid**: `execve(2)` diz que "the capabilities of the program file ... are also ignored" quando o sistema de arquivos está montado com `nosuid` (https://man7.org/linux/man-pages/man2/execve.2.html), e o AppImage é montado via FUSE com `nosuid` (relato com resposta de mantenedor em https://github.com/AppImage/AppImageKit/issues/881). Um AppImage também não instala units systemd nem políticas polkit.
- Arquitetura recomendada se o modo preciso for desejado (só no `.deb`):
  - Serviço de sistema `kvojps-monitor-netd.service` rodando como usuário dedicado (não root) com `AmbientCapabilities=CAP_BPF CAP_PERFMON CAP_NET_ADMIN` e `CapabilityBoundingSet=` com o mesmo conjunto (`systemd.exec(5)`, `AmbientCapabilities=` existe desde a versão 229 — https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html), mais `NoNewPrivileges=yes`, `ProtectSystem=strict`, `PrivateTmp=yes` etc.
  - O serviço expõe só dados agregados (pid, comm, bytes) via D-Bus de sistema ou socket Unix/varlink somente leitura; o app de bandeja apenas consome.
  - Ativação opcional: pacote separado (`kvojps-monitor-netd`) ou ativação pelo app pedindo autorização via polkit (ação própria em `/usr/share/polkit-1/actions/`, com `auth_admin_keep`) — https://www.freedesktop.org/software/polkit/docs/latest/polkit.8.html
  - Expor nomes de processos de todos os usuários vaza informação em máquinas multiusuário: filtrar pelo UID de quem chama (`SO_PEERCRED` ou credenciais do D-Bus).
- AppImage: oferecer só o modo sem privilégio (opção 1) e explicar que "tráfego por processo preciso" requer o `.deb`.

## Sinais de risco: o que dá para detectar com confiança

| Sinal | Como | Confiabilidade |
|---|---|---|
| Rede aberta (sem criptografia) | `key-mgmt` ausente/`none` e AP sem `PRIVACY` (seção a) | **Alta**: é um fato objetivo. OWE é cifrada mas não autenticada (risco médio). |
| WEP / WPA1-TKIP | bits `*_WEP*`, `PAIR_TKIP`, ou só `WpaFlags` | **Alta**: criptografia quebrada ou obsoleta. |
| Portal cativo | `Connectivity == PORTAL (2)` no NM | **Alta**: é o mesmo sinal que o GNOME usa. |
| Sem internet / DNS quebrado | `Connectivity == LIMITED (3)`; ICMP ao gateway ok mas resolução falha | **Alta** para "algo está errado", baixa para apontar a causa. |
| DNS reescrevendo NXDOMAIN | nome aleatório inexistente resolve para um IP | **Média a alta**; é comum em ISPs e nem sempre é malicioso. |
| DNS divergente do DoH | comparação de A/AAAA para domínios fixos | **Baixa a média** sozinho (CDN/GeoDNS); só alertar se vier junto com falha de TLS. |
| MAC do gateway mudou | observar o vizinho da rota default via rtnetlink `RTM_NEWNEIGH`/`ip neigh` (local: `192.0.2.101 lladdr 02:00:00:00:00:01`) e guardar por (SSID/BSSID, IP do gateway) — https://man7.org/linux/man-pages/man7/rtnetlink.7.html | **Média**: troca de roteador, failover (VRRP) e redes mesh geram falsos positivos. Alternância rápida e repetida, ou dois MACs para o mesmo IP, é mais suspeita (a mesma heurística do arpwatch). |
| ARP spoofing ativo | MAC do gateway igual ao MAC de outro host da LAN na tabela de vizinhos | **Média**: só se detecta o que o próprio host vê; sem captura (CAP_NET_RAW) não dá para ver ARP de terceiros. |
| Rogue AP / evil twin | mesmo SSID com BSSID novo **e** segurança diferente (ex. antes WPA2, agora aberta) | **Média**. A mudança de segurança no mesmo SSID é o sinal forte; um BSSID novo sozinho é normal (redes com vários APs). |
| Latência/perda alta | ICMP datagrama (seção b) | **Alta** como métrica de qualidade, não de segurança. |

Evitar, por não ser confiável: "detectar MITM" de forma genérica só por latência; varreduras de portas na LAN (intrusivas e exigem mais privilégio); e qualquer veredito de "rede segura". O app deve dizer "nenhum sinal de risco detectado", não "rede segura".

## Recomendação

1. **Segurança do Wi‑Fi**: ler `ActiveAccessPoint` (`Flags`/`WpaFlags`/`RsnFlags`) e o `key-mgmt` do perfil ativo via D-Bus do NM. Não precisa de privilégio.
2. **Portal cativo e conectividade**: assinar a propriedade `Connectivity` do NM (PORTAL/LIMITED/FULL) e chamar `CheckConnectivity()` sob demanda; complementar com o teste de NXDOMAIN.
3. **Latência e perda**: sockets ICMP datagrama (sem root, graças ao `ping_group_range` que o systemd configura) para o gateway e para um IP público; fallback por TCP-connect.
4. **Tráfego total**: deltas de `/proc/net/dev` a cada 1 s.
5. **Tráfego por processo**: começar pelo modo sem privilégio via `sock_diag`/`tcp_info` (só TCP, só processos do usuário), que funciona igual no `.deb` e no AppImage. Deixar o eBPF como helper systemd opcional, só no `.deb`, com `AmbientCapabilities=CAP_BPF CAP_PERFMON` e usuário dedicado. Nunca usar `setcap` no Python e nunca empacotar nethogs com 4 capabilities.
6. **Velocidade**: Cloudflare (`__down`/`__up`, MIT), só quando o usuário clicar; NDT7 opcional, com aviso de que o IP é publicado; não incluir Ookla.
7. **Alertas de risco**: alertar com confiança sobre rede aberta, WEP/TKIP, portal cativo e DNS sequestrando NXDOMAIN; tratar mudança de MAC do gateway e evil twin como "aviso" com contexto, nunca como certeza.

Nota de privacidade (ticket 40): endereços e MAC da conexão observada foram substituídos por aliases de documentação; preservam-se os contratos e as conclusões técnicas.
