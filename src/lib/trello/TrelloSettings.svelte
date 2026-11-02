<script lang="ts">
  import type { TrelloData } from "../core";
  import FieldError from "../settings/FieldError.svelte";
  import StatusLine, { type Tone } from "../settings/StatusLine.svelte";
  import { t, interpolate } from "../strings";
  let { data, connect, selectBoard, disconnect, openAdmin, authorize }: {
    data?: TrelloData;
    connect: (key: string, token: string) => Promise<string>;
    selectBoard: (board: string) => Promise<void>;
    disconnect: () => Promise<void>;
    openAdmin: () => void;
    authorize: (key: string) => void;
  } = $props();

  type Board = TrelloData["boards"][number];
  type Stage = "checking" | "credential" | "invalid" | "chooseBoard" | "connected";
  /** What a command just changed here; it holds until the next snapshot arrives. */
  type Change =
    | { kind: "connected"; login: string }
    | { kind: "boardChosen"; board: Board }
    | { kind: "disconnected" };

  const uid = $props.id();
  let key = $state("");
  let keyRestored = false;
  let token = $state("");
  let busy = $state(false);
  let credentialError = $state("");
  let boardError = $state("");
  let disconnectError = $state("");
  /** The board list was opened by "Trocar quadro" or by a successful validation. */
  let boardsRequested = $state(false);
  let pending = $state.raw<{ change: Change; snapshotAtChange: TrelloData | undefined }>();

  // The browser closes the panel while the token is generated; restore the key from the first snapshot.
  $effect(() => {
    if (keyRestored || !data) return;
    keyRestored = true;
    if (!key) key = data.pendingKey ?? "";
  });

  const change = $derived(pending?.snapshotAtChange === data ? pending?.change : undefined);
  /** Account, board and stage as the section shows them: a pending change wins over the snapshot it replaced. */
  const view = $derived.by((): { stage: Stage; account: { login: string } | null; board: Board | null } => {
    switch (change?.kind) {
      case "connected": return { stage: "chooseBoard", account: { login: change.login }, board: null };
      case "boardChosen": return { stage: "connected", account: data?.account ?? null, board: change.board };
      case "disconnected": return { stage: "credential", account: null, board: null };
    }
    const account = data?.account ?? null;
    const board = data?.board ?? null;
    if (!data) return { stage: "checking", account, board };
    if (data.state === "missingCredential") return { stage: "credential", account, board };
    if (data.state === "invalidCredential") return { stage: "invalid", account, board };
    if (data.state === "chooseBoard" || !board) return { stage: "chooseBoard", account, board };
    return { stage: "connected", account, board };
  });
  const stage = $derived(view.stage);
  const credentialOpen = $derived(stage === "credential" || stage === "invalid");
  const boardsOpen = $derived(stage === "chooseBoard" || (stage === "connected" && boardsRequested));
  /** Until the snapshot after a validation arrives, the listed boards belong to the previous credential. */
  const awaitingBoards = $derived(change?.kind === "connected");
  const boards = $derived(awaitingBoards ? [] : (data?.boards ?? []));

  const line = $derived.by((): { tone: Tone; text: string } => {
    const { account, board } = view;
    if (stage === "checking") return { tone: "neutral", text: t.settings.welcome.checking };
    if (stage === "invalid") return { tone: "warn", text: t.trello.invalidCredential };
    if (stage === "credential") return { tone: "warn", text: t.trello.settings.notConnected };
    if (stage === "chooseBoard") {
      const text = account ? interpolate(t.trello.settings.chooseBoard, { login: account.login }) : t.trello.boardStep;
      return { tone: "warn", text };
    }
    const name = board?.name ?? "";
    const text = account
      ? interpolate(t.trello.settings.connected, { login: account.login, board: name })
      : interpolate(t.trello.settings.boardOnly, { board: name });
    return { tone: !change && data?.state === "error" ? "warn" : "ok", text };
  });

  function explain(error: unknown) {
    return t.trello.credentialErrors[error as keyof typeof t.trello.credentialErrors] ?? t.trello.credentialErrors.offline;
  }
  async function submit(event: SubmitEvent) {
    event.preventDefault();
    if (busy) return;
    busy = true;
    credentialError = "";
    const candidate = token;
    token = "";
    try {
      const login = await connect(key, candidate);
      pending = { change: { kind: "connected", login }, snapshotAtChange: data };
      boardsRequested = true;
      boardError = disconnectError = "";
    } catch (error) {
      credentialError = explain(error);
    } finally { busy = false; }
  }
  function toggleBoards() {
    boardsRequested = !boardsRequested;
    boardError = "";
  }
  async function choose(chosen: Board, radio: HTMLInputElement) {
    boardError = "";
    try {
      await selectBoard(chosen.id);
      pending = { change: { kind: "boardChosen", board: chosen }, snapshotAtChange: data };
      boardsRequested = false;
    } catch (error) {
      boardError = explain(error);
      // The board did not change: put the check back on the current one.
      radio.checked = false;
      const current = view.board?.id;
      for (const option of radio.closest("fieldset")?.querySelectorAll("input") ?? []) option.checked = option.value === current;
    }
  }
  async function disconnectAccount() {
    disconnectError = "";
    try {
      await disconnect();
      pending = { change: { kind: "disconnected" }, snapshotAtChange: data };
      boardsRequested = false;
      boardError = "";
    } catch { disconnectError = t.trello.disconnectFailed; }
  }
</script>

<section aria-labelledby="{uid}-title">
  <h3 id="{uid}-title">{t.modules.trello}</h3>
  <StatusLine tone={line.tone} text={line.text} />
  {#if stage !== "checking" && stage !== "credential"}
    <div class="actions">
      {#if stage === "connected" && boards.length}
        <button type="button" aria-expanded={boardsRequested} aria-controls="{uid}-boards" onclick={toggleBoards}>
          {t.trello.settings.changeBoard}
        </button>
      {/if}
      <button type="button" aria-describedby={disconnectError ? `${uid}-disconnect-error` : undefined}
        onclick={disconnectAccount}>{t.trello.disconnect}</button>
    </div>
    {#if disconnectError}<FieldError id="{uid}-disconnect-error" message={disconnectError} />{/if}
  {/if}
  {#if credentialOpen}
    <form onsubmit={submit}>
      <p>{t.trello.keyStep}</p>
      <button type="button" onclick={openAdmin}>{t.trello.openAdmin}</button>
      <label for="{uid}-key">{t.trello.keyLabel}</label>
      <input id="{uid}-key" autocomplete="off" spellcheck="false" bind:value={key} disabled={busy} />
      <p>{t.trello.tokenStep}</p>
      <button type="button" disabled={!key.trim()} onclick={() => authorize(key.trim())}>{t.trello.generateToken}</button>
      <label for="{uid}-token">{t.trello.tokenLabel}</label>
      <input id="{uid}-token" type="password" autocomplete="off" bind:value={token} disabled={busy}
        aria-describedby={credentialError ? `${uid}-credential-error` : undefined} aria-invalid={!!credentialError} />
      <button type="submit" disabled={busy || !key.trim() || !token.trim()}>{busy ? t.trello.validating : t.trello.save}</button>
      {#if credentialError}<FieldError id="{uid}-credential-error" message={credentialError} />{/if}
    </form>
  {/if}
  {#if boardsOpen}
    <div id="{uid}-boards">
      <p>{t.trello.boardStep}</p>
      {#if boards.length}
        <!-- Radios, not <select>: its GTK popup takes focus and the panel closes under it. -->
        <fieldset role="radiogroup" aria-label={t.trello.boardLabel}
          aria-describedby={boardError ? `${uid}-board-error` : undefined}>
          {#each boards as option (option.id)}
            <label class="board">
              <input type="radio" name="{uid}-board" value={option.id} checked={option.id === view.board?.id}
                onchange={(event) => choose(option, event.currentTarget)} />
              {option.name}
            </label>
          {/each}
        </fieldset>
      {:else}<p class="secondary">{awaitingBoards ? t.settings.welcome.checking : t.trello.noBoards}</p>{/if}
      {#if boardError}<FieldError id="{uid}-board-error" message={boardError} />{/if}
    </div>
  {/if}
</section>

<style>
  .secondary { color: var(--muted); }
  .actions { display: flex; gap: 8px; }
  fieldset { border: 0; display: flex; flex-direction: column; gap: 4px; max-height: 180px; overflow-y: auto; }
</style>
