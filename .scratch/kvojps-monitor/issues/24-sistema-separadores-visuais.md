# 24: Separadores do Módulo Sistema iguais aos do Módulo Rede

**What to build:** aplicar às seções do Módulo Sistema os mesmos separadores visuais já usados no Módulo Rede, para manter a consistência visual do Painel. Melhoria solicitada pelo usuário após a verificação visual do ticket 04.

**Blocked by:** 03 (Módulo Sistema ao vivo), 04 (Qualidade da rede e tráfego total) — ambos concluídos

**Status:** ready-for-human

- [x] Usar nas seções de CPU, RAM, Discos e Bateria o mesmo padrão de separação do Módulo Rede: linha inferior de `1px solid var(--line)` e espaçamento vertical equivalente (`padding: 12px 0`)
- [x] Ajustar o espaçamento atual do Sistema para evitar somar o `gap` existente ao espaçamento dos separadores
- [x] Manter a bateria oculta quando ausente, sem deixar seção ou separador vazio
- [x] Preservar os gráficos, barras, textos e regras de atenção existentes
- [ ] Verificar visualmente a consistência entre Rede e Sistema nos temas claro e escuro e executar a checagem de tipos e os testes de UI existentes

## Referências

- Referência visual: `src/lib/rede/ModuloRede.svelte`, estilo de `section`.
- Alvo da melhoria: `src/lib/sistema/ModuloSistema.svelte`.
- Origem do Módulo Sistema: [ticket 03](03-modulo-sistema.md).
- Origem do padrão visual da Rede: [ticket 04](04-rede-qualidade-e-trafego.md).

## Notas da implementação

- Alteração restrita ao CSS: remoção do `gap` entre seções e aplicação do padding e da borda usados na Rede. O espaçamento interno e a renderização condicional da bateria foram preservados.
- `npm run check`: sem erros ou avisos; 8 testes do Sistema e suíte completa de UI (20 testes) passaram; build de produção passou.
- Pendente: conferência visual no Painel nos temas claro e escuro. Esta sessão não dispõe de ferramenta de navegador; o token `--line` já tem valores específicos para ambos os temas em `src/tokens.css`.
