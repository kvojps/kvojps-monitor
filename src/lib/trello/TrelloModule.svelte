<script lang="ts">
  import { onMount, tick, untrack } from "svelte";
  import type { TrelloData } from "../core";
  import { t, interpolate } from "../strings";
  let { data, refresh, configure, openCard, moveCard }: {
    data: TrelloData | null; refresh: () => void; configure: () => void; openCard: (url: string) => void;
    /** Rejects when the core could not move the card. */
    moveCard: (card: string, list: string) => Promise<void>;
  } = $props();
  let now = $state(Date.now());
  let allowedAt = $state(0);
  onMount(() => { const timer = setInterval(() => now = Date.now(), 1000); return () => clearInterval(timer); });
  const relative = new Intl.RelativeTimeFormat("pt-BR", { numeric: "auto" });
  function age(when: number) {
    const seconds = Math.max(0, Math.floor((now - when) / 1000));
    if (seconds < 3600) return relative.format(-Math.floor(seconds / 60), "minute");
    if (seconds < 86400) return relative.format(-Math.floor(seconds / 3600), "hour");
    return relative.format(-Math.floor(seconds / 86400), "day");
  }
  const date = new Intl.DateTimeFormat("pt-BR", { day: "2-digit", month: "short" });
  // Trello label palette; _dark/_light variants share their base hue.
  const colors: Record<string, string> = {
    green: "#4bce97", yellow: "#f5cd47", orange: "#fea362", red: "#f87168", purple: "#9f8fef",
    blue: "#579dff", sky: "#6cc3e0", lime: "#94c748", pink: "#e774bb", black: "#8590a2",
  };
  const color = (name: string) => colors[name.replace(/_(dark|light)$/, "")] ?? "var(--muted)";
  /** Labels are named in words too, so they never rely on color alone. */
  function colorName(name: string) {
    const [base, shade] = name.split("_") as [string, "dark" | "light" | undefined];
    const word = (t.trello.labelColors as Record<string, string>)[base] ?? base;
    return shade && t.trello.labelShade[shade] ? interpolate(t.trello.labelShade[shade], { color: word }) : word;
  }
  const paused = $derived(data?.paused ?? false);
  const stale = $derived(data?.state === "invalidCredential" && !paused);
  function refreshNow() { allowedAt = Date.now() + 10000; refresh(); }

  let selected = $state<string | null>(null);
  let destination = $state("");
  /** Last move outcome, shown in the board and, after a failure, in the summary. */
  let outcome = $state<{ kind: "moving" | "moved" | "failed"; text: string } | null>(null);
  /** Destination by card until the core answers; a settled move lasts until the next board. */
  let pending = $state<Record<string, { list: string; move: number; settled?: boolean }>>({});
  let moves = 0;
  $effect(() => {
    void data;
    untrack(() => {
      for (const card in pending) if (pending[card].settled) delete pending[card];
      // A refresh keeps the selection unless the card left the board.
      if (selected && !listOf(selected)) selected = null;
      // A destination that left the board, or now holds the card, is no longer an option.
      if (destination && (!selected || destination === listOf(selected) || !lists.some((l) => l.id === destination))) destination = "";
    });
  });
  // Optimistic view: a pending card shows at the bottom of its destination list; the summary reads it too.
  const lists = $derived.by(() => {
    const cards = (data?.lists ?? []).flatMap((list) => list.cards);
    return (data?.lists ?? []).map((list) => ({
      ...list,
      cards: [
        ...list.cards.filter((card) => (pending[card.id]?.list ?? list.id) === list.id),
        ...cards.filter((card) => pending[card.id]?.list === list.id && !list.cards.includes(card)),
      ],
    }));
  });
  const total = $derived(lists.reduce((n, list) => n + list.cards.length, 0));
  const preview = $derived.by(() => {
    const list = lists.find((l) => l.cards.length > 0);
    return list ? { list, card: list.cards[0] } : null;
  });
  const title = (card: { name: string }) => card.name || t.trello.hiddenTitle;
  const listOf = (card: string) => lists.find((list) => list.cards.some((c) => c.id === card))?.id;
  const nameOf = (list: string) => lists.find((l) => l.id === list)?.name ?? "";
  const movable = $derived(data != null && !paused && data.state !== "invalidCredential");
  const selectedList = $derived(selected ? listOf(selected) : undefined);
  const selectedCard = $derived(lists.flatMap((l) => l.cards).find((c) => c.id === selected));
  const plural = (count: number, one: string, many: string) => count === 1 ? one : interpolate(many, { count: String(count) });
  const cardCount = (count: number) => plural(count, t.trello.cardsOne, t.trello.cardsMany);
  const boardCounts = $derived(interpolate(t.trello.boardCounts, { cards: cardCount(total), lists: plural(lists.length, t.trello.listsOne, t.trello.listsMany) }));

  let dialog: HTMLDialogElement;
  let columns = $state<HTMLElement>();
  const columnEls: Record<string, HTMLElement> = {};
  let detailOpen = $state(false);
  let restoringFocus = false;
  let opener: HTMLElement | null = null;
  let scrollLeft = 0;

  function cardButton(card: string) {
    return dialog?.querySelector<HTMLElement>(`[data-card="${CSS.escape(card)}"]`) ?? null;
  }
  async function openBoard(event: MouseEvent, card?: string) {
    if (dialog.open) return;
    opener = event.currentTarget as HTMLElement;
    if (card) { selected = card; destination = ""; }
    outcome = null;
    dialog.showModal();
    detailOpen = true;
    await tick();
    if (columns) columns.scrollLeft = scrollLeft;
    // Reopening keeps the context: the still valid selection gets the focus back.
    if (selected) {
      columnEls[listOf(selected) ?? ""]?.scrollIntoView({ block: "nearest", inline: "nearest" });
      cardButton(selected)?.focus();
    }
  }
  function closeBoard() {
    if (columns) scrollLeft = columns.scrollLeft;
    // Whatever happened in the board was seen there; a later failure still reaches the summary.
    if (outcome?.kind !== "moving") outcome = null;
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
      closeBoard();
    }
  }

  function select(card: string) {
    selected = selected === card ? null : card;
    destination = "";
    if (outcome?.kind !== "moving") outcome = null;
  }
  function openFromCard(card: { id: string; url: string }) {
    // The two clicks of a double click toggled the selection off; the opened card stays selected.
    selected = card.id;
    openCard(card.url);
  }
  async function cancelSelection() {
    const card = selected;
    selected = null;
    destination = "";
    await tick();
    if (card) cardButton(card)?.focus();
  }
  /** Moves re-create the card element in another column: focus goes with the card. */
  async function focusMovedCard(card: string) {
    await tick();
    if (detailOpen) cardButton(card)?.focus();
  }
  /** After a revert, refocus the card only if its element held the focus, never stealing it. */
  async function refocusIfLost(card: string) {
    await tick();
    const lost = !dialog.contains(document.activeElement) || document.activeElement === dialog;
    if (detailOpen && lost) cardButton(card)?.focus();
  }
  function moveTo(list: string, card = selected) {
    if (!card || !movable) return;
    const origin = listOf(card);
    if (!origin || origin === list) return;
    const name = title(lists.flatMap((l) => l.cards).find((c) => c.id === card)!);
    const target = nameOf(list), originName = nameOf(origin);
    selected = card;
    destination = "";
    const move = ++moves;
    pending[card] = { list, move };
    outcome = { kind: "moving", text: interpolate(t.trello.moving, { card: name, list: target }) };
    void focusMovedCard(card);
    columnEls[list]?.scrollIntoView({ block: "nearest", inline: "nearest" });
    // A later move of the same card owns the overlay.
    const current = () => pending[card]?.move === move;
    moveCard(card, list).then(
      () => {
        if (!current()) return;
        pending[card].settled = true;
        outcome = { kind: "moved", text: interpolate(t.trello.moved, { card: name, list: target }) };
      },
      () => {
        if (!current()) return;
        delete pending[card];
        outcome = { kind: "failed", text: interpolate(t.trello.moveFailedDetail, { card: name, list: originName }) };
        columnEls[origin]?.scrollIntoView({ block: "nearest", inline: "nearest" });
        void refocusIfLost(card);
      },
    );
  }

  let dragged = $state<string | null>(null);
  function dragStart(event: DragEvent, card: string) {
    if (!movable) { event.preventDefault(); return; }
    dragged = card;
    selected = card;
    destination = "";
    // WebKitGTK only starts a drag that carries data: only the opaque card id, never its content.
    event.dataTransfer?.setData("text/plain", card);
    if (event.dataTransfer) event.dataTransfer.effectAllowed = "move";
  }
  function dragOver(event: DragEvent, list: string) {
    if (!dragged || listOf(dragged) === list) return;
    event.preventDefault();
    if (event.dataTransfer) event.dataTransfer.dropEffect = "move";
  }
  function drop(event: DragEvent, list: string) {
    if (!dragged) return;
    event.preventDefault();
    const card = dragged;
    dragged = null;
    moveTo(list, card);
  }
</script>

<svelte:window onkeydowncapture={closeOnEscape} />

{#snippet labels(card: { labels: string[] })}
  {#if card.labels.length}
    {@const named = interpolate(t.trello.labels, { colors: card.labels.map(colorName).join(", ") })}
    <span class="labels" role="img" aria-label={named} title={named}>
      {#each card.labels as label}<i data-testid="label" style:background={color(label)}></i>{/each}
    </span>
  {/if}
{/snippet}
{#snippet due(card: { due: number | null; overdue: boolean; dueComplete: boolean })}
  {#if card.due != null}
    <span class="due num" class:overdue={card.overdue} class:done={card.dueComplete}>
      <span aria-hidden="true">{card.overdue ? "⚠ " : card.dueComplete ? "✓ " : ""}</span>{interpolate(card.overdue ? t.trello.overdue : card.dueComplete ? t.trello.dueDone : t.trello.due, { date: date.format(card.due) })}
    </span>
  {/if}
{/snippet}

<div class="trello" onfocusin={(event) => { if (restoringFocus) event.stopPropagation(); }}>
  {#if data?.state === "missingCredential"}
    <div class="blank"><p>{t.trello.missingCredential}</p>{#if data.pendingKey}<p>{t.trello.pendingToken}</p>{/if}</div>
  {:else if data?.state === "chooseBoard"}
    <p class="blank">{t.trello.chooseBoard}</p>
  {:else if !data}
    <p class="blank">{t.trello.noReading}</p>
  {:else}
    {#if stale}
      <p class="notice alert"><span aria-hidden="true">⚠ </span>{t.trello.invalidCredential}{" · "}<button class="link" onclick={configure}>{t.trello.reconnect}</button></p>
    {/if}
    {#if data.state === "error" && !paused}
      <p class="notice"><span aria-hidden="true">◷ </span>{data.failureSince != null ? interpolate(t.trello.failureAge, { age: age(data.failureSince) }) : t.trello.error}</p>
    {/if}
    {#if data.lastSuccessAt != null && (data.state !== "ready" || paused)}
      <p class="notice num">{interpolate(t.trello.updatedAge, { age: age(data.lastSuccessAt) })}</p>
    {/if}
    {#if outcome?.kind === "failed" && !detailOpen}<p role="alert" class="notice alert">{outcome.text}</p>{/if}
    <div class="summary" data-stale={stale}>
      {#if data.board}
        <div class="board-name"><h3>{data.board.name}</h3>
          <span class="num">{boardCounts}</span></div>
      {/if}
      {#if lists.length === 0}<p class="blank">{t.trello.noLists}</p>{:else}
        <ul class="counts" aria-label={t.trello.countsByList}>
          {#each lists as list (list.id)}<li><span class="list-name">{list.name}</span><b class="num">{list.cards.length}</b></li>{/each}
        </ul>
        {#if preview}
          <button class="preview" onclick={(event) => openBoard(event, preview.card.id)}>
            <small>{interpolate(t.trello.firstCard, { list: preview.list.name })}</small>
            <span class="title" class:secondary={!preview.card.name}>{title(preview.card)}</span>
            {#if preview.card.labels.length || preview.card.due != null}<span class="meta">{@render labels(preview.card)}{@render due(preview.card)}</span>{/if}
          </button>
        {:else}<p class="secondary">{t.trello.noCards}</p>{/if}
        <button class="open-board" onclick={(event) => openBoard(event)}>{t.trello.openBoard}<span aria-hidden="true"> ↗</span></button>
      {/if}
    </div>
  {/if}
  <div class="actions">
    <button class="action" disabled={paused || data?.refreshing || now < allowedAt} onclick={refreshNow}>
      <span aria-hidden="true">↻ </span>{t.trello.refresh}
    </button>
    <button class="action" onclick={configure}>{t.trello.configureTrello}</button>
    {#if data?.refreshing}<span role="status" class="caption">{t.trello.refreshing}</span>{/if}
  </div>

  <dialog bind:this={dialog} aria-labelledby="trello-board" oncancel={(event) => { event.preventDefault(); closeBoard(); }}>
    <header class="dialog-head">
      <div><h2 id="trello-board">{t.trello.boardDetail}</h2>
        {#if data?.board}<p class="secondary">{data.board.name} · {boardCounts}</p>{/if}
      </div>
      <button class="close" onclick={closeBoard}>{t.trello.close}</button>
    </header>
    {#if detailOpen}
    <div class="move-toolbar">
      {#if !movable}
        <p class="secondary">{t.trello.moveUnavailable}</p>
      {/if}
      {#if selectedCard && selectedList}
        <div class="selection"><b>{title(selectedCard)}</b><small>{interpolate(t.trello.selectedIn, { list: nameOf(selectedList) })}</small></div>
        {#if movable}
          <label>{t.trello.moveSelect}
            <select bind:value={destination}>
              <option value="">{t.trello.chooseList}</option>
              {#each lists.filter((list) => list.id !== selectedList) as list (list.id)}<option value={list.id}>{list.name}</option>{/each}
            </select>
          </label>
          <button class="primary" disabled={!destination} onclick={() => moveTo(destination)}>{t.trello.moveCard}</button>
        {/if}
        <button class="link" onclick={() => openCard(selectedCard.url)}>{interpolate(t.trello.open, { card: title(selectedCard) })}</button>
        <button class="plain" onclick={cancelSelection}>{t.trello.cancelSelection}</button>
      {:else if movable}
        <p class="secondary">{t.trello.selectHint}</p>
      {/if}
    </div>
    {#if outcome?.kind === "failed"}
      <p role="alert" class="move-status failed">{outcome.text}</p>
    {:else}
      <p role="status" class="move-status">{outcome?.text ?? t.trello.noChange}</p>
    {/if}
    {#if lists.length === 0}<p class="blank">{t.trello.noLists}</p>{:else}
      <div class="columns" role="group" aria-label={t.trello.boardLists} bind:this={columns} data-stale={stale}>
        {#each lists as list (list.id)}
          {@const target = movable && selectedList != null && selectedList !== list.id}
          <!-- Dropping is a pointer shortcut; "Mover para esta lista" and the toolbar are the keyboard path. -->
          <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
          <section class="column" aria-label={list.name} class:target bind:this={columnEls[list.id]}
            ondragover={(event) => dragOver(event, list.id)} ondrop={(event) => drop(event, list.id)}>
            <header><h4>{list.name}</h4><span class="num" aria-hidden="true">{list.cards.length}</span><span class="sr-only">{cardCount(list.cards.length)}</span></header>
            {#if target}<button class="move-here" onclick={() => moveTo(list.id)}><span aria-hidden="true">→ </span>{t.trello.moveHere}</button>{/if}
            {#if list.cards.length === 0}<p class="empty">{t.trello.emptyList}</p>{/if}
            <ul>
              {#each list.cards as card (card.id)}
                <li>
                  <button class="card" class:selected={selected === card.id} aria-pressed={selected === card.id} data-card={card.id}
                    draggable={movable} ondragstart={(event) => dragStart(event, card.id)} ondragend={() => dragged = null}
                    onclick={() => select(card.id)} ondblclick={() => openFromCard(card)}>
                    <span class="title" class:secondary={!card.name}>{title(card)}</span>
                    {@render labels(card)}
                    {@render due(card)}
                  </button>
                </li>
              {/each}
            </ul>
          </section>
        {/each}
      </div>
    {/if}
    {/if}
  </dialog>
</div>
<style>
  .trello { padding: 8px 12px 10px; min-width: 0; display: flex; flex-direction: column; gap: 8px; flex: 1; font-size: 12px; }
  .sr-only { position: absolute; width: 1px; height: 1px; padding: 0; overflow: hidden; clip-path: inset(50%); white-space: nowrap; }
  .secondary { color: var(--muted); }
  .summary { display: flex; flex-direction: column; gap: 6px; }
  .summary[data-stale="true"] { color: var(--muted); }
  .board-name { display: flex; flex-wrap: wrap; align-items: baseline; justify-content: space-between; gap: 4px 10px; }
  .board-name h3 { font-size: 14px; font-weight: 500; overflow-wrap: anywhere; }
  .board-name span { color: var(--muted); font-size: 11px; }
  .counts { list-style: none; display: grid; grid-template-columns: repeat(auto-fill, minmax(96px, 1fr)); gap: 4px 12px; }
  .counts li { display: flex; justify-content: space-between; gap: 6px; padding: 2px 0; border-bottom: 1px solid var(--line); min-width: 0; }
  .list-name { min-width: 0; overflow-wrap: anywhere; color: var(--muted); }
  .preview { display: flex; flex-direction: column; gap: 3px; text-align: left; padding: 6px 10px; background: var(--surface-2); border-left: 4px solid var(--band-amber); border-radius: 0 12px 12px 0; color: inherit; cursor: pointer; }
  .preview small { color: var(--muted); font-size: 11px; }
  .open-board { align-self: flex-start; color: var(--accent); text-decoration: underline; cursor: pointer; }
  .preview:focus-visible, .open-board:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .title { overflow-wrap: anywhere; }
  .meta { display: flex; flex-wrap: wrap; align-items: center; gap: 4px 8px; }
  .labels { display: flex; flex-wrap: wrap; gap: 4px; }
  .labels i { width: 22px; height: 5px; border-radius: 3px; }
  .due { color: var(--muted); font-size: 11px; }
  .due.overdue { color: var(--warn); }
  .caption { font-size: 11px; color: var(--muted); }
  .blank { display: grid; place-items: center; min-height: 120px; padding: 12px; border: 1px dashed var(--line); font-size: 12px; color: var(--muted); text-align: center; }
  .notice { font-size: 12px; color: var(--muted); }
  .notice.alert { color: var(--warn); }
  .link { color: var(--accent); text-decoration: underline; cursor: pointer; }
  .link:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .actions { display: flex; flex-wrap: wrap; align-items: center; gap: 8px; margin-top: auto; }
  .action { min-height: 28px; padding: 4px 14px; border-radius: 0 18px 18px 0; background: var(--band-steel); color: var(--on-band); font-weight: 600; }
  .action:hover:not(:disabled) { filter: brightness(1.15); }
  .action:disabled { opacity: 0.45; cursor: default; }
  .action:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }

  /* Wide board: 1040 × 610 on the 1100 × 700 target, bounded by the window. */
  dialog { margin: auto; width: min(1040px, 96vw); height: min(610px, 92vh); max-width: 96vw; max-height: 92vh; padding: 16px 18px; color: var(--text); background: var(--surface); border: 1px solid var(--line); }
  dialog[open] { display: flex; flex-direction: column; gap: 8px; overflow: hidden; }
  dialog::backdrop { background: #000a; }
  .dialog-head { display: flex; justify-content: space-between; align-items: flex-start; gap: 16px; }
  .dialog-head h2 { font-size: 15px; font-weight: 500; text-transform: uppercase; letter-spacing: 1px; color: var(--band-amber-text); }
  .dialog-head p { font-size: 12px; margin-top: 2px; overflow-wrap: anywhere; }
  dialog button { cursor: pointer; }
  dialog button:focus-visible, select:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .close { padding: 6px 12px; border: 1px solid var(--line); border-radius: 0 14px 14px 0; }
  .move-toolbar { display: flex; flex-wrap: wrap; align-items: center; gap: 8px 12px; min-height: 52px; padding: 8px 12px; background: var(--surface-2); border-left: 4px solid var(--band-amber); flex: none; }
  .selection { display: grid; gap: 2px; max-width: 240px; overflow-wrap: anywhere; }
  .selection small { color: var(--muted); font-size: 11px; }
  .move-toolbar label { display: flex; align-items: center; gap: 8px; }
  select { min-height: 32px; max-width: 220px; background: var(--bg); color: var(--text); border: 1px solid var(--control-line); padding: 4px 6px; font: inherit; }
  .primary { min-height: 32px; padding: 6px 12px; background: var(--band-cyan); color: var(--on-band); border-radius: 0 16px 16px 0; font-weight: 600; }
  .primary:disabled { opacity: 0.5; cursor: default; }
  .plain { min-height: 32px; padding: 6px 10px; border: 1px solid var(--line); }
  .move-status { flex: none; min-height: 18px; font-size: 12px; color: var(--muted); overflow-wrap: anywhere; }
  .move-status.failed { color: var(--warn); }
  /* Columns scroll sideways, each list scrolls on its own; no card is hidden to fit. */
  .columns { flex: 1; min-height: 0; display: flex; gap: 12px; overflow-x: auto; align-items: stretch; padding: 3px 3px 10px; scrollbar-color: var(--control-line) var(--surface); }
  .columns[data-stale="true"] { color: var(--muted); }
  .column { flex: 0 0 228px; min-width: 0; min-height: 0; display: flex; flex-direction: column; background: var(--bg); border: 1px solid var(--line); border-top: 4px solid var(--band-blue); border-radius: 14px 0 0 0; padding: 10px; }
  .column.target { border-color: var(--accent); border-style: dashed; border-top-style: solid; }
  .column header { display: flex; justify-content: space-between; gap: 12px; padding: 2px 2px 8px; }
  h4 { font-size: 13px; font-weight: 600; overflow-wrap: anywhere; }
  .column header .num { color: var(--muted); }
  .move-here { margin-bottom: 8px; padding: 6px 8px; border: 1px dashed var(--accent); color: var(--accent); text-align: left; }
  .empty { padding: 16px 8px; border: 1px dashed var(--line); color: var(--muted); }
  ul { list-style: none; }
  .column ul { display: flex; flex-direction: column; gap: 8px; overflow-y: auto; min-height: 0; padding: 3px; }
  .column li { flex-shrink: 0; }
  .card { display: flex; flex-direction: column; gap: 6px; width: 100%; padding: 10px; text-align: left; color: inherit; background: var(--surface-2); border: 1px solid var(--line); border-radius: 0 12px 12px 0; user-select: none; }
  .card[draggable="true"] { cursor: grab; }
  .card.selected { outline: 2px solid var(--accent); outline-offset: 0; border-left: 4px solid var(--accent); }
  .card .title { font-size: 13px; }
  .card .labels { order: -1; }
</style>
