# 12: Alinhamento da distribuição vertical dos Módulos

Type: prototype
Status: open
Blocked by: None (can start immediately).

## Question

Com a grade ocupando toda a altura do Painel, como os elementos de cada cartão (IA e Agentes, GitHub, Trello, Rede, Sistema) se distribuem na vertical para que os blocos fiquem alinhados entre os cartões, sem vãos soltos dentro deles nem no fim da tela?

- Desde 2026-11-01 a grade tem `min-height: 100%`: as duas linhas esticam até o fim do Painel. Antes sobrava uma faixa vazia no fim da tela.
- Efeito colateral: GitHub, Trello, Rede e Sistema ficam mais altos que o conteúdo, e a sobra cai no fim de cada cartão.
- O cartão IA e Agentes repassa a sobra aos anéis ([Preenchimento vertical](11-ai-agents-vertical-fill.md)), entre 94 e 180 px; acima disso, a sobra vira vão.
- A linha condicional de uso alto ([Uso alto no estado B](10-high-usage-in-state-b.md)) e as demais linhas condicionais mudam a altura do conteúdo.

## Comments

Origem: conferência do usuário em 2026-11-01, depois do ticket 11: “apesar de termos melhorado, temos que garantir um alinhamento na distribuição dos elementos verticalmente”. Na mesma conferência, o número dentro dos anéis estava pequeno (fonte em `%` em vez de `cqi`); corrigido.

Pontos a decidir no protótipo:

- Onde cada cartão de apoio põe a sobra (instrumentos que crescem, como na IA; espaço entre blocos; rodapé ancorado no fim; nada).
- Que linhas horizontais se alinham entre cartões vizinhos (cabeçalhos, números grandes, barras, rodapés) nas duas linhas da grade.
- Se a grade deve mesmo esticar até o fim do Painel ou se o vazio no fim da tela é preferível a vãos dentro dos cartões.

Conferir no app nativo (`npm run dev:fixtures`, `scripts/capture-panel.sh`) com as fixtures do B, com e sem aviso de Agentes e linhas condicionais, em 1100 × 700 e 1000 × 700, temas escuro e claro, sem rolagem.
