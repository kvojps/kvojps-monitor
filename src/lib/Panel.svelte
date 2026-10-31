<script lang="ts">
  import { onMount } from "svelte";
  import Icon from "./Icon.svelte";
  import { MODULES, type GithubData, type NetworkData, type SystemData, type Module, type Core, type Snapshot } from "./core";
  import GithubModule from "./github/GithubModule.svelte";
  import NetworkModule from "./network/NetworkModule.svelte";
  import SystemModule from "./system/SystemModule.svelte";
  import { interpolate, t } from "./strings";
  import type { View } from "./view";

  let { core }: { core: Core } = $props();

  const items: { view: View; name: string }[] = [
    ...MODULES.map((m) => ({ view: m, name: t.modules[m] })),
    { view: "settings", name: t.settings.title },
  ];

  let view = $state<View | null>(null);
  const current = $derived(items.find((i) => i.view === view));

  let snapshots = $state<Partial<Record<Module, Snapshot>>>({});
  const updatedAt = $derived(view && view !== "settings" ? (snapshots[view]?.updatedAt ?? null) : undefined);

  function receive(s: Snapshot) {
    const previous = snapshots[s.module];
    if (!previous || (previous.updatedAt ?? 0) <= (s.updatedAt ?? 0)) snapshots[s.module] = s;
  }

  const time = new Intl.DateTimeFormat("pt-BR", { hour: "2-digit", minute: "2-digit" });

  onMount(() => {
    // Subscribe before requesting initial state to avoid losing snapshots in between.
    const stop = core.listenSnapshots(receive);
    core.initialState().then((initial) => {
      initial.snapshots.forEach(receive);
      view = initial.lastModule;
    });
    return () => void stop.then((p) => p());
  });

  function selecionar(next: View) {
    view = next;
    core.reportModuleVisible(next === "settings" ? null : next);
  }
  function onKeyDown(e: KeyboardEvent) {
    if (e.key === "Escape") core.closePanel();
  }
</script>

<svelte:window onkeydown={onKeyDown} />

<div class="panel">
  <nav class="rail" aria-label={t.rail.label}>
    {#each items as item (item.view)}
      <button
        class="item"
        aria-label={item.name}
        title={item.name}
        aria-current={item.view === view ? "page" : undefined}
        onclick={() => selecionar(item.view)}
      >
        <Icon name={item.view} />
      </button>
    {/each}
  </nav>
  {#if current}
    <main class="detail">
      <header class="header">
        <h1>{current.name}</h1>
        {#if updatedAt !== undefined}
          <span class="update num">
            {updatedAt === null
              ? t.header.noData
              : interpolate(t.header.updated, { time: time.format(updatedAt) })}
          </span>
        {/if}
      </header>
      {#if view === "github" && snapshots.github}
        <GithubModule data={snapshots.github.data as GithubData} refresh={() => core.refreshGithub?.()} />
      {/if}
      {#if view === "network" && snapshots.network?.data}
        <NetworkModule data={snapshots.network.data as NetworkData} />
      {/if}
      {#if view === "system" && snapshots.system}
        <SystemModule data={snapshots.system.data as SystemData} />
      {/if}
    </main>
  {/if}
</div>

<style>
  .panel {
    display: flex;
    height: 100%;
    background: var(--bg);
    border: 1px solid var(--line);
    border-radius: var(--radius);
    overflow: hidden;
  }

  .rail {
    width: 52px;
    flex: none;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 10px 0;
    background: var(--surface);
    border-right: 1px solid var(--line);
  }

  /* Place Settings at the bottom of the rail. */
  .item:last-child {
    margin-top: auto;
  }

  .item {
    width: 36px;
    height: 36px;
    border-radius: 8px;
    display: grid;
    place-items: center;
    color: var(--muted);
  }

  .item:hover {
    color: var(--text);
  }

  .item[aria-current="page"] {
    color: var(--accent);
    background: var(--accent-dim);
  }

  .item:focus-visible {
    outline: 1px solid var(--accent);
  }

  .detail {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 8px;
    padding: 14px 16px 10px;
  }

  h1 {
    font-size: 15px;
    font-weight: 600;
  }

  .update {
    font-size: 11px;
    color: var(--faint);
  }
</style>
