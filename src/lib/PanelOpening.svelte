<script lang="ts">
  import { onMount } from "svelte";
  import Panel from "./Panel.svelte";
  import type { Core, InitialState } from "./core";

  /** Monotonic presentation clock; tests can inject time without touching source data. */
  export interface OpeningClock {
    presented(callback: () => void): () => void;
    after(ms: number, callback: () => void): () => void;
  }
  const browserClock: OpeningClock = {
    presented(callback) {
      let frame = requestAnimationFrame(() => { frame = requestAnimationFrame(callback); });
      return () => cancelAnimationFrame(frame);
    },
    after(ms, callback) { const timer = setTimeout(callback, ms); return () => clearTimeout(timer); },
  };
  let { core, clock = browserClock }: { core: Core; clock?: OpeningClock } = $props();
  let stage = $state<"local" | "modules" | "ready" | "failed">("local");
  let minimumElapsed = $state(false);
  let recovered = $state(false);
  let openingActive = $state(true);
  let attempt = $state(0);
  let cancelLocal = () => {};
  let cancelMinimum = () => {};
  let minimumStarted = false;
  let cancelPresentation = () => {};
  let cancelRestore = () => {};
  const revealed = $derived(stage === "ready" && minimumElapsed);

  // Panel subscribes to live snapshots before requesting this local-only restore.
  const openingCore: Core = $derived({
    ...core,
    closePanel: close,
    initialState: () => new Promise<InitialState>((resolve) => {
      let settled = false;
      const finish = (initial?: InitialState) => {
        if (settled || !openingActive) return;
        settled = true;
        cancelLocal();
        recovered = !initial;
        stage = "modules";
        resolve(initial ?? { lastAnchor: "ai", panelVisible: true, snapshots: [] });
      };
      cancelRestore = () => { settled = true; cancelLocal(); };
      cancelLocal = clock.after(5000, () => finish());
      Promise.resolve().then(() => core.initialState()).then((value) => finish(value), () => finish());
    }),
  });
  function fail() { if (!openingActive) return; cancelRestore(); cancelMinimum(); cancelPresentation(); stage = "failed"; }
  function retry() {
    cancelRestore();
    cancelMinimum();
    cancelPresentation();
    minimumElapsed = false;
    recovered = false;
    stage = "local";
    attempt += 1;
    minimumStarted = false;
    startMinimum();
  }
  function close() {
    if (!openingActive) return;
    openingActive = false;
    cancelRestore();
    cancelMinimum();
    cancelPresentation();
    core.closePanel();
  }
  function startMinimum() {
    if (minimumStarted || document.hidden || !openingActive) return;
    minimumStarted = true;
    cancelPresentation = clock.presented(() => {
      if (!openingActive) return;
      if (document.hidden) { minimumStarted = false; return; }
      cancelMinimum = clock.after(1200, () => { minimumElapsed = true; });
    });
  }
  onMount(() => {
    minimumStarted = false;
    startMinimum();
    return () => { openingActive = false; cancelRestore(); cancelMinimum(); cancelPresentation(); };
  });
</script>

<svelte:document onvisibilitychange={startMinimum} />
<svelte:window onkeydown={(event) => { if (!revealed && event.key === "Escape") { event.preventDefault(); close(); } }} />

{#if openingActive && stage !== "failed"}
{#key attempt}
  <svelte:boundary onerror={fail}>
    <div class:concealed={!revealed} inert={!revealed} aria-hidden={!revealed ? "true" : undefined}>
      <Panel core={openingCore} recovery={recovered} presented={revealed} prepared={() => { if (openingActive && stage !== "failed") stage = "ready"; }} failed={fail} />
    </div>
    {#snippet failed()}{/snippet}
  </svelte:boundary>
{/key}
{/if}
{#if openingActive && stage === "failed"}
  <div class="opening" role="alert">
    <p>Não foi possível abrir o Painel.</p>
    <button onclick={retry}>Tentar novamente</button>
    <button onclick={close}>Fechar</button>
  </div>
{:else if openingActive && !revealed}
  <div class="opening" role="status" aria-live="polite">
    {#if stage === "local"}Preparando o Painel…{:else if stage === "modules"}Carregando Módulos…{:else}Painel pronto.{/if}
  </div>
{/if}

<style>
  div:has(> :global(.panel)) { height: 100%; }
  .concealed { visibility: hidden; }
  .opening { position: fixed; inset: 0; display: grid; place-content: center; gap: 12px; background: #10131b; color: #eef0f7; }
</style>
