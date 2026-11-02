"""Capture kernel metadata for one temporary loopback TCP connection."""
import json
import os
import pathlib
import socket
import struct

output = pathlib.Path(__file__).parent


def sanitized_socket(data):
    """Keep recorded counters and loopback addresses; replace host identifiers."""
    result = bytearray(data)
    struct.pack_into("=II", result, 8, 1, 123)
    struct.pack_into("!HH", result, 20, 32000, 32001)
    struct.pack_into("=I", result, 56, 0)
    struct.pack_into("=II", result, 60, 1, 0)
    struct.pack_into("=II", result, 80, 1000, 12345)
    offset = 88
    while offset + 4 <= len(result):
        length, kind = struct.unpack_from("=HH", result, offset)
        if length < 4:
            raise RuntimeError("Malformed diagnostic attribute")
        if kind == 21:  # INET_DIAG_CGROUP_ID identifies a host cgroup.
            result[offset + 4:offset + length] = bytes(length - 4)
        offset += (length + 3) & ~3
    return result

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
        (output / f"{index}.bin").write_bytes(sanitized_socket(dump_socket()))
    (output / "status.txt").write_text(
        "Name:\tpython3\nState:\tR (running)\nTgid:\t123\nPid:\t123\nPPid:\t1\n"
        "Uid:\t1000\t1000\t1000\t1000\nGid:\t1000\t1000\t1000\t1000\nGroups:\t1000\n"
    )
    (output / "comm.txt").write_text("python3\n")
    owner = {"uid": 1000, "inode": 12345,
             "fdLink": "socket:[12345]", "exeLink": "/usr/bin/python3"}
    (output / "owner.json").write_text(json.dumps(owner, indent=2) + "\n")
finally:
    server.close()
    client.close()
    listener.close()
