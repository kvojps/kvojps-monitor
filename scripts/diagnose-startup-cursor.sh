#!/usr/bin/env bash
# Human-in-the-loop check: GNOME's pointer is outside the app's webview.
# Record only yes/no observations; never capture the desktop or user data.
set -euo pipefail

printf '%s\n' 'Abra o Painel pelo Indicador e mova o mouse sobre ele.'
read -r -p 'O Painel responde a cliques? [s/n] ' responsive
read -r -p 'O cursor continua em espera depois que o Painel responde? [s/n] ' busy

case "$responsive:$busy" in
  s:s)
    printf '%s\n' 'FAIL: panel responsive, startup cursor still busy'
    exit 1
    ;;
  s:n)
    printf '%s\n' 'PASS: panel responsive, no residual startup cursor'
    ;;
  n:s|n:n)
    printf '%s\n' 'INCONCLUSIVE: panel not responsive'
    exit 2
    ;;
  *)
    printf '%s\n' 'INCONCLUSIVE: expected s or n'
    exit 2
    ;;
esac
