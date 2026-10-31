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
  <h3>{t.modules.trello}</h3>
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
      <!-- Radios, not <select>: its GTK popup takes focus and the panel closes under it. -->
      <fieldset role="radiogroup" aria-label={t.trello.boardLabel}>
        {#each data.boards as board (board.id)}
          <label class="board">
            <input type="radio" name="trello-board" value={board.id} checked={board.id === data.board?.id}
              onchange={() => run(() => selectBoard(board.id))} />
            {board.name}
          </label>
        {/each}
      </fieldset>
    {:else}<p class="secondary">{t.trello.noBoards}</p>{/if}
    <button type="button" onclick={() => run(disconnect, t.trello.disconnectFailed)}>{t.trello.disconnect}</button>
  {/if}
</section>
<style>
  .secondary { color: var(--muted); }
  fieldset { border: 0; display: flex; flex-direction: column; gap: 4px; max-height: 180px; overflow-y: auto; }
</style>
