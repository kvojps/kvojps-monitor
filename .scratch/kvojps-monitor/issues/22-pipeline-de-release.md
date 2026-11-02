# 22: Pipeline de release

**What to build:** um push de tag `v*` gera um release em rascunho no GitHub com `.deb`, AppImage, `SHA256SUMS`, attestations e notas automáticas, verificável com `sha256sum -c` e `gh attestation verify`; o CI falha se a tag não bater com a versão do projeto. Ver spec, "Release e repositório" e ticket 18 do mapa.

**Blocked by:** 02 (CI na main e em PRs)

**Status:** done

- [x] Workflow em tag `v*` que reaproveita o CI
- [x] Confere tag ↔ `tauri.conf.json` ↔ `Cargo.toml` ↔ `package.json`; `tauri.conf.json` é a fonte da verdade; versão inicial `0.1.0`
- [x] `tauri-action` em `ubuntu-24.04` gera `.deb` + AppImage
- [x] Publica `SHA256SUMS` e GitHub artifact attestations (sem GPG)
- [x] Release em rascunho com notas automáticas; sem `CHANGELOG.md`
- [x] Licença MIT no repo

## Implementação e validação — 02/11/2026

- `.github/workflows/release.yml` valida a versão antes do CI reutilizável e restringe permissões de escrita ao job de release. O rascunho só é criado após empacotamento, checksums e verificação das attestations.
- `scripts/check-release-version.py`: sete cenários com manifestos sintéticos temporários passaram (versões iguais; tag divergente, sem prefixo ou ausente; divergência em cada manifesto).
- YAML lido com PyYAML; passo real de coleta executado com dois pacotes sintéticos temporários: `sha256sum -c` aceitou ambos e recusou corrupção posterior.
- `npm run check`, `npm test` (376 testes), `cargo fmt --check`, Clippy com `-D warnings` com e sem `fixtures`, testes Rust (305 unitários + 3 de integração) e 15 testes de fixtures passaram. Testes de socket exigiram execução fora do sandbox.
- Build Tauri sem empacotamento passou. Validação do empacotamento local em andamento; emissão OIDC, attestations e rascunho só serão comprovados na primeira execução no GitHub. Nenhuma tag ou release foi criada nesta implementação.
- Falha preexistente confirmada no run CI `41511529589` e reproduzida localmente: método privado `Operations::new(&mut self, ...)` disparava `wrong_self_convention` e `new_ret_no_self`. Renomeado para `record`, preservando comportamento; ambas as verificações Clippy passaram depois.
- Sanitização: revisados workflow, script, licença, documentação, alteração Rust e este ticket por diff e leitura integral dos arquivos novos. Sem capturas reais adicionadas, credenciais ou conteúdo privado; identidade Git noreply conferida. Os cenários temporários são sintéticos.
- Referências primárias consultadas: [inputs da tauri-action v0](https://github.com/tauri-apps/tauri-action/blob/v0/action.yml) e [actions/attest v4](https://github.com/actions/attest). Instruções de uso em `docs/desenvolvimento.md`.
