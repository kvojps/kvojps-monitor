<script lang="ts">
  import Icon from "../Icon.svelte";
  import { onMount } from "svelte";
  import type { AiData, AiReading, AiWindow } from "../core";
  import { t, interpolate } from "../strings";

  let { data, refresh, updated = "", configure = () => {}, reconnect = () => {} }:
    { data: AiData; refresh: () => void; updated?: string; configure?: () => void; reconnect?: () => void } = $props();
  let now = $state(Date.now());
  let tipsDismissed = $state(false);
  let titleActions: HTMLDivElement;

  /** Manual refreshes stay at least ten seconds apart. */
  let allowedAt = $state(0);
  onMount(() => {
    const timer = setInterval(() => (now = Date.now()), 1000);
    function dismissDescription(event: KeyboardEvent) {
      if (event.key !== "Escape" || tipsDismissed || !titleActions ||
          !(titleActions.contains(document.activeElement) || titleActions.matches(":hover"))) return;
      tipsDismissed = true;
      event.preventDefault();
      event.stopPropagation();
    }
    window.addEventListener("keydown", dismissDescription, true);
    return () => {
      clearInterval(timer);
      window.removeEventListener("keydown", dismissDescription, true);
    };
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

  /** Usage from which the measurement turns amber; the percentage stays the source of truth. */
  const HIGH = 85;

  /** The value read aloud, with the high usage state in words, not only colour. */
  function valueText(percent: number) {
    return percent >= HIGH ? `${percent}% · ${t.ai.high}` : `${percent}%`;
  }

  function restarted(window: AiWindow) {
    return window.reset || (window.resetsAt !== null && window.resetsAt <= now);
  }

  /** Null once the window restarted: never shown as 0%. */
  function usedPercent(window: AiWindow | null) {
    return window && !restarted(window) && window.usedPercent !== null ? Math.round(window.usedPercent) : null;
  }
  /** When the window resets, why it has no value, or when it restarted without a new reading. */
  function status(window: AiWindow | null) {
    if (!window) return t.ai.noReading;
    if (usedPercent(window) === null) {
      return !restarted(window) || window.resetsAt === null ? t.ai.noReading : interpolate(t.ai.restarted, { time: clock.format(window.resetsAt) });
    }
    return window.resetsAt === null ? "" : resets(window.resetsAt);
  }

  const FIVE_HOURS = 300;
  const WEEK = 10080;

  /**
   * The window of that duration in the bucket covering every model; per model
   * buckets stay out of the panel. A window without a duration keeps its slot.
   */
  function windowOf(reading: AiReading | null, durationMins: typeof FIVE_HOURS | typeof WEEK) {
    const bucket = reading?.buckets.find((b) => b.name === null) ?? reading?.buckets[0];
    if (!bucket) return null;
    const slot = durationMins === FIVE_HOURS ? bucket.primary : bucket.secondary;
    return [bucket.primary, bucket.secondary].find((w) => w?.durationMins === durationMins)
      ?? (slot?.durationMins == null ? slot : null);
  }

  const actions = {
    configure: { label: t.ai.configure, run: () => configure() },
    reconnect: { label: t.ai.reconnect, run: () => reconnect() },
  };
  /** A state that explains or recovers a provider; alerts need the user. */
  interface Notice { text: string; alert?: boolean; action?: keyof typeof actions }
  interface Provider {
    name: string;
    reading: AiReading | null;
    /** Explanation replacing the instruments when there is nothing to show. */
    blank: Notice | null;
    notices: Notice[];
  }

  function failure(since: number | null | undefined, paused: boolean | undefined): Notice[] {
    return since != null && !paused ? [{ text: interpolate(t.ai.failureAge, { age: age(since) }) }] : [];
  }

  function codexOf(codex: AiData["codex"]): Provider {
    const missing = codex.state === "missing";
    return {
      name: t.ai.codex,
      reading: missing ? null : codex.reading,
      blank: missing ? { text: t.ai.missingCodex } : codex.reading ? null : { text: t.ai.emptyCodex },
      notices: missing ? [] : [...failure(codex.failureSince, data.paused), ...(data.paused ? [{ text: t.ai.paused }] : [])],
    };
  }

  /** Each provider is independent: Claude's state never hides Codex. */
  function claudeOf(claude: AiData["claude"]): Provider {
    const name = t.ai.claude;
    if (!claude) return { name, reading: null, blank: { text: t.ai.noReading }, notices: [] };
    if (claude.state === "missing") return { name, reading: null, blank: { text: t.ai.missingClaude }, notices: [] };
    const off: Notice | null = claude.statusLine === "off" && claude.usage !== "on"
      ? { text: t.ai.offClaude, action: "configure" } : null;
    const notices: Notice[] = claude.paused ? [{ text: t.ai.paused }] : [];
    if (claude.usage === "expired") notices.push({ text: t.ai.expired, alert: true });
    else if (claude.usage === "on") notices.push(...failure(claude.failureSince, claude.paused));
    if (claude.statusLine === "replaced") notices.push({ text: t.ai.replaced, alert: true, action: "reconnect" });
    if (claude.reading) return { name, reading: claude.reading, blank: null, notices: off ? [...notices, off] : notices };
    const blank = off ?? { text: claude.statusLine === "connected" ? t.ai.emptyClaude : t.ai.noReading };
    return { name, reading: null, blank, notices };
  }

  function refreshNow() {
    allowedAt = Date.now() + 10000;
    refresh();
  }

  const providers = $derived([codexOf(data.codex), claudeOf(data.claude)]);
</script>

{#snippet dial(provider: string, window: AiWindow | null)}
  {@const percent = usedPercent(window)}
  {@const name = interpolate(t.ai.fiveHourOf, { provider })}
  <div class="instrument">
    {#if percent === null}
      <div class="dial" role="img" aria-label="{name}: {status(window)}">
        <svg viewBox="0 0 140 140" aria-hidden="true">
          <circle class="ticks" cx="70" cy="70" r="66" />
          <circle class="track" cx="70" cy="70" r="55" />
        </svg>
        <div class="value" aria-hidden="true"><strong class="num">—</strong></div>
      </div>
    {:else}
      <div class="dial" role="meter" aria-label={name} aria-valuemin={0} aria-valuemax={100} aria-valuenow={percent}
        aria-valuetext={valueText(percent)}>
        <svg viewBox="0 0 140 140" aria-hidden="true">
          <circle class="ticks" cx="70" cy="70" r="66" />
          <circle class="track" cx="70" cy="70" r="55" />
          <circle class="reading" class:high={percent >= HIGH} cx="70" cy="70" r="55" pathLength="100"
            stroke-dasharray="{Math.min(100, percent)} 100" transform="rotate(-90 70 70)" />
        </svg>
        <div class="value">
          <strong class="num">{#if percent >= HIGH}<span class="high-mark">⚠</span>{/if}{percent}<small>%</small></strong>
          <span>{percent >= HIGH ? t.ai.high : t.ai.consumed}</span>
        </div>
      </div>
    {/if}
    <p class="caption">{t.ai.fiveHourLabel}</p>
    <p class="caption num">{status(window)}</p>
  </div>
{/snippet}

{#snippet bar(provider: string, window: AiWindow | null)}
  {@const percent = usedPercent(window)}
  <div class="weekly">
    <span class="caption">{t.ai.weeklyLabel}</span>
    {#if percent !== null}
      <div class="bar" class:high={percent >= HIGH} role="meter" aria-label={interpolate(t.ai.weeklyOf, { provider })}
        aria-valuemin={0} aria-valuemax={100} aria-valuenow={percent} aria-valuetext={valueText(percent)}>
        <i style:width="{Math.min(100, percent)}%"></i>
      </div>
      <b class="num">{#if percent >= HIGH}<span class="high-mark" aria-hidden="true">⚠</span>{/if}{percent}%</b>
    {/if}
    <span class="caption wide num">{status(window)}</span>
  </div>
{/snippet}

{#snippet notice(n: Notice)}
  {n.text}{#if n.action}{" · "}<button class="link" onclick={actions[n.action].run}>{actions[n.action].label}</button>{/if}
{/snippet}

<div class="ai">
  <header class="heading">
    <div><h2 id="module-ai-title">{t.modules.ai}</h2><p class="caption"><span>{t.sources.ai}</span>{#if updated} · <span>{updated}</span>{/if}</p></div>
    <div class="actions" bind:this={titleActions}>
      <span class="action-wrap" role="group" onpointerenter={() => tipsDismissed = false} onfocusin={() => tipsDismissed = false}>
        <button class="action" aria-label={t.ai.refresh} aria-describedby="ai-refresh-description"
          disabled={data.paused || data.refreshing || now < allowedAt} onclick={refreshNow}><span aria-hidden="true">↻</span></button>
        <span class="tooltip" hidden={tipsDismissed} id="ai-refresh-description" role="tooltip">{t.ai.refreshDescription}</span>
      </span>
      <span class="action-wrap" role="group" onpointerenter={() => tipsDismissed = false} onfocusin={() => tipsDismissed = false}>
        <button class="action" aria-label={t.ai.configureAi} aria-describedby="ai-configure-description" onclick={configure}><Icon name="settings" size={16} /></button>
        <span class="tooltip" hidden={tipsDismissed} id="ai-configure-description" role="tooltip">{t.ai.configureDescription}</span>
      </span>
    </div>
  </header>
  {#if data.refreshing}<p role="status" class="caption">{t.ai.refreshing}</p>{/if}

  <div class="providers">
    {#each providers as provider (provider.name)}
      <section aria-label={provider.name}>
        <h3>{provider.name}</h3>
        {#if provider.blank}
          <p class="blank">{@render notice(provider.blank)}</p>
        {:else}
          {@render dial(provider.name, windowOf(provider.reading, FIVE_HOURS))}
          {@render bar(provider.name, windowOf(provider.reading, WEEK))}
        {/if}
        {#if provider.reading}
          <p class="source num">{interpolate(t.ai.reading, { source: t.ai.sources[provider.reading.source], age: age(provider.reading.readAt) })}</p>
          <p class="source num"><time datetime={new Date(provider.reading.readAt).toISOString()}>{interpolate(t.ai.readingTime, { time: clock.format(provider.reading.readAt) })}</time></p>
        {/if}
        {#each provider.notices as n (n.text)}
          <p role="status" class="notice" class:alert={n.alert}><span aria-hidden="true">{n.alert ? "⚠ " : "◷ "}</span>{@render notice(n)}</p>
        {/each}
      </section>
    {/each}
  </div>

</div>

<style>
  .ai { padding: 8px 10px 10px; min-width: 0; display: flex; flex-direction: column; gap: 8px; }
  .providers { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 8px; }
  section { display: flex; flex-direction: column; gap: 5px; min-width: 0; }
  h3 { font-size: 13px; font-weight: 500; letter-spacing: 1px; text-transform: uppercase; text-align: center; }
  .instrument { text-align: center; }
  .dial { position: relative; width: min(100%, 94px); margin: 0 auto; }
  svg { display: block; width: 100%; overflow: visible; fill: none; }
  circle { stroke-width: 5; }
  .ticks { stroke: var(--line); stroke-width: 2; stroke-dasharray: 1 4; }
  .track { stroke: var(--surface-2); }
  .reading { stroke: var(--accent); }
  .reading.high { stroke: var(--warn); }
  .value { position: absolute; inset: 0; display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 2px; }
  strong { font-size: 26px; font-weight: 300; }
  small { font-size: 14px; color: var(--muted); }
  .value span { font: 9px var(--mono); letter-spacing: 0.5px; text-transform: uppercase; color: var(--muted); }
  .caption { margin-top: 6px; font-size: 11px; color: var(--muted); }
  .weekly { display: grid; grid-template-columns: auto minmax(0, 1fr) auto; align-items: center; gap: 4px 10px; }
  .weekly .caption { margin: 0; }
  .weekly .wide { grid-column: 1 / -1; }
  .weekly b { font-size: 12px; font-weight: 500; }
  .bar { height: 5px; background: var(--surface-2); overflow: hidden; }
  .bar > i { display: block; height: 100%; background: var(--accent); }
  .bar.high > i { background: var(--warn); }
  .high-mark { margin-right: 4px; font-size: 0.6em; color: var(--warn); vertical-align: middle; }
  .blank { display: grid; place-items: center; min-height: 94px; padding: 12px; border: 1px dashed var(--line); font-size: 12px; color: var(--muted); text-align: center; }
  .notice { font-size: 12px; color: var(--muted); text-align: center; }
  .notice.alert { color: var(--warn); }
  .actions { display: flex; flex-wrap: wrap; align-items: center; gap: 8px; margin-top: auto; }
  .heading { display: flex; align-items: center; justify-content: space-between; gap: 8px; border-bottom: 1px solid var(--line); padding-bottom: 6px; }
  h2 { font-size: 17px; font-weight: 500; letter-spacing: 1px; color: var(--module-title, var(--band-cyan)); }
  .action-wrap { position: relative; display: inline-flex; }
  .action { display: grid; place-items: center; width: 30px; height: 30px; border: 1px solid var(--control-line); border-radius: 50%; color: var(--accent); }
  .action:hover:not(:disabled) { background: var(--surface-2); }
  .action:disabled { opacity: 0.45; cursor: default; }
  .action:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .tooltip { position: absolute; top: 100%; right: 0; width: 180px; max-width: 65vw; padding: 8px; background: var(--surface); color: var(--text); border: 1px solid var(--control-line); font-size: 12px; z-index: 5; visibility: hidden; }
  .action-wrap:hover .tooltip, .action-wrap:focus-within .tooltip { visibility: visible; }
  .link { color: var(--accent); text-decoration: underline; }
  .link:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .source { font-size: 11px; color: var(--muted); text-align: center; }
</style>
