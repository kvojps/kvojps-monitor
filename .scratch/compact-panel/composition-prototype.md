# Protótipo de distribuição do Painel compacto

Status: aguardando avaliação visual do usuário. Nenhuma composição aprovada.
Origem: [Validar a distribuição compacta do Painel](issues/01-compact-composition.md).
Branch local: `prototype/compact-panel-composition`, sem push.

## Executar e comparar

Execute `npm run prototype:compact` e abra <http://127.0.0.1:1421/compact-prototype.html?variant=A>.

- **A — Ponte horizontal:** IA/Agentes ocupam dois terços da primeira fileira; GitHub completa a fileira; Sistema, Trello e Rede ficam abaixo.
- **B — Núcleo lateral:** IA/Agentes ocupam a coluna esquerda inteira; GitHub/Rede acima e Trello/Sistema abaixo. Recomendação inicial para avaliação.
- **C — Faixa de instrumentos:** IA/Agentes atravessam o topo; GitHub, Trello e Rede ocupam o meio; Sistema vira faixa inferior.

Setas do controle flutuante e teclas ←/→ alternam as composições e atualizam `?variant=`. O controle oferece tema claro, redução de movimento, texto a 125% e nomes longos. O rodapé identifica dimensões, composição e condições ativas. `?theme=light&motion=reduced` permite abrir diretamente essas preferências.

Código descartável em `src/lib/compact-prototype/`; entrada HTML própria no mesmo servidor Vite. A entrada e os componentes de produção não foram alterados. Não há core Tauri, coleta, persistência, credenciais ou operações externas. Atualizações e configurações são simuladas.

As fixtures existentes são importadas diretamente de IA (`claude-usage`), Agentes (`states`), GitHub/Trello/Rede (`full`) e Sistema (`notebook`). Elas representam momentos independentes; não são uma leitura atual coerente. O teste de nomes longos substitui somente textos de demonstração no GitHub e Trello.

## Evidências de navegador

Medição automatizada no Chromium headless em 2026-10-31, em pixels CSS, escala de dispositivo 1. Capturas e retângulos completos em [assets](assets/compact-measurements.json). Altura total inclui o rodapé e a reserva para os controles do laboratório; nenhum Módulo tem altura travada ou conteúdo cortado por overflow.

| Área útil / texto | A: altura total | B: altura total | C: altura total |
| --- | ---: | ---: | ---: |
| 1100 × 700 / 100% | 700 | 700 | 700 |
| 1000 × 700 / 100% | 700 | 700 | 700 |
| 1100 × 650 / 100% | 700 | 686 | 688 |
| 1100 × 700 / 125% | 756 | 772 | 764 |
| 850 × 700 / 100% | 1049 | 1086 | 1024 |

Em 1100 × 700, os resumos terminam aproximadamente em y=629 (A), y=615 (B) e y=617 (C); a barra do laboratório começa em y=646. Não foi detectada rolagem horizontal nos casos medidos. O alvo sem rolagem foi observado com as fixtures e texto padrão entre as duas larguras medidas, 1000 e 1100, com 700 px de altura; isso não certifica todas as dimensões intermediárias ou conteúdo possível.

Com nomes longos, tema claro e movimento reduzido em 1100 × 700, a altura total foi 711 em A/B e 700 em C. Os resumos terminam em y=640 em A/B, mas a reserva dos controles do laboratório requer 11 px adicionais. Não esconder esse excesso nem apresentar o resultado como sucesso sem rolagem.

Proposta para áreas menores, texto ampliado ou conteúdo maior: permitir crescimento e rolagem da superfície; abaixo de 950 px reorganizar em duas colunas, e até 650 px em uma. Não reduzir automaticamente a fonte nem cortar alertas. O controle de 125% amplia a tipografia do conteúdo; não equivale a testar todas as configurações de escala do GNOME/WebKit.

- [A — Ponte horizontal](assets/compact-A.png)
- [B — Núcleo lateral](assets/compact-B.png)
- [C — Faixa de instrumentos](assets/compact-C.png)
- [B — tema claro e nomes longos](assets/compact-B-light-long.png)

Verificadas abertura do diálogo de sessões e saída por Escape nas três composições; redução manual remove animações dos anéis. Dicas dos controles e estados aparecem com ponteiro e foco. Os detalhes são ilustrações; posição, navegação, estados vazios/falha e recuperação serão avaliados em [Validar detalhes sob demanda e estados do Painel compacto](issues/02-details-and-states.md).

`npm run check`: nenhum erro ou aviso. Não foram aferidos contraste completo, CPU, fps, integração nativa, posicionamento ou dimensionamento GNOME/Wayland. O movimento usa anéis e varredura simplificados: fidelidade integral à Coreografia combinada ainda deve ser avaliada na especificação final. O agrupamento visual não redefine Estação nem funde os Módulos IA e Agentes.

## Avaliação pendente

Escolher uma composição ou combinação, avaliar legibilidade dos instrumentos e aceitar ou ajustar o fallback com rolagem quando o conteúdo não couber. A escolha exige resposta do usuário; o ticket permanece reivindicado e aberto.
