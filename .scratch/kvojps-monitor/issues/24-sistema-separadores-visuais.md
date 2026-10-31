# 24: Separadores do Módulo Sistema iguais aos do Módulo Rede

**What to build:** aplicar às seções do Módulo Sistema os mesmos separadores visuais já usados no Módulo Rede, para manter a consistência visual do Painel. Melhoria solicitada pelo usuário após a verificação visual do ticket 04.

**Blocked by:** 03 (Módulo Sistema ao vivo), 04 (Qualidade da rede e tráfego total) — ambos concluídos

**Status:** ready-for-agent

- [ ] Usar nas seções de CPU, RAM, Discos e Bateria o mesmo padrão de separação do Módulo Rede: linha inferior de `1px solid var(--line)` e espaçamento vertical equivalente (`padding: 12px 0`)
- [ ] Ajustar o espaçamento atual do Sistema para evitar somar o `gap` existente ao espaçamento dos separadores
- [ ] Manter a bateria oculta quando ausente, sem deixar seção ou separador vazio
- [ ] Preservar os gráficos, barras, textos e regras de atenção existentes
- [ ] Verificar visualmente a consistência entre Rede e Sistema nos temas claro e escuro e executar a checagem de tipos e os testes de UI existentes

## Referências

- Referência visual: `src/lib/rede/ModuloRede.svelte`, estilo de `section`.
- Alvo da melhoria: `src/lib/sistema/ModuloSistema.svelte`.
- Origem do Módulo Sistema: [ticket 03](03-modulo-sistema.md).
- Origem do padrão visual da Rede: [ticket 04](04-rede-qualidade-e-trafego.md).
