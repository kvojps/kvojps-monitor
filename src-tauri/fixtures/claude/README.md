# Fixtures do Claude

- `statusline-payload.json`: JSON que o Claude Code entrega no stdin da status line, transcrito do exemplo da documentação oficial (https://code.claude.com/docs/en/statusline, lido em 2026-10-31). Não houve captura real: ela exigiria trocar a status line do usuário ou abrir uma sessão interativa só para isso. Os campos lidos pelo app (`rate_limits.five_hour` e `rate_limits.seven_day`, com `used_percentage` de 0 a 100 e `resets_at` em segundos Unix) seguem a tabela "Available data" da mesma página.
- `credentials.json`: as chaves reais de `~/.claude/.credentials.json` (Claude Code 2.1.316), com valores de exemplo; o app só lê `subscriptionType`.
- `settings.json`: forma do `~/.claude/settings.json` desta máquina, com uma status line e um hook instalados por outro app (Orca), encurtados.
