"""Capture only local IP address metadata from NetworkManager."""
import ipaddress
import json
import pathlib
import re
import subprocess


def property_value(path, contract, field):
    return subprocess.check_output([
        "gdbus", "call", "--system", "--dest", "org.freedesktop.NetworkManager",
        "--object-path", path, "--method", "org.freedesktop.DBus.Properties.Get",
        contract, field,
    ], text=True)


def object_path(value):
    return re.search(r"objectpath '([^']+)'", value).group(1)


primary = object_path(property_value(
    "/org/freedesktop/NetworkManager", "org.freedesktop.NetworkManager", "PrimaryConnection",
))
addresses = []
raw = {}
for version in [4, 6]:
    config = object_path(property_value(
        primary, "org.freedesktop.NetworkManager.Connection.Active", f"Ip{version}Config",
    ))
    if config != "/":
        value = property_value(config, f"org.freedesktop.NetworkManager.IP{version}Config", "AddressData")
        raw[str(version)] = value
        addresses.extend(re.findall(r"'address': <'([^']+)'>", value))
# Preserve the captured shape without publishing the host's addresses.
aliases = {}
counts = {"4": 100, "6": 0, "link": 0}
for address in addresses:
    if address in aliases:
        continue
    parsed = ipaddress.ip_address(address)
    category = "link" if parsed.version == 6 and parsed.is_link_local else str(parsed.version)
    counts[category] += 1
    aliases[address] = (
        f"192.0.2.{counts[category]}" if category == "4"
        else f"fe80::{counts[category]}" if category == "link"
        else f"2001:db8::{counts[category]}"
    )
for version, value in raw.items():
    for address in sorted(aliases, key=len, reverse=True):
        value = value.replace(f"'{address}'", f"'{aliases[address]}'")
    raw[version] = value
addresses = [aliases[address] for address in addresses]
pathlib.Path(__file__).with_name("address-data.json").write_text(
    json.dumps({"addresses": addresses, "raw": raw}, indent=2) + "\n",
)
