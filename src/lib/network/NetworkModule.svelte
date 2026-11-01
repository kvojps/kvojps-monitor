<script lang="ts">
  import { onMount } from "svelte";
  import type { NetworkData, NetworkTraffic } from "../core";
  import { interpolate, t } from "../strings";

  let { data, runSpeed = () => {} }: { data: NetworkData | null; runSpeed?: () => void } = $props();
  let now = $state(Date.now());
  onMount(() => {
    const timer = setInterval(() => (now = Date.now()), 1000);
    return () => clearInterval(timer);
  });

  /** The core only weighs a speed test younger than six hours; older ones are shown as old. */
  const STALE_SPEED_MS = 6 * 3600 * 1000;
  const number = new Intl.NumberFormat("pt-BR", { maximumFractionDigits: 1 });
  const date = new Intl.DateTimeFormat("pt-BR", { day: "2-digit", month: "2-digit", hour: "2-digit", minute: "2-digit" });

  function age(when: number) {
    const seconds = Math.max(0, Math.floor((now - when) / 1000));
    if (seconds < 60) return interpolate(t.network.ageSeconds, { value: String(seconds) });
    if (seconds < 3600) return interpolate(t.network.ageMinutes, { value: String(Math.floor(seconds / 60)) });
    if (seconds < 86400) return interpolate(t.network.ageHours, { value: String(Math.floor(seconds / 3600)) });
    return interpolate(t.network.ageDays, { value: String(Math.floor(seconds / 86400)) });
  }
  /** Without a measurement the module says so explicitly, never 0. */
  const measured = (v: number | null, unit: string) => (v === null ? t.network.noValue : `${number.format(v)} ${unit}`);
  /** Traffic in KB/s or MB/s, kept apart from the Mbps of the speed test. */
  const rate = (v: number) => (v >= 1048576 ? `${number.format(v / 1048576)} MB/s` : `${number.format(v / 1024)} KB/s`);

  const connected = $derived(data?.connected ?? false);

  type Tone = "ok" | "warn" | "bad" | "none";
  type Quality = NonNullable<NetworkData["quality"]>;
  type Security = NonNullable<NetworkData["security"]>;
  const QUALITY: Record<Quality, [Tone, string]> = { good: ["ok", "✓"], regular: ["warn", "◐"], poor: ["bad", "✕"] };
  const SECURITY: Record<Security, [Tone, string]> = { noAlerts: ["ok", "✓"], warning: ["warn", "⚠"], risk: ["bad", "✕"] };
  function axis<S extends Quality | Security>(value: S | null | undefined, states: Record<S, [Tone, string]>) {
    if (!connected) return { tone: "none" as Tone, icon: "⊘", label: t.network.disconnected };
    if (!value) return { tone: "none" as Tone, icon: "…", label: t.network.unmeasured };
    const [tone, icon] = states[value];
    return { tone, icon, label: t.network[value] };
  }
  const quality = $derived(axis(data?.quality, QUALITY));
  const security = $derived(axis(data?.security, SECURITY));

  const speed = $derived(data?.speed ?? null);
  const speedStale = $derived(speed !== null && now - speed.measuredAt >= STALE_SPEED_MS);
  /** Old or not repeatable now: the age is shown directly, not only on focus or pointer. */
  const speedOld = $derived(speed !== null && (speedStale || !data?.speedAvailable || data?.speedRunning || !connected));
  const speedAge = $derived(speed === null ? null : interpolate(speedStale ? t.network.speedStale : t.network.speedAge,
    { age: age(speed.measuredAt), date: date.format(speed.measuredAt) }));

  /** Chart: fixed one-second slots anchored on the right, scale from the samples themselves. */
  const SLOTS = 60;
  const W = 360, TOP = 8, BOTTOM = 76;
  const history = $derived(data?.history.slice(-SLOTS) ?? []);
  const scale = $derived(Math.max(1, ...history.flatMap((p) => [p.download, p.upload])));
  const x = (i: number) => W - 6 - (history.length - 1 - i) * ((W - 12) / (SLOTS - 1));
  const y = (v: number) => BOTTOM - (v / scale) * (BOTTOM - TOP);
  const points = (direction: keyof NetworkTraffic) => history.map((p, i) => `${x(i)},${y(p[direction])}`).join(" ");
  const peak = (direction: keyof NetworkTraffic) => Math.max(...history.map((p) => p[direction]));
  const chartSummary = $derived.by(() => {
    if (history.length === 0) return t.network.noSamples;
    const last = history.at(-1)!;
    if (history.length === 1) return interpolate(t.network.chartSingle, { down: rate(last.download), up: rate(last.upload) });
    return interpolate(t.network.chartSummary, {
      count: String(history.length),
      down: rate(last.download), downPeak: rate(peak("download")),
      up: rate(last.upload), upPeak: rate(peak("upload")),
    });
  });

  /** Up to five apps plus Others, on one local scale shared by every bar. */
  const apps = $derived.by(() => {
    const traffic = data?.processTraffic;
    if (!connected || !traffic) return null;
    const rows = [...traffic.apps.map((a) => ({ ...a, others: false })), { name: t.network.others, ...traffic.others, others: true }];
    return { rows, empty: traffic.apps.length === 0, max: Math.max(1, ...rows.flatMap((r) => [r.download, r.upload])) };
  });
  const width = (v: number, max: number) => `${(v / max) * 100}%`;

  const appsSummary = $derived.by(() => {
    if (!apps) return connected ? t.network.unmeasured : t.network.disconnected;
    const count = apps.rows.filter((row) => !row.others).length;
    if (count === 0) return t.network.noProcesses;
    return count === 1 ? t.network.appsCountOne : interpolate(t.network.appsCount, { count: String(count) });
  });

  /** Modal detail listing per-app traffic ("Aplicativos de rede") over the summary; it reads the same snapshot, never a new collection. */
  let dialog: HTMLDialogElement;
  let detailOpen = $state(false);
  let restoringFocus = false;
  let opener: HTMLElement | null = null;
  function openApps(event: MouseEvent) {
    if (dialog.open) return;
    opener = event.currentTarget as HTMLElement;
    dialog.showModal();
    detailOpen = true;
  }
  function closeApps() {
    restoringFocus = true;
    dialog.close();
    detailOpen = false;
    opener?.focus({ preventScroll: true });
    restoringFocus = false;
  }
  function closeOnEscape(event: KeyboardEvent) {
    if (event.key === "Escape" && dialog?.open) {
      event.preventDefault();
      event.stopPropagation();
      closeApps();
    }
  }
</script>

<svelte:window onkeydowncapture={closeOnEscape} />

<div class="network" onfocusin={(event) => { if (restoringFocus) event.stopPropagation(); }}>
  {#if !data}
    <p class="blank">{t.network.noReading}</p>
  {:else}
    <div class="axes">
      {#each [{ id: "quality", name: t.network.quality, state: quality }, { id: "security", name: t.network.security, state: security }] as item (item.id)}
        <div class="axis {item.state.tone}" role="group" aria-labelledby="network-{item.id}">
          <span id="network-{item.id}" class="axis-name">{item.name}</span>
          <b><span aria-hidden="true">{item.state.icon}&nbsp;</span>{item.state.label}</b>
        </div>
      {/each}
    </div>
    {#if connected && (data.security === "warning" || data.security === "risk")}
      <ul class="reasons {data.security}">
        {#each data.reasons as reason (reason)}<li><span aria-hidden="true">⚠&nbsp;</span>{reason}</li>{/each}
      </ul>
    {/if}

    <div class="instruments">
      <div class="latency" role="group" aria-labelledby="network-latency">
        {#if !connected}
          <strong class="absent"><span aria-hidden="true">⊘&nbsp;</span>{t.network.disconnected}</strong>
        {:else}
          <strong class="num">{#if data.internet.latency === null}<span class="absent">{t.network.noValue}</span>{:else}{number.format(data.internet.latency)}<span class="unit">&nbsp;ms</span>{/if}</strong>
        {/if}
        <span id="network-latency" class="caption">{t.network.internetLatency}</span>
        {#if connected}
          <p class="connection">
            <span>{data.wired ? t.network.wired : t.network.wireless}</span>
            <span class:active={data.vpn}>{data.vpn ? t.network.vpn : t.network.noVpn}</span>
          </p>
        {/if}
      </div>
      <div class="speed-instrument" role="group" aria-label={t.network.speed}>
        <button class="speed" disabled={!data.speedAvailable || data.speedRunning} onclick={runSpeed}
          aria-label={data.speedRunning ? t.network.testingSpeed : t.network.testSpeed}
          aria-describedby={speedAge ? "network-speed-age network-speed-values" : "network-speed-values"}>
          {#if speed}
            <span class="speed-values num"><b>↓ {number.format(speed.download)}</b><b>↑ {number.format(speed.upload)}</b></span>
          {:else}
            <span class="speed-none">{t.network.noSpeed}</span>
          {/if}
          <span class="caption">{t.network.capacity} · <span aria-hidden="true">{data.speedRunning ? "◌" : "↻"}&nbsp;</span>{data.speedRunning ? t.network.testingSpeed : t.network.testSpeed}</span>
        </button>
        {#if speed}
          <span id="network-speed-values" class="sr-only">{t.network.download}: {number.format(speed.download)} Mbps. {t.network.upload}: {number.format(speed.upload)} Mbps.</span>
        {:else}
          <span id="network-speed-values" class="sr-only">{t.network.noSpeed}</span>
        {/if}
        {#if speedAge}<span id="network-speed-age" class="when num" class:shown={speedOld} class:old={speedStale}>{speedAge}</span>{/if}
        {#if !data.speedAvailable && !data.speedRunning}
          <p class="note"><span aria-hidden="true">⊘&nbsp;</span>{t.network.speedUnavailable}</p>
        {/if}
        {#if data.speedFailed}<p class="note warn" role="status"><span aria-hidden="true">⚠&nbsp;</span>{t.network.speedFailed}</p>{/if}
      </div>
    </div>

    {#if connected}
      <dl class="metrics">
        <div><dt>{t.network.jitter}</dt><dd class="num">{measured(data.internet.jitter, "ms")}</dd></div>
        <div><dt>{t.network.loss}</dt><dd class="num">{measured(data.internet.loss, "%")}</dd></div>
        <div><dt>{t.network.gatewayLatency}</dt><dd class="num">{measured(data.gateway.latency, "ms")}</dd></div>
      </dl>
      <p class="gateway num">{interpolate(t.network.gatewayBand, { jitter: measured(data.gateway.jitter, "ms"), loss: measured(data.gateway.loss, "%") })}</p>
    {/if}

    <section class="traffic" aria-labelledby="network-traffic">
      <h3 id="network-traffic" class="sr-only">{t.network.traffic}</h3>
      {#if history.length === 0}
        <p class="note">{t.network.noSamples}</p>
      {:else}
        <svg viewBox="0 0 {W} 84" role="img" aria-label="{t.network.chart}. {chartSummary}" preserveAspectRatio="none">
          <path class="grid" d="M6 {TOP}H{W - 6} M6 {(TOP + BOTTOM) / 2}H{W - 6} M6 {BOTTOM}H{W - 6}" />
          {#if history.length === 1}
            <circle class="download" cx={x(0)} cy={y(history[0].download)} r="3" />
            <rect class="upload" x={x(0) - 3} y={y(history[0].upload) - 3} width="6" height="6" />
          {:else}
            <polyline class="download" points={points("download")} />
            <polyline class="upload" points={points("upload")} />
          {/if}
        </svg>
      {/if}
      <dl class="rates num">
        <div class="download"><dt><svg class="key" aria-hidden="true" viewBox="0 0 18 6"><line class="download" x1="0" y1="3" x2="18" y2="3" /></svg><span aria-hidden="true">↓&nbsp;</span>{t.network.download}</dt><dd>{data.traffic ? rate(data.traffic.download) : t.network.noValue}</dd></div>
        <div class="upload"><dt><svg class="key" aria-hidden="true" viewBox="0 0 18 6"><line class="upload" x1="0" y1="3" x2="18" y2="3" /></svg><span aria-hidden="true">↑&nbsp;</span>{t.network.upload}</dt><dd>{data.traffic ? rate(data.traffic.upload) : t.network.noValue}</dd></div>
      </dl>
    </section>

    <button class="apps-open" onclick={openApps} aria-label={t.network.appsDetail} aria-describedby="network-apps-summary">
      <span class="caption">{t.network.appsDetail}</span>
      <span id="network-apps-summary">{appsSummary}</span><span aria-hidden="true">↗</span>
    </button>
  {/if}
  <dialog bind:this={dialog} aria-labelledby="network-apps" oncancel={(event) => { event.preventDefault(); closeApps(); }}>
    <header><h2 id="network-apps">{t.network.appsDetail}</h2><button onclick={closeApps}>{t.network.close}</button></header>
    {#if detailOpen}
      <section class="apps" aria-label={t.network.processTraffic}>
        {#if apps}
          {#if apps.empty}<p class="note">{t.network.noProcesses}</p>{/if}
          <ul aria-describedby="network-apps-scale">
            {#each apps.rows as app (app.others ? "others" : `app:${app.name}`)}
              <li class="app" class:others={app.others}>
                <span class="name" title={app.others ? t.network.othersHint : app.name}>{app.name}</span>
                <span class="bars" aria-hidden="true"><i style:width={width(app.download, apps.max)}></i><i class="up" style:width={width(app.upload, apps.max)}></i></span>
                <span class="app-rates num"><span>↓ {rate(app.download)}</span> <span>↑ {rate(app.upload)}</span></span>
              </li>
            {/each}
          </ul>
          <p class="note">{t.network.others}: {t.network.othersHint}</p>
          <p id="network-apps-scale" class="note">{t.network.appsScale}</p>
        {:else}
          <p class="note">{connected ? t.network.unmeasured : t.network.disconnected}</p>
        {/if}
      </section>
    {/if}
  </dialog>
</div>

<style>
  .network { padding: 8px 12px 10px; min-width: 0; display: flex; flex-direction: column; gap: 6px; flex: 1; font-size: 12px; }
  .sr-only { position: absolute; width: 1px; height: 1px; padding: 0; overflow: hidden; clip-path: inset(50%); white-space: nowrap; }
  .blank { display: grid; place-items: center; min-height: 120px; padding: 12px; border: 1px dashed var(--line); color: var(--muted); text-align: center; }
  .axes { display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: 8px; }
  .axis { display: flex; flex-direction: column; gap: 1px; padding: 4px 8px; background: var(--surface-2); border-left: 4px solid var(--line); }
  .axis-name { font: 10px var(--mono); letter-spacing: 0.4px; text-transform: uppercase; color: var(--muted); }
  .axis b { font-size: 13px; font-weight: 500; }
  .axis.ok { border-left-color: var(--ok); } .axis.ok b { color: var(--ok); }
  .axis.warn { border-left-color: var(--warn); } .axis.warn b { color: var(--warn); }
  .axis.bad { border-left-color: var(--bad); } .axis.bad b { color: var(--bad); }
  .connection { display: flex; gap: 2px 8px; flex-wrap: wrap; margin-top: 2px; color: var(--muted); font-size: 11px; }
  .connection .active { color: var(--accent); }
  .reasons { margin: 0; padding: 0; list-style: none; display: flex; flex-direction: column; gap: 4px; }
  .reasons.warning { color: var(--warn); } .reasons.risk { color: var(--bad); }
  .instruments { display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); align-items: start; gap: 10px; }
  .latency { display: flex; flex-direction: column; gap: 2px; }
  .latency strong { font-size: 32px; font-weight: 300; line-height: 1.05; letter-spacing: -1px; }
  .latency .unit { font-size: 13px; letter-spacing: 0; color: var(--muted); }
  .latency .absent { font-size: 18px; font-weight: 400; color: var(--muted); letter-spacing: 0; }
  .caption { display: block; font: 10px var(--mono); letter-spacing: 0.4px; text-transform: uppercase; color: var(--muted); }
  .speed-instrument { position: relative; display: flex; flex-direction: column; gap: 4px; }
  .speed { width: 100%; padding: 6px 8px; color: var(--accent); background: var(--surface-2); border: 1px solid var(--line); border-radius: 0 20px 0 20px; display: flex; flex-direction: column; align-items: center; gap: 3px; }
  .speed:hover:not(:disabled) { filter: brightness(1.15); }
  .speed:disabled { cursor: default; }
  .speed:disabled .caption { opacity: 0.7; }
  .speed:focus-visible { outline: 2px solid var(--accent); outline-offset: 3px; }
  .speed-values { display: flex; justify-content: center; gap: 12px; }
  .speed-values b { font-size: 18px; font-weight: 400; }
  .speed-none { font-size: 13px; color: var(--muted); }
  /* Test age and date: on pointer and keyboard focus, always when old or not repeatable now. */
  .when { display: none; font-size: 11px; color: var(--muted); }
  .when.shown, .speed:hover ~ .when, .speed:focus-visible ~ .when { display: block; }
  .when.old { color: var(--warn); }
  .note { font-size: 11px; color: var(--muted); }
  .note.warn { color: var(--warn); }
  .metrics { display: flex; flex-wrap: wrap; justify-content: space-between; gap: 2px 12px; margin: 0; }
  .metrics div { display: flex; align-items: baseline; gap: 5px; }
  .metrics dt { font-size: 11px; color: var(--muted); }
  .metrics dd { margin: 0; font-size: 12px; }
  .gateway { font-size: 11px; color: var(--muted); padding: 2px 8px; background: var(--surface); border-left: 3px solid var(--line); }
  h3 { font: 10px var(--mono); letter-spacing: 1.5px; text-transform: uppercase; color: var(--muted); margin-bottom: 4px; }
  .traffic > svg { display: block; width: 100%; height: 34px; }
  .grid { stroke: var(--line); stroke-dasharray: 2 4; fill: none; }
  polyline, .key line { fill: none; stroke-width: 2; vector-effect: non-scaling-stroke; }
  polyline.download, .key .download { stroke: var(--accent); }
  polyline.upload, .key .upload { stroke: var(--chart-secondary); stroke-dasharray: 5 3; }
  .key { width: 12px; height: 6px; margin-right: 4px; vertical-align: middle; }
  circle.download { fill: var(--accent); } rect.upload { fill: var(--chart-secondary); }
  .rates { display: flex; flex-wrap: wrap; justify-content: space-between; gap: 2px 12px; margin: 4px 0 0; font-size: 12px; }
  .rates div { display: flex; gap: 4px; }
  .rates dt { color: var(--muted); }
  .rates dd { margin: 0; }
  .rates .download dd { color: var(--accent); }
  .apps-open { display: grid; grid-template-columns: auto 1fr auto; text-align: left; align-items: baseline; gap: 8px; width: 100%; padding: 4px 10px; color: var(--text); background: var(--surface-2); border: 1px solid var(--line); cursor: pointer; font: inherit; }
  .apps-open:hover { filter: brightness(1.15); }
  .apps-open:focus-visible, dialog button:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  dialog { margin: auto; width: min(680px, 90vw); max-height: 80vh; overflow: auto; padding: 20px; color: var(--text); background: var(--surface); border: 1px solid var(--line); }
  dialog::backdrop { background: #000a; }
  dialog header { display: flex; justify-content: space-between; align-items: center; gap: 16px; margin-bottom: 12px; }
  dialog h2 { font-size: 14px; font-weight: 500; }
  dialog header button { cursor: pointer; }
  .apps { display: flex; flex-direction: column; gap: 6px; }
  .apps ul { margin: 0; padding: 0; list-style: none; }
  .app { display: grid; grid-template-columns: minmax(56px, 90px) minmax(0, 1fr) auto; align-items: center; gap: 10px; margin: 6px 0; font-size: 12px; }
  .app .name { overflow-wrap: anywhere; }
  .app.others .name { color: var(--muted); }
  .bars { display: grid; gap: 3px; }
  .bars i { display: block; height: 4px; min-width: 1px; background: var(--accent); border-radius: 0 4px 4px 0; }
  .bars i.up { background: var(--chart-secondary); }
  .app-rates { display: flex; flex-direction: column; font-size: 11px; color: var(--muted); text-align: right; }
</style>
