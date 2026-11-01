# 12: Alinhamento da distribuição vertical dos Módulos

Type: prototype
Status: resolved
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

## Answer

Aprovada pelo usuário em 2026-11-01: **variante A, instrumentos que crescem** (“Instrumentos que crescem tá perfeito”), a mesma regra do [Preenchimento vertical](11-ai-agents-vertical-fill.md) estendida aos cartões de apoio.

- A grade continua com `min-height: 100%`: as duas linhas vão até o fim do Painel, sem faixa vazia no fim da tela.
- Cada cartão de apoio passa a sobra a um instrumento, sem vãos soltos: GitHub, barras de atividade (mín. 26 px); Rede, gráfico de tráfego (mín. 40 px); Trello, linhas das listas (alturas iguais, número e traço no pé de cada linha); Sistema, medidores RAM/Swap/Disco repartem a altura. Sem altura máxima.
- Com os instrumentos tomando a sobra, o último bloco de cada cartão (repo em destaque, aplicativos, “Abrir quadro completo”, bateria) fica no fim do cartão, e os cabeçalhos ficam alinhados nas duas linhas da grade.
- Ajuste pedido na mesma conferência: o número dentro dos anéis (IA e Agentes) cresce mais devagar. `calc(33px + (100cqi - 94px) / 4)`: o tamanho do B em 94 px, ≈54 px em 180 px (antes 35cqi, ≈63 px).
- Linhas condicionais só encolhem o instrumento do cartão; não abrem vão.
- Conferido no app nativo com as fixtures do B: sem rolagem em 1100 × 700 e 1000 × 700, temas escuro e claro.
- Protótipo (atual, A instrumentos, B rodapé ancorado, C espaço repartido, D grade na altura do conteúdo, com opção de linha condicional simulada) no branch local `prototype/vertical-distribution-alignment` (commit `06ebd36`), não enviado.

### Ajustes pedidos depois da conferência (2026-11-01)

- Número dos anéis menor ainda: `calc(31px + (100cqi - 94px) / 6)`, ≈40 px num anel de 150 px, ≈45 px em 180 px.
- **Divergência aprovada do B** (pedido do usuário: “quando não tiver aquela notificação [...] deixar o espaço [...] tipo Tudo certo por aqui”): sem sessão aguardando, o espaço do aviso fica com uma caixa neutra “✓ Nenhuma sessão aguardando você · Ver sessões ↗” (borda e texto discretos). É a opção 2 do [Preenchimento vertical](11-ai-agents-vertical-fill.md), antes descartada. Texto factual em vez de “Tudo certo”, porque pode haver sessão Com falha. Só aparece quando a leitura sustenta a afirmação: some sem leitura, com falha da coleta ou com aguardando indisponível no Codex.
