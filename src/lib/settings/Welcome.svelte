<script lang="ts">
  import { MODULES, type AiData, type Module, type Snapshot } from "../core";
  import { interpolate, t } from "../strings";
  import { claudeNeedsConnection, moduleStatus } from "./status";

  let { snapshots, configure, connectClaude }: {
    snapshots: Partial<Record<Module, Snapshot>>;
    configure: (module: Module) => void;
    connectClaude: () => void;
  } = $props();

  const marks = { detected: "✓", needsSetup: "!", notNeeded: "–", checking: "…" } as const;
</script>

<section class="welcome" aria-labelledby="welcome-title">
  <h3 id="welcome-title">{t.settings.welcome.title}</h3>
  <p>{t.settings.welcome.intro}</p>
  <ul aria-label={t.settings.welcome.label}>
    {#each MODULES as module (module)}
      {@const status = moduleStatus(module, snapshots[module])}
      <li aria-labelledby="welcome-{module}" class={status}>
        <span class="mark" aria-hidden="true">{marks[status]}</span>
        <span class="name" id="welcome-{module}">{t.modules[module]}</span>
        <span class="status">{status === "checking" ? t.settings.welcome.checking : t.settings.welcome[status]}</span>
        {#if status === "needsSetup"}
          {#if module === "ai" && claudeNeedsConnection(snapshots.ai?.data as AiData | undefined)}
            <button onclick={connectClaude}>{t.ai.settings.connect}</button>
          {:else}
            <button onclick={() => configure(module)}>{interpolate(t.settings.welcome.configure, { module: t.modules[module] })}</button>
          {/if}
        {/if}
      </li>
    {/each}
  </ul>
</section>

<style>
  p { margin: 8px 0 16px; color: var(--muted); }
  ul { list-style: none; display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 10px; }
  li {
    display: grid;
    grid-template-columns: 28px 1fr;
    align-items: center;
    gap: 4px 10px;
    padding: 12px 14px;
    background: var(--surface-2);
    border-left: 5px solid var(--line);
  }
  li.needsSetup { border-left-color: var(--warn); }
  li.detected { border-left-color: var(--ok); }
  .mark {
    grid-row: span 2;
    display: grid;
    place-items: center;
    width: 26px;
    height: 26px;
    border-radius: 50%;
    border: 1px solid currentColor;
    font: 13px var(--mono);
  }
  .needsSetup .mark { color: var(--warn); }
  .detected .mark { color: var(--ok); }
  .name { font-weight: 600; }
  .status { color: var(--muted); }
  li button { grid-column: 2; justify-self: start; margin-top: 6px; }
</style>
