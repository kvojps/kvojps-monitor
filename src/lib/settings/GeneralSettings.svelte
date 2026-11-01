<script lang="ts">
  import { onMount } from "svelte";
  import type { Core, GeneralSettings } from "../core";
  import { interpolate, t } from "../strings";
  import FieldError from "./FieldError.svelte";
  import Switch from "./Switch.svelte";

  let { core }: { core: Core } = $props();
  let general = $state<GeneralSettings | null>(null);
  let pending = $state(false);
  let autostartError = $state(false);
  let loadError = $state(false);
  let versionError = $state(false);
  let logsError = $state(false);

  async function load() {
    loadError = false;
    try {
      if (!core.generalSettings) throw new Error("unavailable");
      general = await core.generalSettings();
    } catch {
      loadError = true;
    }
  }

  onMount(() => {
    let disposed = false;
    let stop = () => {};
    let revision = 0;
    const ready = core.listenGeneralSettings?.((settings) => {
      revision += 1;
      general = settings;
      loadError = false;
    }) ?? Promise.resolve(() => {});
    void ready.then(async (unlisten) => {
      if (disposed) { unlisten(); return; }
      stop = unlisten;
      const before = revision;
      try {
        const settings = await core.generalSettings?.();
        if (!disposed && revision === before && settings) general = settings;
      } catch {
        if (!disposed && revision === before) loadError = true;
      }
    }).catch(() => { if (!disposed) void load(); });
    return () => { disposed = true; stop(); };
  });

  /** The switch keeps the confirmed value until the core answers. */
  async function change(enabled: boolean, action: ((enabled: boolean) => Promise<GeneralSettings>) | undefined, failed: (v: boolean) => void) {
    pending = true;
    failed(false);
    try {
      if (!action) throw new Error("unavailable");
      general = await action(enabled);
    } catch {
      failed(true);
    } finally {
      pending = false;
    }
  }

  async function openLogs() {
    logsError = false;
    try {
      await (core.openLogs ? core.openLogs() : Promise.reject());
    } catch {
      logsError = true;
    }
  }
</script>

<section aria-labelledby="settings-general-title">
  <h3 id="settings-general-title">{t.settings.general.title}</h3>
  {#if loadError}
    <FieldError message={t.settings.general.loadFailed} />
    <button onclick={load}>{t.settings.general.retry}</button>
  {/if}
  {#if general}
    <Switch label={t.settings.general.autostart} checked={general.autostart} disabled={pending}
      error={autostartError ? t.settings.general.autostartFailed : undefined}
      onchange={(enabled) => change(enabled, core.setAutostart, (v) => (autostartError = v))} />
    <Switch label={t.settings.general.versionCheck} checked={general.versionCheck} disabled={pending}
      error={versionError ? t.settings.general.versionCheckFailed : undefined}
      onchange={(enabled) => change(enabled, core.setVersionCheck, (v) => (versionError = v))} />
    {#if general.release}
      <button class="link" onclick={() => core.openRelease?.()}>{interpolate(t.settings.general.release, { version: general.release.version })}</button>
    {/if}
    <p class="version num">{interpolate(t.settings.general.version, { version: general.version })}</p>
  {/if}
  <button onclick={openLogs} aria-describedby={logsError ? "logs-error" : undefined}>{t.settings.general.openLogs}</button>
  {#if logsError}<FieldError id="logs-error" message={t.settings.general.openLogsFailed} />{/if}
</section>

<style>
  .version { color: var(--muted); font-size: 11px; }
  .link {
    display: block;
    padding: 0;
    border: 0;
    min-height: 0;
    background: none;
    color: var(--accent);
    text-decoration: underline;
  }
</style>
