# 38: Alinhar barras semanais dos Provedores de IA

**What to build:** corrigir o desalinhamento vertical das barras semanais de Claude e Codex no Módulo IA quando apenas um Provedor de IA está sem leitura nova. Os medidores semanais devem ocupar a mesma altura, preservando as explicações de ausência de leitura.

**Blocked by:** nenhum.

**Status:** ready-for-agent

## Problema observado

Quando um provedor exibe uma mensagem de ausência de leitura e o outro tem leitura disponível, as barras semanais deixam de ficar alinhadas. Em `src/lib/ai/AiModule.svelte`, uma linha condicional para ausência de leitura da janela de 5 h precede o medidor semanal em colunas independentes; esse é um ponto inicial de investigação, não uma causa confirmada por reprodução visual.

## Critérios de aceite

- [ ] Manter os medidores semanais na mesma altura quando ambos têm leitura e quando apenas um está sem leitura nova, alternando Claude e Codex.
- [ ] Cobrir ausência de leitura na janela de 5 h e na semanal, incluindo janela reiniciada e nenhuma leitura anterior; preservar mensagens explicativas e ações existentes.
- [ ] Preservar a distinção entre leitura antiga com janela válida e janela reiniciada: após reinício sem leitura nova, não exibir barra de consumo nem assumir 0%; o espaço do medidor pode ser mantido para alinhar a composição.
- [ ] Verificar temas claro e escuro, largura reduzida e texto ampliado, sem sobreposição, corte das explicações ou transbordamento.
- [ ] Validar com fixtures de snapshots da UI e inspeção visual dos estados assimétricos; registrar evidências do alinhamento, pois testes de conteúdo isolados não comprovam geometria.

## Referências

- [Spec — dados desatualizados e visual do Módulo IA](../spec.md).
- [Limites do Codex](14-ia-limites-codex.md), [status line do Claude](15-ia-claude-status-line.md) e [usage do Claude](16-ia-claude-oauth-usage.md).
- Implementação: `src/lib/ai/AiModule.svelte`; fixtures em `src/lib/ai/fixtures/`.

## Comments

### 02/11/2026 — escopo aprovado

O usuário confirmou o alinhamento das barras semanais mesmo com apenas um provedor sem leitura nova, preservando mensagens e sem representar dado desconhecido como 0%. Este ticket registra a correção; não altera as regras de coleta ou validade das leituras.
