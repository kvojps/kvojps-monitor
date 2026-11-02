<script lang="ts">
  import type { GithubData } from "../core";
  import FieldError from "../settings/FieldError.svelte";
  import StatusLine, { type Tone } from "../settings/StatusLine.svelte";
  import { t, interpolate } from "../strings";
  let { data, save }: { data?: GithubData; save: (token: string) => Promise<string> } = $props();

  type Credential = NonNullable<GithubData["credential"]>;

  const uid = $props.id();
  let token = $state("");
  let busy = $state(false);
  let error = $state("");
  /** The user opened the token form with "Usar token próprio" or "Trocar token". */
  let formRequested = $state(false);
  /** Credential just saved here and the snapshot it replaced; it holds until the next snapshot arrives. */
  let pendingSave = $state.raw<{ credential: Credential; snapshotAtSave: GithubData | undefined }>();

  const justSaved = $derived(pendingSave?.snapshotAtSave === data ? pendingSave?.credential : undefined);
  const credential = $derived(justSaved ?? data?.credential);
  /** What keeps the section from a working credential; the last two leave the form open, with the gh instruction. */
  const problem = $derived.by((): "checking" | "invalid" | "missing" | undefined => {
    if (justSaved) return undefined;
    if (!data) return "checking";
    if (data.state === "invalidCredential") return "invalid";
    if (!data.credential || data.state === "missingGh") return "missing";
  });
  const needsCredential = $derived(problem === "invalid" || problem === "missing");
  const formOpen = $derived(needsCredential || formRequested);

  const line = $derived.by((): { tone: Tone; text: string } => {
    if (problem === "checking") return { tone: "neutral", text: t.settings.welcome.checking };
    if (problem === "invalid") return { tone: "warn", text: t.github.invalidCredential };
    if (problem === "missing" || !credential) return { tone: "warn", text: t.github.settings.notConnected };
    const text = credential.source === "gh" ? t.github.settings.viaGh : t.github.settings.ownToken;
    return { tone: "ok", text: interpolate(text, { login: credential.login }) };
  });

  async function submit(event: SubmitEvent) {
    event.preventDefault();
    if (busy) return;
    busy = true;
    error = "";
    const candidate = token;
    token = "";
    try {
      const login = await save(candidate);
      pendingSave = { credential: { source: "token", login }, snapshotAtSave: data };
      formRequested = false;
    } catch (code) {
      error = t.github.credentialErrors[code as keyof typeof t.github.credentialErrors] ?? t.github.credentialErrors.offline;
    } finally { busy = false; }
  }
</script>

<section aria-labelledby="{uid}-title">
  <h3 id="{uid}-title">{t.modules.github}</h3>
  <StatusLine tone={line.tone} text={line.text} />
  {#if formOpen}
    {#if needsCredential}<p class="hint">{t.github.instruction}</p>{/if}
    <form onsubmit={submit}>
      <label for="{uid}-token">{t.github.tokenLabel}</label>
      <input id="{uid}-token" type="password" autocomplete="off" bind:value={token}
        disabled={busy} aria-describedby={error ? `${uid}-error` : undefined} aria-invalid={!!error} />
      <button type="submit" disabled={busy || !token.trim()}>{busy ? t.github.validating : t.github.saveToken}</button>
      {#if error}<FieldError id="{uid}-error" message={error} />{/if}
    </form>
  {:else if credential}
    <button onclick={() => (formRequested = true)}>{credential.source === "gh" ? t.github.settings.useOwnToken : t.github.settings.replaceToken}</button>
  {/if}
</section>

<style>
  .hint { color: var(--muted); }
</style>
