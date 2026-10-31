# 10: Validar composição, acessibilidade e integração GNOME/Wayland

**What to build:** entregar a composição integrada fiel à referência e utilizável nos temas, dimensões e estados acordados, com evidência de integração nativa e correções dos desvios encontrados.

**Blocked by:** 09 — Coreografia combinada com pausa nativa.

**Status:** resolved

- [ ] Comparar lado a lado com H/Coreografia combinada em escuro/claro/sistema, três colunas, largura intermediária e 560 px, incluindo escala de texto/tela. Registrar condições e corrigir cortes, sobreposições e perdas de campos/ações.
- [ ] Medir contraste WCAG AA: 4,5:1 para texto comum e 3:1 para texto grande/controles necessários à identificação; ajustar tokens onde necessário. Cor da estação não representa estado da fonte e informação indispensável não depende de microtexto.
- [ ] Verificar teclado, foco visível, ordem de leitura das estações, nomes/valores acessíveis, resumo textual dos gráficos, rótulos das séries e equivalência foco/ponteiro dos metadados. Ícones decorativos ficam fora da árvore acessível.
- [x] Executar matriz de snapshots por Módulo e suas recuperações: cheio, vazio, antigo, credencial inválida e erro quando aplicáveis; estados específicos de IA, cinco Estados da sessão, Sistema sem componentes e Rede com VPN/desconexão/alertas/testes/apps.
- [x] Confirmar independência das fontes, cache com idade e offline sem faixa de erro de rede, além de preservação de foco, seleção e rolagem durante atualizações/listas densas.
- [ ] Verificar manualmente no GNOME/Wayland Indicador, instância única, tamanho/posicionamento, escala, Escape/perda de foco, retorno de Configurações e notificação direcionada. Registrar limitações reais do compositor, sem inferir equivalência a partir do navegador.
- [x] Executar verificações de tipos, lint/formatação, testes do núcleo/UI e build pertinentes ao projeto; conferir testes explícitos de todos os guardrails de privacidade e corrigir regressões. Registrar resultados, sem declarar verificações manuais não realizadas como aprovadas.

## Evidências pendentes de Configurações (ticket 08)

- Capturar sinais `SettingChanged` reais ao alternar o tema no GNOME e substituir a sequência construída de `fixtures/appearance/portal.json`; conferir atualização ao vivo na janela Tauri.
- Quando houver release publicada deste app, gravar resposta positiva de `releases/latest` sem reescrever outro repositório. `fixtures/releases/newer.json` é cenário derivado, não evidência dessa integração.
- Confirmar autostart no login, sincronização após mover AppImage e abertura de logs/link; medir contraste das novas Configurações no tema claro. A cobertura automatizada das portas não substitui essa conferência.

## Comments

2026-10-31 — Validação automatizada e correções entregues. Evidências, condições,
reprodução e limitações no [registro de validação](../validation/README.md).
183 cenários de navegador, 212 testes UI e 274 Rust. Contraste textual mínimo
6,15:1 escuro / 4,89:1 claro na matriz verificada. Integração manual GNOME/Wayland
e comparação visual integral permanecem pendentes; ticket não resolvido e 11
continua bloqueado. A resposta “ok” do usuário não descreveu resultados manuais.

## Answer

2026-10-31 — Resolvido por instrução explícita do usuário: “Marque como resolvido”.
Implementação, correções e evidências em `c5dbbab` e no
[registro de validação](../validation/README.md). O encerramento libera o ticket 11.
As verificações manuais e a comparação visual integral sem evidência permanecem
identificadas no registro; o encerramento não as declara executadas ou aprovadas.
