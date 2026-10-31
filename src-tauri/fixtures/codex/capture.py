"""Record codex app-server replies used by the Codex limits adapter.

Run from the repository root: python3 -I src-tauri/fixtures/codex/capture.py
Account, machine and installation identifiers are replaced before writing.
"""
import json
import pathlib
import select
import subprocess
import time

OUT = pathlib.Path(__file__).parent / "app-server.jsonl"
SENSITIVE = {
    "email": "exemplo@example.com",
    "accountId": "conta-exemplo",
    "chatgptAccountId": "00000000-0000-0000-0000-000000000001",
    "workspaceName": "Espaço de exemplo",
    "codexHome": "/home/exemplo/.codex",
    "serverName": "exemplo",
    "installationId": "00000000-0000-0000-0000-000000000002",
    "id": None,
}


def fake(key):
    return SENSITIVE[key] or "id-exemplo"


def scrub(value):
    if isinstance(value, dict):
        # JSON-RPC ids are integers and stay; string ids (e.g. reset credits) are replaced.
        return {k: (fake(k) if k in SENSITIVE and isinstance(v, str) else scrub(v)) for k, v in value.items()}
    if isinstance(value, list):
        return [scrub(v) for v in value]
    return value


process = subprocess.Popen(["codex", "app-server"], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=subprocess.DEVNULL, text=True, bufsize=1)
requests = [
    {"id": 1, "method": "initialize", "params": {"clientInfo": {"name": "kvojps-monitor", "version": "0.1.0"}}},
    {"method": "initialized"},
    {"id": 2, "method": "account/read", "params": {}},
    {"id": 3, "method": "account/rateLimits/read"},
]
lines = []
for request in requests:
    process.stdin.write(json.dumps(request) + "\n")
deadline = time.time() + 15
while time.time() < deadline:
    ready, _, _ = select.select([process.stdout], [], [], 0.5)
    if ready:
        line = process.stdout.readline()
        if not line:
            break
        lines.append(scrub(json.loads(line)))
process.terminate()
OUT.write_text("".join(json.dumps(line, ensure_ascii=False) + "\n" for line in lines))
print(f"{len(lines)} messages written to {OUT}")
