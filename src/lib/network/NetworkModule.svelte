<script lang="ts">
  import { onMount } from "svelte";
  import type { NetworkData } from "../core";
  import { interpolate, t } from "../strings";
  let { data, runSpeed = () => {} }: { data: NetworkData; runSpeed?: () => void } = $props();
  let now = $state(Date.now());
  onMount(() => {
    const timer = setInterval(() => { now = Date.now(); }, 1000);
    return () => clearInterval(timer);
  });
  const age = $derived.by(() => {
    const seconds = Math.max(0, Math.floor((now - (data.speed?.measuredAt ?? now)) / 1000));
    return interpolate(seconds >= 3600 ? t.network.ageHours : seconds >= 60 ? t.network.ageMinutes : t.network.ageSeconds,
      { value: String(seconds >= 3600 ? Math.floor(seconds / 3600) : seconds >= 60 ? Math.floor(seconds / 60) : seconds) });
  });
  const number = new Intl.NumberFormat("pt-BR", { maximumFractionDigits: 1 });
  const measurement = (v: number | null, unit: string) => v === null ? t.network.noValue : `${number.format(v)} ${unit}`;
  function rate(v: number) {
    return v >= 1048576 ? `${number.format(v / 1048576)} MB/s` : v >= 1024 ? `${number.format(v / 1024)} KB/s` : `${number.format(v)} B/s`;
  }
  const maximum = $derived(Math.max(1, ...data.history.flatMap((p) => [p.download, p.upload])));
  function points(direction: "download" | "upload") {
    return data.history.map((p, i) => `${i * 300 / 59},${60 - p[direction] / maximum * 56}`).join(" ");
  }
</script>

<div class="network">
  <section aria-label={t.network.security}>
    <h2>{t.network.security}</h2>
    <strong class:regular={data.security === "warning"} class:poor={data.security === "risk"}>
      {!data.connected ? t.network.disconnected : data.security ? t.network[data.security] : t.network.unmeasured}
    </strong>
    {#if data.connected}
      {#each data.reasons as reason}<p>{reason}</p>{/each}
    {/if}
  </section>
  <section aria-label={t.network.quality}>
    <h2>{t.network.quality}</h2>
    <strong class:good={data.quality === "good"} class:regular={data.quality === "regular"} class:poor={data.quality === "poor"}>
      {!data.connected ? t.network.disconnected : data.quality ? t.network[data.quality] : t.network.unmeasured}
    </strong>
    <div class="tags">
      {#if data.connected && data.wired}<span>{t.network.wired}</span>{/if}
      {#if data.connected && data.vpn}<span>{t.network.vpn}</span>{/if}
    </div>
  </section>
  {#if data.connected}
    {#each [{ name: t.network.gateway, measurements: data.gateway }, { name: t.network.internet, measurements: data.internet }] as destination}
      <section aria-label={destination.name}>
        <h2>{destination.name}</h2>
        <dl>
          <div><dt>{t.network.latency}</dt><dd class="num">{measurement(destination.measurements.latency, "ms")}</dd></div>
          <div><dt>{t.network.jitter}</dt><dd class="num">{measurement(destination.measurements.jitter, "ms")}</dd></div>
          <div><dt>{t.network.loss}</dt><dd class="num">{measurement(destination.measurements.loss, "%")}</dd></div>
        </dl>
      </section>
    {/each}
  {/if}
  <section aria-label={t.network.speed}>
    <h2>{t.network.speed}</h2>
    {#if data.speed}
      <dl>
        <div><dt>{t.network.download}</dt><dd class="num">↓ {measurement(data.speed.download, "Mbps")}</dd></div>
        <div><dt>{t.network.upload}</dt><dd class="num">↑ {measurement(data.speed.upload, "Mbps")}</dd></div>
      </dl>
      <p>{age}</p>
    {:else}<p>{t.network.unmeasured}</p>{/if}
    <button disabled={!data.speedAvailable || data.speedRunning} onclick={runSpeed}>{data.speedRunning ? t.network.testingSpeed : t.network.testSpeed}</button>
    {#if !data.speedAvailable}<p>{t.network.speedUnavailable}</p>{/if}
    {#if data.speedFailed}<p role="status">{t.network.speedFailed}</p>{/if}
  </section>
  <section aria-label={t.network.traffic}>
    <h2>{t.network.traffic}</h2>
    <svg role="img" aria-label={t.network.chart} viewBox="0 0 300 64">
      <polyline points={points("download")} class="download" />
      <polyline points={points("upload")} class="upload" />
    </svg>
    <dl>
      <div><dt>{t.network.download}</dt><dd class="num">↓ {data.traffic ? rate(data.traffic.download) : t.network.noValue}</dd></div>
      <div><dt>{t.network.upload}</dt><dd class="num">↑ {data.traffic ? rate(data.traffic.upload) : t.network.noValue}</dd></div>
    </dl>
  </section>
  <section aria-label={t.network.processTraffic}>
    <h2>{t.network.processTraffic}</h2>
    {#if data.connected && data.processTraffic}
      {#each data.processTraffic.apps as app}
        <div class="process-row"><span>{app.name}</span><span class="num">↓ {rate(app.download)}</span><span class="num">↑ {rate(app.upload)}</span></div>
      {:else}<p>{t.network.noProcesses}</p>{/each}
      <div class="process-row"><span title={t.network.othersHint}>{t.network.others}</span><span class="num">↓ {rate(data.processTraffic.others.download)}</span><span class="num">↑ {rate(data.processTraffic.others.upload)}</span></div>
      <p>{t.network.othersHint}</p>
    {:else}<p>{t.network.unmeasured}</p>{/if}
  </section>
</div>
<style>
  .process-row { display: grid; grid-template-columns: minmax(0, 1fr) auto auto; gap: 12px; padding: 4px 0; font-size: 12px; }
  .process-row > span:first-child { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .network { padding: 4px 16px 16px; overflow-y: auto; }
  section { padding: 12px 0; border-bottom: 1px solid var(--line); }
  h2 { font-size: 12px; color: var(--muted); font-weight: 500; margin-bottom: 8px; }
  strong { font-size: 22px; }
  p { font-size: 12px; margin-top: 6px; color: var(--muted); }
  .good { color: var(--ok); } .regular { color: var(--warn); } .poor { color: var(--bad); }
  .tags { display: flex; gap: 6px; margin-top: 6px; color: var(--muted); font-size: 11px; }
  .tags span { background: var(--surface-2); padding: 3px 6px; border-radius: 4px; }
  dl { display: flex; justify-content: space-between; gap: 12px; }
  dt { font-size: 11px; color: var(--muted); margin-bottom: 4px; }
  dd { font-size: 12px; }
  svg { width: 100%; height: 64px; background: var(--surface); margin-bottom: 8px; }
  polyline { fill: none; stroke-width: 1.5; }
  .download { stroke: var(--accent); } .upload { stroke: var(--muted); }
</style>
