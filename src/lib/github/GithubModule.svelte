<script lang="ts">
  import { onMount } from "svelte";
  import type { GithubData } from "../core";
  import { t, interpolate } from "../strings";

  let { data, refresh, configure = () => {} }:
    { data: GithubData | null; refresh: () => void; configure?: () => void } = $props();
  let now = $state(Date.now());
  /** Manual refreshes stay at least ten seconds apart. */
  let allowedAt = $state(0);
  onMount(() => {
    const timer = setInterval(() => (now = Date.now()), 1000);
    return () => clearInterval(timer);
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
              <p class="caption num">{eight.total === 0 ? t.github.weeksNone : interpolate(t.github.weeks, { total: count.format(eight.total) })}</p>
            </div>
          {/if}
        </div>
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
  <div class="actions">
    <button class="action" disabled={paused || data?.refreshing || now < allowedAt} onclick={refreshNow}>
      <span aria-hidden="true">↻ </span>{t.github.refresh}
    </button>
    <button class="action" onclick={configure}>{t.github.configureGithub}</button>
    {#if data?.refreshing}<span role="status" class="caption">{t.github.refreshing}</span>{/if}
  </div>
</div>

<style>
  .github { padding: 16px 16px 18px; min-width: 0; display: flex; flex-direction: column; gap: 14px; flex: 1; }
  .readings[data-stale="true"] { --text: var(--muted); --accent: var(--muted); color: var(--muted); }
  .readings { display: flex; flex-direction: column; gap: 12px; flex: 1; }
  .lead { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 12px; }
  .annual { display: flex; flex-direction: column; gap: 2px; }
  strong { font-size: 36px; font-weight: 300; line-height: 1.1; }
  .annual span { font: 10px var(--mono); letter-spacing: 1.5px; text-transform: uppercase; color: var(--muted); }
  .weeks { display: flex; flex-direction: column; align-items: flex-end; gap: 6px; }
  .heatmap { display: grid; grid-template-rows: repeat(7, 8px); grid-auto-columns: 8px; gap: 3px; }
  .heatmap span { background: color-mix(in srgb, var(--accent) calc(var(--intensity) * 100%), transparent); border: 1px solid color-mix(in srgb, var(--accent) 20%, transparent); }
  .bars { display: flex; align-items: flex-end; gap: 5px; height: 90px; margin: 0; padding: 0; list-style: none; border-bottom: 1px solid var(--line); }
  .bars li { flex: 1; height: 100%; display: flex; align-items: flex-end; }
  .bars i { display: block; width: 100%; background: var(--accent); border-radius: 4px 4px 0 0; }
  .caption { font-size: 11px; color: var(--muted); }
  .repo { display: flex; flex-direction: column; gap: 6px; margin-top: auto; padding-top: 12px; border-top: 1px solid var(--line); min-width: 0; }
  h3 { font: 10px var(--mono); letter-spacing: 1.5px; text-transform: uppercase; color: var(--muted); }
  .repo p { font-size: 12px; }
  .repo .name { font-size: 15px; font-weight: 500; overflow-wrap: anywhere; }
  .title { color: var(--muted); overflow-wrap: anywhere; }
  .blank { display: grid; place-items: center; min-height: 120px; padding: 12px; border: 1px dashed var(--line); font-size: 12px; color: var(--muted); text-align: center; }
  .notice { font-size: 12px; color: var(--muted); }
  .notice.alert { color: var(--warn); }
  .link { color: var(--accent); text-decoration: underline; }
  .link:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .actions { display: flex; flex-wrap: wrap; align-items: center; gap: 8px; }
  .action { min-height: 36px; padding: 8px 15px; border-radius: 0 18px 18px 0; background: var(--band-steel); color: var(--on-band); font-weight: 600; }
  .action:hover:not(:disabled) { filter: brightness(1.15); }
  .action:disabled { opacity: 0.45; cursor: default; }
  .action:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
</style>
