"""Reject release tags that disagree with the project's version manifests."""

import json
from pathlib import Path
import sys
import tomllib


root = Path(__file__).resolve().parent.parent
version = json.loads((root / "src-tauri/tauri.conf.json").read_text())["version"]
versions = {
    "tag": sys.argv[1] if len(sys.argv) == 2 else None,
    "package.json": json.loads((root / "package.json").read_text())["version"],
    "Cargo.toml": tomllib.loads((root / "src-tauri/Cargo.toml").read_text())["package"]["version"],
}
for source, actual in versions.items():
    expected = f"v{version}" if source == "tag" else version
    if actual != expected:
        sys.exit(f"Release version mismatch in {source}: expected {expected!r}, got {actual!r}")
print(f"Release version verified: v{version}")
