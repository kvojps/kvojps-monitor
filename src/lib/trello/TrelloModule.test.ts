import { fireEvent, render, screen, within } from "@testing-library/svelte";
import { expect, it, vi } from "vitest";
import TrelloModule from "./TrelloModule.svelte";
import type { TrelloData } from "../core";
import fullJson from "./fixtures/full.json";
import overdueJson from "./fixtures/overdue.json";
import emptyJson from "./fixtures/empty.json";
import chooseJson from "./fixtures/choose-board.json";
import invalidJson from "./fixtures/invalid-credential.json";
import errorJson from "./fixtures/error.json";

const full = fullJson as TrelloData;
const props = (data: unknown) => ({ data: data as TrelloData, refresh: vi.fn(), configure: vi.fn(), openCard: vi.fn() });

it("shows each list as a column with its cards", () => {
  render(TrelloModule, props(full));
  for (const list of full.lists!) {
    const column = screen.getByRole("region", { name: list.name });
    for (const card of list.cards) expect(within(column).getByText(card.name)).toBeInTheDocument();
  }
});

it("shows labels as colored dots and flags overdue due dates", () => {
  render(TrelloModule, props(overdueJson));
  const card = screen.getByText("Card 1.1").closest("li")!;
  expect(within(card).getAllByTestId("label")).toHaveLength(2);
  expect(within(card).getByText(/Vencido/)).toHaveClass("overdue");
  for (const name of ["Card concluído", "Card futuro"]) {
    const later = screen.getByText(name).closest("li")!;
    expect(within(later).getByText(/Entrega/)).not.toHaveClass("overdue");
  }
});

it("opens a card in the browser on double click", async () => {
  const p = props(full);
  render(TrelloModule, p);
  const card = full.lists![0].cards[0];
  await fireEvent.dblClick(screen.getByText(card.name));
  expect(p.openCard).toHaveBeenCalledWith(card.url);
});

it("keeps the module visible without credentials and offers setup", async () => {
  const p = props(emptyJson);
  render(TrelloModule, p);
  expect(screen.getByText("Trello não conectado")).toBeInTheDocument();
  await fireEvent.click(screen.getByRole("button", { name: "Configurar" }));
  expect(p.configure).toHaveBeenCalledOnce();
});

it("asks for a board after connecting", () => {
  render(TrelloModule, props(chooseJson));
  expect(screen.getByText("Escolha um quadro nas Configurações")).toBeInTheDocument();
});

it("dims the last board and offers reconnection for a refused credential", async () => {
  const p = props(invalidJson);
  render(TrelloModule, p);
  expect(screen.getByText("Lista 1").closest("[data-stale]")).toHaveAttribute("data-stale", "true");
  expect(screen.getByText(/Atualizado/)).toBeInTheDocument();
  await fireEvent.click(screen.getByRole("button", { name: "Credencial inválida · Reconectar" }));
  expect(p.configure).toHaveBeenCalledOnce();
});

it("shows sustained failure and keeps the last board", () => {
  render(TrelloModule, props(errorJson));
  expect(screen.getByText(/Falha ao atualizar/)).toBeInTheDocument();
  expect(screen.getByText("Lista 1")).toBeInTheDocument();
});

it("keeps cached cards without titles readable", () => {
  const cached = structuredClone(full);
  cached.state = "stale";
  cached.lists![0].cards[0].name = "";
  render(TrelloModule, props(cached));
  expect(screen.getAllByText("Título disponível após atualizar").length).toBeGreaterThan(0);
});

it("refreshes manually with a ten second cooldown", async () => {
  vi.useFakeTimers();
  try {
    const p = props(full);
    render(TrelloModule, p);
    const button = screen.getByRole("button", { name: "Atualizar Trello" });
    await fireEvent.click(button);
    expect(p.refresh).toHaveBeenCalledOnce();
    expect(button).toBeDisabled();
    await vi.advanceTimersByTimeAsync(10_000);
    expect(button).toBeEnabled();
  } finally { vi.useRealTimers(); }
});

it("disables refresh while offline", () => {
  render(TrelloModule, props({ ...full, paused: true, lastSuccessAt: 1 }));
  expect(screen.getByRole("button", { name: "Atualizar Trello" })).toBeDisabled();
  expect(screen.queryByText(/Falha ao atualizar/)).not.toBeInTheDocument();
});
