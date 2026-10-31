"""Capture anonymized Trello responses used by the adapter tests.

Reads the key and token from the terminal (never from arguments or files) and
sends them only in the Authorization header. Names, titles, IDs and URLs are
replaced; dates, label colors, ordering and structure come from the API.
"""
import getpass
import itertools
import json
import pathlib
import sys
import urllib.request

API = "https://api.trello.com/1"
BOARD_LISTS = "/lists?cards=open&card_fields=name,due,dueComplete,labels,shortUrl&fields=name"


def get(path, key, token):
    request = urllib.request.Request(API + path, headers={
        "Authorization": f'OAuth oauth_consumer_key="{key}", oauth_token="{token}"',
    })
    with urllib.request.urlopen(request, timeout=30) as response:
        return json.load(response)


ids = itertools.count(1)
aliases = {}


def alias(value):
    if value not in aliases:
        aliases[value] = f"{next(ids):024x}"
    return aliases[value]


def anonymize_lists(lists):
    for number, column in enumerate(lists, 1):
        column["id"] = alias(column["id"])
        column["name"] = f"Lista {number}"
        for card_number, card in enumerate(column.get("cards", []), 1):
            card["id"] = alias(card["id"])
            card["name"] = f"Card {number}.{card_number}"
            card["shortUrl"] = f"https://trello.com/c/{card['id'][-8:]}"
            for label in card.get("labels", []):
                label["id"] = alias(label["id"])
                if "idBoard" in label:
                    label["idBoard"] = alias(label["idBoard"])
                label["name"] = ""
    return lists


def main():
    key = getpass.getpass("Trello key: ").strip()
    token = getpass.getpass("Trello token: ").strip()
    me = get("/members/me?fields=username,fullName", key, token)
    boards = get("/members/me/boards?filter=open&fields=name", key, token)
    if not boards:
        sys.exit("No open boards to capture.")
    for number, board in enumerate(boards, 1):
        print(f"{number}. {board['name']}")
    choice = boards[int(input("Board to capture: ")) - 1]
    lists = get(f"/boards/{choice['id']}{BOARD_LISTS}", key, token)
    here = pathlib.Path(__file__).parent
    me = {"id": alias(me["id"]), "username": "exemplo", "fullName": "Exemplo"}
    boards = [{"id": alias(b["id"]), "name": f"Quadro {n}"} for n, b in enumerate(boards, 1)]
    for name, value in [("me", me), ("boards", boards), ("lists", anonymize_lists(lists))]:
        (here / f"{name}.json").write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


main()
