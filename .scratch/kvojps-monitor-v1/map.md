# Mapa: kvojps-monitor v1

Label: wayfinder:map

## Destination

Uma **spec da v1** do kvojps-monitor pronta para implementação: um app de bandeja (Indicador + Painel) para Ubuntu/GNOME/Wayland que inicia com o login, com todos os Módulos validados como viáveis e gratuitos, stack escolhida, visual prototipado e instalação pública via GitHub Releases (`.deb` + AppImage).

## Notes

- Domínio: app desktop pessoal, só leitura/apresentação de dados (exceto mover cards no Trello). Custo zero: tudo local, sem backend hospedado, só APIs gratuitas com credenciais do usuário.
- Vocabulário em [GLOSSARY.md](../../GLOSSARY.md): Indicador, Painel, Módulo, Configurações, Provedor de IA, Limite de uso, Sessão de agente.
- Máquina-alvo: Ubuntu 26.04, GNOME em Wayland, extensão AppIndicator ativa; Claude Code e Codex instalados; `gh` e Node disponíveis.
- Módulos da v1: IA (limites 5h + semanal de Claude e ChatGPT/Codex), Agentes (sessões locais em andamento/finalizadas + notificações), GitHub (calendário de contribuições do ano + repo com mais contribuições nos últimos 7 dias, última atividade e nº de commits seus), Trello (um quadro, ver listas/cards, mover card entre listas), Rede (segurança do Wi‑Fi, latência/perda, DNS/portal cativo, velocidade, tráfego em tempo real, tráfego por processo), Sistema (CPU, RAM, disco, bateria), Configurações.
- Visual: minimalista, moderno, tech; escuro por padrão seguindo o tema do sistema, números em monoespaçada, acento discreto.
- Stack: decidida em [Escolher a stack](issues/07-escolher-stack.md).
- Planos do usuário (detectados): Claude Pro e ChatGPT Plus. O app deve suportar Claude Pro/Max e ChatGPT Plus/Pro, detectando o plano.
- Skills a consultar: `grilling` + `domain-modeling` em tickets de decisão; `research` em pesquisa; `prototype` no visual; `to-spec` ao consolidar.
- Tracker: markdown local (`.scratch/kvojps-monitor-v1/issues/`); sem remote git, branches `research/*` ficam locais.

## Decisions so far

- [Stack para Indicador e Painel no GNOME/Wayland](issues/06-stack-indicador.md): Tauri v2 recomendado; clique simples e Painel ancorado ao ícone só com extensão GNOME fina opcional
- [Rede no Linux: segurança, latência, velocidade, tráfego e permissões](issues/05-rede-linux.md): quase tudo sem privilégio (NM, ping socket, /proc); velocidade via Cloudflare sob demanda; por processo exato só com serviço eBPF opcional no .deb
- [Como ler os Limites de uso de Claude e Codex localmente](issues/01-limites-de-uso-ia.md): Codex via `codex app-server` (oficial); Claude via status line (documentado, defasado) ou `oauth/usage` (não documentado, risco de ToS); plano detectável nos dois
- [GitHub: calendário do ano e repo destaque dos últimos 7 dias](issues/03-github-atividade.md): calendário via GraphQL com token do `gh`; destaque via `history` por autor (só commits) ou `read:user` para contar tudo; cache local
- [Como detectar Sessões de agente do Claude Code e do Codex e seu estado](issues/02-sessoes-de-agente.md): Claude via `claude agents --json` + hooks; Codex via SQLite + rollouts + hooks; hooks aditivos, adapters por fonte
- [Trello no plano gratuito: ler quadro e mover card](issues/04-trello-api.md): viável no Free com key+token próprios do usuário; polling (sem webhook)
- [Escolher a stack](issues/07-escolher-stack.md): Tauri v2 + Svelte 5; Indicador só de bandeja (`ksni`), sem extensão GNOME; processo único sem privilégio que empurra snapshots à UI; keyring obrigatório; `.deb` + AppImage em runner 24.04
- [Definir rede saudável ou em risco e cadência do teste de velocidade](issues/08-saude-da-rede.md): dois eixos — Segurança (Sem alertas/Atenção/Risco) e Qualidade (Boa/Regular/Ruim com números); ping 5 s; velocidade sob demanda + teste leve ao conectar e a cada 6 h; notifica e marca o Indicador só em Risco
- [Quais eventos de Sessão de agente geram notificação](issues/09-notificacoes-agentes.md): notificam Aguardando você, Com falha e Turno concluído ≥ 30 s; uma notificação por sessão, agrupada a partir de 3 em 10 s; ponto no Indicador enquanto algo aguarda; hooks opt-in, senão polling de 2 s no Claude
- [Layout e visual do Painel](issues/10-visual-do-painel.md): trilho lateral de ícones + um Módulo por vez, ~440×600, Configurações no rodapé; escuro com acento verde-água e números monoespaçados; memória medida no 1º esqueleto Tauri
- [Critério do repo destaque no Módulo GitHub](issues/12-criterio-repo-destaque.md): só seus commits na branch padrão (token do `gh`, sem `read:user`); mostra commits em 7 dias e total; última atividade = seu último commit; estado vazio sem substituto
- [Fonte dos Limites de uso do Claude](issues/13-fonte-limites-claude.md): status line padrão (conectada por botão, encadeia a existente, reversível); `oauth/usage` opt-in desligado, 5 min, sem refresh de token; dado velho mostra idade e nunca assume 0% após reinício
- [Modo do tráfego por processo](issues/14-trafego-por-processo.md): só sem privilégio (TCP, processos do usuário); top 5 por app com taxa ↓/↑ + linha "Outros"; coleta a cada 2 s só com o Módulo Rede visível

## Not yet specified

- Intervalos de atualização (polling) de cada Módulo e comportamento quando o Painel está fechado (Rede já definida em [Modo do tráfego por processo](issues/14-trafego-por-processo.md) e em [Definir rede saudável ou em risco e cadência do teste de velocidade](issues/08-saude-da-rede.md); Agentes em [Quais eventos de Sessão de agente geram notificação](issues/09-notificacoes-agentes.md)).
- Pipeline de release no GitHub Actions e estrutura pública do repo (README, licença).
- Módulo Sistema: fontes de dados (provavelmente trivial; layout definido em [Layout e visual do Painel](issues/10-visual-do-painel.md)).
- Futuro, ainda no horizonte: PRs aguardando revisão e status de CI.

## Out of scope

- Descoberta de dispositivos desconhecidos na rede local: varredura ativa, invasiva para a v1.
- Criar/editar cards, comentários e múltiplos quadros no Trello.
- Sessões de agente na nuvem (claude.ai/code, Codex cloud).
- Suporte a Windows/macOS.
- Qualquer backend hospedado ou serviço pago.
- Alerta de Limite de uso a 80% e atalho global de teclado (usuário dispensou na v1).
- Extensão do GNOME Shell para clique simples e Painel ancorado ao ícone: custo de manutenção a cada versão do GNOME e inexistente no AppImage ([ADR 0001](../../docs/adr/0001-indicador-sem-extensao-gnome.md); decidido em [Escolher a stack](issues/07-escolher-stack.md)).
- Serviço eBPF privilegiado para tráfego por processo exato (UDP/QUIC, processos do sistema): exigiria capabilities, usuário dedicado e componente separado só no `.deb`; o modo sem privilégio basta para um top consumidores ([Modo do tráfego por processo](issues/14-trafego-por-processo.md)).
