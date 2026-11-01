<script lang="ts">
  import { onMount } from "svelte";
  import type { Memory, SystemData } from "../core";
  import { interpolate, t } from "../strings";
  import { pair } from "./format";

  let { data, updatedAt }: { data: SystemData | null; updatedAt: number | null } = $props();

  /** CPU is sampled every 2 s while visible; a few missed samples make the reading old. */
  const STALE_AFTER = 10_000;

  let now = $state(Date.now());
  onMount(() => {
    const timer = setInterval(() => (now = Date.now()), 1000);
    return () => clearInterval(timer);
  });

  const relative = new Intl.RelativeTimeFormat("pt-BR", { numeric: "auto" });
  function age(when: number) {
    const seconds = Math.max(0, Math.floor((now - when) / 1000));
    if (seconds < 60) return relative.format(-seconds, "second");
    if (seconds < 3600) return relative.format(-Math.floor(seconds / 60), "minute");
    return relative.format(-Math.floor(seconds / 3600), "hour");
  }
  /** When the reading is old, the moment it was taken; null while fresh. */
  const staleSince = $derived(updatedAt !== null && now - updatedAt > STALE_AFTER ? updatedAt : null);

  /** `note` explains a related absence right below the bar. */
  type Resource = { name: string; used: number; total: number; percent: number; warning: boolean; note?: string };

  const percent = (used: number, total: number) => (total > 0 ? Math.round((used / total) * 100) : 0);
  const memory = (name: string, m: Memory): Resource =>
    ({ name, used: m.used, total: m.total, percent: percent(m.used, m.total), warning: false });

  /** The root disk represents the machine; the others only appear when they warn. */
  const disk = $derived(data?.disks.find((d) => d.mount === "/") ?? data?.disks[0] ?? null);
  const resources = $derived.by(() => {
    if (!data) return [];
    const list: Resource[] = [{ ...memory(t.system.ram, data.ram), note: data.swap ? undefined : t.system.noSwap }];
    if (data.swap) list.push(memory(t.system.swap, data.swap));
    if (disk) {
      list.push({ name: interpolate(t.system.disk, { mount: disk.mount }), used: disk.used, total: disk.total,
        percent: percent(disk.used, disk.total), warning: disk.warning });
    }
    return list;
  });
  const fullDisks = $derived(data?.disks.filter((d) => d.warning) ?? []);
  const valueText = (r: Resource) => `${pair(r.used, r.total)} · ${r.percent}%`;
</script>

{#if !data}
  <div class="system"><p class="blank">{t.system.noReading}</p></div>
{:else}
  {@const cpu = data.cpu}
  {@const battery = data.battery}
  <div class="system">
    {#if staleSince !== null}
      <p class="notice" role="status"><span aria-hidden="true">◷ </span>{interpolate(t.system.stale, { age: age(staleSince) })}</p>
    {/if}

    <div class="readings">
      <div class="top">
        <div class="cpu" role="group" aria-labelledby="system-cpu">
          {#if cpu.usage === null}
            <strong class="absent">{t.system.measuring}</strong>
          {:else}
            <strong class="num">{Math.round(cpu.usage)}<span class="unit">%</span></strong>
          {/if}
          <span class="caption" id="system-cpu">{t.system.cpu}</span>
        </div>
        <div class="temperature" role="group" aria-labelledby="system-temperature">
          {#if cpu.temperature}
            <strong class="num" class:warning={cpu.temperature.warning}>{Math.round(cpu.temperature.celsius)} °C</strong>
            {#if cpu.temperature.warning}
              <span class="alert-text"><span aria-hidden="true">⚠ </span>{t.system.hot}</span>
            {/if}
          {:else}
            <strong class="absent">{t.system.noSensor}</strong>
          {/if}
          <span class="caption" id="system-temperature">{t.system.temperature}</span>
        </div>
      </div>

      {#each resources as r (r.name)}
        <div class="resource">
          <div class="line"><span>{r.name}</span><b class="num">{pair(r.used, r.total)}</b></div>
          <div class="bar" class:warning={r.warning} role="meter" aria-label={r.name} aria-valuemin={0} aria-valuemax={100}
            aria-valuenow={r.percent} aria-valuetext={valueText(r)}>
            <i style:width="{r.percent}%"></i>
          </div>
        </div>
        {#if r.note}
          <p class="note">{r.note}</p>
        {/if}
      {/each}

      {#if fullDisks.length > 0}
        <ul class="alerts" role="alert">
          {#each fullDisks as d (d.mount)}
            <li><span aria-hidden="true">⚠ </span>{interpolate(t.system.diskFull, { mount: d.mount })}</li>
          {/each}
        </ul>
      {/if}

      <div class="battery" role="group" aria-labelledby="system-battery">
        <div class="line">
          <span id="system-battery">{t.system.battery}</span>
          {#if battery}
            <b class="num" class:warning={battery.warning}>{Math.round(battery.percentage)}%</b>
            {#if battery.warning}
              <span class="alert-text"><span aria-hidden="true">⚠ </span>{t.system.lowBattery}</span>
            {/if}
            <span class="state">{t.system.statesBattery[battery.state]}</span>
          {:else}
            <span class="absent-line">{t.system.noBattery}</span>
          {/if}
        </div>
        {#if battery}
          <div class="bar" class:warning={battery.warning} role="meter" aria-label={t.system.battery}
            aria-valuemin={0} aria-valuemax={100} aria-valuenow={Math.round(battery.percentage)}
            aria-valuetext="{Math.round(battery.percentage)}% · {t.system.statesBattery[battery.state]}">
            <i style:width="{battery.percentage}%"></i>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .system { display: flex; flex-direction: column; gap: 8px; flex: 1; min-width: 0; padding: 8px 12px 12px; font-size: 12px; }
  .blank { display: grid; place-items: center; min-height: 120px; padding: 12px; border: 1px dashed var(--line); color: var(--muted); text-align: center; }
  .notice { font-size: 11px; color: var(--warn); }
  .readings { display: flex; flex-direction: column; gap: 8px; flex: 1; }
  .top { display: flex; justify-content: space-between; align-items: flex-end; flex-wrap: wrap; gap: 10px; }
  .cpu, .temperature { display: flex; flex-direction: column; gap: 4px; }
  .temperature { align-items: flex-end; text-align: right; }
  .cpu strong { font-size: 34px; font-weight: 300; line-height: 1.05; letter-spacing: -1px; }
  .unit { margin-left: 4px; font-size: 15px; letter-spacing: 0; color: var(--muted); }
  .temperature strong { font-size: 25px; font-weight: 400; color: var(--accent); }
  .cpu strong.absent, .temperature strong.absent { font-size: 18px; font-weight: 400; letter-spacing: 0; color: var(--muted); }
  .caption { font: 10px var(--mono); letter-spacing: 1px; text-transform: uppercase; color: var(--muted); }
  .line { display: flex; flex-wrap: wrap; justify-content: space-between; align-items: baseline; gap: 8px; margin-bottom: 4px; color: var(--muted); }
  .line b { font-weight: 400; color: var(--text); }
  .bar { height: 5px; background: var(--surface-2); overflow: hidden; }
  .bar > i { display: block; height: 100%; background: var(--accent); }
  .note, .absent-line { font-size: 11px; color: var(--muted); }
  .note { margin-top: -6px; }
  .alerts { margin: 0; padding: 0; list-style: none; display: flex; flex-direction: column; gap: 4px; font-size: 11px; color: var(--warn); }
  .alert-text { font-size: 11px; color: var(--warn); }
  .temperature strong.warning, .line b.warning { color: var(--warn); }
  .bar.warning > i { background: var(--warn); }
  .battery { margin-top: 4px; padding-top: 8px; border-top: 1px solid var(--line); }
  .battery .line { justify-content: flex-start; }
  .battery .state, .battery .absent-line { margin-left: auto; }
</style>
