<script lang="ts">
  import { untrack } from "svelte";
  import type { TrelloData } from "../core";
  import { t, interpolate } from "../strings";
  let { data, connect, selectBoard, disconnect, openAdmin, authorize }: {
    data?: TrelloData;
    connect: (key: string, token: string) => Promise<string>;
    selectBoard: (board: string) => Promise<void>;
    disconnect: () => Promise<void>;
    openAdmin: () => void;
    authorize: (key: string) => void;
  } = $props();
  // The browser closes the panel while the token is generated; restore the key once.
  let key = $state(untrack(() => data?.pendingKey ?? ""));
  let token = $state("");
  let busy = $state(false);
  let message = $state("");
  let failed = $state(false);
  function explain(error: unknown) {
    return t.trello.credentialErrors[error as keyof typeof t.trello.credentialErrors] ?? t.trello.credentialErrors.offline;
  }
  async function submit(event: SubmitEvent) {
    event.preventDefault();
    if (busy) return;
    busy = true;
    message = "";
    failed = false;
    const candidate = token;
    token = "";
    try {
      const login = await connect(key, candidate);
      message = interpolate(t.trello.connected, { login });
    } catch (error) {
      failed = true;
      message = explain(error);
    } finally { busy = false; }
  }
  async function run(action: () => Promise<void>, failure?: string) {
    try { await action(); message = ""; failed = false; } catch (error) { failed = true; message = failure ?? explain(error); }
  }
</script>
<section aria-label={t.modules.trello}>
  <h2>{t.modules.trello}</h2>
  {#if data?.account && !message}<p>{interpolate(t.trello.connected, { login: data.account.login })}</p>{/if}
  <p>{t.trello.keyStep}</p>
  <button type="button" onclick={openAdmin}>{t.trello.openAdmin}</button>
  <form onsubmit={submit}>
    <label for="trello-key">{t.trello.keyLabel}</label>
    <input id="trello-key" autocomplete="off" spellcheck="false" bind:value={key} disabled={busy} />
    <p>{t.trello.tokenStep}</p>
    <button type="button" disabled={!key.trim()} onclick={() => authorize(key.trim())}>{t.trello.generateToken}</button>
    <label for="trello-token">{t.trello.tokenLabel}</label>
    <input id="trello-token" type="password" autocomplete="off" bind:value={token} disabled={busy}
      aria-describedby={message ? "trello-message" : undefined} aria-invalid={failed} />
    <button type="submit" disabled={busy || !key.trim() || !token.trim()}>{busy ? t.trello.validating : t.trello.save}</button>
    {#if message}<p id="trello-message" role={failed ? "alert" : "status"}>{message}</p>{/if}
  </form>
  {#if data?.account}
    <p>{t.trello.boardStep}</p>
    {#if data.boards.length}
      <label for="trello-board">{t.trello.boardLabel}</label>
      <select id="trello-board" value={data.board?.id ?? ""} onchange={(event) => run(() => selectBoard(event.currentTarget.value))}>
        {#if !data.board}<option value="" disabled></option>{/if}
        {#each data.boards as board (board.id)}<option value={board.id}>{board.name}</option>{/each}
      </select>
    {:else}<p class="secondary">{t.trello.noBoards}</p>{/if}
    <button type="button" onclick={() => run(disconnect, t.trello.disconnectFailed)}>{t.trello.disconnect}</button>
  {/if}
</section>
<style>
  section { padding: 16px; font-size: 12px; }
  h2 { font-size: 14px; margin-bottom: 12px; }
  p { margin: 8px 0; }
  .secondary { color: var(--muted); }
  label { display: block; margin: 12px 0 6px; }
  input, select { width: 100%; box-sizing: border-box; color: var(--text); background: var(--surface); padding: 8px; border: 1px solid var(--line); border-radius: 5px; }
  button { display: block; margin-top: 8px; padding: 6px 8px; border: 1px solid var(--line); border-radius: 5px; }
  button:disabled { opacity: .5; }
</style>
