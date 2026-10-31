<script lang="ts">
  import { onMount, untrack } from "svelte";
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
  const paused = $derived(data?.paused ?? false);
  const stale = $derived(data?.state === "invalidCredential" && !paused);
  function refreshNow() { allowedAt = Date.now() + 10000; refresh(); }
  let selected = $state<string | null>(null);
  let moveFailed = $state(false);
  /** Destination by card until the core answers; a settled move lasts until the next board. */
  let pending = $state<Record<string, { list: string; move: number; settled?: boolean }>>({});
  let moves = 0;
  $effect(() => {
    void data;
    untrack(() => {
      for (const card in pending) if (pending[card].settled) delete pending[card];
      // A refresh keeps the selection unless the card left the board.
      if (selected && !listOf(selected)) selected = null;
    });
  });
  // Optimistic view: a pending card shows at the bottom of its destination list.
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
  const title = (card: { name: string }) => card.name || t.trello.hiddenTitle;
  const listOf = (card: string) => lists.find((list) => list.cards.some((c) => c.id === card))?.id;
  const movable = $derived(data != null && !paused && data.state !== "invalidCredential");
  function selectCard(event: MouseEvent, card: string) {
    // With a card selected, a click on a card of another list targets that list.
    if (movable && selected && listOf(selected) !== listOf(card)) return;
    event.stopPropagation();
    selected = selected === card ? null : card;
    moveFailed = false;
  }
  function moveTo(list: string) {
    const card = selected;
    selected = null;
    if (!card || !movable || listOf(card) === list) return;
    moveFailed = false;
    const move = ++moves;
    pending[card] = { list, move };
    // A later move of the same card owns the overlay.
    const current = () => pending[card]?.move === move;
    moveCard(card, list).then(
      () => { if (current()) pending[card].settled = true; },
      () => { moveFailed = true; if (current()) delete pending[card]; },
    );
  }
</script>
<div class="trello">
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
    {#if moveFailed}<p role="alert" class="notice alert">{t.trello.moveFailed}</p>{/if}
    {#if data.board}<h3 class="board">{data.board.name}</h3>{/if}
    {#if lists.length === 0}<p class="blank">{t.trello.noLists}</p>{:else}
    <div class="columns" data-stale={stale}>
      {#each lists as list (list.id)}
        {@const target = movable && selected != null && listOf(selected) !== list.id}
        <!-- Clicking anywhere in a column is a mouse shortcut; the header button is the keyboard path. -->
        <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_noninteractive_element_interactions -->
        <section class="column" aria-label={list.name} class:target onclick={() => moveTo(list.id)}>
          <h4>
            {#if target}
              <button class="move" aria-label={interpolate(t.trello.moveTo, { list: list.name })}
                onclick={(event) => { event.stopPropagation(); moveTo(list.id); }}><span aria-hidden="true">→ </span>{list.name}</button>
            {:else}{list.name}{/if}
          </h4>
          {#if list.cards.length === 0}<p class="secondary">{t.trello.emptyList}</p>{/if}
          <ul>
            {#each list.cards as card (card.id)}
              <li class:selected={selected === card.id}>
                <button class="card" aria-pressed={selected === card.id}
                  onclick={(event) => selectCard(event, card.id)} ondblclick={() => openCard(card.url)}>
                  <span class="title" class:secondary={!card.name}>{title(card)}</span>
                  {#if card.labels.length || card.due != null}
                    <span class="meta">
                      {#if card.labels.length}
                        <span class="labels" role="img" aria-label={interpolate(t.trello.labels, { colors: card.labels.join(", ") })}>
                          {#each card.labels as label}<i data-testid="label" style:background={color(label)}></i>{/each}
                        </span>
                      {/if}
                      {#if card.due != null}
                        <span class="due num" class:overdue={card.overdue}>
                          {interpolate(card.overdue ? t.trello.overdue : t.trello.due, { date: date.format(card.due) })}
                        </span>
                      {/if}
                    </span>
                  {/if}
                </button>
                {#if selected === card.id}
                  <button class="link open" onclick={(event) => { event.stopPropagation(); openCard(card.url); }}>
                    {interpolate(t.trello.open, { card: title(card) })}
                  </button>
                {/if}
              </li>
            {/each}
          </ul>
        </section>
      {/each}
    </div>
    {/if}
  {/if}
  <div class="actions">
    <button class="action" disabled={paused || data?.refreshing || now < allowedAt} onclick={refreshNow}>
      <span aria-hidden="true">↻ </span>{t.trello.refresh}
    </button>
    <button class="action" onclick={configure}>{t.trello.configureTrello}</button>
    {#if data?.refreshing}<span role="status" class="caption">{t.trello.refreshing}</span>{/if}
  </div>
</div>
<style>
  .trello { padding: 16px 16px 18px; min-width: 0; display: flex; flex-direction: column; gap: 12px; flex: 1; font-size: 12px; }
  .board { font: 10px var(--mono); letter-spacing: 1.5px; text-transform: uppercase; color: var(--muted); }
  h4 { font-size: 12px; font-weight: 600; margin-bottom: 8px; overflow-wrap: anywhere; }
  .columns[data-stale="true"] { color: var(--muted); }
  .secondary { color: var(--muted); }
  /* Long lists scroll inside their column; no card is hidden to keep the rows even. */
  .columns { display: flex; gap: 8px; overflow-x: auto; align-items: flex-start; padding-bottom: 4px; min-height: 240px; }
  .column { flex: 0 0 180px; max-height: 420px; display: flex; flex-direction: column; min-height: 0; background: var(--surface); border: 1px solid var(--line); border-top: 6px solid var(--band-blue); border-radius: 16px 0 0 0; padding: 8px; }
  ul { list-style: none; display: flex; flex-direction: column; gap: 6px; overflow-y: auto; min-height: 0; padding: 2px; }
  li { background: var(--surface-2); border-radius: 0 12px 12px 0; display: flex; flex-direction: column; align-items: stretch; }
  .card { display: block; width: 100%; padding: 6px 8px; text-align: left; border-radius: inherit; cursor: default; user-select: none; color: inherit; }
  .card:focus-visible, .move:focus-visible { outline: 2px solid var(--accent); outline-offset: -2px; }
  .target { border-color: var(--accent); cursor: pointer; }
  .move { color: var(--accent); font: inherit; text-align: left; cursor: pointer; overflow-wrap: anywhere; }
  li.selected { outline: 2px solid var(--accent); }
  .open { align-self: flex-start; margin: 0 8px 6px; font-size: 11px; text-align: left; }
  .title { display: -webkit-box; -webkit-line-clamp: 2; line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; overflow-wrap: anywhere; }
  .meta { display: flex; margin-top: 4px; flex-wrap: wrap; align-items: center; gap: 4px 8px; }
  .labels { display: flex; gap: 3px; }
  .labels i { width: 8px; height: 8px; border-radius: 50%; }
  .due { color: var(--muted); font-size: 11px; }
  .due.overdue { color: var(--warn); }
  .caption { font-size: 11px; color: var(--muted); }
  .blank { display: grid; place-items: center; min-height: 120px; padding: 12px; border: 1px dashed var(--line); font-size: 12px; color: var(--muted); text-align: center; }
  .notice { font-size: 12px; color: var(--muted); }
  .notice.alert { color: var(--warn); }
  .link { color: var(--accent); text-decoration: underline; }
  .link:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .actions { display: flex; flex-wrap: wrap; align-items: center; gap: 8px; margin-top: auto; }
  .action { min-height: 36px; padding: 8px 15px; border-radius: 0 18px 18px 0; background: var(--band-steel); color: var(--on-band); font-weight: 600; }
  .action:hover:not(:disabled) { filter: brightness(1.15); }
  .action:disabled { opacity: 0.45; cursor: default; }
  .action:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
</style>
