# 02: CI na main e em PRs

**What to build:** todo push na `main` e todo PR rodam o CI e falham se a formatação, o lint, os testes do núcleo ou da UI, ou o build quebrarem. Ver spec, "Release e repositório".

**Blocked by:** 01 (Esqueleto: Indicador, Painel e trilho)

**Status:** ready-for-agent

- [ ] Workflow do GitHub Actions disparado em push na `main` e em PRs
- [ ] Passos: `cargo fmt --check`, `clippy` (avisos como erro), `cargo test`, `svelte-check`, testes da UI (Vitest) e build do Tauri sem empacotar
- [ ] Roda em `ubuntu-24.04` com as dependências de sistema do Tauri
- [ ] O workflow é reaproveitável pelo pipeline de release (ticket 22)
