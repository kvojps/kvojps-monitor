# Respostas TCP gravadas

`0.bin` a `3.bin` são mensagens `SOCK_DIAG_BY_FAMILY` capturadas nesta máquina
(Ubuntu/Linux, little endian), sem privilégios, em uma conexão IPv4 de loopback.
Após a primeira captura, cada amostra acrescenta 4096 bytes recebidos e 2048
enviados. Os arquivos contêm cabeçalho netlink, `inet_diag_msg` e atributos reais,
incluindo `INET_DIAG_INFO` (`tcp_info`). Nenhum conteúdo de aplicação é capturado.
`owner.json` registra UID, inode, destino do FD e do executável. `status.txt` e
`comm.txt` são cópias reais de `/proc/self` do processo de captura. Os testes
derivam nomes de apps, UIDs, PIDs e FDs dessa base gravada.

Para recapturar, execute `python3 src-tauri/fixtures/network/tcp/capture.py` na raiz.
Só usa uma conexão local temporária. O kernel pode alterar campos voláteis.

Nos testes, cópias em memória recebem o endereço local gravado via NetworkManager, destino de documentação, UIDs, inodes e cookies
controlados para representar conexões externas e múltiplos proprietários. O
payload `tcp_info` permanece gravado, exceto nos casos explícitos de reset.
A janela é avançada com relógio falso; não há esperas reais.

Formato de referência: [sock_diag(7)](https://man7.org/linux/man-pages/man7/sock_diag.7.html)
e os cabeçalhos Linux `inet_diag.h`, `tcp.h` e `netlink.h`.
