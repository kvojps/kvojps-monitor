# logind

`preparingForSleep` foi capturado nesta máquina em 2026-10-31 com:

```sh
busctl --system get-property org.freedesktop.login1 /org/freedesktop/login1 org.freedesktop.login1.Manager PreparingForSleep
```

Saída: `b false`. `sleepResumeSequence` é uma variação controlada dessa leitura
booleana, representando os corpos `(b)` dos sinais `PrepareForSleep`. Não foi
suspensa a máquina para capturar sinais reais. O teste constrói mensagens D-Bus
com esses corpos e exercita a mesma desserialização usada pelo adapter.
