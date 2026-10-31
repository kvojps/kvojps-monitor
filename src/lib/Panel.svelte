<script lang="ts">
  import { onMount, tick, type Snippet } from "svelte";
  import Icon from "./Icon.svelte";
  import { MODULES, type AgentsData, type AiData, type GithubData, type TrelloData, type NetworkData, type SystemData, type Module, type Core, type Snapshot } from "./core";
  import AgentSettings from "./agents/AgentSettings.svelte";
  import AgentsModule from "./agents/AgentsModule.svelte";
  import AiModule from "./ai/AiModule.svelte";
  import AiSettings from "./ai/AiSettings.svelte";
  import GithubSettings from "./github/GithubSettings.svelte";
  import GithubModule from "./github/GithubModule.svelte";
  import TrelloModule from "./trello/TrelloModule.svelte";
  import TrelloSettings from "./trello/TrelloSettings.svelte";
  import NetworkModule from "./network/NetworkModule.svelte";
  import SystemModule from "./system/SystemModule.svelte";
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

{#snippet moduleSection(module: Module, body: Snippet)}
  <section id="module-{module}" data-module={module} class="module {module}" aria-labelledby="module-{module}-title"
    tabindex="-1" bind:this={sections[module]}>
    <header class="module-head">
      <h2 id="module-{module}-title">{t.modules[module]}</h2>
      <p class="source"><span>{t.sources[module]}</span><span aria-hidden="true"> · </span><span class="num">{updated(module)}</span></p>
    </header>
    {@render body()}
  </section>
{/snippet}

{#snippet ai()}
  {#if snapshots.ai?.data}
    <AiModule data={snapshots.ai.data as AiData} refresh={() => core.refreshAi?.()}
      configure={openSettings} reconnect={() => void core.connectClaude?.().catch(openSettings)} />
  {/if}
{/snippet}
{#snippet system()}
  {#if snapshots.system}
    <SystemModule data={snapshots.system.data as SystemData} />
  {/if}
{/snippet}
{#snippet agents()}
  {#if snapshots.agents}
    <AgentsModule data={snapshots.agents.data as AgentsData} target={agentTarget} />
  {/if}
{/snippet}
{#snippet trello()}
  {#if snapshots.trello}
    <TrelloModule data={snapshots.trello.data as TrelloData} refresh={() => core.refreshTrello?.()}
      configure={openSettings} openCard={(url) => core.openTrelloCard?.(url)}
      moveCard={(card, list) => core.moveTrelloCard ? core.moveTrelloCard(card, list) : Promise.reject("offline")} />
  {/if}
{/snippet}
{#snippet github()}
  {#if snapshots.github}
    <GithubModule data={snapshots.github.data as GithubData} refresh={() => core.refreshGithub?.()} configure={openSettings} />
  {/if}
{/snippet}
{#snippet network()}
  {#if snapshots.network?.data}
    <NetworkModule data={snapshots.network.data as NetworkData} runSpeed={() => core.runSpeedTest?.()} />
  {/if}
{/snippet}

<div class="panel">
  <header class="masthead">
    <span class="eyebrow">{t.panel.eyebrow}</span>
    <h1>{t.panel.title}</h1>
  </header>
  <div class="divider" aria-hidden="true"><span class="elbow"></span><span class="segment lavender"></span><span class="segment cyan"></span><span class="segment blue"></span><span class="cap"></span></div>

  <nav aria-label={t.nav.label}>
    {#each MODULES as module (module)}
      <button aria-current={area === "panel" && anchor === module ? "location" : undefined} onclick={() => choose(module)}>
        <Icon name={module} size={16} />
        <span>{t.modules[module]}</span>
        {#if module === "agents" && (snapshots.agents?.data as AgentsData | undefined)?.accentDot}
          <span class="alert" role="img" aria-label={t.agents.states.waiting}></span>
        {/if}
      </button>
    {/each}
    <button class="settings-item" aria-current={area === "settings" ? "page" : undefined} onclick={openSettings}>
      <Icon name="settings" size={16} />
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
              {@render moduleSection(module, { ai, system, agents, trello, github, network }[module])}
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
    border: 1px solid var(--line);
    border-radius: var(--radius);
    overflow: hidden;
  }

  .masthead {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 14px 20px 12px;
  }

  .eyebrow {
    font: 10px var(--mono);
    letter-spacing: 2px;
    color: var(--muted);
  }

  h1 {
    font-size: 26px;
    font-weight: 500;
    letter-spacing: 2px;
    text-transform: uppercase;
  }

  /* LCARS divider: structural decoration, outside the accessibility tree. */
  .divider {
    display: flex;
    gap: 7px;
    height: 40px;
    flex: none;
  }
  .elbow {
    position: relative;
    width: 90px;
    flex: none;
    background: var(--band-amber);
    border-radius: 36px 0 0 0;
  }
  .elbow::after {
    content: "";
    position: absolute;
    inset: 15px 0 0 26px;
    background: var(--bg);
    border-top-left-radius: 22px;
  }
  .segment {
    height: 15px;
    flex: 1;
  }
  .lavender { flex: 0.7; background: var(--band-lavender); }
  .cyan { flex: 1.8; background: var(--band-cyan); }
  .blue { background: var(--band-blue); }
  .cap {
    width: 60px;
    height: 15px;
    background: var(--band-amber);
    border-radius: 0 15px 15px 0;
  }

  nav {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    padding: 0 0 12px 30px;
    border-left: 18px solid var(--band-amber);
    flex: none;
  }
  nav button {
    position: relative;
    display: flex;
    align-items: center;
    gap: 8px;
    min-height: 36px;
    padding: 8px 16px;
    border-radius: 0 20px 20px 0;
    background: var(--band-lavender);
    color: var(--on-band);
    font-weight: 600;
  }
  nav button:nth-child(2n) { background: var(--band-cyan); }
  nav button[aria-current] {
    background: var(--band-amber);
    box-shadow: inset 0 -4px var(--on-band);
  }
  nav button:hover { filter: brightness(1.12); }
  nav button:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .settings-item { margin-left: auto; }
  .settings-item:not([aria-current]) { background: var(--band-blue); }
  .alert {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--on-band);
    outline: 2px solid var(--warn);
  }

  .workspace {
    position: relative;
    flex: 1;
    min-height: 0;
    border-left: 18px solid var(--band-amber);
    border-radius: 0 0 0 40px;
  }

  .scroller {
    height: 100%;
    overflow-y: auto;
    padding: 6px 12px 24px 30px;
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
  .capacity { --station-band: var(--band-lavender); }
  .execution { --station-band: var(--band-amber); }
  .observation { --station-band: var(--band-cyan); }

  .module {
    min-width: 0;
    display: flex;
    flex-direction: column;
    background: var(--surface);
    border-top: 5px solid var(--station-band);
    scroll-margin-top: 8px;
  }
  .module:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .module-head {
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    justify-content: space-between;
    gap: 4px 12px;
    padding: 16px 20px 12px;
    border-bottom: 1px solid var(--line);
  }
  .module-head h2 {
    font-size: 16px;
    font-weight: 500;
    letter-spacing: 1px;
    text-transform: uppercase;
  }
  .source {
    font-size: 11px;
    color: var(--muted);
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
