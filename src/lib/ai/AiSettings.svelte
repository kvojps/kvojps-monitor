<script lang="ts">
  import type { AiData } from "../core";
  import { t } from "../strings";
  let { data, connect, disconnect }:
    { data?: AiData; connect: () => Promise<void>; disconnect: () => Promise<void> } = $props();
  let busy = $state(false);
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
  <h2>{t.modules.ai}</h2>
  <h3>{t.ai.claude}</h3>
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
  {#if error}<p role="alert">{error}</p>{/if}
</section>
<style>
  section { padding: 16px; font-size: 12px; }
  h2 { font-size: 14px; margin-bottom: 12px; }
  h3 { font-size: 13px; margin-bottom: 8px; }
  p { margin: 8px 0; }
  button { margin-top: 8px; padding: 6px 8px; border: 1px solid var(--line); border-radius: 5px; }
  button:disabled { opacity: .5; }
</style>
