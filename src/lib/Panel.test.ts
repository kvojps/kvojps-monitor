import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import Panel from "./Panel.svelte";
import type { Module, Core, Snapshot } from "./core";
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
