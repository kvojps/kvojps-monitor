"""Capture only local IP address metadata from NetworkManager."""
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
pathlib.Path(__file__).with_name("address-data.json").write_text(
    json.dumps({"addresses": addresses, "raw": raw}, indent=2) + "\n",
)
