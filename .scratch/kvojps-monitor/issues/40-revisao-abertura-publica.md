# 40: Revisão antes da abertura pública

**What to build:** uma revisão documentada dos arquivos e do histórico Git que serão expostos ao tornar o repositório público, sem registrar os próprios dados sensíveis encontrados.

**Status:** resolved

- [x] Delimitar quais branches, tags e histórico ficarão públicos; preservar a regra de nunca enviar branches `research/*` e `prototype/*`
- [x] Revisar os arquivos e o histórico da base examinada para credenciais, dados pessoais e conteúdo de prompts, respostas, commits e cards; incluir versões textuais, imagens e binários históricos
- [x] Corrigir os achados históricos e verificar a `main` pública resultante por clone novo
- [x] Incluir fixtures em `src-tauri/fixtures/`, pesquisas em `docs/research/`, tickets e validações em `.scratch/`, imagens em `docs/assets/` e skills versionadas na revisão
- [x] Distinguir dados sintéticos/sanitizados de capturas reais e revisar visualmente as imagens existentes, incluindo o protótipo destinado ao README
- [x] Revisar visualmente as capturas finais do README quando forem produzidas no ticket 23; o protótipo não substitui capturas da versão distribuída
- [x] Registrar escopo, método, limitações e pendências em `.scratch/kvojps-monitor/validation/40/README.md`, usando apenas caminhos e categorias, nunca reproduzindo segredos ou conteúdo privado
- [x] Se houver exposição, registrar a correção necessária e bloquear a abertura até a resolução; não reescrever histórico nem revogar credenciais sem autorização específica
- [x] Inventariar a presença e as ausências de `LICENSE` MIT, README, documentação de segurança e guia para compilar/fazer fork
- [x] Registrar a decisão do mantenedor de não acionar o Support; manter explícito que o expurgo dos objetos antigos no servidor não foi comprovado
- [x] Conferir os documentos finais entregues pelos tickets 22, 23, 41 e 42, incluindo links e orientações de instalação, privacidade, segurança e forks
- [x] Registrar a revisão final depois que documentos e capturas estiverem prontos; não alterar a visibilidade do repositório como parte deste ticket

## Comments

Estado atual: revisão e limpeza históricas concluídas; revisão final dos documentos e da captura do README registrada em `validation/40/README.md`. Não há solicitação de Support a enviar nem retorno de suporte aguardado; o acesso residual por SHA permanece documentado como limitação conhecida.

- 2026-11-02 — Revisão final concluída após os tickets 22, 23, 41 e 42. Conferidos `LICENSE`, `README.md`, `SECURITY.md`, `docs/desenvolvimento.md`, links relativos e orientações de instalação, privacidade, segurança e forks; captura final do README inspecionada visualmente e identificada como demonstração sintética, não como captura da versão distribuída. Registro e limitações em [validation/40/README.md](../validation/40/README.md). A revisão confirma os documentos locais e a base histórica já verificada; não confirma acesso ao SHA antigo no servidor como removido. Nenhuma alteração de visibilidade foi feita. Status resolvido sem aguardar Support, conforme decisão do mantenedor.

- 2026-11-02 — Aprovado pelo mantenedor na preparação da abertura pública e lançamento da v1. Este ticket não afirma que exista vazamento; formaliza a revisão do material que será publicado. A implementação dos documentos pode ocorrer em paralelo à revisão inicial.

- 2026-11-02 — Revisão inicial registrada em [validation/40/README.md](../validation/40/README.md), com base na `main` em `447c220`. Abertura bloqueada por dados reais em fixtures/histórico, cobertura semântica/visual histórica restante e dependências 22/23/41/42. Escopo previsto: somente `main`, nenhuma tag; inventário remoto ainda pendente. Sem reescrita de histórico, revogação ou alteração de visibilidade. Revisão final permanece aberta; decisões sobre exposição/expurgo exigem mantenedor.

- 2026-11-02 — Mantenedor autorizou sanitizar os dados identificados e removê-los também do histórico Git. Consulta ao GitHub revelou repositório já público, com somente `main` publicada. Execução e limitações registradas em [validation/40/sanitizacao.md](../validation/40/sanitizacao.md). Esta autorização substitui a restrição anterior apenas para o expurgo solicitado; não autoriza publicar branches de protótipo nem mudar visibilidade. Revisão final da v1 continua pendente.

- 2026-11-02 — Sanitização dos dados identificados aplicada também às referências locais e à `main` pública, com verificação independente por clone novo. Reflogs/objetos antigos expurgados localmente; 376 testes UI e 308 Rust aprovados. O GitHub ainda responde a SHA antigo: pendência de expurgo do servidor pelo Support, com texto preparado na validação. Status `needs-info` mantido para esse retorno e para a revisão final da v1; não declarar remoção completa do servidor.

- 2026-11-02 — Mantenedor solicitou registrar prevenção de novas exposições. `AGENTS.md` passa a exigir sanitização antes de versionar e leitura de `docs/agents/sanitizacao.md` para capturas, evidências, commits e pushes. O procedimento cobre identificadores pessoais/de rede, imagens, binários e metadados Git; distingue instrução documental de bloqueio automatizado e risco de privacidade de credencial utilizável.

- 2026-11-02 — Revisão histórica na `main` em `a02568b`: 175 commits, 1.726 blobs e 783 caminhos. Inspeção visual das 88 versões PNG e inspeção por formato do SQLite e das quatro capturas TCP concluídas; revisão textual dirigida às categorias de privacidade e dados de fixtures registrada em [revisao-historica.md](../validation/40/revisao-historica.md), com inventário de binários. Novo bloqueio: duas capturas da validação compacta, removidas da ponta mas alcançáveis no histórico, ainda exibem identificadores de sessão/projeto. Nenhuma reescrita/push executado. Leitura semântica integral dos textos, correção das imagens, suporte e revisão final seguem pendentes; checklist geral e `needs-info` preservados.

- 2026-11-02 — Revisão semântica ampliada aos 1.633 blobs textuais da mesma base, com leitura por conteúdo distinto, cruzamento de origem e inventário por blob/caminhos em [revisao-textual.md](../validation/40/revisao-textual.md). Novo achado textual: contagens e horários de atividade de repositório privado associados à conta do mantenedor na pesquisa do GitHub, com ligação preservada na fixture e exemplos derivados. Consolidada a correção de seis caminhos textuais e das duas imagens históricas; nenhum valor privado reproduzido no relatório. Esta etapa conclui a revisão de privacidade da base com bloqueios, sem sanitização, reescrita, commit ou push. A verificação após correção, o retorno do Support e a revisão final da v1 permanecem pendentes; `needs-info` e checklist geral preservados.

- 2026-11-02 — Mantenedor autorizou executar a limpeza consolidada e atualizar a `main` pública. Segunda sanitização concluída em [segunda-sanitizacao.md](../validation/40/segunda-sanitizacao.md): exemplos privados substituídos, metadados de branch dispensáveis omitidos da fixture e duas imagens expurgadas em todas as referências locais. Reescritos 19 blobs textuais; ponta testada preservada. Testes: 36 Rust do GitHub e 98 UI; Svelte sem erros/avisos. `main` pública atualizada com lease; clone novo confirmou somente `main`, sem blobs originais afetados, horários privados identificados ou imagens expurgadas. Expurgo físico local confirmado. API ainda responde ao SHA da base anterior: pendência de servidor mantida, com complemento privado preparado para o Support, sem envio. `needs-info` permanece pelo expurgo do servidor e documentos/capturas finais; não houve mudança de visibilidade.

- 2026-11-02 — Mantenedor informou que não enviará solicitação ao Support. Não aguardar esse envio ou um retorno para prosseguir com a preparação da v1; preservar a limitação confirmada de acesso residual por SHA, sem afirmar expurgo completo do servidor. Checklist ajustada para marcar revisão, distinção de origem, inspeção das imagens existentes, correção e verificação histórica como concluídas. Documentos e capturas finais continuam pendentes; a conferência inicial de ausências não foi confundida com a entrega desses documentos.
