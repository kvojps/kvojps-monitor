<script lang="ts">
  import { onMount } from "svelte";
  import type { GithubData } from "../core";
  import { t, interpolate } from "../strings";

  /** The GitHub summary as prototype B shows it; states B does not show become body lines. */
  let { data, refresh, configure = () => {} }:
    { data: GithubData | null; refresh: () => void; configure?: () => void } = $props();
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
  const total = (days: Day[]) => days.reduce((sum, d) => sum + d.contributionCount, 0);
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
      .filter((w) => w.length > 0)
      .slice(-8);
    const recent = weeks.flat();
    const days = calendar.contributionCalendar.weeks.flatMap((w) => w.contributionDays).filter((d) => d.date <= until).slice(-14);
    return {
      annual: calendar.contributionCalendar.totalContributions,
      weeks,
      recent,
      days,
      weeksMax: Math.max(1, ...recent.map((d) => d.contributionCount)),
      daysMax: Math.max(1, ...days.map((d) => d.contributionCount)),
    };
  });

  const paused = $derived(data?.paused ?? false);
  const missing = $derived(data?.state === "missingGh");
  const invalid = $derived(data?.state === "invalidCredential" && !paused);
  /** Weekday row (Sunday first) of a calendar date. */
  const weekday = (date: string) => new Date(`${date}T00:00:00Z`).getUTCDay();
  /** Opacity on the panel's own local scale, as in B; never GitHub's colours. */
  const opacity = (n: number, max: number) => (n === 0 ? 0.1 : Math.max(0.3, n / max));

  function refreshNow() {
    allowedAt = Date.now() + 10000;
    refresh();
  }
</script>

<header>
  <h2 id="module-github-title">{t.modules.github}</h2>
  <div class="actions" bind:this={titleActions}>
    <span class="action-wrap" role="group" onpointerenter={() => tipsDismissed = false} onfocusin={() => tipsDismissed = false}>
      <button class="icon" aria-label={t.github.refresh} aria-describedby="github-refresh-description"
        disabled={paused || data?.refreshing || now < allowedAt} onclick={refreshNow}><span aria-hidden="true">↻</span></button>
      <span class="tip" hidden={tipsDismissed} id="github-refresh-description" role="tooltip">{t.github.refreshDescription}</span>
    </span>
    <span class="action-wrap" role="group" onpointerenter={() => tipsDismissed = false} onfocusin={() => tipsDismissed = false}>
      <button class="icon" aria-label={t.github.configureGithub} aria-describedby="github-configure-description" onclick={configure}><span aria-hidden="true">⚙</span></button>
      <span class="tip" hidden={tipsDismissed} id="github-configure-description" role="tooltip">{t.github.configureDescription}</span>
    </span>
  </div>
</header>

{#if data?.refreshing}<small role="status" class="line">{t.github.refreshing}</small>{/if}
{#if paused}<small role="status" class="line">{t.github.paused}</small>{/if}
{#if missing}
  <small class="line alert"><span aria-hidden="true">⚠ </span>{t.github.missingSource}</small>
{:else}
  {#if invalid}
    <small class="line alert"><span aria-hidden="true">⚠ </span>{t.github.invalidCredential}{" · "}<button class="link" onclick={configure}>{t.github.reconnect}</button></small>
  {/if}
  {#if data?.state === "error" && !paused}
    <small class="line alert"><span aria-hidden="true">⚠ </span>{data.failureSince != null ? interpolate(t.github.failureAge, { age: age(data.failureSince) }) : t.github.error}</small>
  {/if}
  {#if data?.lastSuccessAt != null && (data.state !== "ready" || paused)}
    <small class="line num">{interpolate(t.github.updatedAge, { age: age(data.lastSuccessAt) })}</small>
  {/if}
  <div class="readings" data-stale={invalid}>
    {#if activity}
      <div class="github-top">
        <div><strong class="num">{count.format(activity.annual)}</strong><small>{t.github.annual}</small></div>
        {#if activity.weeks.length > 0}
          {@const sum = total(activity.recent)}
          {@const from = dayMonth(activity.recent[0].date)}
          {@const to = dayMonth(activity.recent.at(-1)!.date)}
          <div>
            <div class="heatmap" role="img" aria-label={sum === 0
              ? interpolate(t.github.weeksMapNone, { from, to })
              : interpolate(t.github.weeksMap, { total: count.format(sum), from, to, peak: count.format(activity.weeksMax) })}>
              {#each activity.weeks as week, column (week[0].date)}
                {#each week as day (day.date)}
                  <i data-date={day.date} style:grid-column={column + 1} style:grid-row={weekday(day.date) + 1}
                    style:opacity={opacity(day.contributionCount, activity.weeksMax)}></i>
                {/each}
              {/each}
            </div>
            <small>{t.github.weeks}</small>
          </div>
        {/if}
      </div>
      {#if activity.days.length > 0}
        <ul class="activity" aria-label={t.github.daysList}>
          {#each activity.days as day (day.date)}
            <li aria-label={interpolate(plural(day.contributionCount, t.github.day), { day: dayMonth(day.date), count: count.format(day.contributionCount) })}>
              <i style:height="{Math.max(2, (day.contributionCount / activity.daysMax) * 100)}%"></i>
            </li>
          {/each}
        </ul>
        <small>{t.github.days}</small>
      {/if}
    {:else}
      <small class="line">{t.github.noReading}</small>
    {/if}
    <!-- Without any successful reading, "no commits" would be a claim nothing supports. -->
    {#if data && (data.featured || data.state === "ready" || data.lastSuccessAt != null)}
      <div class="featured" role="region" aria-label={t.github.featured}>
        {#if data.featured}
          <small class="num">{interpolate(plural(data.featured.commits, t.github.featuredCommits), { commits: count.format(data.featured.commits) })}</small>
          <b>{data.featured.name}</b>
          <!-- The disk cache never stores commit titles. -->
          {#if data.featured.title}<span>{data.featured.title}</span>{/if}
        {:else}
          <small>{t.github.featured}</small>
          <span>{t.github.empty}</span>
        {/if}
      </div>
    {/if}
  </div>
{/if}

<style>
  header { display: flex; align-items: center; gap: 10px; justify-content: space-between; min-height: 28px; margin-bottom: 6px; }
  h2 { font-size: 1.25em; font-weight: 500; text-transform: uppercase; letter-spacing: 1px; }
  small { font-size: .88em; color: var(--muted); }
  .actions { display: flex; gap: 5px; }
  .action-wrap { position: relative; display: inline-flex; }
  .icon { width: 28px; height: 28px; border: 1px solid var(--line); border-radius: 50%; font-size: 19px; line-height: 1; display: grid; place-items: center; }
  .icon:hover:not(:disabled) { background: var(--surface-2); }
  .icon:disabled { opacity: .45; cursor: default; }
  .tip { position: absolute; z-index: 4; right: 0; top: 100%; min-width: 150px; background: var(--surface-2); color: var(--text); padding: 9px; border: 1px solid var(--control-line); box-shadow: 0 4px 20px #0006; font-size: 12px; font-weight: 400; visibility: hidden; }
  .action-wrap:hover .tip, .action-wrap:focus-within .tip { visibility: visible; }
  .readings[data-stale="true"] { --text: var(--muted); --accent: var(--muted); --band-steel: var(--muted); color: var(--muted); }
  .github-top { display: flex; align-items: center; justify-content: space-between; gap: 12px; }
  strong { display: block; font-size: 2.8em; font-weight: 400; line-height: 1.15; }
  .github-top small { display: block; margin-top: 5px; }
  .heatmap { display: grid; grid-template-rows: repeat(7, 7px); grid-auto-columns: 7px; gap: 3px; }
  .heatmap i { background: var(--accent); }
  .activity { height: 26px; display: flex; align-items: flex-end; gap: 5px; margin: 6px 0 4px; padding: 0; list-style: none; }
  .activity li { flex: 1; height: 100%; display: flex; align-items: flex-end; }
  .activity i { display: block; width: 100%; background: var(--band-steel); border-radius: 3px 3px 0 0; }
  .featured { border-top: 1px solid var(--line); padding-top: 8px; margin-top: 9px; display: grid; gap: 4px; overflow-wrap: anywhere; }
  .featured span { font-size: .92em; }
  .line { display: block; margin-bottom: 4px; overflow-wrap: anywhere; }
  .line.alert { color: var(--warn); }
  .link { color: var(--accent); text-decoration: underline; font-size: inherit; }
  .icon:focus-visible, .link:focus-visible { outline: 2px solid var(--accent); outline-offset: 3px; }
</style>
