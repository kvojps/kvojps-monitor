import { fireEvent, render, screen, within } from "@testing-library/svelte";
import { expect, it, vi } from "vitest";
import TrelloSettings from "./TrelloSettings.svelte";
import type { TrelloData } from "../core";
import emptyJson from "./fixtures/empty.json";
import chooseJson from "./fixtures/choose-board.json";
import fullJson from "./fixtures/full.json";

function props(data: unknown, overrides = {}) {
  return {
    data: data as TrelloData,
    connect: vi.fn().mockResolvedValue("exemplo"),
    selectBoard: vi.fn().mockResolvedValue(undefined),
    disconnect: vi.fn().mockResolvedValue(undefined),
    openAdmin: vi.fn(),
    authorize: vi.fn(),
    ...overrides,
  };
}

async function fill(key: string, token: string) {
  await fireEvent.input(screen.getByLabelText("API key"), { target: { value: key } });
  await fireEvent.input(screen.getByLabelText("Token"), { target: { value: token } });
}

it("generates the token with the pasted key", async () => {
  const p = props(emptyJson);
  render(TrelloSettings, p);
  const generate = screen.getByRole("button", { name: "Gerar token" });
  expect(generate).toBeDisabled();
  await fireEvent.input(screen.getByLabelText("API key"), { target: { value: "0123abcd" } });
  await fireEvent.click(generate);
  expect(p.authorize).toHaveBeenCalledWith("0123abcd");
  await fireEvent.click(screen.getByRole("button", { name: "Abrir trello.com/apps/admin" }));
  expect(p.openAdmin).toHaveBeenCalledOnce();
});

it("validates key and token, then clears the secret", async () => {
  const p = props(emptyJson);
  render(TrelloSettings, p);
  expect(screen.getByLabelText("Token")).toHaveAttribute("type", "password");
  await fill("0123abcd", "private-token");
  await fireEvent.click(screen.getByRole("button", { name: "Validar e salvar" }));
  expect(p.connect).toHaveBeenCalledWith("0123abcd", "private-token");
  expect(await screen.findByText("Conectado como exemplo")).toBeInTheDocument();
  expect(screen.getByLabelText("Token")).toHaveValue("");
});

it.each([
  ["invalidCredential", "Key ou token inválido"],
  ["offline", "Sem conexão, tente de novo"],
  ["keyringUnavailable", "GNOME Keyring indisponível. Ative ou desbloqueie o chaveiro para salvar; a key e o token não foram salvos."],
])("shows %s beside the fields", async (error, message) => {
  render(TrelloSettings, props(emptyJson, { connect: vi.fn().mockRejectedValue(error) }));
  await fill("0123abcd", "private-token");
  await fireEvent.click(screen.getByRole("button", { name: "Validar e salvar" }));
  expect(await screen.findByRole("alert")).toHaveTextContent(message);
  expect(screen.getByLabelText("Token")).toHaveAccessibleDescription(message);
  expect(screen.queryByText(/Conectado como/)).not.toBeInTheDocument();
});

it("chooses among the open boards without a native popup", async () => {
  // A <select> opens a GTK popup that takes focus and closes the panel under it.
  const p = props(chooseJson);
  const { container } = render(TrelloSettings, p);
  expect(container.querySelector("select")).toBeNull();
  const board = (chooseJson as TrelloData).boards[1];
  const group = screen.getByRole("radiogroup", { name: "Quadro" });
  await fireEvent.click(within(group).getByRole("radio", { name: board.name }));
  expect(p.selectBoard).toHaveBeenCalledWith(board.id);
});

it("shows the chosen board and disconnects", async () => {
  const p = props(fullJson);
  render(TrelloSettings, p);
  expect(screen.getByText("Conectado como exemplo")).toBeInTheDocument();
  const chosen = (fullJson as TrelloData).board!;
  expect(screen.getByRole("radio", { name: chosen.name })).toBeChecked();
  await fireEvent.click(screen.getByRole("button", { name: "Desconectar" }));
  expect(p.disconnect).toHaveBeenCalledOnce();
});

it("hides disconnect and the board selector without an account", () => {
  render(TrelloSettings, props(emptyJson));
  expect(screen.queryByRole("button", { name: "Desconectar" })).not.toBeInTheDocument();
  expect(screen.queryByRole("radiogroup", { name: "Quadro" })).not.toBeInTheDocument();
});

it("explains when the keyring refuses to disconnect", async () => {
  render(TrelloSettings, props(fullJson, { disconnect: vi.fn().mockRejectedValue("keyringUnavailable") }));
  await fireEvent.click(screen.getByRole("button", { name: "Desconectar" }));
  expect(await screen.findByRole("alert")).toHaveTextContent("Não foi possível apagar a key e o token");
});

it("keeps the key typed before generating the token when the panel reopens", async () => {
  render(TrelloSettings, props({ ...emptyJson, pendingKey: "0123abcd" }));
  expect(screen.getByLabelText("API key")).toHaveValue("0123abcd");
  expect(screen.getByRole("button", { name: "Gerar token" })).toBeEnabled();
});
