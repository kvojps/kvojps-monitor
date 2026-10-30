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
- Stack: a que funcionar melhor (usuário sem preferência).
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

## Not yet specified

- Intervalos de atualização (polling) de cada Módulo e comportamento quando o Painel está fechado.
- Fluxo de credenciais nas Configurações (onboarding, validação de token, armazenamento) — depende da stack.
- Pipeline de release no GitHub Actions e estrutura pública do repo (README, licença).
- Módulo Sistema: fontes de dados e layout (provavelmente trivial; pode se resolver junto da stack/protótipo).
- Futuro, ainda no horizonte: PRs aguardando revisão e status de CI.

## Out of scope

- Descoberta de dispositivos desconhecidos na rede local: varredura ativa, invasiva para a v1.
- Criar/editar cards, comentários e múltiplos quadros no Trello.
- Sessões de agente na nuvem (claude.ai/code, Codex cloud).
- Suporte a Windows/macOS.
- Qualquer backend hospedado ou serviço pago.
- Alerta de Limite de uso a 80% e atalho global de teclado (usuário dispensou na v1).
