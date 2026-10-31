<script lang="ts">
  import type { AgentsData, AgentSession } from "../core";
  import { t } from "../strings";
  let { data }: { data: AgentsData } = $props();
  function elapsed(session: AgentSession) {
    const seconds = Math.max(0, Math.floor(((session.closedAt ?? data.observedAt) - session.startedAt) / 1000));
    return seconds < 60 ? `${seconds} s` : `${Math.floor(seconds / 60)} min ${seconds % 60} s`;
  }
  function detail(session: AgentSession) {
    if (session.state === "closed" && (session.lastState === "done" || session.lastState === "failed")) return t.agents.states[session.lastState];
    const reasons: Record<string, string> = {
      "permission prompt": t.agents.permission, "input needed": t.agents.input,
      "sandbox request": t.agents.sandbox, "dialog open": t.agents.dialog,
    };
    return session.state === "waiting" ? reasons[session.waitingFor ?? ""] ?? t.agents.waitingDetail : (session.provider === "codex" ? t.agents.codex : t.agents.provider);
  }
</script>

{#snippet row(session: AgentSession)}
  <article class:waiting={session.state === "waiting"} data-state={session.state}>
    <div class="heading"><strong>{session.project}</strong><span class="state">{t.agents.states[session.state]}</span></div>
    <p>{session.name ?? t.agents.unnamed}</p>
    <div class="meta"><span>{detail(session)}</span><span class="num">{elapsed(session)}</span></div>
  </article>
{/snippet}
<section aria-label={t.modules.agents}>
  {#if data.codexWaitingUnavailable}<p class="muted">{t.agents.codexWaitingUnavailable}</p>{/if}
  {#if data.failed}<p role="status" class="muted">{t.agents.failure}</p>{/if}
  {#if data.sessions.length === 0}<p class="muted">{t.agents.empty}</p>{/if}
  {#each data.sessions as session (session.id)}{@render row(session)}{/each}
  {#if data.closed.length}
    <details><summary>{t.agents.closed} ({data.closed.length})</summary>
      {#each data.closed as session (session.id)}{@render row(session)}{/each}
    </details>
  {/if}
</section>
<style>
  section { padding: 6px 16px 16px; }
  article { padding: 12px; margin: 8px 0; background: var(--surface); border: 1px solid var(--line); border-radius: 8px; }
  article.waiting { border-color: var(--accent); background: var(--accent-dim); }
  .heading, .meta { display: flex; justify-content: space-between; gap: 8px; }
  strong, p { overflow-wrap: anywhere; }
  p { margin: 6px 0; }
  .meta, .state, summary { font-size: 11px; }
  .meta, .muted, summary { color: var(--muted); }
  [data-state="working"] .state { color: var(--warn); }
  [data-state="waiting"] .state { color: var(--accent); }
  [data-state="done"] .state { color: var(--ok); }
  [data-state="failed"] .state { color: var(--bad); }
  summary { cursor: pointer; padding: 10px 0; }
</style>
