import { act, render, screen, within } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import Panel from "../Panel.svelte";
import type { Appearance, Core, GeneralSettings, InitialState, Snapshot, SystemTheme } from "../core";
import aiOff from "../ai/fixtures/claude-off.json";
import aiConnected from "../ai/fixtures/claude-connected.json";
import githubMissing from "../github/fixtures/missing-gh.json";
import githubFull from "../github/fixtures/full.json";
import trelloEmpty from "../trello/fixtures/empty.json";
import trelloFull from "../trello/fixtures/full.json";
import trelloChooseBoard from "../trello/fixtures/choose-board.json";

/** Media queries the tests control, standing in for the webview environment. */
const media: Record<string, { matches: boolean; listeners: ((e: { matches: boolean }) => void)[] }> = {};
function query(name: string) {
  return (media[name] ??= { matches: false, listeners: [] });
}
function setMedia(name: string, matches: boolean) {
  const q = query(name);
  q.matches = matches;
  q.listeners.forEach((l) => l({ matches }));
}

beforeEach(() => {
  for (const key of Object.keys(media)) delete media[key];
  vi.stubGlobal("matchMedia", (name: string) => ({
    get matches() { return query(name).matches; },
    addEventListener: (_: string, l: (e: { matches: boolean }) => void) => query(name).listeners.push(l),
    removeEventListener: (_: string, l: (e: { matches: boolean }) => void) => {
      const q = query(name);
      q.listeners = q.listeners.filter((x) => x !== l);
    },
  }));
  delete document.documentElement.dataset.theme;
  delete document.documentElement.dataset.motion;
});
afterEach(() => vi.unstubAllGlobals());

const GENERAL: GeneralSettings = { version: "0.1.0", autostart: true, versionCheck: true, release: null };

function fakeCore(initial: Partial<InitialState> = {}, general: GeneralSettings = GENERAL) {
  let generalListener: ((settings: GeneralSettings) => void) | undefined;
  let themeListener: ((theme: SystemTheme) => void) | undefined;
  const core = {
    initialState: vi.fn(async (): Promise<InitialState> => ({ lastAnchor: "ai", snapshots: [], ...initial })),
    listenSnapshots: vi.fn(async () => () => {}),
    listenSystemTheme: vi.fn(async (cb: (theme: SystemTheme) => void) => {
      themeListener = cb;
      return () => {};
    }),
    reportVisibleModules: vi.fn(),
    chooseAnchor: vi.fn(),
    closePanel: vi.fn(),
    completeWelcome: vi.fn(),
    listenGeneralSettings: vi.fn(async (cb: (settings: GeneralSettings) => void) => { generalListener = cb; return () => {}; }),
    generalSettings: vi.fn(async () => general),
    setAutostart: vi.fn(async (enabled: boolean) => ({ ...general, autostart: enabled })),
    setVersionCheck: vi.fn(async (enabled: boolean) => ({ ...general, versionCheck: enabled, release: enabled ? general.release : null })),
    openRelease: vi.fn(),
    openLogs: vi.fn(async () => {}),
    setAppearance: vi.fn(async (_appearance: Appearance) => {}),
    connectClaude: vi.fn(async () => {}),
    refreshGithub: vi.fn(),
  } satisfies Core;
  return { core, generalChanged: (settings: GeneralSettings) => act(() => generalListener?.(settings)), systemTheme: (theme: SystemTheme) => act(() => themeListener?.(theme)) };
}

const snapshot = (module: Snapshot["module"], data: unknown): Snapshot => ({ module, updatedAt: 1, data });
const openSettings = () => userEvent.click(screen.getByRole("button", { name: "Configurações" }));

describe("first run", () => {
  it("opens Settings with the welcome, the status of each module and its shortcuts", async () => {
    const { core } = fakeCore({
      welcome: true,
      snapshots: [snapshot("ai", aiOff), snapshot("github", githubMissing), snapshot("trello", trelloFull)],
    });
    render(Panel, { core });

    const welcome = await screen.findByRole("region", { name: "Boas-vindas" });
    expect(screen.getByRole("heading", { name: "Configurações" })).toBeInTheDocument();
    expect(core.completeWelcome).toHaveBeenCalledOnce();
    expect(core.reportVisibleModules).toHaveBeenLastCalledWith([]);

    const row = (name: string) => within(welcome).getByRole("listitem", { name });
    expect(row("IA")).toHaveTextContent("Precisa configurar");
    expect(row("GitHub")).toHaveTextContent("Precisa configurar");
    expect(row("Trello")).toHaveTextContent("Detectado");
    for (const name of ["Agentes", "Rede", "Sistema"]) expect(row(name)).toHaveTextContent("Não precisa configurar");

    // The Claude Code shortcut acts only on this click.
    expect(core.connectClaude).not.toHaveBeenCalled();
    await userEvent.click(within(row("IA")).getByRole("button", { name: "Conectar ao Claude Code" }));
    expect(core.connectClaude).toHaveBeenCalledOnce();

    await userEvent.click(within(row("GitHub")).getByRole("button", { name: "Configurar GitHub" }));
    expect(document.getElementById("settings-github")).toHaveFocus();
    expect(within(row("Agentes")).queryByRole("button")).not.toBeInTheDocument();
  });

  it("explains modules still being checked and those that need a board or a connection", async () => {
    const { core } = fakeCore({ welcome: true, snapshots: [snapshot("trello", trelloChooseBoard), snapshot("ai", aiConnected)] });
    render(Panel, { core });

    const welcome = await screen.findByRole("region", { name: "Boas-vindas" });
    expect(within(welcome).getByRole("listitem", { name: "Trello" })).toHaveTextContent("Precisa configurar");
    expect(within(welcome).getByRole("listitem", { name: "IA" })).toHaveTextContent("Detectado");
    expect(within(welcome).getByRole("listitem", { name: "GitHub" })).toHaveTextContent("Verificando");
  });

  it("leaves the welcome for the panel and never shows it on later openings", async () => {
    const { core } = fakeCore({ welcome: true, snapshots: [snapshot("trello", trelloEmpty), snapshot("github", githubFull)] });
    render(Panel, { core });
    await screen.findByRole("region", { name: "Boas-vindas" });

    await userEvent.click(screen.getByRole("button", { name: "Voltar ao Painel" }));
    expect(screen.queryByRole("heading", { name: "Configurações" })).not.toBeInTheDocument();
    await openSettings();
    expect(screen.queryByRole("region", { name: "Boas-vindas" })).not.toBeInTheDocument();
  });

  it("opens on the panel after the first run", async () => {
    const { core } = fakeCore({ welcome: false });
    render(Panel, { core });
    await screen.findByRole("region", { name: "IA" });
    expect(screen.queryByRole("heading", { name: "Configurações" })).not.toBeInTheDocument();
    expect(core.completeWelcome).not.toHaveBeenCalled();
  });
});

describe("general settings", () => {
  it("turns autostart and the version check off and on", async () => {
    const { core } = fakeCore();
    render(Panel, { core });
    await openSettings();
    const general = await screen.findByRole("region", { name: "Geral" });

    const autostart = await within(general).findByRole("checkbox", { name: "Iniciar com o login" });
    expect(autostart).toBeChecked();
    await userEvent.click(autostart);
    expect(core.setAutostart).toHaveBeenCalledWith(false);
    expect(autostart).not.toBeChecked();

    const check = within(general).getByRole("checkbox", { name: "Verificar nova versão diariamente" });
    expect(check).toBeChecked();
    await userEvent.click(check);
    expect(core.setVersionCheck).toHaveBeenCalledWith(false);
    expect(check).not.toBeChecked();
    expect(within(general).getByText("kvojps-monitor 0.1.0")).toBeInTheDocument();
  });

  it("shows the autostart failure next to its switch and keeps the previous state", async () => {
    const { core } = fakeCore();
    core.setAutostart.mockRejectedValueOnce("unwritable");
    render(Panel, { core });
    await openSettings();
    const autostart = await screen.findByRole("checkbox", { name: "Iniciar com o login" });

    await userEvent.click(autostart);

    expect(autostart).toBeChecked();
    expect(autostart).toHaveAccessibleDescription("Não foi possível alterar a inicialização com o login");
  });

  it("explains a failed version preference and keeps its confirmed value", async () => {
    const { core } = fakeCore();
    core.setVersionCheck.mockRejectedValueOnce("unwritable");
    render(Panel, { core });
    await openSettings();
    const check = await screen.findByRole("checkbox", { name: "Verificar nova versão diariamente" });
    await userEvent.click(check);
    expect(check).toBeChecked();
    expect(check).toHaveAccessibleDescription("Não foi possível alterar a verificação de versão");
  });

  it("explains a failed settings load and lets the user retry", async () => {
    const { core } = fakeCore();
    core.generalSettings.mockRejectedValueOnce("unavailable");
    render(Panel, { core });
    await openSettings();
    expect(await screen.findByRole("alert")).toHaveTextContent("Não foi possível carregar as configurações gerais");
    await userEvent.click(screen.getByRole("button", { name: "Tentar novamente" }));
    expect(await screen.findByRole("checkbox", { name: "Iniciar com o login" })).toBeChecked();
  });

  it("links discreetly to a newer release without updating by itself", async () => {
    const { core } = fakeCore({}, { ...GENERAL, release: { version: "0.2.0", url: "https://github.com/Kvojps/kvojps-monitor/releases/tag/v0.2.0" } });
    render(Panel, { core });
    await openSettings();

    await userEvent.click(await screen.findByRole("button", { name: "Versão 0.2.0 disponível · ver release" }));
    expect(core.openRelease).toHaveBeenCalledOnce();
  });

  it("shows a release discovered while Settings stays open", async () => {
    const { core, generalChanged } = fakeCore();
    render(Panel, { core });
    await openSettings();
    await screen.findByRole("checkbox", { name: "Iniciar com o login" });
    await generalChanged({ ...GENERAL, release: { version: "0.2.0", url: "https://github.com/Kvojps/kvojps-monitor/releases/tag/v0.2.0" } });
    expect(screen.getByRole("button", { name: "Versão 0.2.0 disponível · ver release" })).toBeInTheDocument();
  });

  it("opens the logs folder", async () => {
    const { core } = fakeCore();
    render(Panel, { core });
    await openSettings();

    await userEvent.click(await screen.findByRole("button", { name: "Abrir pasta de logs" }));
    expect(core.openLogs).toHaveBeenCalledOnce();
  });
});

describe("appearance", () => {
  const appearance = (a: Partial<Appearance>): Partial<InitialState> => ({ appearance: { theme: "system", reduceMotion: false, ...a } });

  it("applies the saved theme and persists a new choice", async () => {
    const { core } = fakeCore(appearance({ theme: "light" }));
    render(Panel, { core });
    await screen.findByRole("region", { name: "IA" });
    await vi.waitFor(() => expect(document.documentElement.dataset.theme).toBe("light"));

    await openSettings();
    await userEvent.click(screen.getByRole("radio", { name: "Escuro" }));

    expect(document.documentElement.dataset.theme).toBe("dark");
    expect(core.setAppearance).toHaveBeenCalledWith({ theme: "dark", reduceMotion: false });
  });

  it("explains a failed appearance save and restores the confirmed preference", async () => {
    const { core } = fakeCore(appearance({ theme: "light" }));
    core.setAppearance.mockRejectedValueOnce("unwritable");
    render(Panel, { core });
    await openSettings();
    await userEvent.click(screen.getByRole("radio", { name: "Escuro" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Não foi possível salvar a aparência");
    expect(screen.getByRole("radio", { name: "Claro" })).toBeChecked();
    expect(document.documentElement.dataset.theme).toBe("light");
  });

  it("follows the desktop theme while the system theme is chosen", async () => {
    const { core, systemTheme } = fakeCore({ ...appearance({ theme: "system" }), systemTheme: "dark" });
    render(Panel, { core });
    await vi.waitFor(() => expect(document.documentElement.dataset.theme).toBe("dark"));

    await systemTheme("light");
    expect(document.documentElement.dataset.theme).toBe("light");

    await openSettings();
    await userEvent.click(screen.getByRole("radio", { name: "Escuro" }));
    await systemTheme("light");
    expect(document.documentElement.dataset.theme).toBe("dark");
  });

  it("falls back to the webview color scheme without the desktop portal", async () => {
    setMedia("(prefers-color-scheme: light)", true);
    const { core } = fakeCore({ ...appearance({ theme: "system" }), systemTheme: null });
    render(Panel, { core });
    await vi.waitFor(() => expect(document.documentElement.dataset.theme).toBe("light"));

    await act(() => setMedia("(prefers-color-scheme: light)", false));
    expect(document.documentElement.dataset.theme).toBe("dark");
  });

  it("reduces motion by choice and always when the system asks", async () => {
    const { core } = fakeCore({ ...appearance({}), panelVisible: true });
    render(Panel, { core });
    await openSettings();
    expect(document.documentElement.dataset.decoration).toBe("running");
    expect(document.documentElement.dataset.motion).toBe("full");

    const reduce = screen.getByRole("checkbox", { name: "Reduzir animações" });
    await userEvent.click(reduce);
    expect(document.documentElement.dataset.motion).toBe("reduced");
    expect(document.documentElement.dataset.decoration).toBe("paused");
    expect(core.setAppearance).toHaveBeenLastCalledWith({ theme: "system", reduceMotion: true });

    await userEvent.click(reduce);
    await act(() => setMedia("(prefers-reduced-motion: reduce)", true));
    expect(document.documentElement.dataset.motion).toBe("reduced");
    expect(document.documentElement.dataset.decoration).toBe("paused");
    await act(() => setMedia("(prefers-reduced-motion: reduce)", false));
    expect(document.documentElement.dataset.decoration).toBe("running");
    await act(() => setMedia("(prefers-reduced-motion: reduce)", true));
    expect(reduce).toHaveAccessibleDescription("O sistema já pede menos animações; essa preferência prevalece.");
  });
});

describe("configure shortcuts", () => {
  it("opens Settings at the section of the module asking for configuration", async () => {
    const { core } = fakeCore({ snapshots: [snapshot("github", githubMissing)] });
    render(Panel, { core });

    await userEvent.click(await screen.findByRole("button", { name: "Configurar GitHub" }));

    expect(document.getElementById("settings-github")).toHaveFocus();
    await userEvent.click(screen.getByRole("button", { name: "Voltar ao Painel" }));
    expect(screen.getByRole("button", { name: "Configurar GitHub" })).toHaveFocus();
  });
});
