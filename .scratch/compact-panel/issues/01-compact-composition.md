# Validar a distribuição compacta do Painel

Type: prototype
Status: claimed
Map: [Painel compacto sem rolagem](../map.md)

## Question

Qual distribuição permite ler os resumos dos seis Módulos em uma janela de aproximadamente 1100 × 700 sem rolagem na visão principal, mantendo a linguagem visual aprovada?

Criar protótipo descartável com fixtures existentes, sem alterar produção. Remover título/subtítulo gerais; experimentar reduzir ou retirar navegação redundante, faixas de Estação e metadados repetidos. Integrar visualmente IA e Agentes, com anel de sessões trabalhando, segmentos por estado, legenda e alerta de espera. Colocar ações como ícones junto aos títulos, acessíveis por teclado. Preservar limites de 5 h/semana, os resumos de Sistema, GitHub, Rede e Trello e acesso a Configurações.

Avaliar tamanho real do conteúdo, equilíbrio dos espaços e legibilidade com o usuário. Verificar que a ausência de scroll resulta da composição, sem simplesmente cortar overflow. Registrar faixa de dimensões viáveis e comportamento proposto quando escala de texto ou área útil impedirem o tamanho alvo. Vincular artefato e evidências; não presumir validação nativa pelo navegador.

## Comments

### 2026-10-31 — Primeira exploração visual

Autorização de revisão registrada no mapa: explorar tamanho, composição, navegação e detalhes do complemento cinematográfico, mantendo regras de domínio e privacidade. A implementação de produção permanece fora deste esforço.

Três alternativas disponíveis no [protótipo de distribuição do Painel compacto](../composition-prototype.md), com capturas e medições. Branch local `prototype/compact-panel-composition`; executar `npm run prototype:compact`.

O alvo 1100 × 700 foi atingido nas três alternativas com fixtures e texto padrão, sem cortar conteúdo. Texto ampliado, nomes longos e área menor têm limites documentados. Aguardando escolha e avaliação visual do usuário; não há resolução ou aprovação presumida.
