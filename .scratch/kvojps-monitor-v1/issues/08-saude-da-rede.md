# Definir rede saudável ou em risco e cadência do teste de velocidade

Type: grilling
Status: resolved
Blocked by: 05
Map: [kvojps-monitor v1](../map.md)

## Question

Quais sinais e limiares tornam a rede "saudável", "atenção" ou "risco" no Módulo Rede, e o teste de velocidade roda só sob demanda ou também periodicamente (com que frequência e consumo de dados)?

## Answer

O Módulo Rede tem **dois eixos independentes**: Segurança da rede e Qualidade da rede.

- **Segurança da rede**: **Sem alertas / Atenção / Risco**. Nunca "Segura". **Risco**: rede aberta (sem OWE), WEP, WPA com TKIP, DNS respondendo a nomes inexistentes. **Atenção**: portal cativo, MAC do gateway mudou na mesma conexão. "SSID parecido" fora da v1. Segurança do Wi‑Fi e MAC do gateway lidos por evento do NetworkManager; DNS e portal checados ao conectar e a cada 10 min.
- **Qualidade da rede**: nível **Boa / Regular / Ruim** com as métricas em números (latência, jitter e perda até gateway e internet; velocidade). Ping a cada 5 s a cada alvo, sempre (também com o Painel fechado), janela móvel de 2 min. Limiares fixos na v1:
  - **Ruim**: perda > 5%, latência mediana até a internet > 300 ms, ou sem internet com o gateway respondendo; ou velocidade de descida < 2 Mbps.
  - **Regular**: perda > 1%, internet > 100 ms, gateway > 30 ms, jitter > 30 ms; ou descida < 10 Mbps.
  - **Boa**: abaixo de tudo.
  - Velocidade só pesa no nível se medida há menos de 6 h na rede atual; sempre exibida com a idade ("↓ 312 Mbps · ↑ 48 Mbps · há 2 h").
- **Velocidade (Cloudflare)**: teste completo **sob demanda** (botão) + **teste leve automático** (~25 MB ↓, ~10 MB ↑) ao conectar numa rede e a cada 6 h conectado (~140 MB/dia). Desligável nas Configurações; nunca roda em conexão marcada como limitada (metered) no NetworkManager. Último resultado guardado localmente.
- **Notificação**: só quando a Segurança da rede entra em **Risco**, uma vez por conexão. Qualidade não notifica.
- **Indicador**: badge discreto só com Segurança em Risco.
- **Sem conexão**: os dois eixos mostram "Sem conexão"; último teste de velocidade segue visível com a idade; sem badge.
- **Cabeada**: regra de criptografia do Wi‑Fi não se aplica (mostra "Cabeada"); valem DNS, portal e MAC do gateway.
- **VPN ativa**: Segurança avalia o link físico; Qualidade é medida como sai de fato (pela VPN); etiqueta "VPN ativa"; teste de DNS roda como está.
