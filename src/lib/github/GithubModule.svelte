<script lang="ts">
  import Icon from "../Icon.svelte";
  import { onMount } from "svelte";
  import type { GithubData } from "../core";
  import { t, interpolate } from "../strings";

  let { data, refresh, updated = "", configure = () => {} }:
    { data: GithubData | null; refresh: () => void; updated?: string; configure?: () => void } = $props();
  let tipsDismissed = $state(false);
  let titleActions: HTMLDivElement;
  let now = $state(Date.now());
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
  const count = new Intl.NumberFormat("pt-BR");
  function age(when: number) {
    const seconds = Math.max(0, Math.floor((now - when) / 1000));
    if (seconds < 3600) return relative.format(-Math.floor(seconds / 60), "minute");
    if (seconds < 86400) return relative.format(-Math.floor(seconds / 3600), "hour");
    return relative.format(-Math.floor(seconds / 86400), "day");
  }

  type Day = { date: string; contributionCount: number };
  /** Calendar dates are YYYY-MM-DD; shown as DD/MM. */
  const dayMonth = (date: string) => `${date.slice(8, 10)}/${date.slice(5, 7)}`;
  /** Local date of now, comparable with calendar dates. */
  function today() {
    const d = new Date(now);
    return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
  }
  /** Total, busiest day and date range of a window; null when it has no days. */
  function summary(days: Day[]) {
    if (days.length === 0) return null;
    return {
      total: days.reduce((sum, d) => sum + d.contributionCount, 0),
      peak: days.reduce((top, d) => (d.contributionCount > top.contributionCount ? d : top)),
      from: dayMonth(days[0].date),
      to: dayMonth(days.at(-1)!.date),
    };
  }
  /** pt-BR singular for exactly one, plural otherwise. */
  const plural = (n: number, forms: { one: string; other: string }) => (n === 1 ? forms.one : forms.other);

  /**
   * The calendar covers the whole calendar year, future days included; the
   * panel shows only its last eight weeks and 14 days up to today.
   */
  const activity = $derived.by(() => {
    const calendar = data?.calendar;
    if (!calendar) return null;
    const until = today();
    const weeks = calendar.contributionCalendar.weeks
      .map((w) => w.contributionDays.filter((d) => d.date <= until))
      .filter((w) => w.length > 0);
    const all = weeks.flat();
    const recent = weeks.slice(-8).flat();
    const days = all.slice(-14);
    return {
      year: (all.at(-1)?.date ?? until).slice(0, 4),
      annual: calendar.contributionCalendar.totalContributions,
      restricted: calendar.restrictedContributionsCount,
      weeks: weeks.slice(-8),
      eight: summary(recent),
      days,
      fourteen: summary(days),
      max: Math.max(1, ...recent.map((d) => d.contributionCount)),
    };
  });

  const paused = $derived(data?.paused ?? false);
  const missing = $derived(data?.state === "missingGh");
  const invalid = $derived(data?.state === "invalidCredential" && !paused);
  /** Weekday row (Sunday first) of a calendar date. */
  const weekday = (date: string) => new Date(`${date}T00:00:00Z`).getUTCDay();
  /** Intensity from the panel's own local scale, never GitHub's colours. */
  const intensity = (n: number, max: number) => (n === 0 ? 0.06 : Math.min(1, 0.2 + (n / max) * 0.8));

  function refreshNow() {
    allowedAt = Date.now() + 10000;
    refresh();
  }
</script>

<div class="github">
  <header class="heading">
    <div><h2 id="module-github-title">{t.modules.github}</h2><p class="caption">{t.sources.github}{#if updated} · {updated}{/if}</p></div>
    <div class="actions" bind:this={titleActions}>
      <span class="action-wrap" role="group" onpointerenter={() => tipsDismissed = false} onfocusin={() => tipsDismissed = false}>
        <button class="action" aria-label={t.github.refresh} aria-describedby="github-refresh-description"
          disabled={paused || data?.refreshing || now < allowedAt} onclick={refreshNow}><span aria-hidden="true">↻</span></button>
        <span class="tooltip" hidden={tipsDismissed} id="github-refresh-description" role="tooltip">{t.github.refreshDescription}</span>
      </span>
      <span class="action-wrap" role="group" onpointerenter={() => tipsDismissed = false} onfocusin={() => tipsDismissed = false}>
        <button class="action" aria-label={t.github.configureGithub} aria-describedby="github-configure-description" onclick={configure}><Icon name="settings" size={16} /></button>
        <span class="tooltip" hidden={tipsDismissed} id="github-configure-description" role="tooltip">{t.github.configureDescription}</span>
      </span>
    </div>
  </header>
  {#if data?.refreshing}<p role="status" class="caption">{t.github.refreshing}</p>{/if}
  {#if paused}<p class="notice" role="status">{t.github.paused}</p>{/if}
  {#if missing}
    <p class="blank">{t.github.missingSource}</p>
  {:else}
    {#if invalid}
      <p class="notice alert"><span aria-hidden="true">⚠ </span>{t.github.invalidCredential}{" · "}<button class="link" onclick={configure}>{t.github.reconnect}</button></p>
    {/if}
    {#if data?.state === "error" && !paused}
      <p class="notice"><span aria-hidden="true">◷ </span>{data.failureSince != null ? interpolate(t.github.failureAge, { age: age(data.failureSince) }) : t.github.error}</p>
    {/if}
    {#if data?.lastSuccessAt != null && (data.state !== "ready" || paused)}
      <p class="notice num">{interpolate(t.github.updatedAge, { age: age(data.lastSuccessAt) })}</p>
    {/if}
    <div class="readings" data-stale={invalid}>
      {#if activity}
        {@const { eight, fourteen } = activity}
        <div class="lead">
          <p class="annual">
            <strong class="num">{count.format(activity.annual)}</strong>
            <span>{interpolate(t.github.annual, { year: activity.year })}</span>
          </p>
          {#if eight}
            <div class="weeks">
              <div class="heatmap" role="img" aria-label={eight.total === 0
                ? interpolate(t.github.weeksMapNone, { from: eight.from, to: eight.to })
                : interpolate(t.github.weeksMap, { total: count.format(eight.total), from: eight.from, to: eight.to, peak: count.format(eight.peak.contributionCount) })}>
                {#each activity.weeks as week, column (week[0].date)}
                  {#each week as day (day.date)}
                    <span data-date={day.date} style:grid-column={column + 1} style:grid-row={weekday(day.date) + 1}
                      style:--intensity={intensity(day.contributionCount, activity.max)}></span>
                  {/each}
                {/each}
              </div>
            </div>
          {/if}
        </div>
        {#if eight}<p class="caption num">{eight.total === 0 ? t.github.weeksNone : interpolate(t.github.weeks, { total: count.format(eight.total) })}</p>{/if}
        <p class="caption">{interpolate(t.github.private, { total: count.format(activity.restricted) })}</p>
        {#if fourteen}
          <ul class="bars" aria-label={t.github.daysList}>
            {#each activity.days as day (day.date)}
              <li aria-label={interpolate(plural(day.contributionCount, t.github.day), { day: dayMonth(day.date), count: count.format(day.contributionCount) })}>
                <i style:height="{Math.max(2, (day.contributionCount / Math.max(1, fourteen.peak.contributionCount)) * 100)}%"></i>
              </li>
            {/each}
          </ul>
          <p class="caption num">{fourteen.total === 0 ? t.github.daysNone
            : interpolate(t.github.days, { total: count.format(fourteen.total), peak: count.format(fourteen.peak.contributionCount), day: dayMonth(fourteen.peak.date) })}</p>
        {/if}
      {:else}
        <p class="blank">{t.github.noReading}</p>
      {/if}
      <!-- Without any successful reading, "no commits" would be a claim nothing supports. -->
      {#if data && (data.featured || data.state === "ready" || data.lastSuccessAt != null)}
        <section class="repo" aria-labelledby="github-featured">
          <h3 id="github-featured">{t.github.featured}</h3>
          {#if data.featured}
            <p class="name">{data.featured.name}</p>
            <p class="num">{interpolate(plural(data.featured.commits, t.github.commits), { commits: count.format(data.featured.commits) })}</p>
            <!-- The disk cache never stores commit titles. -->
            {#if data.featured.title}<p class="title">{data.featured.title}</p>{/if}
          {:else}
            <p class="caption">{t.github.empty}</p>
          {/if}
        </section>
      {/if}
    </div>
  {/if}

</div>

<style>
  .github { padding: 12px; min-width: 0; display: flex; flex-direction: column; gap: 8px; flex: 1; }
  .readings[data-stale="true"] { --text: var(--muted); --accent: var(--muted); color: var(--muted); }
  .readings { display: flex; flex-direction: column; gap: 6px; flex: 1; }
  .lead { display: flex; align-items: center; justify-content: space-between; gap: 12px; }
  .annual { min-width: 0; display: flex; flex-direction: column; gap: 2px; }
  strong { font-size: 36px; font-weight: 300; line-height: 1.1; }
  .annual span { font: 10px var(--mono); letter-spacing: 1.5px; text-transform: uppercase; color: var(--muted); }
  .weeks { flex-shrink: 0; display: flex; flex-direction: column; align-items: flex-end; gap: 6px; }
  .heatmap { display: grid; grid-template-rows: repeat(7, 7px); grid-auto-columns: 7px; gap: 3px; }
  .heatmap span { background: color-mix(in srgb, var(--accent) calc(var(--intensity) * 100%), transparent); border: 1px solid color-mix(in srgb, var(--accent) 20%, transparent); }
  .bars { display: flex; align-items: flex-end; gap: 5px; height: 26px; margin: 0; padding: 0; list-style: none; border-bottom: 1px solid var(--line); }
  .bars li { flex: 1; height: 100%; display: flex; align-items: flex-end; }
  .bars i { display: block; width: 100%; background: var(--accent); border-radius: 4px 4px 0 0; }
  .caption { font-size: 11px; color: var(--muted); }
  .repo { display: flex; flex-direction: column; gap: 4px; margin-top: 3px; padding-top: 8px; border-top: 1px solid var(--line); min-width: 0; }
  h3 { font: 10px var(--mono); letter-spacing: 1.5px; text-transform: uppercase; color: var(--muted); }
  .repo p { font-size: 12px; }
  .repo .name { font-size: 15px; font-weight: 500; overflow-wrap: anywhere; }
  .title { color: var(--muted); overflow-wrap: anywhere; }
  .blank { display: grid; place-items: center; min-height: 120px; padding: 12px; border: 1px dashed var(--line); font-size: 12px; color: var(--muted); text-align: center; }
  .notice { font-size: 12px; color: var(--muted); }
  .notice.alert { color: var(--warn); }
  .link { color: var(--accent); text-decoration: underline; }
  .link:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .heading { display: flex; align-items: flex-start; justify-content: space-between; gap: 8px; }
  .heading h2 { font-size: 14px; font-weight: 600; }
  .heading .caption { margin-top: 3px; }
  .heading > div:first-child { min-width: 0; }
  .action-wrap { position: relative; display: inline-flex; }
  .tooltip { position: absolute; top: 100%; right: 0; width: 180px; max-width: 65vw; padding: 8px; background: var(--surface); color: var(--text); border: 1px solid var(--control-line); font-size: 12px; z-index: 5; visibility: hidden; }
  .action-wrap:hover .tooltip, .action-wrap:focus-within .tooltip { visibility: visible; }
  .actions { display: flex; align-items: center; gap: 6px; }
  .action { display: grid; place-items: center; width: 28px; height: 28px; border: 1px solid var(--control-line); border-radius: 6px; color: var(--text); background: var(--surface-2); }
  .action:hover:not(:disabled) { filter: brightness(1.15); }
  .action:disabled { opacity: 0.45; cursor: default; }
  .action:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
</style>
