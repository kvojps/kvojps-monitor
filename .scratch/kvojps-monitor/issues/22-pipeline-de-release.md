# 22: Pipeline de release

**What to build:** um push de tag `v*` gera um release em rascunho no GitHub com `.deb`, AppImage, `SHA256SUMS`, attestations e notas automáticas, verificável com `sha256sum -c` e `gh attestation verify`; o CI falha se a tag não bater com a versão do projeto. Ver spec, "Release e repositório" e ticket 18 do mapa.

**Blocked by:** 02 (CI na main e em PRs)

**Status:** ready-for-agent

- [ ] Workflow em tag `v*` que reaproveita o CI
- [ ] Confere tag ↔ `tauri.conf.json` ↔ `Cargo.toml` ↔ `package.json`; `tauri.conf.json` é a fonte da verdade; versão inicial `0.1.0`
- [ ] `tauri-action` em `ubuntu-24.04` gera `.deb` + AppImage
- [ ] Publica `SHA256SUMS` e GitHub artifact attestations (sem GPG)
- [ ] Release em rascunho com notas automáticas; sem `CHANGELOG.md`
- [ ] Licença MIT no repo
