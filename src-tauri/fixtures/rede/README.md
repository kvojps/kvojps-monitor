# Fixtures de Rede

`conectada.json` contém os campos usados da leitura real do NetworkManager via D-Bus de sistema nesta máquina em 30/10/2026: `PrimaryConnection`, `Connectivity`, `Type`, `Gateway` e `Interface`. A segunda conexão ativa era `loopback`, sem VPN. Não são coletados SSID, nome do perfil nem credenciais.

`proc/net/dev` foi gravado na mesma máquina. Os testes derivam incrementos, resets e interfaces virtuais dessa amostra para exercitar os deltas sem esperar tempo real. Os tempos de sonda e suas perdas são controlados nos testes; não são benchmarks da conexão desta máquina.

Os snapshots em `src/lib/rede/fixtures/` são gerados pelo núcleo com essas portas injetadas. VPN, cabo e desconexão são variações controladas da leitura real. Para regravá-los: `ATUALIZAR_FIXTURES=1 cargo test --manifest-path src-tauri/Cargo.toml rede::testes`.
