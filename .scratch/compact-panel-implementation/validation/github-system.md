# Validação focada de GitHub e Sistema — ticket 04

Registro de 2026-10-31; baseline `8284f272523cb5a5b2a597e98fcfc648987fc3d6`.

O script `github-system-audit.mjs` monta o Painel real na superfície de fixtures
existente, sem coleta, comandos nativos ou credenciais. Execute o Vite em 1420 e
um Chromium descartável em CDP 9222, conforme o [procedimento existente](../../cinematic-panel-implementation/validation/README.md), e rode:

```sh
node .scratch/compact-panel-implementation/validation/github-system-audit.mjs > /tmp/compact-github-system-results.json
```

Resultados em [github-system-results.json](github-system-results.json): 57 cenários
nos temas escuro/claro/sistema, todas as fixtures de GitHub e Sistema, larguras de
1100/1000/650 px e texto a 125%. Relógio fixado no instante das fixtures GitHub.
Tema sistema usa a preferência da webview; não comprova alternância pelo portal.

- Nenhum overflow horizontal nos contêineres medidos; menor contraste textual
  de 5,39:1. A varredura exclui gráficos/SVG, controles desabilitados e conteúdo
  oculto; não constitui auditoria completa WCAG.
- Descrições de ações visíveis por foco e ponteiro, alcançáveis pelo ponteiro e
  dispensáveis por Escape. Teste integrado confirma que o primeiro Escape mantém
  o Painel e o foco; o próximo fecha o Painel.
- Os instrumentos de GitHub/Sistema permanecem estáticos nos modos de movimento
  completo, reduzido e decoração pausada. Nenhuma animação de dados foi adicionada.
- Fixture padrão em 1100 × 700: GitHub com 320 px de altura; Sistema com cerca de
  270 px. O mapa fica junto ao total anual; barras de 14 dias têm 26 px.
- Capturas: GitHub [escuro](github-system-dark.png)/[claro](github-system-light.png)
  e Sistema [escuro](system-dark.png)/[claro](system-light.png).
- Testes existentes continuam cobrindo resumo acessível do mapa, datas/valores
  das barras, métricas dos meters, cache, recuperação e estados sem dados,
  ausência de atividade, credencial ausente/inválida, offline, antigo e falha.
  Sistema conserva as ausências e os alertas textuais de sensor/swap/bateria/disco.

Verificação final: 224 testes UI, 271 testes unitários Rust e 3 testes de integração
de hooks aprovados; check sem erros/avisos e build aprovado. Os testes de hooks
foram repetidos fora do sandbox porque ele bloqueia sockets Unix.

Revisão pela skill code-review: Standards sem violações documentadas, uma sugestão
não bloqueante de extrair as descrições de ações compartilhadas com IA; Spec sem
achados. Mantida a implementação local de descrições já usada em IA para esta
entrega; a extração compartilhada pode ser feita como manutenção separada.

A comparação integrada dos seis resumos, contraste gráfico completo e GNOME/Wayland
permanecem no ticket 07. Os demais Módulos ainda têm alterações pendentes nos tickets
seguintes; estas capturas não comprovam ausência de rolagem do Painel completo.
O estado de falha do Sistema continua representado por leitura antiga, conforme
[a decisão anterior aceita](../../cinematic-panel-implementation/issues/07-system-instruments.md).
