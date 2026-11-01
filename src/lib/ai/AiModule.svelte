<script lang="ts">
  import { onMount } from "svelte";
  import type { AiData, AiReading, AiWindow } from "../core";
  import { t, interpolate } from "../strings";

  /** Renders the top of the "IA e Agentes" card: its shared heading and the AI instruments. */
  let { data, refresh, configure = () => {}, reconnect = () => {} }:
    { data: AiData; refresh: () => void; configure?: () => void; reconnect?: () => void } = $props();
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
  const dayAndClock = new Intl.DateTimeFormat("pt-BR", { day: "2-digit", month: "2-digit", hour: "2-digit", minute: "2-digit" });
  const date = new Intl.DateTimeFormat("pt-BR", { day: "2-digit", month: "2-digit", year: "numeric" });

  function age(when: number) {
    const seconds = Math.max(0, Math.floor((now - when) / 1000));
    if (seconds < 60) return t.ai.now;
    if (seconds < 3600) return relative.format(-Math.floor(seconds / 60), "minute");
    if (seconds < 86400) return relative.format(-Math.floor(seconds / 3600), "hour");
    return relative.format(-Math.floor(seconds / 86400), "day");
  }

  /** "Leitura HH:MM"; a reading older than a day also names its date. */
  function readAt(when: number) {
    return interpolate(t.ai.readAt, { time: (now - when >= 86_400_000 ? dayAndClock : clock).format(when) });
  }

  /** Usage from which the reading is named high; the ring keeps the provider colour. */
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
  /** Why a window has no value: no reading, or restarted without a new one. */
  function missing(window: AiWindow | null) {
    return !window || !restarted(window) || window.resetsAt === null
      ? t.ai.noReading : interpolate(t.ai.restarted, { time: clock.format(window.resetsAt) });
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
  /** A conditional line in the card body; alerts need the user. */
  interface Notice { text: string; alert?: boolean; action?: keyof typeof actions }
  interface Provider {
    name: string;
    /** Ring colour: always the provider's, whatever the usage. */
    tone: string;
    reading: AiReading | null;
    /** Explanation shown under an empty ring when there is nothing to show. */
    blank: Notice | null;
    notices: Notice[];
  }

  function failure(since: number | null | undefined, paused: boolean | undefined): Notice[] {
    return since != null && !paused ? [{ text: interpolate(t.ai.failureAge, { age: age(since) }), alert: true }] : [];
  }

  function codexOf(codex: AiData["codex"]): Provider {
    const missing = codex.state === "missing";
    return {
      name: t.ai.codex,
      tone: "var(--accent)",
      reading: missing ? null : codex.reading,
      blank: missing ? { text: t.ai.missingCodex, alert: true } : codex.reading ? null : { text: t.ai.emptyCodex },
      notices: missing ? [] : [...failure(codex.failureSince, data.paused), ...(data.paused ? [{ text: t.ai.paused }] : [])],
    };
  }

  /** Each provider is independent: Claude's state never hides Codex. */
  function claudeOf(claude: AiData["claude"]): Provider {
    const base = { name: t.ai.claude, tone: "var(--band-lavender-text)" };
    if (!claude) return { ...base, reading: null, blank: { text: t.ai.noReading }, notices: [] };
    if (claude.state === "missing") return { ...base, reading: null, blank: { text: t.ai.missingClaude, alert: true }, notices: [] };
    const off: Notice | null = claude.statusLine === "off" && claude.usage !== "on"
      ? { text: t.ai.offClaude, action: "configure" } : null;
    const notices: Notice[] = claude.paused ? [{ text: t.ai.paused }] : [];
    if (claude.usage === "expired") notices.push({ text: t.ai.expired, alert: true });
    else if (claude.usage === "on") notices.push(...failure(claude.failureSince, claude.paused));
    if (claude.statusLine === "replaced") notices.push({ text: t.ai.replaced, alert: true, action: "reconnect" });
    if (claude.reading) return { ...base, reading: claude.reading, blank: null, notices: off ? [...notices, off] : notices };
    const blank = off ?? { text: claude.statusLine === "connected" ? t.ai.emptyClaude : t.ai.noReading };
    return { ...base, reading: null, blank, notices };
  }

  function refreshNow() {
    allowedAt = Date.now() + 10000;
    refresh();
  }

  const providers = $derived([codexOf(data.codex), claudeOf(data.claude)]);
</script>

{#snippet rings(arc: number | null)}
  <svg viewBox="0 0 120 120" aria-hidden="true">
    <circle class="orbit" cx="60" cy="60" r="55" />
    <circle class="base" cx="60" cy="60" r="47" />
    {#if arc !== null}
      <circle class="arc" cx="60" cy="60" r="47" pathLength="100" stroke-dasharray="{Math.min(100, arc)} 100" transform="rotate(-90 60 60)" />
    {/if}
  </svg>
{/snippet}

{#snippet dial(provider: string, window: AiWindow | null)}
  {@const percent = usedPercent(window)}
  {@const name = interpolate(t.ai.fiveHourOf, { provider })}
  {#if percent === null}
    <div class="dial" role="img" aria-label="{name}: {missing(window)}">
      {@render rings(null)}<span class="reading num" aria-hidden="true">—</span>
    </div>
  {:else}
    <div class="dial" role="meter" aria-label={name} aria-valuemin={0} aria-valuemax={100} aria-valuenow={percent}
      aria-valuetext={valueText(percent)}>
      {@render rings(percent)}<span class="reading num">{percent}<small>%</small></span>
    </div>
  {/if}
{/snippet}

{#snippet notice(n: Notice)}
  <small role="status" class="line" class:alert={n.alert}>{#if n.alert}<span aria-hidden="true">{"⚠ "}</span>{/if}{n.text}{#if n.action}{" · "}<button class="link" onclick={actions[n.action].run}>{actions[n.action].label}</button>{/if}</small>
{/snippet}

<header>
  <h2>{t.intelligence.title}</h2>
  <div class="actions" bind:this={titleActions}>
    <span class="action-wrap" role="group" onpointerenter={() => tipsDismissed = false} onfocusin={() => tipsDismissed = false}>
      <button class="icon" aria-label={t.ai.refresh} aria-describedby="ai-refresh-description"
        disabled={data.paused || data.refreshing || now < allowedAt} onclick={refreshNow}><span aria-hidden="true">↻</span></button>
      <span class="tip" hidden={tipsDismissed} id="ai-refresh-description" role="tooltip">{t.ai.refreshDescription}</span>
    </span>
    <span class="action-wrap" role="group" onpointerenter={() => tipsDismissed = false} onfocusin={() => tipsDismissed = false}>
      <button class="icon" aria-label={t.ai.configureAi} aria-describedby="ai-configure-description" onclick={configure}><span aria-hidden="true">⚙</span></button>
      <span class="tip" hidden={tipsDismissed} id="ai-configure-description" role="tooltip">{t.ai.configureDescription}</span>
    </span>
  </div>
</header>
{#if data.refreshing}<small role="status" class="line">{t.ai.refreshing}</small>{/if}

<div class="instruments">
  {#each providers as provider (provider.name)}
    {@const five = windowOf(provider.reading, FIVE_HOURS)}
    {@const week = windowOf(provider.reading, WEEK)}
    {@const fivePercent = provider.blank ? null : usedPercent(five)}
    {@const weekPercent = provider.blank ? null : usedPercent(week)}
    <section class="provider" aria-label={provider.name} style:--instrument={provider.tone}>
      <div class="instrument">
        {#if provider.blank}
          <div class="dial" aria-hidden="true">{@render rings(null)}<span class="reading num">—</span></div>
        {:else}
          {@render dial(provider.name, five)}
        {/if}
        <h3>{provider.name}</h3>
        <small>{fivePercent !== null && five?.resetsAt != null
          ? interpolate(t.ai.fiveHourResets, { time: clock.format(five.resetsAt) }) : t.ai.fiveHour}</small>
      </div>
      {#if provider.blank}
        {@render notice(provider.blank)}
      {:else}
        {#if fivePercent === null}<small class="line">{missing(five)}</small>{/if}
        <div class="meter">
          <div class="meter-label"><span>{t.ai.weeklyLabel}</span><b class="num">{weekPercent === null ? "—" : `${weekPercent}%`}</b></div>
          {#if weekPercent !== null}
            <div class="track" role="meter" aria-label={interpolate(t.ai.weeklyOf, { provider: provider.name })}
              aria-valuemin={0} aria-valuemax={100} aria-valuenow={weekPercent} aria-valuetext={valueText(weekPercent)}>
              <i style:width="{Math.min(100, weekPercent)}%"></i>
            </div>
            {#if week?.resetsAt != null}<small>{interpolate(t.ai.weeklyResets, { date: date.format(week.resetsAt) })}</small>{/if}
          {:else}
            <div class="track" aria-hidden="true"></div>
            <small>{missing(week)}</small>
          {/if}
        </div>
      {/if}
      {#if provider.reading}
        <small class="source"><time datetime={new Date(provider.reading.readAt).toISOString()}>{readAt(provider.reading.readAt)}</time></small>
      {/if}
      {#if fivePercent !== null && fivePercent >= HIGH}{@render notice({ text: t.ai.highFiveHour, alert: true })}{/if}
      {#if weekPercent !== null && weekPercent >= HIGH}{@render notice({ text: t.ai.highWeekly, alert: true })}{/if}
      {#each provider.notices as n (n.text)}{@render notice(n)}{/each}
    </section>
  {/each}
</div>

<style>
  header { display: flex; align-items: center; gap: 10px; justify-content: space-between; min-height: 28px; margin-bottom: 6px; }
  h2 { font-size: 1.25em; font-weight: 500; text-transform: uppercase; letter-spacing: 1px; }
  h3 { font-size: 1em; font-weight: 500; }
  small { font-size: .88em; color: var(--muted); }
  .actions { display: flex; gap: 5px; }
  .action-wrap { position: relative; display: inline-flex; }
  .icon { width: 28px; height: 28px; border: 1px solid var(--line); border-radius: 50%; font-size: 19px; line-height: 1; display: grid; place-items: center; }
  .icon:hover:not(:disabled) { background: var(--surface-2); }
  .icon:disabled { opacity: .45; cursor: default; }
  .tip { position: absolute; z-index: 4; right: 0; top: 100%; min-width: 150px; background: var(--surface-2); color: var(--text); padding: 9px; border: 1px solid var(--control-line); box-shadow: 0 4px 20px #0006; font-size: 12px; font-weight: 400; visibility: hidden; }
  .action-wrap:hover .tip, .action-wrap:focus-within .tip { visibility: visible; }
  .instruments { display: grid; grid-template-columns: 1fr 1fr; gap: 18px 12px; align-items: start; }
  .provider { min-width: 0; display: flex; flex-direction: column; }
  .instrument { text-align: center; display: flex; flex-direction: column; }
  /* --dial grows with the free height of the card (Panel); never wider than the column. */
  .dial { position: relative; width: min(var(--dial, 94px), 100%); aspect-ratio: 1; margin: auto; container-type: inline-size; }
  .dial svg { width: 100%; height: 100%; display: block; }
  /* The B size at 94 px, growing at a quarter of the dial so big rings keep a light number. */
  .reading { position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-size: calc(33px + (100cqi - 94px) / 4); gap: 2px; }
  .reading small { font-size: .4em; }
  .base { fill: none; stroke: var(--line); stroke-width: 8; }
  .arc { fill: none; stroke: var(--instrument); stroke-width: 8; }
  .orbit { fill: none; stroke: var(--control-line); stroke-width: 1; stroke-dasharray: 24 12 6 30; transform-origin: center; }
  .meter { margin-top: 6px; display: flex; flex-direction: column; }
  .meter-label { display: flex; justify-content: space-between; gap: 8px; margin-bottom: 4px; }
  .track { height: 5px; background: var(--line); border-radius: 5px; }
  .track i { display: block; height: 100%; background: var(--accent); border-radius: inherit; }
  .meter small { margin-top: 4px; }
  .source { display: block; margin-top: 4px; }
  .line { display: block; margin-top: 4px; overflow-wrap: anywhere; }
  .line.alert { color: var(--warn); }
  .link { color: var(--accent); text-decoration: underline; font-size: inherit; }
  .icon:focus-visible, .link:focus-visible { outline: 2px solid var(--accent); outline-offset: 3px; }
</style>
