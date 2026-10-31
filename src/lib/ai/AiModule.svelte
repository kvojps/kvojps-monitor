<script lang="ts">
  import { onMount } from "svelte";
  import type { AiData, AiWindow } from "../core";
  import { t, interpolate } from "../strings";

  let { data, refresh }: { data: AiData; refresh: () => void } = $props();
  let now = $state(Date.now());
  let allowedAt = $state(0);
  onMount(() => {
    const timer = setInterval(() => (now = Date.now()), 1000);
    return () => clearInterval(timer);
  });

  const relative = new Intl.RelativeTimeFormat("pt-BR", { numeric: "auto" });
  const clock = new Intl.DateTimeFormat("pt-BR", { hour: "2-digit", minute: "2-digit" });
  const weekday = new Intl.DateTimeFormat("pt-BR", { weekday: "short" });

  function age(when: number) {
    const seconds = Math.max(0, Math.floor((now - when) / 1000));
    if (seconds < 60) return t.ai.now;
    if (seconds < 3600) return relative.format(-Math.floor(seconds / 60), "minute");
    if (seconds < 86400) return relative.format(-Math.floor(seconds / 3600), "hour");
    return relative.format(-Math.floor(seconds / 86400), "day");
  }
  function label(window: AiWindow) {
    if (window.durationMins === 300) return t.ai.fiveHour;
    if (window.durationMins === 10080) return t.ai.weekly;
    return interpolate(t.ai.windowHours, { hours: String(Math.round((window.durationMins ?? 0) / 60)) });
  }
  /** Within a day: time left; later: weekday and time. */
  function resets(at: number) {
    const minutes = Math.max(0, Math.round((at - now) / 60000));
    if (minutes < 1440) {
      const hours = Math.floor(minutes / 60);
      const time = hours > 0 ? `${hours} h ${minutes % 60} min` : `${minutes} min`;
      return interpolate(t.ai.resetsIn, { time });
    }
    return interpolate(t.ai.resetsAt, { day: weekday.format(at), time: clock.format(at) });
  }
  function plan(type: string) {
    return (t.ai.plans as Record<string, string>)[type] ?? type;
  }
  function refreshNow() {
    allowedAt = Date.now() + 10000;
    refresh();
  }
  const codex = $derived(data.codex);
  const windows = (bucket: { primary: AiWindow | null; secondary: AiWindow | null }) =>
    [bucket.primary, bucket.secondary].filter((w): w is AiWindow => w !== null);
</script>

<div class="ai">
  <div class="tools">
    {#if data.refreshing}<span role="status" class="secondary">{t.ai.refreshing}</span>{/if}
    <button aria-label={t.ai.refresh} disabled={data.paused || data.refreshing || now < allowedAt} onclick={refreshNow}>↻</button>
  </div>
  <section aria-label={t.ai.codex}>
    <div class="line">
      <b>{t.ai.codex}</b>
      {#if codex.plan && codex.state !== "missing"}<span class="pill">{plan(codex.plan)}</span>{/if}
    </div>
    {#if codex.state !== "missing" && codex.failureSince != null && !data.paused}
      <p role="status" class="secondary">{interpolate(t.ai.failureAge, { age: age(codex.failureSince) })}</p>
    {/if}
    {#if codex.state === "missing"}
      <p class="secondary">{t.ai.missingCodex}</p>
    {:else if !codex.reading}
      <p class="secondary">{t.ai.emptyCodex}</p>
    {:else}
      <span class="faint">{interpolate(t.ai.reading, { source: t.ai.sources[codex.reading.source], age: age(codex.reading.readAt) })}</span>
      {#each codex.reading.buckets as bucket (bucket.id)}
        {#if codex.reading.buckets.length > 1}<span class="label">{bucket.name ?? bucket.id}</span>{/if}
        {#each windows(bucket) as window}
          <div class="window">
            <div class="line">
              <span class="label">{label(window)}</span>
              {#if !window.reset && window.usedPercent !== null}<span class="num">{Math.round(window.usedPercent)}%</span>{/if}
            </div>
            {#if window.reset || window.usedPercent === null}
              <span class="faint">{window.resetsAt !== null ? interpolate(t.ai.restarted, { time: clock.format(window.resetsAt) }) : ""}</span>
            {:else}
              <div class="bar" class:warning={window.usedPercent >= 75} class:bad={window.usedPercent >= 90}
                role="meter" aria-label={label(window)} aria-valuemin={0} aria-valuemax={100} aria-valuenow={Math.round(window.usedPercent)}>
                <i style:width="{Math.min(100, window.usedPercent)}%"></i>
              </div>
              {#if window.resetsAt !== null}<span class="faint">{resets(window.resetsAt)}</span>{/if}
            {/if}
          </div>
        {/each}
      {/each}
    {/if}
  </section>
</div>

<style>
  .ai { padding: 0 16px 16px; min-width: 0; }
  .tools { display: flex; justify-content: flex-end; align-items: center; gap: 8px; }
  button { padding: 5px 8px; border: 1px solid var(--line); border-radius: 5px; }
  button:disabled { opacity: .4; }
  section { display: flex; flex-direction: column; gap: 8px; padding: 8px 0; }
  .line { display: flex; align-items: baseline; justify-content: space-between; gap: 8px; }
  .window { display: flex; flex-direction: column; gap: 4px; }
  .label, .secondary { color: var(--muted); }
  .secondary, .num { font-size: 12px; }
  .faint { color: var(--faint); font-size: 11px; }
  .pill { font-size: 11px; padding: 1px 7px; border: 1px solid var(--line); border-radius: 999px; color: var(--muted); }
  .bar { height: 4px; border-radius: 2px; background: var(--surface-2); overflow: hidden; }
  .bar > i { display: block; height: 100%; background: var(--accent); border-radius: 2px; }
  .bar.warning > i { background: var(--warn); }
  .bar.bad > i { background: var(--bad); }
</style>
