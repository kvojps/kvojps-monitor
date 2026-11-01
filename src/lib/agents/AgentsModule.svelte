<script lang="ts">
  import { tick, untrack } from "svelte";
  import type { AgentsData, AgentSession } from "../core";
  import { interpolate, t } from "../strings";

  /** `notified` is the session a notification pointed to; it is not the last anchor. */
  let { data, notified = null, notificationRequest = 0 }: { data: AgentsData | null; notified?: string | null; notificationRequest?: number } = $props();

  /** Same allowlist as the core notifications: free-form waiting detail can carry user content. */
  const TOOLS = new Set(["Bash", "Read", "Write", "Edit", "Glob", "Grep", "WebFetch", "WebSearch", "Agent"]);
  const REASONS: Record<string, string> = {
    "permission prompt": t.agents.permission, "input needed": t.agents.input,
    "sandbox request": t.agents.sandbox, "dialog open": t.agents.dialog,
  };
  /** Live states summarized in the counters; closed sessions are counted from `closed`. */
  const LIVE_STATES = ["working", "waiting", "done", "failed"] as const;
  const time = new Intl.DateTimeFormat("pt-BR", { hour: "2-digit", minute: "2-digit" });

  let list = $state<HTMLElement>();

  let dialog: HTMLDialogElement;
  let detailOpen = $state(false);
  let restoringFocus = false;
  let opener: HTMLElement | null = null;
  let ring = $state<HTMLButtonElement>();
  let selected = $state<string | null>(null);
  const shown = $derived(data ? [...data.sessions, ...data.closed] : []);
  const states = [...LIVE_STATES, "closed"] as const;
  const colors: Record<AgentSession["state"], string> = {
    working: "var(--accent)", waiting: "var(--warn)", done: "var(--ok)",
    failed: "var(--bad)", closed: "var(--muted)",
  };
  const count = (state: AgentSession["state"]) => shown.filter((s) => s.state === state).length;
  const waiting = $derived(count("waiting"));
  const gradient = $derived.by(() => {
    let offset = 0;
    return states.map((state) => {
      const start = offset;
      offset += shown.length ? count(state) / shown.length * 100 : 0;
      return `${colors[state]} ${start}% ${offset}%`;
    }).join(", ");
  });

  async function open(id: string | null = selected) {
    selected = id;
    if (!dialog.open) {
      opener = document.activeElement instanceof HTMLElement && document.activeElement !== document.body ? document.activeElement : ring ?? null;
      dialog.showModal();
      detailOpen = true;
    }
    await tick();
    Array.from(list?.children ?? []).find((row) => (row as HTMLElement).dataset.sessionId === id)
      ?.scrollIntoView?.({ block: "nearest" });
  }
  function close() {
    restoringFocus = true;
    dialog.close();
    detailOpen = false;
    opener?.focus({ preventScroll: true });
    restoringFocus = false;
  }
  function keyboard(event: KeyboardEvent) {
    if (event.key === "Escape" && dialog?.open) {
      event.preventDefault();
      event.stopPropagation();
      close();
    }
  }
  $effect(() => {
    notificationRequest;
    const id = notified;
    if (id) untrack(() => void open(id));
  });
  const targetAvailable = $derived(!!notified && shown.some((session) => session.id === notified));
  $effect(() => {
    const id = notified;
    if (!detailOpen || !targetAvailable) return;
    void tick().then(() => {
      Array.from(list?.children ?? []).find((row) => (row as HTMLElement).dataset.sessionId === id)
        ?.scrollIntoView?.({ block: "nearest" });
    });
  });
  const provider = (session: AgentSession) => session.provider === "codex" ? "codex" : "claude";
  const providerName = (session: AgentSession) => provider(session) === "codex" ? t.agents.codex : t.agents.provider;

  function elapsed(session: AgentSession, observedAt: number) {
    const seconds = Math.max(0, Math.floor(((session.closedAt ?? observedAt) - session.startedAt) / 1000));
    if (seconds < 60) return `${seconds} s`;
    if (seconds < 3600) return `${Math.floor(seconds / 60)} min ${seconds % 60} s`;
    return `${Math.floor(seconds / 3600)} h ${String(Math.floor((seconds % 3600) / 60)).padStart(2, "0")} min`;
  }

  function reason(session: AgentSession) {
    if (session.state === "waiting") {
      const detail = session.waitingFor ?? "";
      return TOOLS.has(detail) ? interpolate(t.agentNotifications.approval, { tool: detail }) : REASONS[detail] ?? t.agents.waitingDetail;
    }
    if (session.state === "closed" && (session.lastState === "done" || session.lastState === "failed"))
      return interpolate(t.agents.lastState, { state: t.agents.states[session.lastState] });
    return null;
  }
</script>

<svelte:window onkeydowncapture={keyboard} />

<div class="agents" onfocusin={(event) => { if (restoringFocus) event.stopPropagation(); }}>
  {#if !data}
    <p class="blank">{t.agents.noReading}</p>
  {:else}
    <div class="banner" class:pending={waiting > 0} role="group" aria-label={t.agents.intervention}>
      <span class="attention">{#if waiting > 0}<span aria-hidden="true">● </span>{t.agents.states.waiting}{:else}{t.agents.noIntervention}{/if}</span>
      <button disabled={!waiting} onclick={() => open(shown.find((s) => s.state === "waiting")?.id ?? null)}><strong class="num">{String(waiting).padStart(2, "0")}</strong>{#if waiting}<span class="sr-only">{t.agents.states.waiting}</span>{/if}</button>
    </div>

    {#if data.codexWaitingUnavailable}<p class="notice">{t.agents.codexWaitingUnavailable}</p>{/if}
    {#if data.failed}
      <p class="notice alert">{[t.agents.failure, data.lastSuccessAt != null && interpolate(t.agents.lastRead, { time: time.format(data.lastSuccessAt) })].filter(Boolean).join(" · ")}</p>
    {/if}

    <div class="compact-summary">
      <button class="ring" bind:this={ring} aria-label={t.agents.list} onclick={() => open()} style:background={shown.length ? `conic-gradient(${gradient})` : "var(--surface-2)"}>
        <span><strong>{count("working")}</strong><small>{t.agents.states.working}</small></span>
      </button>
      <dl class="flow" aria-label={t.agents.summary}>
        {#each states as state}
          <div><dt><button style:color={colors[state]} onclick={() => open(shown.find((s) => s.state === state)?.id ?? null)} aria-describedby={"agent-legend-" + state}>{state === "closed" ? t.agents.closed : t.agents.states[state]}</button>
            <span class="legend-projects" id={"agent-legend-" + state}>{shown.filter((s) => s.state === state).map((s) => `${s.project} · ${s.name ?? t.agents.unnamed}`).join(", ") || "—"}</span>
          </dt><dd>{count(state)}</dd></div>
        {/each}
      </dl>
    </div>
    {#if shown.length === 0}<p class="blank">{t.agents.empty}</p>{/if}
  {/if}
  <dialog bind:this={dialog} aria-label={t.agents.list} oncancel={(event) => { event.preventDefault(); close(); }}>
    <header><h2>{t.agents.list}</h2><button onclick={close}>{t.agents.close}</button></header>
    {#if notified && !shown.some((s) => s.id === notified)}<p role="status">{t.agents.targetUnavailable}</p>{/if}
    {#if detailOpen}
      {#if !data}<p class="blank">{t.agents.noReading}</p>{:else if shown.length === 0}<p class="blank">{t.agents.empty}</p>{/if}
    {/if}
    {#if data}
      <ul class="sessions" aria-label={t.agents.list} bind:this={list}>
        {#each shown as session (session.id)}
          {@const waitReason = reason(session)}
          <li data-session-id={session.id} data-state={session.state} aria-current={selected === session.id ? "true" : undefined}>
            <span class="code num" aria-hidden="true">{t.agents.providerCodes[provider(session)]}</span>
            <div class="body">
              <h3>{session.name ?? t.agents.unnamed}</h3>
              <p class="origin">{session.project} · {providerName(session)}</p>
              {#if session.branch}<p>{session.branch}</p>{/if}
              <p class="status"><span class="state">{t.agents.states[session.state]}</span>{#if waitReason}<span>{waitReason}</span>{/if}</p>
              {#if notified === session.id}<p class="tag">{t.agents.notified}</p>{/if}
            </div>
            <span class="elapsed num">{elapsed(session, data.observedAt)}</span>
          </li>
        {/each}
      </ul>
    {/if}
  </dialog>
</div>

<style>
  button { cursor: pointer; }
  .sr-only { position: absolute; width: 1px; height: 1px; overflow: hidden; clip-path: inset(50%); }
  .compact-summary { display: flex; align-items: center; gap: 16px; }
  .ring { flex: none; width: 112px; height: 112px; border-radius: 50%; padding: 10px; border: 0; }
  .ring > span { display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; border-radius: 50%; background: var(--surface); }
  .ring strong { font-size: 28px; }
  .ring small { font-size: 10px; }
  .flow { flex: 1; min-width: 0; }
  dt { position: relative; }
  .legend-projects { display: none; position: absolute; z-index: 2; background: var(--surface); padding: 8px; border: 1px solid var(--line); width: 220px; overflow-wrap: anywhere; }
  dt:hover .legend-projects, dt:focus-within .legend-projects { display: block; }
  dialog { margin: auto; width: min(680px, 90vw); max-height: 80vh; overflow: auto; padding: 20px; color: var(--text); background: var(--surface); border: 1px solid var(--line); }
  dialog::backdrop { background: #000a; }
  dialog header { display: flex; justify-content: space-between; gap: 16px; }

  .agents { display: flex; flex-direction: column; gap: 8px; padding: 8px 14px 12px; min-width: 0; flex: 1; }
  .banner {
    display: flex; align-items: center; justify-content: space-between; gap: 12px;
    padding: 0 4px 6px; border-bottom: 3px solid var(--band-violet);
  }
  .attention { font: 11px var(--mono); letter-spacing: 1px; text-transform: uppercase; color: var(--muted); }
  .banner strong { font-size: 30px; font-weight: 300; line-height: 1; color: var(--muted); }
  .pending .attention, .pending strong { color: var(--warn); }
  .notice { font-size: 12px; color: var(--muted); }
  .notice.alert { color: var(--warn); }
  .blank { display: grid; place-items: center; min-height: 100px; padding: 12px; border: 1px dashed var(--line); font-size: 12px; color: var(--muted); text-align: center; }

  .sessions { list-style: none; display: flex; flex-direction: column; }
  li { display: flex; gap: 12px; align-items: flex-start; padding: 14px 4px; border-bottom: 1px solid var(--line); scroll-margin: 60px 0 16px; }
  li[aria-current] { outline: 2px solid var(--accent); outline-offset: -2px; background: var(--accent-dim); }
  .code {
    flex: none; display: grid; place-items: center; width: 35px; height: 30px;
    border-radius: 20px 0 0 20px; background: var(--band-violet); color: var(--on-band); font-size: 11px;
  }
  .body { flex: 1; min-width: 0; display: flex; flex-direction: column; gap: 5px; }
  h3 { font-size: 13px; font-weight: 500; overflow-wrap: anywhere; }
  .origin { font-size: 11px; color: var(--muted); overflow-wrap: anywhere; }
  .status { display: flex; flex-wrap: wrap; gap: 4px 10px; font: 10px var(--mono); letter-spacing: 0.5px; color: var(--muted); }
  .state { text-transform: uppercase; }
  [data-state="working"] .state { color: var(--accent); }
  [data-state="waiting"] .state { color: var(--warn); }
  [data-state="done"] .state { color: var(--ok); }
  [data-state="failed"] .state { color: var(--bad); }
  .tag { font: 10px var(--mono); color: var(--accent); }
  .elapsed { flex: none; font-size: 11px; color: var(--muted); white-space: nowrap; }

  .flow { display: flex; flex-direction: column; margin-top: auto; }
  .flow div { display: flex; justify-content: space-between; gap: 8px; padding: 4px 4px; border-bottom: 2px solid var(--surface-2); font-size: 11px; color: var(--muted); }
  .flow dd { color: var(--band-lavender-text); font-weight: 600; }
</style>
