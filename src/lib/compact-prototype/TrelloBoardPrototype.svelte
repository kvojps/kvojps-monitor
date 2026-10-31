<script lang="ts">
  // Throwaway board interaction: local fixtures only, no Trello requests.
  import type fixture from "../trello/fixtures/full.json";
  let { lists = $bindable(), boardName }: { lists: typeof fixture.lists; boardName: string } = $props();
  let selected = $state<string | null>(null);
  let destination = $state("");
  let moving = $state(false);
  let failNext = $state(false);
  let message = $state("");
  let failed = $state(false);
  const columns: Record<string, HTMLElement> = {};
  const currentList = $derived(lists.find(list => list.cards.some(card => card.id === selected)));
  const currentCard = $derived(currentList?.cards.find(card => card.id === selected));
  function select(card: string) {
    if (moving) return;
    selected = selected === card ? null : card;
    destination = ""; message = ""; failed = false;
  }
  async function move(listId: string, cardId = selected) {
    if (moving || !cardId) return;
    const origin = lists.find(list => list.cards.some(card => card.id === cardId));
    const target = lists.find(list => list.id === listId);
    const card = origin?.cards.find(card => card.id === cardId);
    if (!origin || !target || !card || origin.id === target.id) return;
    const previous = structuredClone($state.snapshot(lists));
    const shouldFail = failNext;
    selected = cardId; moving = true; failed = false;
    lists = lists.map(list => ({ ...list, cards: list.id === target.id ? [...list.cards,card] : list.cards.filter(item => item.id !== cardId) }));
    message = `Movendo “${card.name}” para ${target.name}…`;
    await new Promise(resolve => setTimeout(resolve, 450));
    if (shouldFail) {
      lists = previous; failed = true; failNext = false;
      message = `Não foi possível mover “${card.name}”. O card voltou para ${origin.name}. Tente novamente.`;
    } else {
      message = `“${card.name}” movido para ${target.name}.`;
    }
    moving = false; destination = "";
    columns[shouldFail ? origin.id : target.id]?.scrollIntoView({ block: "nearest", inline: "nearest", behavior: "instant" });
  }
  const labelColor = (label: string) => label.startsWith("green") ? "#4bce97" : label.startsWith("red") ? "#f87168" : "#579dff";
</script>

<div class="board-toolbar">
  <div><h3>{boardName}</h3><p>{lists.reduce((total,list)=>total+list.cards.length,0)} cards · {lists.length} listas</p></div>
  <p>Selecione um card e a lista de destino, ou arraste entre colunas.</p>
</div>
<div class="move-toolbar">
  {#if currentCard}
    <div class="selection"><b>{currentCard.name}</b><small>em {currentList?.name}</small></div>
    <label>Mover para <select bind:value={destination} disabled={moving}><option value="">Escolha uma lista</option>{#each lists.filter(list=>list.id!==currentList?.id) as list}<option value={list.id}>{list.name}</option>{/each}</select></label>
    <button class="primary" disabled={!destination || moving} onclick={()=>move(destination)}>Mover card</button>
    <button disabled={moving} onclick={()=>{selected=null; destination="";}}>Cancelar seleção</button>
  {:else}<p>Selecione um card para habilitar a movimentação.</p>{/if}
</div>
<div class="move-status" role="status" class:failed>{message || "Nenhuma alteração realizada."}</div>
<div class="board-columns" aria-label="Listas do quadro">
  {#each lists as list (list.id)}
    <!-- Dragging is an optional pointer shortcut; selection and buttons provide the keyboard path. -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <section class="board-column" aria-label={list.name} class:target={selected && currentList?.id!==list.id} bind:this={columns[list.id]}
      ondragover={event=>{if(!moving){event.preventDefault();if(event.dataTransfer)event.dataTransfer.dropEffect="move";}}}
      ondrop={event=>{event.preventDefault();const cardId=event.dataTransfer?.getData("text/plain");if(cardId)void move(list.id,cardId);}}>
      <header><h4>{list.name}</h4><span class="num">{list.cards.length}</span></header>
      {#if selected && currentList?.id!==list.id}<button class="move-here" disabled={moving} onclick={()=>move(list.id)}>→ Mover para esta lista</button>{/if}
      <ul>
        {#each list.cards as card (card.id)}
          <li>
            <button class="board-card" class:selected={selected===card.id} aria-pressed={selected===card.id} disabled={moving}
              draggable={!moving} ondragstart={event=>{select(card.id);event.dataTransfer?.setData("text/plain",card.id);if(event.dataTransfer)event.dataTransfer.effectAllowed="move";}}
              onclick={()=>select(card.id)} ondblclick={()=>message=`Abrir “${card.name}” no navegador — ação simulada.`}>
              <span class="labels">{#each card.labels as label}<span style:background={labelColor(label)} title={label} aria-label={`Etiqueta ${label}`}></span>{/each}</span>
              <b>{card.name}</b><small>{card.due ? new Date(card.due).toLocaleDateString("pt-BR") : "Sem vencimento"}</small>
            </button>
            {#if selected===card.id}<button class="open-card" onclick={()=>message=`Abrir “${card.name}” no navegador — ação simulada.`}>Abrir no Trello ↗</button>{/if}
          </li>
        {/each}
        {#if list.cards.length===0}<li class="empty">Nenhum card nesta lista</li>{/if}
      </ul>
    </section>
  {/each}
</div>
<div class="lab"><label><input type="checkbox" bind:checked={failNext} disabled={moving}/> Simular falha na próxima movimentação</label><span>Demonstração · alterações somente em memória</span></div>

<style>
  .board-toolbar { display:flex; justify-content:space-between; gap:20px; align-items:center; margin:10px 0 12px; }
  h3 { font-size:17px; font-weight:500; }
  p, small { color:var(--muted); font-size:12px; }
  .board-toolbar p { margin-top:5px; }
  .move-toolbar { display:flex; gap:12px; align-items:center; flex-wrap:wrap; min-height:66px; padding:10px 12px; background:var(--surface-2); border-left:4px solid var(--band-amber); }
  .selection { display:grid; gap:4px; max-width:240px; overflow-wrap:anywhere; }
  label { display:flex; align-items:center; gap:8px; font-size:12px; }
  select { min-height:34px; max-width:220px; background:var(--bg); color:var(--text); border:1px solid var(--control-line); padding:6px; font:inherit; }
  button { min-height:32px; padding:6px 10px; }
  .primary { background:var(--band-cyan); color:var(--on-band); border-radius:0 16px 16px 0; }
  button:disabled { opacity:.55; cursor:default; }
  button:focus-visible, select:focus-visible { outline:2px solid var(--accent); outline-offset:2px; }
  .move-status { min-height:36px; padding:8px 0; font-size:12px; color:var(--muted); overflow-wrap:anywhere; }
  .move-status.failed { color:var(--warn); }
  .board-columns { display:flex; gap:12px; overflow-x:auto; min-height:0; flex:1; padding:3px 3px 12px; align-items:stretch; scrollbar-color:var(--control-line) var(--surface); }
  .board-column { flex:0 0 228px; min-width:0; display:flex; flex-direction:column; border:1px solid var(--line); border-top:4px solid var(--band-blue); background:var(--bg); border-radius:14px 0 0 0; padding:10px; }
  .board-column.target { border-color:var(--accent); }
  .board-column header { display:flex; justify-content:space-between; gap:12px; padding:3px 2px 10px; }
  h4 { font-size:14px; overflow-wrap:anywhere; }
  .board-column header span { color:var(--muted); }
  .move-here { border:1px dashed var(--accent); color:var(--accent); margin:0 0 8px; text-align:left; }
  ul { list-style:none; overflow-y:auto; min-height:0; padding:3px; display:flex; flex-direction:column; gap:9px; }
  li { flex-shrink:0; }
  .board-card { display:flex; flex-direction:column; gap:9px; width:100%; padding:12px; text-align:left; background:var(--surface-2); border:1px solid var(--line); border-radius:0 12px 12px 0; cursor:grab; }
  .board-card.selected { outline:2px solid var(--accent); outline-offset:0; }
  .board-card b { font-size:13px; font-weight:500; overflow-wrap:anywhere; }
  .labels { display:flex; gap:4px; flex-wrap:wrap; }
  .labels:empty { display:none; }
  .labels span { width:26px; height:5px; border-radius:3px; }
  .open-card { color:var(--accent); padding-left:4px; font-size:12px; text-align:left; }
  .empty { color:var(--muted); padding:20px 8px; border:1px dashed var(--line); font-size:12px; }
  .lab { display:flex; justify-content:space-between; flex-wrap:wrap; gap:10px; font-size:11px; color:var(--muted); border-top:1px solid var(--line); padding-top:10px; }
  @media(max-width:650px) { .board-toolbar { align-items:start; flex-direction:column; gap:4px; } .board-column { flex-basis:210px; } .selection { max-width:100%; } }
</style>
