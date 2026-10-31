"""Capture the oauth/usage response used by the opt-in adapter tests.

Reads the Claude Code token from ~/.claude/.credentials.json into memory only,
sends it only in the Authorization header and never refreshes it. The response
carries usage percentages and reset times, not account data; it is written as is.
"""
import json
import pathlib
import sys
import urllib.error
import urllib.request

credentials = json.loads((pathlib.Path.home() / ".claude/.credentials.json").read_text())
token = credentials["claudeAiOauth"]["accessToken"]
request = urllib.request.Request("https://api.anthropic.com/api/oauth/usage", headers={
    "Authorization": f"Bearer {token}",
    "anthropic-beta": "oauth-2025-04-20",
    "User-Agent": "kvojps-monitor",
})
try:
    with urllib.request.urlopen(request, timeout=30) as response:
        body = json.load(response)
except urllib.error.HTTPError as error:
    sys.exit(f"HTTP {error.code}: open Claude Code to renew the token, then run again.")
target = pathlib.Path(__file__).parent / "oauth-usage.json"
target.write_text(json.dumps(body, indent=2, ensure_ascii=False) + "\n")
print(f"Wrote {target}")
