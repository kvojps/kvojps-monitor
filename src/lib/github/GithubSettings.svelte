<script lang="ts">
  import type { GithubData } from "../core";
  import { t, interpolate } from "../strings";
  let { data, save }: { data?: GithubData; save: (token: string) => Promise<string> } = $props();
  let token = $state("");
  let busy = $state(false);
  let message = $state("");
  let failed = $state(false);
  async function submit(event: SubmitEvent) {
    event.preventDefault();
    if (busy) return;
    busy = true;
    message = "";
    failed = false;
    const candidate = token;
    token = "";
    try {
      const login = await save(candidate);
      message = interpolate(t.github.connected, { login });
    } catch (error) {
      failed = true;
      message = t.github.credentialErrors[error as keyof typeof t.github.credentialErrors] ?? t.github.credentialErrors.offline;
    } finally { busy = false; }
  }
</script>
<section aria-label={t.modules.github}>
  <h2>{t.modules.github}</h2>
  {#if data?.credential}
    <p>{data.credential.source === "gh" ? interpolate(t.github.viaGh, { login: data.credential.login }) : t.github.ownToken}</p>
    {#if data.credential.source === "token"}<p>{interpolate(t.github.connected, { login: data.credential.login })}</p>{/if}
  {/if}
  <p>{t.github.login}</p>
  <p>{t.github.instruction}</p>
  <form onsubmit={submit}>
    <label for="github-token">{t.github.tokenLabel}</label>
    <input id="github-token" type="password" autocomplete="off" bind:value={token}
      disabled={busy} aria-describedby={message ? "github-token-message" : undefined} aria-invalid={failed} />
    <button type="submit" disabled={busy || !token.trim()}>{busy ? t.github.validating : t.github.saveToken}</button>
    {#if message}<p id="github-token-message" role={failed ? "alert" : "status"}>{message}</p>{/if}
  </form>
</section>
<style>
  section { padding: 16px; font-size: 12px; }
  h2 { font-size: 14px; margin-bottom: 12px; }
  p { margin: 8px 0; }
  label { display: block; margin: 12px 0 6px; }
  input { width: 100%; box-sizing: border-box; color: var(--text); background: var(--surface); padding: 8px; border: 1px solid var(--line); border-radius: 5px; }
  button { margin-top: 8px; padding: 6px 8px; border: 1px solid var(--line); border-radius: 5px; }
  button:disabled { opacity: .5; }
</style>
