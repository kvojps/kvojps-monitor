<script lang="ts">
  import { onMount } from "svelte";
  import type { Core, GeneralSettings } from "../core";
  import { interpolate, t } from "../strings";

  let { core }: { core: Core } = $props();
  let general = $state<GeneralSettings | null>(null);
  let pending = $state(false);
  let autostartError = $state(false);
  let logsError = $state(false);

  onMount(() => {
    core.generalSettings?.().then((g) => (general = g)).catch(() => {});
  });

  /** Keep the switch on the confirmed value until the core answers. */
  async function change(event: Event, action: ((enabled: boolean) => Promise<GeneralSettings>) | undefined, failed: (v: boolean) => void) {
    const input = event.currentTarget as HTMLInputElement;
    const enabled = input.checked;
    input.checked = !enabled;
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
  {#if general}
    <label class="switch">
      <input type="checkbox" checked={general.autostart} disabled={pending}
        aria-describedby={autostartError ? "autostart-error" : undefined}
        onchange={(e) => change(e, core.setAutostart, (v) => (autostartError = v))} />
      {t.settings.general.autostart}
    </label>
    {#if autostartError}<p id="autostart-error" class="error" role="alert">{t.settings.general.autostartFailed}</p>{/if}
    <label class="switch">
      <input type="checkbox" checked={general.versionCheck} disabled={pending}
        onchange={(e) => change(e, core.setVersionCheck, () => {})} />
      {t.settings.general.versionCheck}
    </label>
    {#if general.release}
      <button class="link" onclick={() => core.openRelease?.()}>{interpolate(t.settings.general.release, { version: general.release.version })}</button>
    {/if}
    <p class="version num">{interpolate(t.settings.general.version, { version: general.version })}</p>
  {/if}
  <button onclick={openLogs} aria-describedby={logsError ? "logs-error" : undefined}>{t.settings.general.openLogs}</button>
  {#if logsError}<p id="logs-error" class="error" role="alert">{t.settings.general.openLogsFailed}</p>{/if}
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
