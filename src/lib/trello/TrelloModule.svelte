<script lang="ts">
  import { onMount } from "svelte";
  import type { TrelloData } from "../core";
  import { t, interpolate } from "../strings";
  let { data, refresh, configure, openCard }: {
    data: TrelloData; refresh: () => void; configure: () => void; openCard: (url: string) => void;
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
  const stale = $derived(data.state === "invalidCredential" && !data.paused);
  function refreshNow() { allowedAt = Date.now() + 10000; refresh(); }
</script>
<section class="trello">
  <div class="bar">
    {#if data.board}<h2>{data.board.name}</h2>{/if}
    <button aria-label={t.trello.refresh} disabled={data.paused || data.refreshing || now < allowedAt} onclick={refreshNow}>↻</button>
  </div>
  {#if data.refreshing}<span role="status">{t.trello.refreshing}</span>{/if}
  {#if data.state === "missingCredential"}
    <p>{t.trello.missingCredential}</p>
    <button onclick={configure}>{t.trello.configure}</button>
  {:else if data.state === "chooseBoard"}
    <p>{t.trello.chooseBoard}</p>
    <button onclick={configure}>{t.trello.configure}</button>
  {:else}
    {#if stale}<button onclick={configure}>{t.trello.invalidCredential}</button>{/if}
    {#if data.state === "error" && !data.paused}<p role="status">{data.failureSince != null ? interpolate(t.trello.failureAge, { age: age(data.failureSince) }) : t.trello.error}</p>{/if}
    {#if data.lastSuccessAt != null && (data.state !== "ready" || data.paused)}<p class="secondary">{interpolate(t.trello.updatedAge, { age: age(data.lastSuccessAt) })}</p>{/if}
    <div class="columns" data-stale={stale} class:dimmed={stale}>
      {#each data.lists ?? [] as list (list.id)}
        <section class="column" aria-label={list.name}>
          <h3>{list.name}</h3>
          {#if list.cards.length === 0}<p class="secondary">{t.trello.emptyList}</p>{/if}
          <ul>
            {#each list.cards as card (card.id)}
              <li title={t.trello.openCard} ondblclick={() => openCard(card.url)}>
                <span class="title" class:secondary={!card.name}>{card.name || t.trello.hiddenTitle}</span>
                {#if card.labels.length || card.due != null}
                  <div class="meta">
                    {#if card.labels.length}
                      <span class="labels" aria-label={interpolate(t.trello.labels, { colors: card.labels.join(", ") })}>
                        {#each card.labels as label}<i data-testid="label" style:background={color(label)}></i>{/each}
                      </span>
                    {/if}
                    {#if card.due != null}
                      <span class="due num" class:overdue={card.overdue}>
                        {interpolate(card.overdue ? t.trello.overdue : t.trello.due, { date: date.format(card.due) })}
                      </span>
                    {/if}
                  </div>
                {/if}
              </li>
            {/each}
          </ul>
        </section>
      {/each}
    </div>
  {/if}
</section>
<style>
  .trello { padding: 0 16px 16px; min-width: 0; display: flex; flex-direction: column; gap: 8px; font-size: 12px; }
  .bar { display: flex; align-items: center; justify-content: space-between; gap: 8px; }
  h2 { font-size: 13px; }
  h3 { font-size: 12px; font-weight: 600; margin-bottom: 8px; }
  p { margin: 4px 0; }
  button { align-self: flex-start; padding: 5px 8px; border: 1px solid var(--line); border-radius: 5px; }
  button:disabled { opacity: .4; }
  .dimmed { opacity: .5; }
  .secondary { color: var(--muted); }
  .columns { display: flex; gap: 8px; overflow-x: auto; align-items: flex-start; padding-bottom: 4px; }
  .column { flex: 0 0 180px; background: var(--surface); border: 1px solid var(--line); border-radius: 8px; padding: 8px; }
  ul { list-style: none; display: flex; flex-direction: column; gap: 6px; }
  li { background: var(--surface-2); border-radius: 6px; padding: 6px 8px; cursor: default; user-select: none; }
  .title { display: -webkit-box; -webkit-line-clamp: 2; line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
  .meta { display: flex; align-items: center; gap: 8px; margin-top: 4px; }
  .labels { display: flex; gap: 3px; }
  .labels i { width: 8px; height: 8px; border-radius: 50%; }
  .due { color: var(--muted); font-size: 11px; }
  .due.overdue { color: var(--warn); }
</style>
