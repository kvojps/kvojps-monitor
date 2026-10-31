# 04: Qualidade da rede e tráfego total

**What to build:** no Módulo Rede vejo a Qualidade da rede como Boa / Regular / Ruim, com latência, jitter e perda até o gateway e a internet, um gráfico de tráfego total em tempo real e as etiquetas "Cabeada", "VPN ativa" e "Sem conexão" quando aplicável. A velocidade ainda não entra na classificação (ticket 06). Ver spec, "Módulo Rede".

**Blocked by:** 03 (Módulo Sistema ao vivo)

**Status:** feito

- [x] Porta D-Bus do NetworkManager (conexão ativa, tipo, VPN, conectividade) com fixtures gravadas
- [x] Ping sem root via ping socket (fallback: tempo de conexão TCP 443) ao gateway e à internet a cada 5 s, sempre, janela móvel de 2 min
- [x] Regras de Qualidade: Ruim (perda > 5%, internet > 300 ms, sem internet com gateway respondendo); Regular (perda > 1%, internet > 100 ms, gateway > 30 ms, jitter > 30 ms); Boa no resto
- [x] Tráfego total pelo delta de `/proc/net/dev` a cada 1 s, sempre, com gráfico
- [x] Etiquetas Cabeada / VPN ativa; sem conexão, o eixo mostra "Sem conexão"
- [x] Testes do núcleo com relógio falso cobrindo cada limiar; testes de UI por fixtures (boa, ruim, sem conexão, VPN)

## Notas da implementação

- Porta D-Bus do NetworkManager consulta conexão ativa, tipo, gateway, interface, VPN e conectividade. Loopback não é tratado como conexão externa. As fixtures e a origem das variações estão em `src-tauri/fixtures/rede/README.md`.
- Ping socket ICMP sem root; quando a criação do socket não está disponível, usa tempo de conexão TCP 443. Timeout de 700 ms por destino, sondas de gateway e internet em paralelo, a cada 5 s. Destino de internet: `1.1.1.1` (Cloudflare), pela rota efetiva, inclusive VPN. Nenhum teste de velocidade é feito nesta fatia.
- Latência é a mediana da janela móvel de 2 min. Jitter é a média das diferenças absolutas entre respostas consecutivas; não liga respostas separadas por perdas. Gateway não informado fica sem medidas e não conta como perda. Classificação usa ambos os destinos para perda e jitter.
- Tráfego em bytes/s pelo delta da interface física selecionada pelo NetworkManager, excluindo loopback e a duplicação dos bytes do túnel VPN; histórico de 60 pontos. Primeira amostra, reset e leitura indisponível ficam sem taxa, nunca geram pico artificial.
- Coleta e publicação de snapshots continuam com o Painel fechado. Sem cache em disco, logs de conteúdo, notificações ou mudanças no Indicador.
- Testes no seam do núcleo com relógio falso e na UI com snapshots boa, ruim, sem conexão e VPN. Validação manual da janela no GNOME continua fora da suíte automatizada, conforme spec.
