import { act, render, screen, within } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import Panel from "./Panel.svelte";
import type { Module, Core, Snapshot, InitialState } from "./core";
import networkSpeed from "./network/fixtures/speed.json";
import networkFull from "./network/fixtures/full.json";
import notebook from "./system/fixtures/notebook.json";
import waitingAgents from "./agents/fixtures/waiting.json";
import aiFull from "./ai/fixtures/full.json";
import githubFull from "./github/fixtures/full.json";
import trelloFull from "./trello/fixtures/full.json";

/** Stands in for the webview IntersectionObserver: tests decide what intersects. */
class FakeObserver {
  static instances: FakeObserver[] = [];
  targets = new Set<Element>();
  constructor(public callback: IntersectionObserverCallback) {
    FakeObserver.instances.push(this);
  }
  observe(target: Element) {
    this.targets.add(target);
  }
  unobserve(target: Element) {
    this.targets.delete(target);
  }
  disconnect() {
    this.targets.clear();
  }
}

/** Simulate scrolling so that exactly these modules intersect the visible area. */
async function intersect(modules: Module[]) {
  await act(() => {
    for (const observer of FakeObserver.instances) {
      const entries = Array.from(observer.targets).map((target) => ({
        target,
        isIntersecting: modules.some((m) => target.id === `module-${m}`),
      }));
      observer.callback(entries as unknown as IntersectionObserverEntry[], observer as unknown as IntersectionObserver);
    }
  });
}

beforeEach(() => {
  FakeObserver.instances = [];
  vi.stubGlobal("IntersectionObserver", FakeObserver);
});
afterEach(() => vi.unstubAllGlobals());

function fakeCore(initial: { lastAnchor?: Module; snapshots?: Snapshot[]; agentTarget?: string } = {}) {
  let listener: ((s: Snapshot) => void) | undefined;
  const core = {
    initialState: vi.fn(async (): Promise<InitialState> => ({
      lastAnchor: initial.lastAnchor ?? "ai",
      agentTarget: initial.agentTarget ?? null,
      snapshots: initial.snapshots ?? [],
    })),
    listenSnapshots: vi.fn(async (cb: (s: Snapshot) => void) => {
      listener = cb;
      return () => {};
    }),
    reportVisibleModules: vi.fn(),
    chooseAnchor: vi.fn(),
    closePanel: vi.fn(),
  } satisfies Core;
  return { core, publish: (s: Snapshot) => act(() => listener?.(s)) };
}

const nav = () => screen.getByRole("navigation", { name: "Módulos" });
// The outer module section comes first; some modules label an inner region too.
const section = (name: string) => screen.getAllByRole("region", { name })[0];

describe("Panel stations", () => {
  it("shows the six modules together, grouped in the three stations", async () => {
    const { core } = fakeCore();
    render(Panel, { core });

    await screen.findByRole("group", { name: "Capacidade" });
    const modules = (station: string) =>
      Array.from(screen.getByRole("group", { name: station }).querySelectorAll(":scope > section"))
        .map((s) => s.getAttribute("aria-label") ?? within(s as HTMLElement).getAllByRole("heading")[0].textContent);
    expect(modules("Capacidade")).toEqual(["IA", "Sistema"]);
    expect(modules("Execução")).toEqual(["Agentes", "Trello"]);
    expect(modules("Observação")).toEqual(["GitHub", "Rede"]);
    // Reading order follows the stations.
    const order = Array.from(document.querySelectorAll("[role=group] > section")).map((s) => s.id);
    expect(order).toEqual(["module-ai", "module-system", "module-agents", "module-trello", "module-github", "module-network"]);
  });

  it("identifies each module source and its last update", async () => {
    const fourteenThirty = new Date(2026, 9, 30, 14, 32).getTime();
    const { core } = fakeCore({ snapshots: [{ module: "ai", updatedAt: fourteenThirty, data: null }] });
    render(Panel, { core });

    const ai = await screen.findByRole("region", { name: "IA" });
    expect(within(ai).getByText("Codex e Claude Code")).toBeInTheDocument();
    expect(within(ai).getByText("atualizado 14:32")).toBeInTheDocument();
    expect(within(section("Rede")).getByText("NetworkManager e kernel")).toBeInTheDocument();
    expect(within(section("Rede")).getByText("sem dados ainda")).toBeInTheDocument();
  });

  it("shows cached snapshots at once and updates only the module the core pushes", async () => {
    const { core, publish } = fakeCore({ snapshots: [notebook as Snapshot] });
    render(Panel, { core });
    expect(await within(await screen.findByRole("region", { name: "Sistema" })).findByText("atualizado 11:32")).toBeInTheDocument();

    await publish({ module: "network", updatedAt: new Date(2026, 9, 30, 9, 5).getTime(), data: null });

    expect(within(section("Rede")).getByText("atualizado 09:05")).toBeInTheDocument();
    expect(within(section("Sistema")).getByText("atualizado 11:32")).toBeInTheDocument();
  });

  it("keeps keyboard focus when a snapshot arrives", async () => {
    const { core, publish } = fakeCore({ snapshots: [{ module: "ai", updatedAt: 1, data: aiFull }] });
    render(Panel, { core: { ...core, refreshAi: vi.fn() } });
    const configure = await screen.findByRole("button", { name: "Configurar IA" });
    configure.focus();

    await publish({ module: "ai", updatedAt: 2, data: aiFull });

    expect(screen.getByRole("button", { name: "Configurar IA" })).toHaveFocus();
  });
});

describe("Panel navigation", () => {
  it("lists the six modules and Settings", async () => {
    const { core } = fakeCore();
    render(Panel, { core });

    await screen.findByRole("region", { name: "IA" });
    // The decorative index is hidden from assistive technology.
    const names = within(nav()).getAllByRole("button").map((b) => b.textContent?.replace(/^\s*\d+/, "").trim());
    expect(names).toEqual(["IA", "Agentes", "GitHub", "Trello", "Rede", "Sistema", "Configurações"]);
  });

  it("restores the last anchor when the panel opens", async () => {
    const scroll = vi.mocked(Element.prototype.scrollIntoView);
    scroll.mockClear();
    const { core } = fakeCore({ lastAnchor: "network" });
    render(Panel, { core });

    await vi.waitFor(() => expect(scroll).toHaveBeenCalled());
    expect(scroll.mock.contexts.at(-1)).toBe(section("Rede"));
    expect(within(nav()).getByRole("button", { name: "Rede" })).toHaveAttribute("aria-current", "location");
    expect(core.chooseAnchor).not.toHaveBeenCalled();
  });

  it("scrolls to the chosen module, focuses it and remembers the anchor", async () => {
    const scroll = vi.mocked(Element.prototype.scrollIntoView);
    const { core } = fakeCore();
    render(Panel, { core });
    await screen.findByRole("region", { name: "IA" });
    scroll.mockClear();

    await userEvent.click(within(nav()).getByRole("button", { name: "Trello" }));

    expect(scroll.mock.contexts.at(-1)).toBe(section("Trello"));
    expect(section("Trello")).toHaveFocus();
    expect(core.chooseAnchor).toHaveBeenLastCalledWith("trello");
    expect(within(nav()).getByRole("button", { name: "Trello" })).toHaveAttribute("aria-current", "location");
  });

  it("reaches modules from the keyboard", async () => {
    const { core } = fakeCore();
    render(Panel, { core });
    await screen.findByRole("region", { name: "IA" });
    within(nav()).getByRole("button", { name: "IA" }).focus();

    await userEvent.tab();
    await userEvent.keyboard("{Enter}");

    expect(section("Agentes")).toHaveFocus();
    expect(core.chooseAnchor).toHaveBeenLastCalledWith("agents");
  });

  it("Escape asks the core to close the panel, also in Settings", async () => {
    const { core } = fakeCore();
    render(Panel, { core });
    await screen.findByRole("region", { name: "IA" });

    await userEvent.keyboard("{Escape}");
    await userEvent.click(within(nav()).getByRole("button", { name: "Configurações" }));
    await userEvent.keyboard("{Escape}");

    expect(core.closePanel).toHaveBeenCalledTimes(2);
  });
});

describe("Panel visibility", () => {
  it("reports every module intersecting the visible area", async () => {
    const { core } = fakeCore();
    render(Panel, { core });
    await screen.findByRole("region", { name: "IA" });

    await intersect(["ai", "agents", "github"]);
    expect(core.reportVisibleModules).toHaveBeenLastCalledWith(["ai", "agents", "github"]);

    // Scrolling down: the first row leaves the area.
    await intersect(["system", "trello", "network"]);
    expect(core.reportVisibleModules).toHaveBeenLastCalledWith(["trello", "network", "system"]);
  });

  it("does not report again when intersection events repeat the same set", async () => {
    const { core } = fakeCore();
    render(Panel, { core });
    await screen.findByRole("region", { name: "IA" });

    await intersect(["ai", "system"]);
    await intersect(["system", "ai"]);
    await intersect(["ai", "system"]);

    expect(core.reportVisibleModules).toHaveBeenCalledOnce();
  });

  it("Settings reports an empty set and returning restores the visible set and context", async () => {
    const { core } = fakeCore();
    render(Panel, { core });
    await screen.findByRole("region", { name: "IA" });
    await intersect(["trello", "network"]);
    const opener = within(nav()).getByRole("button", { name: "Configurações" });

    await userEvent.click(opener);

    expect(core.reportVisibleModules).toHaveBeenLastCalledWith([]);
    expect(screen.getByRole("heading", { name: "Configurações" })).toBeInTheDocument();
    expect(screen.queryByRole("group", { name: "Execução" })).not.toBeInTheDocument();
    // Intersection events while in Settings do not count.
    await intersect(["ai"]);
    expect(core.reportVisibleModules).toHaveBeenLastCalledWith([]);

    await userEvent.click(screen.getByRole("button", { name: "Voltar ao Painel" }));

    expect(core.reportVisibleModules).toHaveBeenLastCalledWith(["ai"]);
    expect(screen.queryByRole("heading", { name: "Configurações" })).not.toBeInTheDocument();
    expect(screen.getByRole("group", { name: "Execução" })).toBeInTheDocument();
    expect(opener).toHaveFocus();
    expect(core.chooseAnchor).not.toHaveBeenCalled();
  });

  it("choosing a module from Settings returns to the panel at that module", async () => {
    const { core } = fakeCore();
    render(Panel, { core });
    await screen.findByRole("region", { name: "IA" });
    await userEvent.click(within(nav()).getByRole("button", { name: "Configurações" }));

    await userEvent.click(within(nav()).getByRole("button", { name: "Sistema" }));

    expect(screen.queryByRole("heading", { name: "Configurações" })).not.toBeInTheDocument();
    expect(section("Sistema")).toHaveFocus();
    expect(core.chooseAnchor).toHaveBeenLastCalledWith("system");
  });
});

it("sends the network automatic speed preference from Settings", async () => {
  const { core } = fakeCore({ snapshots: [networkSpeed as Snapshot] });
  const setAutomaticSpeed = vi.fn();
  render(Panel, { core: { ...core, setAutomaticSpeed } });
  await screen.findByRole("region", { name: "Rede" });
  await userEvent.click(within(nav()).getByRole("button", { name: "Configurações" }));
  const toggle = screen.getByRole("checkbox", { name: "Teste de velocidade automático" });
  expect(toggle).toBeChecked();
  await userEvent.click(toggle);
  expect(setAutomaticSpeed).toHaveBeenCalledWith(false);
});

it("sends a pasted GitHub token from Settings to the core", async () => {
  const { default: missing } = await import("./github/fixtures/missing-gh.json");
  const { core } = fakeCore({ snapshots: [{ module: "github", updatedAt: null, data: missing }] });
  const saveGithubToken = vi.fn().mockResolvedValue("exemplo");
  render(Panel, { core: { ...core, saveGithubToken } });
  await screen.findByRole("region", { name: "GitHub" });
  await userEvent.click(within(nav()).getByRole("button", { name: "Configurações" }));
  expect(screen.getByRole("heading", { name: "Configurações" })).toBeInTheDocument();
  await userEvent.type(screen.getByLabelText("Token clássico (escopo repo)"), "private-token");
  const settings = screen.getByRole("region", { name: "GitHub" });
  await userEvent.click(within(settings).getByRole("button", { name: "Validar e salvar" }));
  expect(saveGithubToken).toHaveBeenCalledWith("private-token");
  expect(await screen.findByText("Conectado como exemplo")).toBeInTheDocument();
});

it("sends Trello key and token from Settings to the core", async () => {
  const { default: empty } = await import("./trello/fixtures/empty.json");
  const { core } = fakeCore({ snapshots: [{ module: "trello", updatedAt: null, data: empty }] });
  const connectTrello = vi.fn().mockResolvedValue("exemplo");
  render(Panel, { core: { ...core, connectTrello } });
  await screen.findByRole("region", { name: "Trello" });
  await userEvent.click(within(nav()).getByRole("button", { name: "Configurações" }));
  expect(core.reportVisibleModules).toHaveBeenLastCalledWith([]);
  const trello = screen.getByRole("region", { name: "Trello" });
  await userEvent.type(within(trello).getByLabelText("API key"), "0123abcd");
  await userEvent.type(within(trello).getByLabelText("Token"), "private-token");
  await userEvent.click(within(trello).getByRole("button", { name: "Validar e salvar" }));
  expect(connectTrello).toHaveBeenCalledWith("0123abcd", "private-token");
  expect(await within(trello).findByText("Conectado como exemplo")).toBeInTheDocument();
});

it("mirrors and clears the waiting alert on the Agents navigation item", async () => {
  const empty = (await import("./agents/fixtures/empty.json")).default;
  const { core, publish } = fakeCore({ snapshots: [{ module: "agents", updatedAt: 1, data: waitingAgents }] });
  render(Panel, { core });
  await screen.findByRole("region", { name: "IA" });
  expect(await within(nav()).findByLabelText("1 aguardando você")).toBeInTheDocument();
  await publish({ module: "agents", updatedAt: 2, data: { ...waitingAgents, sessions: [waitingAgents.sessions[0], { ...waitingAgents.sessions[0], id: "second" }] } });
  expect(within(nav()).getByLabelText("2 aguardando você")).toBeInTheDocument();
  await publish({ module: "agents", updatedAt: 3, data: empty });
  expect(within(nav()).queryByLabelText(/aguardando você/)).not.toBeInTheDocument();
});

it("opens at Agents for a notification target without changing the last anchor", async () => {
  const scroll = vi.mocked(Element.prototype.scrollIntoView);
  const { core } = fakeCore({ lastAnchor: "agents", agentTarget: waitingAgents.sessions[0].id,
    snapshots: [{ module: "agents", updatedAt: 1000, data: waitingAgents }] });
  render(Panel, { core });
  await screen.findByRole("region", { name: "Agentes" });
  await vi.waitFor(() => expect(scroll.mock.contexts).toContain(section("Agentes")));
  expect(core.chooseAnchor).not.toHaveBeenCalled();
});

it("shows three enabled Agent notification switches and sends the changed preferences", async () => {
  const { core } = fakeCore({ snapshots: [{ module: "agents", updatedAt: 1000, data: waitingAgents }] });
  const setAgentNotifications = vi.fn();
  render(Panel, { core: { ...core, setAgentNotifications } });
  await screen.findByRole("region", { name: "IA" });
  await userEvent.click(within(nav()).getByRole("button", { name: "Configurações" }));
  for (const name of ["Aguardando", "Turno concluído", "Falha"]) expect(screen.getByRole("checkbox", { name })).toBeChecked();
  await userEvent.click(screen.getByRole("checkbox", { name: "Aguardando" }));
  expect(setAgentNotifications).toHaveBeenCalledWith({ waiting: false, done: true, failed: true });
});

it("brings an already open panel, even in Settings, to the session targeted by a notification", async () => {
  const scroll = vi.mocked(Element.prototype.scrollIntoView);
  const { core } = fakeCore({ lastAnchor: "network", snapshots: [{ module: "agents", updatedAt: 1000, data: waitingAgents }] });
  let target: ((session: string) => void) | undefined;
  render(Panel, { core: { ...core, listenAgentTarget: async (cb: (session: string) => void) => { target = cb; return () => {}; } } });
  await screen.findByRole("region", { name: "Rede" });
  await userEvent.click(within(nav()).getByRole("button", { name: "Configurações" }));
  scroll.mockClear();

  await act(() => target?.(waitingAgents.sessions[0].id));

  await vi.waitFor(() => expect(scroll.mock.contexts).toContain(section("Agentes")));
  expect(screen.queryByRole("heading", { name: "Configurações" })).not.toBeInTheDocument();
  const targeted = await screen.findByRole("listitem", { current: true });
  expect(within(targeted).getByText("Minha sessão")).toBeInTheDocument();
  await vi.waitFor(() => expect(scroll.mock.contexts).toContain(targeted));
  expect(core.chooseAnchor).not.toHaveBeenCalled();
});

it("keeps focus on a module chosen in the navigation when a snapshot arrives", async () => {
  const { core, publish } = fakeCore({ snapshots: [{ module: "ai", updatedAt: 1, data: aiFull }] });
  render(Panel, { core });
  await screen.findByRole("region", { name: "IA" });
  await userEvent.click(within(nav()).getByRole("button", { name: "IA" }));

  await publish({ module: "ai", updatedAt: 2, data: aiFull });

  expect(section("IA")).toHaveFocus();
});

describe("AI module in the panel", () => {
  it("explains there is no reading yet and keeps its actions before the first snapshot", async () => {
    const { core } = fakeCore();
    const refreshAi = vi.fn();
    render(Panel, { core: { ...core, refreshAi } });
    const ai = await screen.findByRole("region", { name: "IA" });
    expect(within(ai).getByText("Sem leitura dos limites do Codex ainda · use o Codex ou atualize com ↻")).toBeInTheDocument();
    expect(within(ai).getByRole("button", { name: "Configurar IA" })).toBeInTheDocument();
    await userEvent.click(within(ai).getByRole("button", { name: "Atualizar IA" }));
    expect(refreshAi).toHaveBeenCalledOnce();
  });

  it("shows the AI instruments and sends a refresh to the core", async () => {
    const { core } = fakeCore({ snapshots: [{ module: "ai", updatedAt: 1, data: aiFull }] });
    const refreshAi = vi.fn();
    render(Panel, { core: { ...core, refreshAi } });
    const ai = await screen.findByRole("region", { name: "IA" });
    expect(within(ai).getByRole("meter", { name: "Codex · janela de 5 h" })).toBeInTheDocument();
    await userEvent.click(within(ai).getByRole("button", { name: "Atualizar IA" }));
    expect(refreshAi).toHaveBeenCalledOnce();
  });

  it("opens Settings from the module and returns to it", async () => {
    const { core } = fakeCore({ snapshots: [{ module: "ai", updatedAt: 1, data: aiFull }] });
    render(Panel, { core });
    await userEvent.click(await screen.findByRole("button", { name: "Configurar IA" }));
    expect(screen.getByRole("heading", { name: "Configurações" })).toBeInTheDocument();
    expect(core.reportVisibleModules).toHaveBeenLastCalledWith([]);
    await userEvent.click(screen.getByRole("button", { name: "Voltar ao Painel" }));
    expect(screen.getByRole("button", { name: "Configurar IA" })).toHaveFocus();
  });

  it("reconnects Claude only on click and falls back to Settings when it fails", async () => {
    const { default: replaced } = await import("./ai/fixtures/claude-replaced.json");
    const { core } = fakeCore({ snapshots: [{ module: "ai", updatedAt: 1, data: replaced }] });
    const connectClaude = vi.fn().mockRejectedValue("unwritable");
    render(Panel, { core: { ...core, connectClaude } });
    const reconnect = await screen.findByRole("button", { name: "Reconectar" });
    expect(connectClaude).not.toHaveBeenCalled();
    await userEvent.click(reconnect);
    expect(connectClaude).toHaveBeenCalledOnce();
    expect(await screen.findByRole("heading", { name: "Configurações" })).toBeInTheDocument();
  });
});

describe("Agents module in the panel", () => {
  it("explains there is no reading yet before the first snapshot", async () => {
    const { core } = fakeCore();
    render(Panel, { core });
    const agents = await screen.findByRole("region", { name: "Agentes" });
    expect(within(agents).getByText("Sem leitura das sessões de agente ainda")).toBeInTheDocument();
  });

  it("shows the sessions and the intervention count from the snapshot", async () => {
    const { core } = fakeCore({ snapshots: [{ module: "agents", updatedAt: 1, data: waitingAgents }] });
    render(Panel, { core });
    const agents = await screen.findByRole("region", { name: "Agentes" });
    expect(within(agents).getByRole("group", { name: "Intervenção" })).toHaveTextContent("01");
    expect(within(within(agents).getByRole("list", { name: "Sessões de agente" })).getByText("Minha sessão")).toBeInTheDocument();
  });

  it("highlights the notified session when the webview is recreated with a target", async () => {
    const scroll = vi.mocked(Element.prototype.scrollIntoView);
    const { core } = fakeCore({ lastAnchor: "network", agentTarget: waitingAgents.sessions[0].id,
      snapshots: [{ module: "agents", updatedAt: 1000, data: waitingAgents }] });
    render(Panel, { core });
    const targeted = await screen.findByRole("listitem", { current: true });
    expect(within(targeted).getByText("Da notificação")).toBeInTheDocument();
    await vi.waitFor(() => expect(scroll.mock.contexts).toContain(targeted));
    expect(within(nav()).getByRole("button", { name: /Agentes/ })).toHaveAttribute("aria-current", "location");
    expect(core.chooseAnchor).not.toHaveBeenCalled();
  });

  it("drops the notification highlight once the user chooses a module by keyboard", async () => {
    const { core } = fakeCore({ agentTarget: waitingAgents.sessions[0].id,
      snapshots: [{ module: "agents", updatedAt: 1000, data: waitingAgents }] });
    render(Panel, { core });
    await screen.findByRole("listitem", { current: true });
    within(nav()).getByRole("button", { name: /Agentes/ }).focus();
    await userEvent.keyboard("{Enter}");
    expect(section("Agentes")).toHaveFocus();
    expect(core.chooseAnchor).toHaveBeenLastCalledWith("agents");
    expect(screen.queryByRole("listitem", { current: true })).not.toBeInTheDocument();
  });
});

describe("GitHub module in the panel", () => {
  it("explains there is no reading yet and sends a refresh to the core", async () => {
    const { core } = fakeCore();
    const refreshGithub = vi.fn();
    render(Panel, { core: { ...core, refreshGithub } });
    const github = await screen.findByRole("region", { name: "GitHub" });
    expect(within(github).getByText("Sem leitura da atividade do GitHub ainda")).toBeInTheDocument();
    await userEvent.click(within(github).getByRole("button", { name: "Atualizar GitHub" }));
    expect(refreshGithub).toHaveBeenCalledOnce();
  });

  it("shows the activity and featured repository from the snapshot", async () => {
    const { core } = fakeCore({ snapshots: [{ module: "github", updatedAt: 1, data: githubFull }] });
    render(Panel, { core });
    const github = await screen.findByRole("region", { name: "GitHub" });
    expect(within(github).getByRole("region", { name: "Repo em destaque" })).toHaveTextContent("exemplo/repo-1");
    expect(within(github).getByRole("list", { name: "Contribuições por dia nos últimos 14 dias" })).toBeInTheDocument();
  });

  it("opens Settings from the module and returns focus to it", async () => {
    const { core } = fakeCore({ snapshots: [{ module: "github", updatedAt: 1, data: githubFull }] });
    render(Panel, { core });
    const github = await screen.findByRole("region", { name: "GitHub" });
    await userEvent.click(within(github).getByRole("button", { name: "Configurar GitHub" }));
    expect(screen.getByRole("heading", { name: "Configurações" })).toBeInTheDocument();
    await userEvent.click(screen.getByRole("button", { name: "Voltar ao Painel" }));
    expect(within(github).getByRole("button", { name: "Configurar GitHub" })).toHaveFocus();
  });

  it("keeps focus and scroll position when a GitHub snapshot arrives", async () => {
    const { core, publish } = fakeCore({ snapshots: [{ module: "github", updatedAt: 1, data: githubFull }] });
    render(Panel, { core });
    const github = await screen.findByRole("region", { name: "GitHub" });
    const configure = within(github).getByRole("button", { name: "Configurar GitHub" });
    configure.focus();
    const scroller = screen.getByRole("main");
    scroller.scrollTop = 120;

    await publish({ module: "github", updatedAt: 2, data: { ...githubFull, refreshing: true } });

    expect(within(github).getByRole("status")).toHaveTextContent("Atualizando…");
    expect(configure).toHaveFocus();
    expect(scroller.scrollTop).toBe(120);
  });
});

describe("Trello module in the panel", () => {
  it("explains there is no board yet and sends a refresh to the core", async () => {
    const { core } = fakeCore();
    const refreshTrello = vi.fn();
    render(Panel, { core: { ...core, refreshTrello } });
    const trello = await screen.findByRole("region", { name: "Trello" });
    expect(within(trello).getByText("Sem leitura do quadro do Trello ainda")).toBeInTheDocument();
    await userEvent.click(within(trello).getByRole("button", { name: "Atualizar Trello" }));
    expect(refreshTrello).toHaveBeenCalledOnce();
  });

  it("moves and opens cards through the core", async () => {
    const { core } = fakeCore({ snapshots: [{ module: "trello", updatedAt: 1, data: trelloFull }] });
    const moveTrelloCard = vi.fn(async () => {});
    const openTrelloCard = vi.fn();
    render(Panel, { core: { ...core, moveTrelloCard, openTrelloCard } });
    const trello = await screen.findByRole("region", { name: "Trello" });
    const [from, to] = trelloFull.lists;
    const card = from.cards[0];
    await userEvent.click(within(trello).getByRole("button", { name: new RegExp(card.name) }));
    await userEvent.click(within(trello).getByRole("button", { name: `Abrir ${card.name} no navegador` }));
    expect(openTrelloCard).toHaveBeenCalledWith(card.url);
    await userEvent.click(within(trello).getByRole("button", { name: `Mover para ${to.name}` }));
    expect(moveTrelloCard).toHaveBeenCalledWith(card.id, to.id);
  });

  it("reverts a move when the core cannot reach Trello", async () => {
    const { core } = fakeCore({ snapshots: [{ module: "trello", updatedAt: 1, data: trelloFull }] });
    render(Panel, { core });
    const trello = await screen.findByRole("region", { name: "Trello" });
    const [from, to] = trelloFull.lists;
    await userEvent.click(within(trello).getByRole("button", { name: new RegExp(from.cards[0].name) }));
    await userEvent.click(within(trello).getByRole("button", { name: `Mover para ${to.name}` }));
    expect(await within(trello).findByRole("alert")).toHaveTextContent("Não foi possível mover");
    expect(within(within(trello).getByRole("region", { name: from.name })).getByText(from.cards[0].name)).toBeInTheDocument();
  });

  it("opens Settings from the module and returns focus to it", async () => {
    const { core } = fakeCore({ snapshots: [{ module: "trello", updatedAt: 1, data: trelloFull }] });
    render(Panel, { core });
    const trello = await screen.findByRole("region", { name: "Trello" });
    await userEvent.click(within(trello).getByRole("button", { name: "Configurar Trello" }));
    expect(screen.getByRole("heading", { name: "Configurações" })).toBeInTheDocument();
    await userEvent.click(screen.getByRole("button", { name: "Voltar ao Painel" }));
    expect(within(trello).getByRole("button", { name: "Configurar Trello" })).toHaveFocus();
  });

  it("keeps the selected card, focus and scroll position when a Trello snapshot arrives", async () => {
    const { core, publish } = fakeCore({ snapshots: [{ module: "trello", updatedAt: 1, data: trelloFull }] });
    render(Panel, { core });
    const trello = await screen.findByRole("region", { name: "Trello" });
    const card = within(trello).getByRole("button", { name: new RegExp(trelloFull.lists[0].cards[0].name) });
    await userEvent.click(card);
    const scroller = screen.getByRole("main");
    scroller.scrollTop = 120;

    await publish({ module: "trello", updatedAt: 2, data: { ...trelloFull, refreshing: true } });

    expect(within(trello).getByRole("status")).toHaveTextContent("Atualizando…");
    expect(card).toHaveFocus();
    expect(card).toHaveAttribute("aria-pressed", "true");
    expect(scroller.scrollTop).toBe(120);
  });
});

describe("Network module in the panel", () => {
  it("explains there is no reading before the first network snapshot", async () => {
    const { core } = fakeCore();
    render(Panel, { core });
    const network = await screen.findByRole("region", { name: "Rede" });
    expect(within(network).getByText("Sem leitura da rede ainda")).toBeInTheDocument();
  });

  it("shows the instruments from the snapshot and sends a speed test to the core", async () => {
    const { core } = fakeCore({ snapshots: [networkFull as Snapshot] });
    const runSpeedTest = vi.fn();
    render(Panel, { core: { ...core, runSpeedTest } });
    const network = await screen.findByRole("region", { name: "Rede" });
    expect(within(network).getByRole("group", { name: "Latência da internet" })).toHaveTextContent("20 ms");
    expect(within(network).getAllByRole("listitem")).toHaveLength(6);
    await userEvent.click(within(network).getByRole("button", { name: "Testar velocidade" }));
    expect(runSpeedTest).toHaveBeenCalledOnce();
  });

  it("keeps focus on the speed test when a network snapshot arrives", async () => {
    const { core, publish } = fakeCore({ snapshots: [networkFull as Snapshot] });
    render(Panel, { core });
    const network = await screen.findByRole("region", { name: "Rede" });
    within(network).getByRole("button", { name: "Testar velocidade" }).focus();
    await publish({ ...(networkFull as Snapshot), updatedAt: networkFull.updatedAt + 1000 });
    expect(within(network).getByRole("button", { name: "Testar velocidade" })).toHaveFocus();
  });
});
