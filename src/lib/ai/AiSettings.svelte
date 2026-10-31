<script lang="ts">
  import type { AiData } from "../core";
  import { interpolate, t } from "../strings";
  let { data, connect, disconnect, setUsage }: {
    data?: AiData; connect: () => Promise<void>; disconnect: () => Promise<void>;
    setUsage: (enabled: boolean) => Promise<void>;
  } = $props();
  let busy = $state(false);
  /** The warning about the undocumented endpoint is open. */
  let warning = $state(false);
  let error = $state("");
  const claude = $derived(data?.claude);
  async function run(action: () => Promise<void>) {
    if (busy) return;
    busy = true;
    error = "";
    try {
      await action();
    } catch (code) {
      error = t.ai.settings.errors[code as keyof typeof t.ai.settings.errors] ?? t.ai.settings.errors.unwritable;
    } finally { busy = false; }
  }
</script>
<section aria-label={t.modules.ai}>
  <h3>{t.modules.ai}</h3>
  {#if data}
    <h4>{t.ai.codex}</h4>
    <p>{data.codex.state === "missing" ? t.ai.missingCodex
      : data.codex.plan ? interpolate(t.settings.codexPlan, { plan: data.codex.plan }) : t.settings.codexDetected}</p>
  {/if}
  <h4>{t.ai.claude}</h4>
  {#if claude?.state === "missing"}<p>{t.ai.missingClaude}</p>{/if}
  {#if claude?.statusLine === "connected"}
    <p>{t.ai.settings.connected}</p>
    <button disabled={busy} onclick={() => run(disconnect)}>{t.ai.settings.disconnect}</button>
  {:else if claude?.statusLine === "replaced"}
    <p role="status">{t.ai.settings.replaced}</p>
    <button disabled={busy} onclick={() => run(connect)}>{t.ai.reconnect}</button>
    <button disabled={busy} onclick={() => run(disconnect)}>{t.ai.settings.disconnect}</button>
  {:else}
    <p>{t.ai.settings.explanation}</p>
    <button disabled={busy} onclick={() => run(connect)}>{t.ai.settings.connect}</button>
  {/if}
  <h4>{t.ai.settings.usage.title}</h4>
  {#if claude?.usage === "on" || claude?.usage === "expired"}
    <p>{claude.usage === "expired" ? t.ai.expired : t.ai.settings.usage.on}</p>
    <button disabled={busy} onclick={() => run(() => setUsage(false))}>{t.ai.settings.usage.disable}</button>
  {:else if warning}
    <div role="alertdialog" aria-labelledby="usage-warning" class="warning">
      <b id="usage-warning">{t.ai.settings.usage.warningTitle}</b>
      <p>{t.ai.settings.usage.warning}</p>
      <button disabled={busy} onclick={() => run(async () => { await setUsage(true); warning = false; })}>{t.ai.settings.usage.confirm}</button>
      <button disabled={busy} onclick={() => (warning = false)}>{t.ai.settings.usage.cancel}</button>
    </div>
  {:else}
    <p>{t.ai.settings.usage.explanation}</p>
    <button disabled={busy} onclick={() => (warning = true)}>{t.ai.settings.usage.enable}</button>
  {/if}
  {#if error}<p role="alert">{error}</p>{/if}
</section>
