<script lang="ts">
  import { tick } from "svelte";
  import type { AgentsData, AiData, Core, GithubData, Module, NetworkData, Snapshot, TrelloData } from "../core";
  import AgentSettings from "../agents/AgentSettings.svelte";
  import AiSettings from "../ai/AiSettings.svelte";
  import GithubSettings from "../github/GithubSettings.svelte";
  import TrelloSettings from "../trello/TrelloSettings.svelte";
  import { t } from "../strings";
  import type { AppearanceState } from "./appearance.svelte";
  import AppearanceSettings from "./AppearanceSettings.svelte";
  import GeneralSettings from "./GeneralSettings.svelte";
  import Welcome from "./Welcome.svelte";

  let { core, snapshots, welcome, appearance, back }: {
    core: Core;
    snapshots: Partial<Record<Module, Snapshot>>;
    welcome: boolean;
    appearance: AppearanceState;
    back: () => void;
  } = $props();

  let area: HTMLElement;

  /** Focus a module section, or the whole area. */
  export async function focus(module?: Module) {
    await tick();
    const target = module ? area.querySelector<HTMLElement>(`#settings-${module}`) : null;
    (target ?? area).focus({ preventScroll: !target });
    target?.scrollIntoView({ block: "start" });
  }

  const unavailable = (code: string) => () => Promise.reject(code);
</script>

<div class="settings" bind:this={area} tabindex="-1">
  <header class="settings-head">
    <h2>{t.settings.title}</h2>
    <button class="back" onclick={back}><span aria-hidden="true">← </span>{t.panel.back}</button>
  </header>

  {#if welcome}
    <Welcome {snapshots} configure={(m) => void focus(m)}
      connectClaude={() => void (core.connectClaude?.() ?? Promise.reject()).catch(() => focus("ai"))} />
  {/if}

  <div class="cards">
    <div class="card general"><GeneralSettings {core} /></div>
    <div class="card appearance"><AppearanceSettings {appearance} save={(a) => core.setAppearance ? core.setAppearance(a) : Promise.reject()} /></div>
    <div class="card ai" id="settings-ai" tabindex="-1">
      <AiSettings data={snapshots.ai?.data as AiData | undefined}
        connect={core.connectClaude ?? unavailable("unwritable")}
        disconnect={core.disconnectClaude ?? unavailable("unwritable")}
        setUsage={core.setClaudeUsage ?? unavailable("unwritable")} />
    </div>
    <div class="card agents" id="settings-agents" tabindex="-1">
      <AgentSettings setHooks={core.setAgentHooks} data={snapshots.agents?.data as AgentsData | undefined}
        save={(preferences) => core.setAgentNotifications?.(preferences)} />
    </div>
    <div class="card github" id="settings-github" tabindex="-1">
      <GithubSettings data={snapshots.github?.data as GithubData | undefined}
        save={core.saveGithubToken ?? unavailable("offline")} />
    </div>
    <div class="card trello" id="settings-trello" tabindex="-1">
      <TrelloSettings data={snapshots.trello?.data as TrelloData | undefined}
        connect={core.connectTrello ?? unavailable("offline")}
        selectBoard={core.selectTrelloBoard ?? unavailable("offline")}
        disconnect={core.disconnectTrello ?? unavailable("offline")}
        openAdmin={() => core.openTrelloAdmin?.()} authorize={(key) => core.openTrelloAuthorization?.(key)} />
    </div>
    {#if snapshots.network}
      <div class="card network" id="settings-network" tabindex="-1">
        <section aria-label={t.modules.network}>
          <h3>{t.modules.network}</h3>
          <label class="switch">
            <input type="checkbox" checked={(snapshots.network.data as NetworkData).automaticSpeed}
              onchange={(event) => core.setAutomaticSpeed?.(event.currentTarget.checked)} />
            {t.network.automaticSpeed}
          </label>
        </section>
      </div>
    {/if}
  </div>
</div>

<style>
  .settings {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    gap: 18px;
    overflow-y: auto;
    padding: 6px 12px 24px 30px;
    background: var(--bg);
    user-select: text;
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

  .cards {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(min(100%, 340px), 1fr));
    align-items: start;
    gap: 18px 22px;
  }
  .card {
    --card-band: var(--band-cyan);
    min-width: 0;
    background: var(--module-bg);
    border-top: 5px solid var(--card-band);
    scroll-margin-top: 12px;
  }
  .card:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .card:focus:not(:focus-visible) { outline: none; }
  .general { --card-band: var(--band-amber); }
  .appearance { --card-band: var(--band-lavender); }
  .agents { --card-band: var(--band-violet); }
  .trello { --card-band: var(--band-amber); }
  .github { --card-band: var(--band-steel); }
  .network { --card-band: var(--band-blue); }

  /* Shared form language for every settings section, including module ones. */
  .settings :global(section) { padding: 18px 20px 20px; font-size: 13px; line-height: 1.45; }
  .settings :global(h3) {
    margin: 0 0 12px;
    font-size: 16px;
    font-weight: 500;
    letter-spacing: 1px;
    text-transform: uppercase;
    color: var(--module-title, var(--card-band, var(--band-cyan)));
  }
  .settings :global(.welcome h3) { color: var(--band-amber-text); }
  .settings :global(h4) { margin: 18px 0 6px; font-size: 13px; font-weight: 600; color: var(--text); }
  .settings :global(h3 + h4) { margin-top: 0; }
  .settings :global(p) { margin: 8px 0; }
  .settings :global(label) { display: block; margin: 12px 0 6px; color: var(--muted); }
  .settings :global(label.switch),
  .settings :global(label:has(> input[type="checkbox"])) {
    display: flex;
    align-items: center;
    gap: 10px;
    min-height: 32px;
    margin: 6px 0;
    color: var(--text);
    cursor: pointer;
  }
  .settings :global(input:not([type="checkbox"]):not([type="radio"])) {
    width: 100%;
    min-height: 36px;
    padding: 8px 10px;
    font: 13px var(--mono);
    color: var(--text);
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 6px;
  }
  .settings :global(input[aria-invalid="true"]) { border-color: var(--bad); }
  .settings :global(input[type="checkbox"]),
  .settings :global(input[type="radio"]) { width: 18px; height: 18px; accent-color: var(--accent); }
  .settings :global(button) {
    display: inline-flex;
    align-items: center;
    min-height: 36px;
    margin: 8px 8px 0 0;
    padding: 8px 16px;
    border: 1px solid var(--line);
    border-radius: 0 18px 18px 0;
    background: var(--surface-2);
    color: var(--text);
  }
  .settings :global(button[type="submit"]) { background: var(--band-cyan); border-color: var(--band-cyan); color: var(--on-band); font-weight: 600; }
  .settings :global(button:hover:not(:disabled)) { border-color: var(--accent); }
  .settings :global(button:disabled) { opacity: 0.5; cursor: default; }
  .settings :global(:is(button, input):focus-visible) { outline: 2px solid var(--accent); outline-offset: 2px; }
  .settings :global([role="alert"]),
  .settings :global(.error) { color: var(--bad); }
  .settings :global(form [role="status"]),
  .settings :global(p[role="status"]) { color: var(--ok); }
  .settings :global(.warning) { margin: 8px 0; padding: 12px; border: 1px solid var(--warn); }
  .settings :global(.warning b) { color: var(--warn); }
  .settings .back { border-radius: 18px; }

  @media (max-width: 650px) {
    .settings { padding: 4px 4px 24px 8px; }
  }
</style>
