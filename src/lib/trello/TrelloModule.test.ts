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
const props = (data: unknown, moveCard = vi.fn(() => new Promise<void>(() => {}))) =>
  ({ data: data as TrelloData, refresh: vi.fn(), configure: vi.fn(), openCard: vi.fn(), moveCard });
const column = (name: string) => screen.getByRole("region", { name });

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

it("points back to Settings to paste the token after generating it", () => {
  render(TrelloModule, props({ ...emptyJson, pendingKey: "0123abcd" }));
  expect(screen.getByText("Token gerado? Cole-o nas Configurações para concluir.")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Configurar" })).toBeInTheDocument();
});

it("moves a selected card to the clicked list right away", async () => {
  const p = props(full);
  render(TrelloModule, p);
  const [from, to] = full.lists!;
  const card = from.cards[0];
  await fireEvent.click(screen.getByText(card.name));
  await fireEvent.click(column(to.name));
  expect(p.moveCard).toHaveBeenCalledExactlyOnceWith(card.id, to.id);
  expect(within(column(to.name)).getByText(card.name)).toBeInTheDocument();
  expect(within(column(from.name)).queryByText(card.name)).not.toBeInTheDocument();
});

it("puts the card back and explains when the move fails", async () => {
  const p = props(full, vi.fn(() => Promise.reject("failed")));
  render(TrelloModule, p);
  const [from, to] = full.lists!;
  const card = from.cards[0];
  await fireEvent.click(screen.getByText(card.name));
  await fireEvent.click(column(to.name));
  expect(await screen.findByRole("alert")).toHaveTextContent("Não foi possível mover");
  expect(within(column(from.name)).getByText(card.name)).toBeInTheDocument();
  expect(within(column(to.name)).queryByText(card.name)).not.toBeInTheDocument();
});

it("cancels a selection by clicking the card again or its own list", async () => {
  const p = props(full);
  render(TrelloModule, p);
  const [from, to] = full.lists!;
  const card = screen.getByText(from.cards[0].name);
  const cancelThenTarget = async (cancel: () => Promise<unknown>) => {
    await fireEvent.click(card);
    await cancel();
    await fireEvent.click(column(to.name));
  };
  await cancelThenTarget(() => fireEvent.click(card));
  await cancelThenTarget(() => fireEvent.click(column(from.name)));
  expect(p.moveCard).not.toHaveBeenCalled();
  expect(within(column(from.name)).getByText(from.cards[0].name)).toBeInTheDocument();
});

it("does not move cards while offline or with a refused credential", async () => {
  for (const data of [{ ...full, paused: true, lastSuccessAt: 1 }, invalidJson]) {
    const p = props(data);
    const { unmount } = render(TrelloModule, p);
    const [from, to] = (data as TrelloData).lists!;
    await fireEvent.click(screen.getByText(from.cards[0].name));
    await fireEvent.click(column(to.name));
    expect(p.moveCard).not.toHaveBeenCalled();
    unmount();
  }
});

it("keeps a moved card in place until the core publishes the reread board", async () => {
  let done!: () => void;
  const p = props(full, vi.fn(() => new Promise<void>((resolve) => done = resolve)));
  const { rerender } = render(TrelloModule, p);
  const [from, to] = full.lists!;
  const card = from.cards[0];
  await fireEvent.click(screen.getByText(card.name));
  await fireEvent.click(column(to.name));
  done();
  await new Promise((settle) => setTimeout(settle));
  expect(within(column(to.name)).getByText(card.name)).toBeInTheDocument();
  const moved = structuredClone(full);
  moved.lists![1].cards.push(moved.lists![0].cards.shift()!);
  await rerender({ data: moved });
  expect(within(column(to.name)).getAllByText(card.name)).toHaveLength(1);
  expect(within(column(from.name)).queryByText(card.name)).not.toBeInTheDocument();
});

it("shows the latest destination when a card is moved again before the core answers", async () => {
  const replies: (() => void)[] = [];
  const p = props(full, vi.fn(() => new Promise<void>((_, reject) => replies.push(() => reject("failed")))));
  render(TrelloModule, p);
  const [from, second, third] = full.lists!;
  const card = from.cards[0];
  await fireEvent.click(screen.getByText(card.name));
  await fireEvent.click(column(second.name));
  await fireEvent.click(screen.getByText(card.name));
  await fireEvent.click(column(third.name));
  replies[0]();
  await screen.findByRole("alert");
  expect(within(column(third.name)).getByText(card.name)).toBeInTheDocument();
});
