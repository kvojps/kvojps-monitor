# Portal de aparência

`colorScheme` foi capturado nesta máquina em 2026-10-31 (GNOME, estilo escuro) com:

```sh
gdbus call --session --dest org.freedesktop.portal.Desktop \
  --object-path /org/freedesktop/portal/desktop \
  --method org.freedesktop.portal.Settings.ReadOne org.freedesktop.appearance color-scheme
```

Saída: `(<uint32 1>,)` (1 = preferir escuro). `changedSequence` é uma variação
controlada dos valores documentados (0 = sem preferência, 1 = escuro, 2 = claro),
representando os corpos `(ssv)` do sinal `SettingChanged`. O tema do sistema não
foi alternado para capturar sinais reais; o teste constrói mensagens D-Bus com
esses corpos e exercita a mesma desserialização do adapter.
