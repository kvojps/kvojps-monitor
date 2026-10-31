# 02: IA com instrumentos de Limite de uso

**What to build:** acompanhar Codex e Claude pelos instrumentos circulares de uso de 5 h e barras semanais aprovados, com leitura confiável e caminhos de recuperação no próprio Painel.

**Blocked by:** 01 — Painel em estações com navegação e coleta por visibilidade.

**Status:** ready-for-human

- [x] Exibir dois instrumentos de 5 h com percentual/reinício e duas barras semanais, identificação e idade da leitura; não adicionar buckets, planos, calendário ou rolagem de detalhes.
- [x] Atualizar/configurar e reconectar continuam funcionais; atualização manual respeita intervalo mínimo de 10 s. Reconexão ocorre somente por ação explícita.
- [x] Preservar estados de CLI ausente, sem leitura, status line desconectada, token expirado e consentimento para oauth/usage, mantendo independência entre Provedores de IA.
- [x] Ao reiniciar uma janela, remover seu arco/barra e mostrar “Reiniciado às HH:MM · sem leitura nova”; preservar dado válido antigo com idade, sem inferir 0%.
- [x] Testar fixtures cheio, vazio, antigo, credencial inválida e erro, estados específicos por provedor, comandos e recuperação; instrumentos têm nomes/valores acessíveis e estados não dependem apenas da cor.
- [x] Preservar aquisição existente; testar que o token Claude fica só em memória, nunca é renovado e não aparece em logs/cache/notificações, usando o seam de E/S existente.

## Registro da implementação (2026-10-31)

- **Módulo IA no Painel:** `AiModule` volta a montar o conteúdo do Módulo como na composição H. Cada provedor (Codex, Claude) é uma região com o instrumento circular da janela de 5 h (percentual, “consumido”, reinício), a barra semanal (percentual, reinício) e a origem/idade da leitura. Ações no Módulo: “↻ Atualizar IA” (intervalo mínimo de 10 s, desabilitada offline ou durante atualização) e “Configurar IA” (abre Configurações; “Voltar ao Painel” devolve o foco). Antes do primeiro snapshot o Módulo já mostra a explicação de sem leitura e as duas ações.
- **Fora do Painel, conforme a spec:** planos (Plus/Pro) e limites semanais por modelo do oauth/usage. Só entra o bucket que cobre todos os modelos; janelas de outra duração não viram “5 h”/“semana” (aparecem como sem leitura).
- **Estados por provedor, independentes:** CLI ausente, sem leitura, status line desconectada (“Reconectar” só por clique; falha cai em Configurações, como na v1), status line não conectada (“Configurar”), token expirado (mantém a leitura da status line), falha sustentada com idade (oculta quando pausado offline). Sem dado, o instrumento é substituído pela explicação. O consentimento para oauth/usage continua em Configurações (`AiSettings`, inalterado).
- **Reinício:** o arco/barra some e aparece “Reiniciado às HH:MM · sem leitura nova”; nunca 0%. O instrumento sem valor tem nome acessível com o motivo.
- **Acessibilidade:** instrumentos são `meter` com nome (“Codex · janela de 5 h”, “Claude · semana”) e valor; uso a partir de 85% (limiar do protótipo H, substitui os 75/90% de cor da v1) fica âmbar **e** ganha “⚠ uso alto” em texto e em `aria-valuetext`.
- **Privacidade:** aquisição inalterada. Os testes do núcleo pelo seam existente (`the_token_is_only_read_never_renewed_written_or_kept`, `restart_shows_the_cached_reading_and_never_the_token`) cobrem token só em memória, sem renovação e ausente de cache, log e snapshot publicado. O coletor de IA não tem porta de notificador, então o token não tem caminho até notificações.
- **Decisões locais a validar:** nome do provedor passou de “ChatGPT / Codex” para “Codex”, como na spec e no protótipo H. Os anéis decorativos ficam parados até a Coreografia combinada (09), para respeitar redução manual e Painel oculto.
- **Pendente (usuário):** conferência visual nativa nas três larguras e nos temas escuro/claro.
