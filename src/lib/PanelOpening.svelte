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
  let fading = $state(false);
  let finished = $state(false);
  let cancelTransition = () => {};
  let recovered = $state(false);
  let openingActive = $state(true);
  let attempt = $state(0);
  let cancelLocal = () => {};
  let cancelMinimum = () => {};
  let minimumStarted = false;
  let cancelPresentation = () => {};
  let cancelRestore = () => {};
  const revealed = $derived(stage === "ready" && finished);
  $effect(() => {
    if (openingActive && stage === "ready" && minimumElapsed && !fading) {
      fading = true;
      cancelTransition = clock.after(240, () => { finished = true; });
    }
  });

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
  function cancelOpening() {
    cancelRestore();
    cancelMinimum();
    cancelPresentation();
    cancelTransition();
  }
  function fail() { if (!openingActive) return; cancelOpening(); stage = "failed"; }
  function retry() {
    cancelOpening();
    minimumElapsed = false;
    fading = false;
    finished = false;
    recovered = false;
    stage = "local";
    attempt += 1;
    minimumStarted = false;
    startMinimum();
  }
  function close() {
    if (!openingActive) return;
    openingActive = false;
    cancelOpening();
    core.closePanel();
  }
  function startMinimum() {
    if (minimumStarted || document.hidden || !openingActive) return;
    minimumStarted = true;
    cancelPresentation = clock.presented(() => {
      if (!openingActive) return;
      if (document.hidden) { minimumStarted = false; return; }
      cancelMinimum = clock.after(960, () => { minimumElapsed = true; });
    });
  }
  onMount(() => {
    minimumStarted = false;
    startMinimum();
    return () => { openingActive = false; cancelOpening(); };
  });
</script>

<svelte:document onvisibilitychange={startMinimum} />
<svelte:window onkeydown={(event) => { if (!revealed && event.key === "Escape") { event.preventDefault(); close(); } }} />

{#if openingActive && stage !== "failed"}
{#key attempt}
  <svelte:boundary onerror={fail}>
    <div class:concealed={!revealed && !fading} inert={!revealed} aria-hidden={!revealed ? "true" : undefined}>
      <Panel core={openingCore} recovery={recovered} presented={revealed} prepared={() => { if (openingActive && stage !== "failed") stage = "ready"; }} failed={fail} />
    </div>
    {#snippet failed()}{/snippet}
  </svelte:boundary>
{/key}
{/if}
{#if openingActive && !revealed}
  <div class="opening" class:fading={fading && stage !== "failed"}>
    <div class="geometry" class:paused={stage === "local" || stage === "failed"} aria-hidden="true"><i></i><i></i><i></i><i></i></div>
    <div class="identity">
      <h1>kvojps monitor</h1>
      {#if stage === "failed"}
        <p role="alert">Não foi possível abrir o Painel.</p>
        <div class="actions"><button onclick={retry}>Tentar novamente</button><button onclick={close}>Fechar</button></div>
      {:else}
        <p role="status" aria-live="polite">{#if stage === "local"}Preparando o Painel…{:else if stage === "modules"}Carregando Módulos…{/if}</p>
      {/if}
    </div>
  </div>
{/if}

<style>
  div:has(> :global(.panel)) { height: 100%; }
  .concealed { visibility: hidden; }
  .opening {
    position: fixed; inset: 0; z-index: 10; box-sizing: border-box;
    display: flex; align-items: flex-end; padding: 8%;
    overflow: auto; isolation: isolate; background: #02070d; color: #d8f5ff;
    transition: opacity 240ms ease;
  }
  .fading { opacity: 0; }
  .geometry { position: absolute; inset: 12%; z-index: 0; pointer-events: none;
    border: 1px solid #24465b; transform: perspective(500px) rotateX(25deg); }
  .geometry i { position: absolute; display: block; width: 1px; height: 100%; left: 25%;
    background: #4ce5ff; box-shadow: 0 0 18px #4ce5ff88; animation: illuminate 1.8s ease-in-out infinite alternate; }
  .geometry i:nth-child(2) { left: 75%; }
  .geometry i:nth-child(3) { width: 100%; height: 1px; left: 0; top: 28%; background: #b9a0e8; }
  .geometry i:nth-child(4) { width: 100%; height: 1px; left: 0; top: 65%; }
  .identity { position: relative; z-index: 1; min-width: 0; max-width: 100%; }
  h1 { font: 300 clamp(1.5rem, 4vw, 3.25rem) var(--sans); letter-spacing: .2em; overflow-wrap: anywhere; margin: 14px 0; }
  p { min-height: 1.5em; color: #bbd2df; font: .875rem/1.5 var(--sans); }
  .actions { display: flex; flex-wrap: wrap; gap: 12px; }
  button { background: #253344; color: #edf9ff; border: 1px solid #7690a4; padding: 8px 12px; border-radius: 6px; font: inherit; cursor: pointer; }
  button:focus-visible { outline: 2px solid #4ce5ff; outline-offset: 3px; }
  .geometry.paused i { animation: none; }
  :global([data-motion="reduced"]) .geometry i { animation: none; }
  :global([data-motion="reduced"]) .opening { transition: none; opacity: 1; }
  @media (prefers-reduced-motion: reduce) { .geometry i { animation: none; } .opening { transition: none; opacity: 1; } }
  @keyframes illuminate { from { opacity: .3; filter: brightness(.7); } to { opacity: 1; filter: brightness(1.5); } }
</style>
