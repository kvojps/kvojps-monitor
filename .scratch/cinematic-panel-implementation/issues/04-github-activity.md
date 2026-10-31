# 04: GitHub com atividade e Repo em destaque

**What to build:** consultar atividade anual e recente e o Repo em destaque na seleção visual aprovada, com atualização e recuperação acessíveis.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** resolved

- [x] Exibir total identificado como anual, mapa das últimas oito semanas e barras dos últimos 14 dias, sem calendário anual completo ou rolagem de detalhes.
- [x] Mostrar Repo em destaque com commits em 7 dias e título do último commit, preservando o cálculo existente e contagem anônima de contribuições privadas.
- [x] Atualizar/configurar funcionam no Módulo, respeitando intervalo manual mínimo de 10 s; ausência de fonte nunca vira número de demonstração.
- [x] Cobrir vazio sem atividade, fonte ausente, credencial inválida, antigo, falha com cache e offline com idade sem faixa de erro de rede.
- [x] Testar snapshots e comandos, resumo textual dos gráficos e identificação acessível de valores/séries, mantendo foco e rolagem nas atualizações.
- [x] Preservar coleta/armazenamento e testar que credenciais e títulos de commits não são gravados em logs/cache nem enviados em notificações.

## Registro da implementação (2026-10-31)

- **Módulo GitHub no Painel:** `GithubModule` volta a montar o conteúdo como na composição H: total anual em destaque (“1.608 contribuições em 2026”, o ano vem da data do último dia lido), mapa das últimas oito semanas, barras dos últimos 14 dias e Repo em destaque com commits em 7 dias e título do último commit. Calendário anual completo, total de commits do repo e idade do último commit saem do Painel, conforme “sem dados adicionais de detalhe”.
- **Dias futuros:** a coleta pede o ano civil inteiro e o GitHub devolve dias até 31/12 (53 semanas na pesquisa de 30/10). O Módulo descarta dias posteriores à data local antes de cortar as oito semanas e os 14 dias. Limitação conhecida, sem mudar a coleta: no começo do ano o mapa mostra só as semanas do ano corrente (menos de oito).
- **Escala:** intensidade e altura das barras usam escala local das próprias janelas, nunca as cores do GitHub (a fixture traz cores do tema de Halloween).
- **Contribuições privadas:** “inclui N privadas (anônimas)”, pois o total do GitHub já as soma.
- **Acessibilidade:** o mapa é `img` com resumo textual (total, intervalo e máximo diário); as barras são uma lista com nome por dia (“21/10: 35 contribuições”); legendas visíveis dão total e pico dos 14 dias. Sem `title` só em hover.
- **Estados:** fonte ausente explica `gh auth login`/token sem nenhum número; antes do primeiro snapshot, “Sem leitura da atividade do GitHub ainda” com as ações; credencial inválida esmaece o cache e oferece “Reconectar” (abre Configurações); falha sustentada mostra idade da falha e mantém dados; antigo e offline mostram idade sem faixa de erro, com atualizar desabilitado offline. Título vazio restaurado do cache (que nunca grava títulos) não é exibido.
- **Ações:** “Atualizar GitHub” (10 s entre cliques manuais) e “Configurar GitHub” no Módulo; o antigo alternador de instruções foi substituído pelo texto direto, sem expansor.
- **Privacidade:** coleta e armazenamento inalterados. Logs e cache sem token nem título já são cobertos por `restart_publishes_sanitized_last_snapshot_before_any_network_request` e `own_token_and_commit_content_never_reach_cache_or_logs`. O coletor do GitHub não tem porta de notificação, então nada dele chega a notificações.
- **Decisões validadas pelo usuário (2026-10-31), divergências registradas:** (1) Repo em destaque como em H: commits em 7 dias e título, sem o total de commits e a idade do último commit da v1; (2) no começo do ano o mapa tem menos de oito semanas (e, no início de janeiro, menos de 14 barras), sem mudar a coleta do ano civil; (3) o mapa tem só resumo textual; valores por dia ficam nas barras dos 14 dias.
- **Revisão:** o Repo em destaque some quando nada foi lido (antes dizia “Sem commits” numa falha sem cache); o total anual aparece mesmo sem dias até hoje no calendário; singular para 1 contribuição/commit; números formatados em pt-BR.
- **Pendente (usuário):** conferência visual nativa nas três larguras e nos temas escuro/claro.

## Comments

**2026-10-31 — resolvido a pedido do usuário.** Total anual, oito semanas, 14 dias e Repo em destaque entregues em `46ca8a6`. A conferência visual nativa listada em “Pendente (usuário)” segue para o ticket 10, junto com a validação de composição e acessibilidade no GNOME/Wayland.
