<script lang="ts">
  import type { SystemData } from "../core";
  import { interpolate, t } from "../strings";
  import { bytes, duration, pair } from "./format";

  let { data }: { data: SystemData } = $props();

  /** The chart holds about 60 seconds and grows from the left. */
  const POINTS = 30;
  const HEIGHT = 34;

  const line = $derived(
    data.cpu.history
      .map((v, i) => `${((i / (POINTS - 1)) * 100).toFixed(1)},${(HEIGHT - (v / 100) * (HEIGHT - 2)).toFixed(1)}`)
      .join(" "),
  );

  const pct = (used: number, total: number) => (total > 0 ? Math.round((used / total) * 100) : 0);

  const battery = $derived(data.battery);
  const detailBattery = $derived.by(() => {
    if (!battery) return "";
    const state = t.system.statesBattery[battery.state];
    const s = battery.secondsRemaining;
    if (s === null) return state;
    const template = battery.state === "charging" ? t.system.toCharge : t.system.remaining;
    return `${state} · ${interpolate(template, { time: duration(s) })}`;
  });
</script>

<div class="system">
  <section aria-label={t.system.cpu}>
    <div class="line">
      <span class="label">{t.system.cpu}</span>
      <span class="num">{data.cpu.usage === null ? t.system.noValue : `${Math.round(data.cpu.usage)}%`}</span>
    </div>
    <svg class="chart" role="img" aria-label={t.system.chartCpu} viewBox="0 0 100 {HEIGHT}" preserveAspectRatio="none">
      {#if data.cpu.history.length > 1}
        <polyline class="area" points="0,{HEIGHT} {line} {(((data.cpu.history.length - 1) / (POINTS - 1)) * 100).toFixed(1)},{HEIGHT}" />
        <polyline class="stroke" points={line} />
      {/if}
    </svg>
    {#if data.cpu.temperature}
      <div class="line secondary">
        <span>{t.system.temperature}</span>
        <span class="num" class:warning={data.cpu.temperature.warning}>{Math.round(data.cpu.temperature.celsius)} °C</span>
      </div>
    {/if}
  </section>

  <section aria-label={t.system.ram}>
    <div class="line">
      <span class="label">{t.system.ram}</span>
      <span class="num">{pair(data.ram.used, data.ram.total)}</span>
    </div>
    <div class="bar" role="meter" aria-label={t.system.ram} aria-valuemin={0} aria-valuemax={100} aria-valuenow={pct(data.ram.used, data.ram.total)}>
      <i style:width="{pct(data.ram.used, data.ram.total)}%"></i>
    </div>
    {#if data.swap}
      <div class="line secondary">
        <span>{t.system.swap}</span>
        <span class="num">{pair(data.swap.used, data.swap.total)}</span>
      </div>
    {/if}
  </section>

  <section aria-label={t.system.disks}>
    <span class="label">{t.system.disks}</span>
    {#each data.disks as disk (disk.mount)}
      <div class="disk">
        <div class="line">
          <span class="mount num">{disk.mount}</span>
          <span class="num">{pair(disk.used, disk.total)} · {interpolate(t.system.freeSpace, { value: bytes(disk.free) })}</span>
        </div>
        <div
          class="bar"
          class:warning={disk.warning}
          role="meter"
          aria-label={disk.mount}
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={pct(disk.used, disk.used + disk.free)}
        >
          <i style:width="{pct(disk.used, disk.used + disk.free)}%"></i>
        </div>
      </div>
    {/each}
  </section>

  {#if battery}
    <section aria-label={t.system.battery}>
      <div class="line">
        <span class="label">{t.system.battery}</span>
        <span class="num" class:warning={battery.warning}>{Math.round(battery.percentage)}%</span>
      </div>
      <div
        class="bar"
        class:warning={battery.warning}
        role="meter"
        aria-label={t.system.battery}
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={Math.round(battery.percentage)}
      >
        <i style:width="{battery.percentage}%"></i>
      </div>
      <div class="line secondary"><span>{detailBattery}</span></div>
    </section>
  {/if}
</div>

<style>
  .system {
    display: flex;
    flex-direction: column;
    padding: 4px 16px 16px;
    min-height: 0;
    overflow-y: auto;
  }

  section {
    padding: 12px 0;
    border-bottom: 1px solid var(--line);
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .line {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 8px;
  }

  .label {
    color: var(--muted);
  }

  .secondary {
    font-size: 12px;
    color: var(--muted);
  }

  .disk {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .disk + .disk {
    margin-top: 6px;
  }

  .mount {
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .num {
    font-size: 12px;
  }

  .warning {
    color: var(--warn);
  }

  .bar {
    height: 4px;
    border-radius: 2px;
    background: var(--surface-2);
    overflow: hidden;
  }

  .bar > i {
    display: block;
    height: 100%;
    background: var(--accent);
    border-radius: 2px;
  }

  .bar.warning > i {
    background: var(--warn);
  }

  .chart {
    width: 100%;
    height: 34px;
    display: block;
  }

  .area {
    fill: var(--accent-dim);
    stroke: none;
  }

  .stroke {
    fill: none;
    stroke: var(--accent);
    stroke-width: 1;
    vector-effect: non-scaling-stroke;
  }
</style>
