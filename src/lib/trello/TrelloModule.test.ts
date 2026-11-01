import { act, cleanup, fireEvent, render, screen, within } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import TrelloModule from "./TrelloModule.svelte";
import type { TrelloData } from "../core";
import fullJson from "./fixtures/full.json";
import overdueJson from "./fixtures/overdue.json";
import emptyJson from "./fixtures/empty.json";
import chooseJson from "./fixtures/choose-board.json";
import invalidJson from "./fixtures/invalid-credential.json";
import errorJson from "./fixtures/error.json";
import staleJson from "./fixtures/stale.json";
import offlineJson from "./fixtures/offline.json";

const full = fullJson as TrelloData;
const stale = staleJson as TrelloData;
const offline = offlineJson as TrelloData;
const invalid = invalidJson as TrelloData;
const props = (data: unknown, moveCard = vi.fn(() => new Promise<void>(() => {}))) =>
  ({ data: data as TrelloData, refresh: vi.fn(), configure: vi.fn(), openCard: vi.fn(), moveCard });
const board = () => screen.getByRole("dialog", { name: "Quadro Trello" });
const column = (name: string) => within(board()).getByRole("region", { name });
const cardIn = (name: string) => within(board()).getByRole("button", { name: new RegExp(`^${name}`) });
const counts = () => screen.getByRole("list", { name: "Cards por lista" });
const countOf = (list: string) => within(counts()).getByText(list).nextElementSibling!.textContent;
async function openBoard() {
  await userEvent.click(screen.getByRole("button", { name: "Abrir quadro completo" }));
  return board();
}
async function select(card: string) { await userEvent.click(cardIn(card)); }

it("summarizes the real board with counts per list and no card preview", () => {
  render(TrelloModule, props(full));
  expect(screen.getByText("Quadro 1")).toBeInTheDocument();
  expect(screen.getByText("17 cards · 8 listas")).toBeInTheDocument();
  expect(countOf("Lista 1")).toBe("4");
  expect(countOf("Lista 2")).toBe("0");
  expect(screen.queryByText(/Primeiro card/)).not.toBeInTheDocument();
  expect(screen.queryByText("Card 1.1")).not.toBeInTheDocument();
  expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
});

it("keeps each list name on one line and names it in full on hover", () => {
  render(TrelloModule, props(full));
  const name = within(screen.getByRole("list", { name: "Cards por lista" })).getByText("Lista 1");
  expect(name).toHaveClass("list-name");
  expect(name).toHaveAttribute("title", "Lista 1");
});

it("opens the full board with every list, its count and its cards", async () => {
  render(TrelloModule, props(full));
  await openBoard();
  for (const list of full.lists!) {
    const region = column(list.name);
    expect(within(region).getByText(new RegExp(`^${list.cards.length} cards?$`))).toBeInTheDocument();
    for (const card of list.cards) expect(within(region).getByText(card.name)).toBeInTheDocument();
  }
});

it("paints labels in the board with their real Trello colors", async () => {
  render(TrelloModule, props(full));
  await openBoard();
  expect(within(cardIn("Card 1.1")).getAllByTestId("label").map((l) => l.style.background))
    .toEqual(["rgb(75, 206, 151)", "rgb(87, 157, 255)"]);
});

it("shows labels and due states in words, not only in color", async () => {
  render(TrelloModule, props(overdueJson));
  await openBoard();
  const late = cardIn("Card 1.1");
  expect(within(late).getAllByTestId("label")).toHaveLength(2);
  expect(within(late).getByRole("img")).toHaveAttribute("aria-label", "Etiquetas: vermelho, azul-celeste claro");
  expect(within(late).getByText(/Vencido/)).toHaveClass("overdue");
  expect(within(cardIn("Card concluído")).getByText(/Entrega concluída/)).not.toHaveClass("overdue");
  expect(within(cardIn("Card futuro")).getByText(/^Entrega \d/)).not.toHaveClass("overdue");
});

it("opens a card in the browser on double click or explicit action only", async () => {
  const p = props(full);
  render(TrelloModule, p);
  await openBoard();
  const card = full.lists![0].cards[0];
  await select(card.name);
  expect(p.openCard).not.toHaveBeenCalled();
  await userEvent.click(within(board()).getByRole("button", { name: `Abrir no Trello: ${card.name}` }));
  await fireEvent.dblClick(cardIn(card.name));
  expect(p.openCard).toHaveBeenCalledTimes(2);
  expect(p.openCard).toHaveBeenCalledWith(card.url);
});

it("moves a card after choosing Mover para and confirming Mover card", async () => {
  const p = props(full);
  render(TrelloModule, p);
  await openBoard();
  const [from, to] = full.lists!;
  const card = from.cards[0];
  await select(card.name);
  const confirm = within(board()).getByRole("button", { name: "Mover card" });
  expect(confirm).toBeDisabled();
  await userEvent.selectOptions(within(board()).getByRole("combobox", { name: "Mover para" }), to.name);
  await userEvent.click(confirm);
  expect(p.moveCard).toHaveBeenCalledExactlyOnceWith(card.id, to.id);
  expect(within(column(to.name)).getByText(card.name)).toBeInTheDocument();
  expect(within(column(from.name)).queryByText(card.name)).not.toBeInTheDocument();
  expect(within(board()).getByRole("status")).toHaveTextContent(`Movendo “${card.name}” para ${to.name}`);
  expect(countOf(from.name)).toBe("3");
  expect(countOf(to.name)).toBe("1");
});

it("offers every other list in the selector, even off-screen ones", async () => {
  render(TrelloModule, props(full));
  await openBoard();
  await select(full.lists![0].cards[0].name);
  const options = within(within(board()).getByRole("combobox", { name: "Mover para" })).getAllByRole("option");
  expect(options.map((o) => o.textContent)).toEqual(["Escolha uma lista", ...full.lists!.slice(1).map((l) => l.name)]);
});

it("moves a card with Mover para esta lista on the destination column", async () => {
  const p = props(full);
  render(TrelloModule, p);
  await openBoard();
  const [from, , to] = full.lists!;
  await select(from.cards[0].name);
  expect(within(column(from.name)).queryByRole("button", { name: "Mover para esta lista" })).not.toBeInTheDocument();
  await userEvent.click(within(column(to.name)).getByRole("button", { name: "Mover para esta lista" }));
  expect(p.moveCard).toHaveBeenCalledExactlyOnceWith(from.cards[0].id, to.id);
  expect(cardIn(from.cards[0].name)).toHaveFocus();
});

it("moves a card by dragging it to another list", async () => {
  const p = props(full);
  render(TrelloModule, p);
  await openBoard();
  const [from, to] = full.lists!;
  const card = cardIn(from.cards[0].name);
  expect(card).toHaveAttribute("draggable", "true");
  await fireEvent.dragStart(card);
  const sameList = new Event("dragover", { bubbles: true, cancelable: true });
  column(from.name).dispatchEvent(sameList);
  expect(sameList.defaultPrevented).toBe(false);
  const over = new Event("dragover", { bubbles: true, cancelable: true });
  column(to.name).dispatchEvent(over);
  expect(over.defaultPrevented).toBe(true);
  await fireEvent.drop(column(to.name));
  expect(p.moveCard).toHaveBeenCalledExactlyOnceWith(from.cards[0].id, to.id);
  expect(within(column(to.name)).getByText(from.cards[0].name)).toBeInTheDocument();
});

it("puts the card back, restores the summary and explains when the move fails", async () => {
  const p = props(full, vi.fn(() => Promise.reject("failed")));
  render(TrelloModule, p);
  await openBoard();
  const [from, to] = full.lists!;
  const card = from.cards[0];
  await select(card.name);
  await userEvent.click(within(column(to.name)).getByRole("button", { name: "Mover para esta lista" }));
  expect(await within(board()).findByRole("alert"))
    .toHaveTextContent(`Não foi possível mover “${card.name}”. O card voltou para ${from.name}.`);
  expect(within(column(from.name)).getByText(card.name)).toBeInTheDocument();
  expect(within(column(to.name)).queryByText(card.name)).not.toBeInTheDocument();
  expect(countOf(from.name)).toBe("4");
  expect(cardIn(card.name)).toHaveFocus();
});

it("explains a failure in the summary when the board was already closed", async () => {
  let fail!: () => void;
  const p = props(full, vi.fn(() => new Promise<void>((_, reject) => fail = () => reject("failed"))));
  render(TrelloModule, p);
  await openBoard();
  const [from, to] = full.lists!;
  await select(from.cards[0].name);
  await userEvent.click(within(column(to.name)).getByRole("button", { name: "Mover para esta lista" }));
  await userEvent.click(within(board()).getByRole("button", { name: "Fechar" }));
  fail();
  expect(await screen.findByRole("alert")).toHaveTextContent("Não foi possível mover");
  expect(countOf(from.name)).toBe("4");
});

it("cancels a selection without moving and returns focus to the card", async () => {
  const p = props(full);
  render(TrelloModule, p);
  await openBoard();
  const [from, to] = full.lists!;
  await select(from.cards[0].name);
  await userEvent.selectOptions(within(board()).getByRole("combobox", { name: "Mover para" }), to.name);
  await userEvent.click(within(board()).getByRole("button", { name: "Cancelar seleção" }));
  expect(p.moveCard).not.toHaveBeenCalled();
  expect(within(board()).queryByRole("button", { name: "Mover para esta lista" })).not.toBeInTheDocument();
  expect(cardIn(from.cards[0].name)).toHaveFocus();
  await select(from.cards[0].name);
  await select(from.cards[0].name);
  expect(cardIn(from.cards[0].name)).toHaveAttribute("aria-pressed", "false");
  expect(within(column(from.name)).getByText(from.cards[0].name)).toBeInTheDocument();
});

it("offers no move offline or with a refused credential, but still opens cards", async () => {
  for (const data of [offline, invalid]) {
    const p = props(data);
    const { unmount } = render(TrelloModule, p);
    await openBoard();
    const card = data.lists![0].cards[0];
    const name = card.name || "Título disponível após atualizar";
    expect(within(board()).getByText("Movimentação indisponível até o Trello voltar a atualizar.")).toBeInTheDocument();
    expect(within(board()).getAllByRole("button", { name: new RegExp(`^${name}`) })[0]).toHaveAttribute("draggable", "false");
    await userEvent.click(within(board()).getAllByRole("button", { name: new RegExp(`^${name}`) })[0]);
    expect(within(board()).queryByRole("combobox")).not.toBeInTheDocument();
    expect(within(board()).queryByRole("button", { name: "Mover para esta lista" })).not.toBeInTheDocument();
    await userEvent.click(within(board()).getByRole("button", { name: `Abrir no Trello: ${name}` }));
    expect(p.openCard).toHaveBeenCalledWith(card.url);
    expect(p.moveCard).not.toHaveBeenCalled();
    unmount();
  }
});

it("keeps a moved card in place until the core publishes the reread board", async () => {
  let done!: () => void;
  const p = props(full, vi.fn(() => new Promise<void>((resolve) => done = resolve)));
  const { rerender } = render(TrelloModule, p);
  await openBoard();
  const [from, to] = full.lists!;
  const card = from.cards[0];
  await select(card.name);
  await userEvent.click(within(column(to.name)).getByRole("button", { name: "Mover para esta lista" }));
  done();
  expect(await within(board()).findByText(`“${card.name}” movido para ${to.name}.`)).toBeInTheDocument();
  expect(within(column(to.name)).getByText(card.name)).toBeInTheDocument();
  const moved = structuredClone(full);
  moved.lists![1].cards.push(moved.lists![0].cards.shift()!);
  await rerender({ data: moved });
  expect(within(column(to.name)).getAllByText(card.name)).toHaveLength(1);
  expect(within(column(from.name)).queryByText(card.name)).not.toBeInTheDocument();
  expect(countOf(to.name)).toBe("1");
});

it("shows the latest destination when a card is moved again before the core answers", async () => {
  const replies: (() => void)[] = [];
  const p = props(full, vi.fn(() => new Promise<void>((_, reject) => replies.push(() => reject("failed")))));
  render(TrelloModule, p);
  await openBoard();
  const [from, second, third] = full.lists!;
  await select(from.cards[0].name);
  await userEvent.click(within(column(second.name)).getByRole("button", { name: "Mover para esta lista" }));
  await userEvent.click(within(column(third.name)).getByRole("button", { name: "Mover para esta lista" }));
  replies[0]();
  await act(() => {});
  expect(within(column(third.name)).getByText(from.cards[0].name)).toBeInTheDocument();
  expect(within(board()).queryByRole("alert")).not.toBeInTheDocument();
});

it("selects, moves and opens a card entirely from the keyboard", async () => {
  const user = userEvent.setup();
  const p = props(full);
  render(TrelloModule, p);
  screen.getByRole("button", { name: "Abrir quadro completo" }).focus();
  await user.keyboard("{Enter}");
  const [from, to] = full.lists!;
  const card = cardIn(from.cards[0].name);
  card.focus();
  await user.keyboard("{Enter}");
  expect(card).toHaveAttribute("aria-pressed", "true");
  const choose = within(board()).getByRole("combobox", { name: "Mover para" });
  choose.focus();
  await user.selectOptions(choose, to.name);
  await user.tab();
  expect(within(board()).getByRole("button", { name: "Mover card" })).toHaveFocus();
  await user.keyboard("{Enter}");
  expect(p.moveCard).toHaveBeenCalledExactlyOnceWith(from.cards[0].id, to.id);
  expect(cardIn(from.cards[0].name)).toHaveFocus();
  await user.click(within(board()).getByRole("button", { name: `Abrir no Trello: ${from.cards[0].name}` }));
  expect(p.openCard).toHaveBeenCalledWith(from.cards[0].url);
});

it("closes the board with Fechar or Escape and returns focus to its trigger", async () => {
  render(TrelloModule, props(full));
  const trigger = screen.getByRole("button", { name: "Abrir quadro completo" });
  await openBoard();
  await userEvent.click(within(board()).getByRole("button", { name: "Fechar" }));
  expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  expect(trigger).toHaveFocus();
  await openBoard();
  const escape = new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true });
  window.dispatchEvent(escape);
  await act(() => {});
  expect(escape.defaultPrevented).toBe(true);
  expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  expect(trigger).toHaveFocus();
});

it("leaves Escape to the Panel when the board is closed", () => {
  render(TrelloModule, props(full));
  const escape = new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true });
  window.dispatchEvent(escape);
  expect(escape.defaultPrevented).toBe(false);
});

it("keeps the selection when the board is reopened and focuses the selected card", async () => {
  render(TrelloModule, props(full));
  await openBoard();
  await select("Card 3.1");
  await userEvent.click(within(board()).getByRole("button", { name: "Fechar" }));
  await openBoard();
  expect(cardIn("Card 3.1")).toHaveAttribute("aria-pressed", "true");
  expect(cardIn("Card 3.1")).toHaveFocus();
});

it("keeps the board open, the selection and the focus when a refreshed board arrives", async () => {
  const { rerender } = render(TrelloModule, props(full));
  await openBoard();
  const [from, to] = full.lists!;
  await select(from.cards[0].name);
  const card = cardIn(from.cards[0].name);
  await rerender({ data: { ...structuredClone(full), refreshing: true } });
  await rerender({ data: structuredClone(full) });
  expect(board()).toHaveAttribute("open");
  expect(card).toHaveFocus();
  expect(card).toHaveAttribute("aria-pressed", "true");
  expect(within(column(to.name)).getByRole("button", { name: "Mover para esta lista" })).toBeInTheDocument();
});

it("drops the selection when the selected card leaves the board", async () => {
  const p = props(full);
  const { rerender } = render(TrelloModule, p);
  await openBoard();
  await select(full.lists![0].cards[0].name);
  const archived = structuredClone(full);
  archived.lists![0].cards.shift();
  await rerender({ data: archived });
  expect(within(board()).queryByRole("button", { name: "Mover para esta lista" })).not.toBeInTheDocument();
  expect(within(board()).queryByRole("combobox")).not.toBeInTheDocument();
});

it("keeps each list's scroll position when a refreshed board arrives", async () => {
  const { rerender } = render(TrelloModule, props(full));
  await openBoard();
  const cards = within(column(full.lists![0].name)).getByRole("list");
  cards.scrollTop = 80;
  await rerender({ data: structuredClone(full) });
  expect(within(column(full.lists![0].name)).getByRole("list")).toBe(cards);
  expect(cards.scrollTop).toBe(80);
});

it("restores the horizontal board scroll when reopened", async () => {
  render(TrelloModule, props(full));
  await openBoard();
  const lists = within(board()).getByRole("group", { name: "Listas do quadro" });
  lists.scrollLeft = 300;
  await userEvent.click(within(board()).getByRole("button", { name: "Fechar" }));
  await openBoard();
  expect(within(board()).getByRole("group", { name: "Listas do quadro" }).scrollLeft).toBe(300);
});

it("marks empty lists and explains a board without lists or cards", async () => {
  render(TrelloModule, props(full));
  await openBoard();
  expect(within(column("Lista 2")).getByText("Nenhum card nesta lista")).toBeInTheDocument();
  cleanup();
  render(TrelloModule, props({ ...full, lists: [] }));
  expect(screen.getByText("Quadro sem listas")).toBeInTheDocument();
  expect(screen.queryByRole("button", { name: "Abrir quadro completo" })).not.toBeInTheDocument();
  cleanup();
  render(TrelloModule, props({ ...full, lists: full.lists!.map((l) => ({ ...l, cards: [] })) }));
  expect(screen.getByText("Quadro sem cards")).toBeInTheDocument();
  expect(screen.getByText("0 cards · 8 listas")).toBeInTheDocument();
});

it("shows every card of a dense list with its labels and due date", async () => {
  const dense = structuredClone(overdueJson as TrelloData);
  const model = dense.lists![0].cards[0];
  dense.lists![0].cards = Array.from({ length: 40 }, (_, i) => ({ ...model, id: `dense-${i}`, name: `Card denso ${i}` }));
  render(TrelloModule, props(dense));
  expect(countOf(dense.lists![0].name)).toBe("40");
  await openBoard();
  const list = within(column(dense.lists![0].name));
  for (let i = 0; i < 40; i++) {
    const card = list.getByRole("button", { name: new RegExp(`Card denso ${i} `) });
    expect(within(card).getAllByTestId("label")).toHaveLength(model.labels.length);
    expect(within(card).getByText(/Vencido/)).toBeInTheDocument();
  }
});

it("keeps the module visible without credentials and offers setup", async () => {
  const p = props(emptyJson);
  render(TrelloModule, p);
  expect(screen.getByText("Trello não conectado")).toBeInTheDocument();
  expect(screen.queryByRole("button", { name: "Abrir quadro completo" })).not.toBeInTheDocument();
  await fireEvent.click(screen.getByRole("button", { name: "Configurar Trello" }));
  expect(p.configure).toHaveBeenCalledOnce();
});

it("asks for a board after connecting", () => {
  render(TrelloModule, props(chooseJson));
  expect(screen.getByText("Escolha um quadro nas Configurações")).toBeInTheDocument();
});

it("dims the last board and offers reconnection for a refused credential", async () => {
  const p = props(invalid);
  render(TrelloModule, p);
  expect(screen.getByRole("list", { name: "Cards por lista" }).closest("[data-stale]")).toHaveAttribute("data-stale", "true");
  expect(screen.getByText(/Atualizado/)).toBeInTheDocument();
  expect(screen.getByText("Credencial inválida", { exact: false })).toBeInTheDocument();
  await fireEvent.click(screen.getByRole("button", { name: "Reconectar" }));
  expect(p.configure).toHaveBeenCalledOnce();
});

it("shows sustained failure and keeps the last board", () => {
  render(TrelloModule, props(errorJson));
  expect(screen.getByText(/Falha ao atualizar/)).toBeInTheDocument();
  expect(countOf("Lista 1")).toBe("4");
});

it("shows the board restored from cache with its age and titles pending", async () => {
  render(TrelloModule, props(stale));
  expect(screen.getByText(/Atualizado/)).toBeInTheDocument();
  expect(screen.queryByText(/Falha ao atualizar/)).not.toBeInTheDocument();
  await openBoard();
  expect(within(board()).getAllByText("Título disponível após atualizar")).toHaveLength(
    stale.lists!.reduce((n, list) => n + list.cards.length, 0));
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

it("keeps the board with its age and disables refresh while offline", () => {
  render(TrelloModule, props(offline));
  expect(screen.getByRole("button", { name: "Atualizar Trello" })).toBeDisabled();
  expect(screen.getByText(/Atualizado/)).toBeInTheDocument();
  expect(countOf(offline.lists![0].name)).toBe(String(offline.lists![0].cards.length));
  expect(screen.queryByText(/Falha ao atualizar/)).not.toBeInTheDocument();
  expect(screen.queryByRole("alert")).not.toBeInTheDocument();
});

it("points back to Settings to paste the token after generating it", () => {
  render(TrelloModule, props({ ...emptyJson, pendingKey: "0123abcd" }));
  expect(screen.getByText("Token gerado? Cole-o nas Configurações para concluir.")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Configurar Trello" })).toBeInTheDocument();
});

it("explains there is no board reading yet and keeps refresh and configure", async () => {
  const p = props(null);
  render(TrelloModule, p);
  expect(screen.getByText("Sem leitura do quadro do Trello ainda")).toBeInTheDocument();
  await fireEvent.click(screen.getByRole("button", { name: "Atualizar Trello" }));
  await fireEvent.click(screen.getByRole("button", { name: "Configurar Trello" }));
  expect(p.refresh).toHaveBeenCalledOnce();
  expect(p.configure).toHaveBeenCalledOnce();
});

it("keeps the card selected after a double click opens it", async () => {
  const p = props(full);
  render(TrelloModule, p);
  await openBoard();
  const card = full.lists![0].cards[0];
  await userEvent.dblClick(cardIn(card.name));
  expect(p.openCard).toHaveBeenCalledExactlyOnceWith(card.url);
  expect(cardIn(card.name)).toHaveAttribute("aria-pressed", "true");
});

it("forgets a chosen destination that a refreshed board removed", async () => {
  const p = props(full);
  const { rerender } = render(TrelloModule, p);
  await openBoard();
  const [from, to] = full.lists!;
  await select(from.cards[0].name);
  await userEvent.selectOptions(within(board()).getByRole("combobox", { name: "Mover para" }), to.name);
  const without = structuredClone(full);
  without.lists!.splice(1, 1);
  await rerender({ data: without });
  expect(within(board()).getByRole("button", { name: "Mover card" })).toBeDisabled();
  expect(within(board()).getByRole("combobox", { name: "Mover para" })).toHaveValue("");
});

it("starts a reopened board without the previous move outcome", async () => {
  render(TrelloModule, props(full, vi.fn(async () => {})));
  await openBoard();
  const [from, to] = full.lists!;
  await select(from.cards[0].name);
  await userEvent.click(within(column(to.name)).getByRole("button", { name: "Mover para esta lista" }));
  expect(await within(board()).findByText(/movido para/)).toBeInTheDocument();
  await userEvent.click(within(board()).getByRole("button", { name: "Fechar" }));
  await openBoard();
  expect(within(board()).getByRole("status")).toHaveTextContent("Nenhuma alteração realizada.");
});

it("shows nothing beyond prototype B in the normal state", () => {
  const { container } = render(TrelloModule, props(full));
  expect(screen.getByRole("heading", { level: 2, name: "Trello" })).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Atualizar Trello" })).toHaveTextContent(/^↻$/);
  expect(screen.getByRole("button", { name: "Configurar Trello" })).toHaveTextContent(/^⚙$/);
  expect(screen.queryByText(/Primeiro card/)).not.toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Abrir quadro completo" })).toHaveTextContent("Abrir quadro completo ↗");
  expect(container.querySelectorAll(".line")).toHaveLength(0);
  expect(screen.queryByText(/Atualizado|Fonte|Entrega|Sem vencimento/)).not.toBeInTheDocument();
});

it("opens the full board as prototype B: board name, hint, due line and open action under the selected card", async () => {
  render(TrelloModule, props(full));
  await openBoard();
  expect(within(board()).getByRole("heading", { level: 3, name: "Quadro 1" })).toBeInTheDocument();
  expect(within(board()).getByText("Selecione um card e a lista de destino, ou arraste entre colunas.")).toBeInTheDocument();
  expect(within(board()).getByText("Selecione um card para habilitar a movimentação.")).toBeInTheDocument();
  expect(within(cardIn("Card 1.1")).getByText("Sem vencimento")).toBeInTheDocument();
  expect(within(board()).queryByRole("button", { name: /^Abrir no Trello/ })).not.toBeInTheDocument();
  await select("Card 1.1");
  expect(within(board()).getByRole("button", { name: "Abrir no Trello: Card 1.1" })).toHaveTextContent("Abrir no Trello ↗");
  expect(within(board()).getByRole("button", { name: "Fechar" })).toHaveTextContent("Fechar ×");
});

describe("fixed composition without a reading", () => {
  const area = (container: HTMLElement) => container.querySelector<HTMLElement>(".area");

  it("fills the body with one neutral area before the first reading", () => {
    const { container } = render(TrelloModule, props(null));
    expect(area(container)).toHaveAttribute("data-tone", "neutral");
    expect(area(container)).toHaveTextContent("◌ Sem leitura do quadro do Trello ainda");
    expect(screen.queryByRole("list", { name: "Cards por lista" })).not.toBeInTheDocument();
  });

  it("asks for a connection in an amber area, with the pending token step", async () => {
    const p = props({ ...emptyJson, pendingKey: "k" });
    const { container } = render(TrelloModule, p);
    expect(area(container)).toHaveAttribute("data-tone", "warn");
    expect(area(container)).toHaveTextContent("⚠ Trello não conectado Token gerado? Cole-o nas Configurações para concluir. Configurar");
    await fireEvent.click(within(area(container)!).getByRole("button", { name: "Configurar" }));
    expect(p.configure).toHaveBeenCalledOnce();
  });

  it("asks for a board in a neutral area", () => {
    const { container } = render(TrelloModule, props(chooseJson));
    expect(area(container)).toHaveAttribute("data-tone", "neutral");
    expect(area(container)).toHaveTextContent("Escolha um quadro nas Configurações Configurar");
  });

  it("offers a new attempt when the first reading failed", async () => {
    const p = props({ ...errorJson, lists: null, lastSuccessAt: null });
    const { container } = render(TrelloModule, p);
    expect(area(container)).toHaveAttribute("data-tone", "warn");
    expect(area(container)).toHaveTextContent(/Falha ao atualizar .* · sem leitura anterior/);
    await fireEvent.click(within(area(container)!).getByRole("button", { name: "Tentar de novo" }));
    expect(p.refresh).toHaveBeenCalledOnce();
  });

  it("offers reconnection when the credential is invalid and nothing was read", () => {
    const { container } = render(TrelloModule, props({ ...invalidJson, lists: null }));
    expect(area(container)).toHaveTextContent("⚠ Credencial inválida Reconectar");
  });

  it("keeps the old board dimmed instead of the area", () => {
    const { container } = render(TrelloModule, props(errorJson));
    expect(area(container)).toBeNull();
    expect(container.querySelector(".summary")).toHaveAttribute("data-stale", "true");
    expect(counts()).toBeInTheDocument();
  });
});
