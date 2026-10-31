<script lang="ts">
  import { onMount } from "svelte";
  import type { GithubData } from "../core";
  import { t, interpolate } from "../strings";
  let { data, refresh }: { data: GithubData; refresh: () => void } = $props();
  let configure = $state(false);
  let now = $state(Date.now());
  let allowedAt = $state(0);
  onMount(() => { const timer = setInterval(() => now = Date.now(), 1000); return () => clearInterval(timer); });
  const relative = new Intl.RelativeTimeFormat("pt-BR", { numeric: "auto" });
  function age(when: number) {
    const seconds = Math.max(0, Math.floor((now - when) / 1000));
    if (seconds < 3600) return relative.format(-Math.floor(seconds / 60), "minute");
    if (seconds < 86400) return relative.format(-Math.floor(seconds / 3600), "hour");
    return relative.format(-Math.floor(seconds / 86400), "day");
  }
  function refreshNow() { allowedAt = Date.now() + 10000; refresh(); }
</script>
<section class="github">
  <button aria-label={t.github.refresh} disabled={data.paused || data.refreshing || now < allowedAt} onclick={refreshNow}>↻</button>
  {#if data.refreshing}<span role="status">{t.github.refreshing}</span>{/if}
  {#if data.state === "missingGh"}
    <p>{t.github.missingGh}</p>
    <button onclick={() => configure = true}>{t.github.configure}</button>
  {:else}
    {#if data.state === "invalidCredential" && !data.paused}
      <button onclick={() => configure = true}>{t.github.invalidCredential}</button>
    {/if}
    {#if data.state === "error" && !data.paused}<p role="status">{data.failureSince != null ? interpolate(t.github.failureAge, {age: age(data.failureSince)}) : t.github.error}</p>{/if}
    {#if data.lastSuccessAt != null && (data.state !== "ready" || data.paused)}<p class="secondary">{interpolate(t.github.updatedAge, {age: age(data.lastSuccessAt)})}</p>{/if}
    <div data-stale={data.state === "invalidCredential" && !data.paused} class:dimmed={data.state === "invalidCredential" && !data.paused}>
      {#if data.calendar}
        <h2>{t.github.calendar}</h2>
        <p>{interpolate(t.github.contributions, {total:String(data.calendar.contributionCalendar.totalContributions)})}</p>
        <div class="calendar" role="img" aria-label={t.github.calendar}>
          {#each data.calendar.contributionCalendar.weeks as week}
            <div class="week">{#each week.contributionDays as day}<span style:background={day.color} title={`${day.date}: ${day.contributionCount}`}></span>{/each}</div>
          {/each}
        </div>
        <p class="secondary">{interpolate(t.github.private, {total:String(data.calendar.restrictedContributionsCount)})}</p>
      {/if}
      <h2>{t.github.featured}</h2>
      {#if data.featured}
        <p>{data.featured.name}</p>
        <p class="num">{interpolate(t.github.count, {commits:String(data.featured.commits),total:String(data.featured.total)})}</p>
        <p class="secondary">{age(data.featured.when)}</p>
        <p class="title">{data.featured.title}</p>
      {:else}<p>{t.github.empty}</p>{/if}
    </div>
  {/if}
  {#if configure && (data.state === "missingGh" || data.state === "invalidCredential")}
    <p>{t.github.login}</p><p>{t.github.instruction}</p>
  {/if}
</section>
<style>
  .github { padding: 0 16px 16px; min-width: 0; overflow: auto; }
  h2 { font-size: 13px; margin-top: 18px; }
  p { margin: 8px 0; font-size: 12px; }
  button { padding: 5px 8px; border: 1px solid var(--line); border-radius: 5px; }
  button:disabled { opacity: .4; }
  .dimmed { opacity: .5; }
  .secondary { color: var(--muted); }
  .title { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .calendar { display: flex; gap: 2px; margin: 12px 0; overflow-x: auto; }
  .week { display: flex; flex-direction: column; gap: 2px; }
  .week span { width: 6px; height: 6px; border-radius: 1px; }
</style>
