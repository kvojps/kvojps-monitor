<script lang="ts">
  import { onMount, tick } from "svelte";
  import { MODULES, type AgentsData, type AiData, type GithubData, type TrelloData, type NetworkData, type Module, type Core, type Snapshot } from "./core";
  import AgentSettings from "./agents/AgentSettings.svelte";
  import AiSettings from "./ai/AiSettings.svelte";
  import GithubSettings from "./github/GithubSettings.svelte";
  import TrelloSettings from "./trello/TrelloSettings.svelte";
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

  let scroller: HTMLElement;
  let settingsArea = $state<HTMLElement>();
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

  function reducedMotion() {
    return window.matchMedia?.("(prefers-reduced-motion: reduce)").matches ?? true;
  }

  function reveal(module: Module, focus: boolean) {
    const section = sections[module];
    if (!section) return;
    section.scrollIntoView({ block: "start", behavior: focus && !reducedMotion() ? "smooth" : "auto" });
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

  async function openSettings() {
    if (area === "settings") return;
    returnFocus = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    area = "settings";
    syncVisibility();
    await tick();
    settingsArea?.focus({ preventScroll: true });
  }

  /** The panel stays mounted under Settings, so scroll and module state survive. */
  async function backToPanel() {
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
    const stop = core.listenSnapshots(receive);
    const stopTarget = core.listenAgentTarget?.((session) => void target(session));
    core.initialState().then(async (initial) => {
      initial.snapshots.forEach(receive);
      agentTarget ??= initial.agentTarget ?? null;
      anchor ??= agentTarget ? "agents" : initial.lastAnchor;
      await tick();
      if (area === "panel") reveal(anchor, false);
      if (!observer) syncVisibility();
    });
    return () => {
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

{#snippet moduleSection(module: Module)}
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
        {#if module === "agents" && (snapshots.agents?.data as AgentsData | undefined)?.accentDot}
          <span class="alert" role="img" aria-label={t.agents.states.waiting}></span>
        {/if}
      </button>
    {/each}
    <button class="settings-item" aria-current={area === "settings" ? "page" : undefined} onclick={openSettings}>
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
              {@render moduleSection(module)}
            {/each}
          </div>
        {/each}
      </div>
    </main>

    {#if area === "settings"}
      <div class="settings" bind:this={settingsArea} tabindex="-1">
        <header class="settings-head">
          <h2>{t.settings.title}</h2>
          <button class="back" onclick={backToPanel}><span aria-hidden="true">← </span>{t.panel.back}</button>
        </header>
        <AgentSettings setHooks={core.setAgentHooks} data={snapshots.agents?.data as AgentsData | undefined} save={(preferences) => core.setAgentNotifications?.(preferences)} />
        <AiSettings data={snapshots.ai?.data as AiData | undefined}
          connect={() => core.connectClaude ? core.connectClaude() : Promise.reject("unwritable")}
          disconnect={() => core.disconnectClaude ? core.disconnectClaude() : Promise.reject("unwritable")}
          setUsage={(enabled) => core.setClaudeUsage ? core.setClaudeUsage(enabled) : Promise.reject("unwritable")} />
        <GithubSettings data={snapshots.github?.data as GithubData | undefined}
          save={(token) => core.saveGithubToken ? core.saveGithubToken(token) : Promise.reject("offline")} />
        <TrelloSettings data={snapshots.trello?.data as TrelloData | undefined}
          connect={(key, token) => core.connectTrello ? core.connectTrello(key, token) : Promise.reject("offline")}
          selectBoard={(board) => core.selectTrelloBoard ? core.selectTrelloBoard(board) : Promise.reject("offline")}
          disconnect={() => core.disconnectTrello ? core.disconnectTrello() : Promise.reject("offline")}
          openAdmin={() => core.openTrelloAdmin?.()} authorize={(key) => core.openTrelloAuthorization?.(key)} />
        {#if snapshots.network}
          <section class="network-settings" aria-label={t.modules.network}>
            <h2>{t.modules.network}</h2>
            <label>
              <input type="checkbox" checked={(snapshots.network.data as NetworkData).automaticSpeed}
                onchange={(event) => core.setAutomaticSpeed?.(event.currentTarget.checked)} />
              {t.network.automaticSpeed}
            </label>
          </section>
        {/if}
      </div>
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
    /* Module instruments arrive with their own slices (tickets 02–07). */
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

  .settings {
    position: absolute;
    inset: 0;
    overflow-y: auto;
    padding: 6px 12px 24px 30px;
    background: var(--bg);
  }
  .settings:focus { outline: none; }
  .settings-head {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 8px 16px 16px;
    border-bottom: 5px solid var(--band-lavender);
  }
  .settings-head h2 {
    font-size: 22px;
    font-weight: 500;
    letter-spacing: 2px;
    text-transform: uppercase;
    color: var(--band-amber-text);
  }
  .back {
    min-height: 36px;
    padding: 8px 16px;
    border-radius: 18px;
    border: 1px solid var(--line);
    background: var(--surface-2);
  }
  .back:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .network-settings { padding: 16px; font-size: 13px; }
  .network-settings h2 { font-size: 14px; margin-bottom: 12px; }
</style>
