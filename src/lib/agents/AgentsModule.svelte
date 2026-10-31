<script lang="ts">
  import { tick } from "svelte";
  import type { AgentsData, AgentSession } from "../core";
  import { interpolate, t } from "../strings";

  /** `notified` is the session a notification pointed to; it is not the last anchor. */
  let { data, notified = null }: { data: AgentsData | null; notified?: string | null } = $props();

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

  /** A targeted session that closed since the notification still has to be reached. */
  const shown = $derived.by(() => {
    if (!data) return [];
    const closed = data.closed.find((s) => s.id === notified);
    return closed ? [...data.sessions, closed] : data.sessions;
  });
  const count = (state: AgentSession["state"]) => data?.sessions.filter((s) => s.state === state).length ?? 0;
  const waiting = $derived(count("waiting"));

  $effect(() => {
    const session = notified;
    if (!session || !shown.length) return;
    void tick().then(() => {
      Array.from(list?.children ?? []).find((row) => (row as HTMLElement).dataset.sessionId === session)
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

<div class="agents">
  {#if !data}
    <p class="blank">{t.agents.noReading}</p>
  {:else}
    <div class="banner" class:pending={waiting > 0} role="group" aria-label={t.agents.intervention}>
      <span class="attention">{#if waiting > 0}<span aria-hidden="true">● </span>{t.agents.states.waiting}{:else}{t.agents.noIntervention}{/if}</span>
      <strong class="num">{String(waiting).padStart(2, "0")}</strong>
    </div>

    {#if data.codexWaitingUnavailable}<p class="notice">{t.agents.codexWaitingUnavailable}</p>{/if}
    {#if data.failed}
      <p class="notice alert">{[t.agents.failure, data.lastSuccessAt != null && interpolate(t.agents.lastRead, { time: time.format(data.lastSuccessAt) })].filter(Boolean).join(" · ")}</p>
    {/if}

    {#if shown.length === 0}
      <p class="blank">{t.agents.empty}</p>
    {:else}
      <ul class="sessions" aria-label={t.agents.list} bind:this={list}>
        {#each shown as session (session.id)}
          {@const waitReason = reason(session)}
          <li data-session-id={session.id} data-state={session.state} aria-current={notified === session.id ? "true" : undefined}>
            <span class="code num" aria-hidden="true">{t.agents.providerCodes[provider(session)]}</span>
            <div class="body">
              <h3>{session.name ?? t.agents.unnamed}</h3>
              <p class="origin">{session.project} · {providerName(session)}</p>
              <p class="status"><span class="state">{t.agents.states[session.state]}</span>{#if waitReason}<span>{waitReason}</span>{/if}</p>
              {#if notified === session.id}<p class="tag">{t.agents.notified}</p>{/if}
            </div>
            <span class="elapsed num">{elapsed(session, data.observedAt)}</span>
          </li>
        {/each}
      </ul>
    {/if}

    <dl class="flow" aria-label={t.agents.summary}>
      {#each LIVE_STATES as state (state)}
        <div><dt>{t.agents.states[state]}</dt><dd class="num">{count(state)}</dd></div>
      {/each}
      <div><dt>{t.agents.closed}</dt><dd class="num">{data.closed.length}</dd></div>
    </dl>
  {/if}
</div>

<style>
  .agents { display: flex; flex-direction: column; gap: 12px; padding: 12px 16px 18px; min-width: 0; flex: 1; }
  .banner {
    display: flex; align-items: center; justify-content: space-between; gap: 12px;
    padding: 4px 4px 14px; border-bottom: 3px solid var(--band-violet);
  }
  .attention { font: 11px var(--mono); letter-spacing: 1px; text-transform: uppercase; color: var(--muted); }
  .banner strong { font-size: 56px; font-weight: 300; line-height: 1; color: var(--muted); }
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
  .flow div { display: flex; justify-content: space-between; gap: 8px; padding: 9px 4px; border-bottom: 3px solid var(--surface-2); font-size: 11px; color: var(--muted); }
  .flow dd { color: var(--band-lavender-text); font-weight: 600; }
</style>
