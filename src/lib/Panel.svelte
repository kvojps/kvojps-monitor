<script lang="ts">
  import { onMount, tick, type Snippet } from "svelte";
  import { MODULES, type AgentsData, type AiData, type GithubData, type TrelloData, type NetworkData, type SystemData, type Module, type Core, type Snapshot } from "./core";
  import AgentsModule from "./agents/AgentsModule.svelte";
  import AiModule from "./ai/AiModule.svelte";
  import GithubModule from "./github/GithubModule.svelte";
  import NetworkModule from "./network/NetworkModule.svelte";
  import SystemModule from "./system/SystemModule.svelte";
  import TrelloModule from "./trello/TrelloModule.svelte";
  import { AppearanceState } from "./settings/appearance.svelte";
  import Settings from "./settings/Settings.svelte";
  import { interpolate, t } from "./strings";

  let { core }: { core: Core } = $props();

  type Station = keyof typeof t.stations;
  const stations: { key: Station; modules: [Module, Module] }[] = [
    { key: "capacity", modules: ["ai", "system"] },
    { key: "execution", modules: ["agents", "trello"] },
    { key: "observation", modules: ["github", "network"] },
  ];

  let area = $state<"panel" | "settings">("panel");
  /** Last anchor chosen in the navigation; independent from what is visible. */
  let anchor = $state<Module | null>(null);
  let agentTarget = $state<string | null>(null);
  let snapshots = $state<Partial<Record<Module, Snapshot>>>({});

  /** First opening: Settings shows the welcome until the user goes back to the panel. */
  let welcome = $state(false);
  const appearance = new AppearanceState();
  let nativeVisible = $state(false);
  let appearanceRestored = $state(false);
  let pageVisible = $state(!document.hidden);
  $effect(() => {
    const state = appearanceRestored && nativeVisible && pageVisible && !appearance.reduced ? "running" : "paused";
    document.documentElement.dataset.decoration = state;
    // Bounded in-memory trace: no source data or per-frame callbacks.
    performance.clearMarks?.("panel-decoration");
    performance.mark?.("panel-decoration", { detail: { state } });
  });
  $effect(() => {
    document.documentElement.dataset.theme = appearance.theme;
    document.documentElement.dataset.motion = appearance.reduced ? "reduced" : "full";
  });

  let scroller: HTMLElement;
  let settings = $state<ReturnType<typeof Settings>>();
  const sections: Partial<Record<Module, HTMLElement>> = {};
  /** Element focused before opening Settings, focused again on return. */
  let returnFocus: HTMLElement | null = null;

  /** Modules intersecting the visible panel area, kept outside reactivity. */
  const intersecting = new Set<Module>();
  let reported: string | null = null;

  function receive(s: Snapshot) {
    const previous = snapshots[s.module];
    if (!previous || (previous.updatedAt ?? 0) <= (s.updatedAt ?? 0)) snapshots[s.module] = s;
  }

  /** Report the visible set to the core only when it changes. */
  function syncVisibility() {
    const modules = area === "panel" ? MODULES.filter((m) => intersecting.has(m)) : [];
    const key = modules.join();
    if (key === reported) return;
    reported = key;
    core.reportVisibleModules(modules);
  }

  function reveal(module: Module, focus: boolean) {
    const section = sections[module];
    if (!section) return;
    section.scrollIntoView({ block: "start", behavior: focus && !appearance.reduced ? "smooth" : "auto" });
    if (focus) section.focus({ preventScroll: true });
  }

  async function showPanel() {
    if (area === "panel") return;
    area = "panel";
    await tick();
    syncVisibility();
  }

  async function choose(module: Module) {
    agentTarget = null;
    anchor = module;
    core.chooseAnchor(module);
    await showPanel();
    reveal(module, true);
  }

  /** Open Settings, at the section of module when given. */
  async function openSettings(module?: Module) {
    if (area !== "settings") {
      returnFocus = document.activeElement instanceof HTMLElement ? document.activeElement : null;
      area = "settings";
      syncVisibility();
      await tick();
    }
    await settings?.focus(module);
  }

  /** The panel stays mounted under Settings, so scroll and module state survive. */
  async function backToPanel() {
    welcome = false;
    await showPanel();
    if (returnFocus?.isConnected) returnFocus.focus({ preventScroll: true });
    returnFocus = null;
  }

  async function target(session: string) {
    agentTarget = session;
    anchor = "agents";
    await showPanel();
    await tick();
    reveal("agents", false);
  }

  const NO_AI_READING: AiData = { refreshing: false, codex: { state: "empty", plan: null, reading: null } };
  const bodies: Record<Module, Snippet> = { ai, agents, github, trello, network, system };

  const agentsData = $derived((snapshots.agents?.data as AgentsData | undefined) ?? null);
  const waitingCount = $derived(agentsData?.sessions.filter((s) => s.state === "waiting").length ?? 0);

  const stationOf = (module: Module) => stations.find((s) => s.modules.includes(module))!.key;
  /** Decorative module number, as in the navigation. */
  const code = (module: Module) => String(MODULES.indexOf(module) + 1).padStart(2, "0");

  const time = new Intl.DateTimeFormat("pt-BR", { hour: "2-digit", minute: "2-digit" });
  function updated(module: Module) {
    const at = snapshots[module]?.updatedAt ?? null;
    return at === null ? t.header.noData : interpolate(t.header.updated, { time: time.format(at) });
  }

  onMount(() => {
    let observer: IntersectionObserver | undefined;
    if (typeof IntersectionObserver === "function") {
      observer = new IntersectionObserver((entries) => {
        for (const entry of entries) {
          const module = (entry.target as HTMLElement).dataset.module as Module;
          if (entry.isIntersecting) intersecting.add(module);
          else intersecting.delete(module);
        }
        syncVisibility();
      }, { root: scroller });
      for (const section of Object.values(sections)) observer.observe(section);
    } else {
      MODULES.forEach((m) => intersecting.add(m));
    }

    // Subscribe before requesting initial state to avoid losing snapshots in between.
    let disposed = false;
    let visibilityReceived = false;
    const stopVisibility = core.listenPanelVisibility?.((visible) => {
      visibilityReceived = true;
      nativeVisible = visible;
    });
    const stop = core.listenSnapshots(receive);
    const stopTarget = core.listenAgentTarget?.((session) => void target(session));
    const stopMedia = appearance.watch();
    const stopTheme = core.listenSystemTheme?.((theme) => (appearance.desktop = theme));
    core.initialState().then(async (initial) => {
      if (disposed) return;
      if (!visibilityReceived) nativeVisible = initial.panelVisible ?? false;
      initial.snapshots.forEach(receive);
      if (initial.appearance) appearance.preference = initial.appearance;
      appearance.desktop ??= initial.systemTheme ?? null;
      appearanceRestored = true;
      agentTarget ??= initial.agentTarget ?? null;
      anchor ??= agentTarget ? "agents" : initial.lastAnchor;
      await tick();
      if (area === "panel") reveal(anchor, false);
      if (!observer) syncVisibility();
      if (initial.welcome && !agentTarget) {
        welcome = true;
        core.completeWelcome?.();
        await openSettings();
      }
    });
    return () => {
      disposed = true;
      document.documentElement.dataset.decoration = "paused";
      void stopVisibility?.then((stop) => stop());
      stopMedia();
      void stopTheme?.then((p) => p());
      observer?.disconnect();
      void stop.then((p) => p());
      void stopTarget?.then((p) => p());
    };
  });

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === "Escape") core.closePanel();
  }
</script>

<svelte:window onkeydown={onKeyDown} />
<svelte:document onvisibilitychange={() => (pageVisible = !document.hidden)} />

{#snippet ai()}
  <!-- Before the first snapshot the instruments explain there is no reading and keep their actions. -->
  <AiModule data={(snapshots.ai?.data as AiData | null) ?? NO_AI_READING} refresh={() => core.refreshAi?.()} configure={() => openSettings("ai")}
    reconnect={() => void (core.connectClaude?.() ?? Promise.reject()).catch(() => openSettings("ai"))} />
{/snippet}

{#snippet agents()}
  <AgentsModule data={agentsData} notified={agentTarget} />
{/snippet}

{#snippet github()}
  <GithubModule data={(snapshots.github?.data as GithubData | undefined) ?? null} refresh={() => core.refreshGithub?.()} configure={() => openSettings("github")} />
{/snippet}

{#snippet trello()}
  <TrelloModule data={(snapshots.trello?.data as TrelloData | undefined) ?? null} refresh={() => core.refreshTrello?.()}
    configure={() => openSettings("trello")} openCard={(url) => core.openTrelloCard?.(url)}
    moveCard={(card, list) => core.moveTrelloCard?.(card, list) ?? Promise.reject(new Error("unavailable"))} />
{/snippet}

{#snippet network()}
  <NetworkModule data={(snapshots.network?.data as NetworkData | undefined) ?? null} runSpeed={() => core.runSpeedTest?.()} />
{/snippet}

{#snippet system()}
  <SystemModule data={(snapshots.system?.data as SystemData | undefined) ?? null} updatedAt={snapshots.system?.updatedAt ?? null} />
{/snippet}

{#snippet moduleSection(module: Module, body?: Snippet)}
  <section id="module-{module}" data-module={module} class="module {module}" aria-labelledby="module-{module}-title"
    tabindex="-1" bind:this={sections[module]}>
    <header class="module-head">
      <div class="module-title">
        <span class="eyebrow" aria-hidden="true">{code(module)} / {t.stations[stationOf(module)]}</span>
        <h2 id="module-{module}-title">{t.modules[module]}</h2>
        <p class="source"><span>{t.sources[module]}</span><span aria-hidden="true">·</span><span class="num">{updated(module)}</span></p>
      </div>
      <span class="code num" aria-hidden="true">{code(module)}</span>
    </header>
    {@render body?.()}
  </section>
{/snippet}

<div class="panel">
  <header class="masthead">
    <span class="eyebrow">{t.panel.eyebrow}</span>
    <h1>{t.panel.title}<span class="beacon" aria-hidden="true"></span></h1>
  </header>
  <div class="divider" aria-hidden="true"><span class="elbow"></span><span class="segment lavender"></span><span class="segment cyan"></span><span class="divider-label">{t.panel.bridge}</span><span class="segment blue"></span><span class="cap"></span></div>

  <nav aria-label={t.nav.label}>
    {#each MODULES as module (module)}
      <button aria-current={area === "panel" && anchor === module ? "location" : undefined} onclick={() => choose(module)}>
        <span class="index num" aria-hidden="true">{code(module)}</span>
        <span>{t.modules[module]}</span>
        {#if module === "agents" && agentsData?.accentDot}
          <span class="alert" role="img" aria-label={interpolate(t.agents.waitingCount, { count: String(waitingCount) })}></span>
        {/if}
      </button>
    {/each}
    <button class="settings-item" aria-current={area === "settings" ? "page" : undefined} onclick={() => openSettings()}>
      <span class="index num" aria-hidden="true">07</span>
      <span>{t.settings.title}</span>
    </button>
  </nav>

  <div class="workspace">
    <main class="scroller" bind:this={scroller} class:covered={area === "settings"}
      inert={area === "settings"} aria-hidden={area === "settings" ? "true" : undefined}>
      <div class="stations">
        {#each stations as station (station.key)}
          <div role="group" class="station {station.key}" aria-label={t.stations[station.key]}>
            <div class="station-label" aria-hidden="true">
              <span>{t.stations[station.key]}</span><span>{station.modules.map((m) => t.modules[m]).join(" / ")}</span>
            </div>
            {#each station.modules as module (module)}
              {@render moduleSection(module, bodies[module])}
            {/each}
          </div>
        {/each}
      </div>
    </main>

    {#if area === "settings"}
      <Settings bind:this={settings} {core} {snapshots} {welcome} {appearance} back={backToPanel} />
    {/if}
  </div>
</div>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 6px 12px 12px;
    background: var(--bg);
    overflow: hidden;
  }

  .eyebrow {
    font: 10px var(--mono);
    letter-spacing: 2px;
    text-transform: uppercase;
    color: var(--band-lavender-text);
  }

  .masthead {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 16px 22px 18px;
  }
  h1 {
    font-size: 28px;
    font-weight: 500;
    letter-spacing: 2px;
    text-transform: uppercase;
  }
  .beacon {
    display: inline-block;
    width: 7px;
    height: 7px;
    margin: 0 0 5px 16px;
    background: var(--accent);
    box-shadow: 0 0 18px var(--accent);
  }

  /* LCARS divider: structural decoration, outside the accessibility tree. */
  .divider {
    display: flex;
    gap: 7px;
    height: 46px;
    margin-bottom: 8px;
    flex: none;
  }
  .elbow {
    position: relative;
    width: 90px;
    flex: none;
    background: var(--band-amber);
    border-radius: 40px 0 0 0;
  }
  .elbow::after {
    content: "";
    position: absolute;
    inset: 16px 0 0 18px;
    background: var(--bg);
    border-top-left-radius: 26px;
  }
  .segment {
    height: 16px;
    flex: 1;
  }
  .lavender { flex: 0.7; background: var(--band-lavender); }
  .cyan { flex: 1.8; background: var(--band-cyan); }
  .blue { background: var(--band-blue); }
  .divider-label {
    padding: 1px 6px;
    font: 11px var(--mono);
    letter-spacing: 2px;
    white-space: nowrap;
    color: var(--band-lavender-text);
  }
  .cap {
    width: 70px;
    height: 16px;
    background: var(--band-amber);
    border-radius: 0 16px 16px 0;
  }

  nav {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    padding: 0 4px 14px 30px;
    border-left: 18px solid var(--band-amber);
    flex: none;
  }
  nav button {
    display: flex;
    align-items: center;
    gap: 10px;
    min-height: 37px;
    padding: 8px 17px;
    border-radius: 0 24px 24px 0;
    background: var(--band-lavender);
    color: var(--on-band);
    font-weight: 600;
  }
  nav button:nth-child(2n) { background: var(--band-cyan); }
  nav button[aria-current] { background: var(--band-amber); }
  nav button:hover { filter: brightness(1.15); }
  nav button:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .index { font-size: 10px; opacity: 0.7; }
  .settings-item { margin-left: auto; }
  .settings-item:not([aria-current]) { background: var(--band-blue); }
  .alert {
    width: 9px;
    height: 9px;
    border-radius: 50%;
    background: var(--on-band);
  }

  .workspace {
    position: relative;
    flex: 1;
    min-height: 0;
    border-left: 18px solid var(--band-amber);
    border-radius: 0 0 0 45px;
  }

  .scroller {
    height: 100%;
    overflow-y: auto;
    padding: 4px 4px 24px 30px;
  }
  .scroller.covered { visibility: hidden; }

  /* Three equal stations with aligned headers and two aligned rows. */
  .stations {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    grid-template-rows: auto auto auto;
    column-gap: 22px;
    row-gap: 18px;
  }
  .station {
    display: grid;
    grid-row: span 3;
    grid-template-rows: subgrid;
    gap: 18px;
    min-width: 0;
  }
  .station-label {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: 10px;
    min-height: 33px;
    padding: 8px 16px;
    border-radius: 22px 0 0 22px;
    background: var(--station-band);
    color: var(--on-band);
    font: 12px var(--mono);
    letter-spacing: 1px;
    text-transform: uppercase;
  }
  .station-label span:last-child { font-size: 10px; }
  .capacity { --station-band: var(--band-lavender); }
  .execution { --station-band: var(--band-amber); }
  .observation { --station-band: var(--band-cyan); }

  .module {
    --module-band: var(--band-cyan);
    min-width: 0;
    display: flex;
    flex-direction: column;
    background: var(--module-bg);
    border-top: 5px solid var(--module-band);
    /* Keep the station label in view when the first row is revealed. */
    scroll-margin-top: 60px;
    min-height: 260px;
  }
  .module.system { --module-band: var(--band-lavender); }
  .module.agents { --module-band: var(--band-violet); }
  .module.trello { --module-band: var(--band-amber); }
  .module.github { --module-band: var(--band-steel); }
  .module.network { --module-band: var(--band-blue); }
  .module:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .module-head {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 12px;
    margin: 0 16px;
    padding: 18px 4px 14px;
    border-bottom: 1px solid var(--line);
  }
  .module-title {
    display: flex;
    flex-direction: column;
    gap: 6px;
    min-width: 0;
  }
  .module-title .eyebrow {
    font-size: 9px;
    letter-spacing: 1.5px;
    color: var(--muted);
  }
  .module-head h2 {
    font-size: 17px;
    font-weight: 500;
    letter-spacing: 1px;
    text-transform: uppercase;
    color: var(--module-title, var(--module-band));
  }
  .source {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    font-size: 11px;
    color: var(--muted);
  }
  .code {
    flex: none;
    display: grid;
    place-items: center;
    width: 38px;
    height: 26px;
    border-radius: 20px;
    background: var(--module-band);
    color: var(--on-band);
    font-size: 12px;
  }

  /* 651–850 px: stations stacked, their two modules side by side. */
  @media (max-width: 850px) {
    .stations { grid-template-columns: minmax(0, 1fr); grid-template-rows: none; }
    .station {
      grid-row: auto;
      grid-template-rows: auto auto;
      grid-template-columns: repeat(2, minmax(0, 1fr));
      align-items: start;
    }
    .station-label { grid-column: 1 / 3; }
    .scroller { padding-left: 16px; }
  }

  /* Up to 650 px: a single column. */
  @media (max-width: 650px) {
    .station { display: flex; flex-direction: column; align-items: stretch; }
    .panel { padding: 4px; }
    nav, .workspace { border-left-width: 9px; }
    nav { padding-left: 8px; }
    .scroller { padding: 4px 4px 24px 8px; }
    .elbow { width: 45px; }
    .cap { width: 25px; }
    .divider-label { display: none; }
    h1 { font-size: 23px; }
  }

</style>
