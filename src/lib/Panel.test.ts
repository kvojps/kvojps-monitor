import { render, screen, within } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import Panel from "./Panel.svelte";
import type { Module, Core, Snapshot } from "./core";
import networkSpeed from "./network/fixtures/speed.json";
import notebook from "./system/fixtures/notebook.json";

function fakeCore(initial: { lastModule?: Module; snapshots?: Snapshot[] } = {}) {
  let listener: ((s: Snapshot) => void) | undefined;
  const core = {
    initialState: vi.fn(async () => ({
      lastModule: initial.lastModule ?? "ai",
      snapshots: initial.snapshots ?? [],
    })),
    listenSnapshots: vi.fn(async (cb: (s: Snapshot) => void) => {
      listener = cb;
      return () => {};
    }),
    reportModuleVisible: vi.fn(),
    closePanel: vi.fn(),
  } satisfies Core;
  return { core, publish: (s: Snapshot) => listener?.(s) };
}

describe("Panel rail", () => {
  it("lists six modules and Settings", async () => {
    const { core } = fakeCore();
    render(Panel, { core });

    const rail = await screen.findByRole("navigation", { name: "Módulos" });
    const names = Array.from(rail.querySelectorAll("button")).map((b) => b.getAttribute("aria-label"));
    expect(names).toEqual(["IA", "Agentes", "GitHub", "Trello", "Rede", "Sistema", "Configurações"]);
  });

  it("opens the last visited module with its name in the header", async () => {
    const { core } = fakeCore({ lastModule: "network" });
    render(Panel, { core });

    expect(await screen.findByRole("heading", { name: "Rede" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Rede" })).toHaveAttribute("aria-current", "page");
  });

  it("switches modules through the rail and reports visibility to the core", async () => {
    const { core } = fakeCore({ lastModule: "ai" });
    render(Panel, { core });
    await screen.findByRole("heading", { name: "IA" });

    await userEvent.click(screen.getByRole("button", { name: "Sistema" }));

    expect(screen.getByRole("heading", { name: "Sistema" })).toBeInTheDocument();
    expect(core.reportModuleVisible).toHaveBeenLastCalledWith("system");
  });

  it("Settings does not count as a visible module", async () => {
    const { core } = fakeCore();
    render(Panel, { core });
    await screen.findByRole("heading", { name: "IA" });

    await userEvent.click(screen.getByRole("button", { name: "Configurações" }));

    expect(screen.getByRole("heading", { name: "Configurações" })).toBeInTheDocument();
    expect(core.reportModuleVisible).toHaveBeenLastCalledWith(null);
  });

  it("shows the last update time in the header", async () => {
    const fourteenThirty = new Date(2026, 9, 30, 14, 32).getTime();
    const { core } = fakeCore({
      lastModule: "ai",
      snapshots: [{ module: "ai", updatedAt: fourteenThirty, data: null }],
    });
    render(Panel, { core });

    expect(await screen.findByText("atualizado 14:32")).toBeInTheDocument();
  });

  it("updates the header when the core pushes a snapshot", async () => {
    const { core, publish } = fakeCore({ lastModule: "network" });
    render(Panel, { core });
    expect(await screen.findByText("sem dados ainda")).toBeInTheDocument();

    publish({ module: "network", updatedAt: new Date(2026, 9, 30, 9, 5).getTime(), data: null });

    expect(await screen.findByText("atualizado 09:05")).toBeInTheDocument();
  });

  it("Escape asks the core to close the panel", async () => {
    const { core } = fakeCore();
    render(Panel, { core });
    await screen.findByRole("heading", { name: "IA" });

    await userEvent.keyboard("{Escape}");

    expect(core.closePanel).toHaveBeenCalledOnce();
  });

  it("renders the System snapshot pushed by the core", async () => {
    const { core, publish } = fakeCore({ lastModule: "system" });
    render(Panel, { core });
    await screen.findByRole("heading", { name: "Sistema" });
    expect(screen.queryByRole("region", { name: "CPU" })).not.toBeInTheDocument();

    publish(notebook as Snapshot);

    expect(await screen.findByRole("region", { name: "CPU" })).toBeInTheDocument();
  });
});

it("sends the network automatic speed preference from Settings", async () => {
  const { core } = fakeCore({ lastModule: "network", snapshots: [networkSpeed as Snapshot] });
  const setAutomaticSpeed = vi.fn();
  render(Panel, { core: { ...core, setAutomaticSpeed } });
  await screen.findByRole("heading", { name: "Rede" });
  await userEvent.click(screen.getByRole("button", { name: "Configurações" }));
  const toggle = screen.getByRole("checkbox", { name: "Teste de velocidade automático" });
  expect(toggle).toBeChecked();
  await userEvent.click(toggle);
  expect(setAutomaticSpeed).toHaveBeenCalledWith(false);
});

it("opens GitHub settings from setup and sends the pasted token to the core", async () => {
  const { default: missing } = await import("./github/fixtures/missing-gh.json");
  const { core } = fakeCore({ lastModule: "github", snapshots: [{ module: "github", updatedAt: null, data: missing }] });
  const saveGithubToken = vi.fn().mockResolvedValue("exemplo");
  render(Panel, { core: { ...core, saveGithubToken } });
  await userEvent.click(await screen.findByRole("button", { name: "Configurar" }));
  await userEvent.click(screen.getByRole("button", { name: "Colar token clássico" }));
  expect(screen.getByRole("heading", { name: "Configurações" })).toBeInTheDocument();
  await userEvent.type(screen.getByLabelText("Token clássico (escopo repo)"), "private-token");
  const github = screen.getByRole("region", { name: "GitHub" });
  await userEvent.click(within(github).getByRole("button", { name: "Validar e salvar" }));
  expect(saveGithubToken).toHaveBeenCalledWith("private-token");
  expect(await screen.findByText("Conectado como exemplo")).toBeInTheDocument();
});

it("opens Trello settings from the empty module and sends key and token to the core", async () => {
  const { default: empty } = await import("./trello/fixtures/empty.json");
  const { core } = fakeCore({ lastModule: "trello", snapshots: [{ module: "trello", updatedAt: null, data: empty }] });
  const connectTrello = vi.fn().mockResolvedValue("exemplo");
  render(Panel, { core: { ...core, connectTrello } });
  await userEvent.click(await screen.findByRole("button", { name: "Configurar" }));
  expect(core.reportModuleVisible).toHaveBeenLastCalledWith(null);
  const trello = screen.getByRole("region", { name: "Trello" });
  await userEvent.type(within(trello).getByLabelText("API key"), "0123abcd");
  await userEvent.type(within(trello).getByLabelText("Token"), "private-token");
  await userEvent.click(within(trello).getByRole("button", { name: "Validar e salvar" }));
  expect(connectTrello).toHaveBeenCalledWith("0123abcd", "private-token");
  expect(await within(trello).findByText("Conectado como exemplo")).toBeInTheDocument();
});

it("sends a Trello move to the core with card and destination list", async () => {
  const { default: full } = await import("./trello/fixtures/full.json");
  const { core } = fakeCore({ lastModule: "trello", snapshots: [{ module: "trello", updatedAt: 1, data: full }] });
  const moveTrelloCard = vi.fn().mockResolvedValue(undefined);
  render(Panel, { core: { ...core, moveTrelloCard } });
  const [from, to] = full.lists;
  await userEvent.click(await screen.findByText(from.cards[0].name));
  await userEvent.click(screen.getByRole("region", { name: to.name }));
  expect(moveTrelloCard).toHaveBeenCalledExactlyOnceWith(from.cards[0].id, to.id);
});

it("shows the AI module snapshot and sends its refresh to the core", async () => {
  const { default: full } = await import("./ai/fixtures/full.json");
  const { core } = fakeCore({ lastModule: "ai", snapshots: [{ module: "ai", updatedAt: 1, data: full }] });
  const refreshAi = vi.fn();
  render(Panel, { core: { ...core, refreshAi } });
  expect(await screen.findByRole("region", { name: "ChatGPT / Codex" })).toBeInTheDocument();
  await userEvent.click(screen.getByRole("button", { name: "Atualizar IA" }));
  expect(refreshAi).toHaveBeenCalledOnce();
});

it("mirrors and clears the waiting accent on the Agents rail icon", async () => {
  const { act } = await import("@testing-library/svelte");
  const waiting = (await import("./agents/fixtures/waiting.json")).default;
  const empty = (await import("./agents/fixtures/empty.json")).default;
  const { core, publish } = fakeCore({ snapshots: [{ module: "agents", updatedAt: 1, data: waiting }] });
  render(Panel, { core });
  const rail = await screen.findByRole("navigation", { name: "Módulos" });
  expect(await within(rail).findByLabelText("Aguardando você")).toBeInTheDocument();
  await act(() => publish({ module: "agents", updatedAt: 2, data: empty }));
  expect(within(rail).queryByLabelText("Aguardando você")).not.toBeInTheDocument();
});
