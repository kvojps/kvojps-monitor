"""Capture kernel metadata for one temporary loopback TCP connection."""
import json
import os
import pathlib
import socket
import struct

output = pathlib.Path(__file__).parent
listener = socket.socket()
listener.bind(("127.0.0.1", 0))
listener.listen()
client = socket.socket()
client.connect(listener.getsockname())
server, _ = listener.accept()
inode = int(os.readlink(f"/proc/self/fd/{client.fileno()}")[8:-1])


def dump_socket():
    with socket.socket(socket.AF_NETLINK, socket.SOCK_RAW, 4) as diagnostic:
        diagnostic.bind((0, 0))
        diagnostic.settimeout(2)
        request = struct.pack("=BBBBI", 2, 6, 2, 0, 0xFFFFFFFF) + bytes(48)
        diagnostic.sendto(struct.pack("=IHHII", 72, 20, 0x301, 1, 0) + request, (0, 0))
        while True:
            response = diagnostic.recv(65536)
            position = 0
            while position + 16 <= len(response):
                length, kind, _, _, _ = struct.unpack_from("=IHHII", response, position)
                if kind == 3:
                    raise RuntimeError("Socket not found in diagnostic dump")
                if kind == 2:
                    raise RuntimeError("Kernel refused diagnostic request")
                if kind == 20 and struct.unpack_from("=I", response, position + 84)[0] == inode:
                    return response[position:position + length]
                position += (length + 3) & ~3


def receive_bytes(connection, count):
    while count:
        chunk = connection.recv(count)
        if not chunk:
            raise RuntimeError("Connection closed before all bytes arrived")
        count -= len(chunk)


try:
    for index in range(4):
        if index:
            server.sendall(b"x" * 4096)
            receive_bytes(client, 4096)
            client.sendall(b"x" * 2048)
            receive_bytes(server, 2048)
        (output / f"{index}.bin").write_bytes(dump_socket())
    (output / "status.txt").write_text(pathlib.Path("/proc/self/status").read_text())
    (output / "comm.txt").write_text(pathlib.Path("/proc/self/comm").read_text())
    owner = {"uid": os.getuid(), "inode": inode,
             "fdLink": os.readlink(f"/proc/self/fd/{client.fileno()}"),
             "exeLink": os.readlink("/proc/self/exe")}
    (output / "owner.json").write_text(json.dumps(owner, indent=2) + "\n")
finally:
    server.close()
    client.close()
    listener.close()
