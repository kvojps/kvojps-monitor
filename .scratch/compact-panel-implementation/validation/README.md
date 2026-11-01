# Validação focada de IA — ticket 02

Registro de 2026-10-31; baseline `c4c18cbd1f453e0c0cf9f91877f0cbd1c3696cb8`.

O script `ai-audit.mjs` usa a superfície de fixtures existente do complemento
cinematográfico, montando o Painel real sem comandos nativos, coleta ou credenciais.
Execute `npm run dev` e inicie um Chromium descartável, conforme o
[procedimento existente](../../cinematic-panel-implementation/validation/README.md).
Com Vite em 1420 e CDP em 9222, execute na raiz:

```sh
node .scratch/compact-panel-implementation/validation/ai-audit.mjs > /tmp/compact-ai-results.json
```

O relógio da UI é fixado na data da fixture `claude-usage` para reprodução.
As fixtures adicionais são combinações dos snapshots reais existentes com seus
estados públicos de pausa, carregamento e ausência/reinício semanal. Não são
evidência de uma coleta nova nem modificam as fontes de produção.

Resultado preservado em [ai-results.json](ai-results.json): 66 cenários nos temas
escuro, claro e sistema, todas as fixtures de IA, larguras de 1100, 1000 e 650 px,
altura de 700 px e texto de IA a 125%. Tema sistema usou a preferência da webview;
a alternância real pelo portal exige validação nativa.

- Nenhum overflow horizontal nos contêineres de IA medidos.
- Contraste textual mínimo medido de 5,39:1. A varredura exclui SVG, controles
  desabilitados e conteúdo oculto; não constitui auditoria completa WCAG.
- Dicas visíveis por foco e ponteiro, alcançáveis pelo ponteiro e dispensáveis
  por Escape em todos os cenários. O teste integrado garante que esse primeiro
  Escape preserva o Painel e o foco; o seguinte fecha o Painel.
- Instrumentos normais de 94 px conforme a referência B; captura
  [escura](ai-dark.png) e [clara](ai-light.png) em 1100 × 700.
- Decoração verificada com movimento reduzido, movimento completo e pausa;
  nenhum instrumento, percentual ou alerta recebe animação de dados.
- Testes da UI verificam nomes, valores e resumos acessíveis dos gráficos,
  comandos, cache, recuperações explícitas e passagem temporal pelo reinício.

Validação final: 216 testes UI, 271 testes unitários Rust e 3 testes de integração
de hooks aprovados; check sem erros/avisos e build aprovado. Os testes de hooks
precisaram executar fora do sandbox devido à restrição a sockets Unix.

Revisão paralela com a skill code-review: Standards 0 achados restantes;
Spec 0. A revisão corrigiu alcance/dismissão das dicas e associação da pausa
por provedor.

A comparação integrada dos seis resumos com B, contraste dos elementos gráficos,
escala/área útil GNOME/Wayland e medições nativas ficam no ticket 07. As capturas
ainda mostram os outros Módulos anteriores aos tickets 03–06; rolagem do Painel
completo nesta etapa é esperada e não foi declarada resolvida.
