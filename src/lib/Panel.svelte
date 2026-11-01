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

  let area = $state<"panel" | "settings">("panel");
  /** Last anchor chosen through module interaction; independent from what is visible. */
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
    const modules = area === "panel" && nativeVisible && pageVisible ? MODULES.filter((m) => intersecting.has(m)) : [];
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

  function remember(module: Module) {
    agentTarget = null;
    anchor = module;
    if (area === "panel") core.chooseAnchor(module);
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

  const supportingModules: Module[] = ["github", "network", "trello", "system"];
  const agentsData = $derived((snapshots.agents?.data as AgentsData | undefined) ?? null);

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
      syncVisibility();
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
      syncVisibility();
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
<svelte:document onvisibilitychange={() => { pageVisible = !document.hidden; syncVisibility(); }} />

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
    tabindex="-1" onfocusin={() => remember(module)} bind:this={sections[module]}>
    <header class="module-head">
      <div class="module-title">
        <h2 id="module-{module}-title">{t.modules[module]}</h2>
        <p class="source"><span>{t.sources[module]}</span><span aria-hidden="true">·</span><span class="num">{updated(module)}</span></p>
      </div>
      <span class="code num" aria-hidden="true">{code(module)}</span>
    </header>
    {@render body?.()}
  </section>
{/snippet}

<div class="panel">
  <header class="toolbar">
    <div class="segment cyan" aria-hidden="true"></div>
    <button aria-current={area === "settings" ? "page" : undefined} onclick={() => openSettings()}>{t.settings.title}</button>
  </header>

  <div class="workspace">
    <main class="scroller" bind:this={scroller} class:covered={area === "settings"}
      inert={area === "settings"} aria-hidden={area === "settings" ? "true" : undefined}>
      <div class="composition">
        <div class="intelligence">
          {@render moduleSection("ai", bodies.ai)}
          {@render moduleSection("agents", bodies.agents)}
        </div>
        {#each supportingModules as module (module)}
          {@render moduleSection(module, bodies[module])}
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

  .toolbar { display: flex; align-items: center; gap: 12px; padding: 4px 0 10px; flex: none; }
  .segment { height: 8px; flex: 1; background: var(--band-cyan); border-radius: 12px 0 0 12px; }
  .toolbar button { padding: 8px 16px; border-radius: 0 20px 20px 0; background: var(--band-blue); color: var(--on-band); }
  .toolbar button[aria-current] { background: var(--band-amber); }
  .workspace { position: relative; flex: 1; min-height: 0; }
  .scroller { height: 100%; overflow: auto; padding: 4px 4px 16px; }
  .scroller.covered { visibility: hidden; }
  .composition { display: grid; grid-template-columns: minmax(0, 1.2fr) repeat(2, minmax(0, 1fr)); grid-template-areas: "intelligence github network" "intelligence trello system"; gap: 12px; align-items: start; }
  .intelligence { grid-area: intelligence; display: flex; flex-direction: column; gap: 12px; min-width: 0; border-left: 6px solid var(--band-violet); padding-left: 8px; }
  .composition > :global(.github) { grid-area: github; }
  .composition > :global(.network) { grid-area: network; }
  .composition > :global(.trello) { grid-area: trello; }
  .composition > :global(.system) { grid-area: system; }

  .module {
    --module-band: var(--band-cyan);
    min-width: 0;
    display: flex;
    flex-direction: column;
    background: var(--module-bg);
    border-top: 5px solid var(--module-band);
    scroll-margin-top: 4px;
    overflow-wrap: anywhere;
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
    margin: 0 10px;
    padding: 8px 0;
    border-bottom: 1px solid var(--line);
  }
  .module-title {
    display: flex;
    flex-direction: column;
    gap: 6px;
    min-width: 0;
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

  @media (max-width: 950px) {
    .composition { grid-template-columns: repeat(2, minmax(0, 1fr)); grid-template-areas: "intelligence intelligence" "github network" "trello system"; }
  }
  @media (max-width: 650px) {
    .panel { padding: 4px; }
    .composition { grid-template-columns: minmax(0, 1fr); grid-template-areas: "intelligence" "github" "network" "trello" "system"; }
  }
</style>
