<script lang="ts" module>
  /** Why a whole reading is missing: neutral while none exists yet, amber when something needs the user. */
  export interface Absence {
    text: string;
    tone: "neutral" | "warn";
    /** Second line, e.g. the next step of a connection in progress. */
    detail?: string;
    action?: { label: string; run: () => void; disabled?: boolean };
  }
</script>

<script lang="ts">
  /**
   * The body of a Module, or of one source in the "IA e Agentes" card, when the
   * whole reading is missing: one area in the place of the instruments, so the
   * card keeps the composition of prototype B.
   */
  let { absence }: { absence: Absence } = $props();
</script>

<div class="area" data-tone={absence.tone}>
  <span class="icon" aria-hidden="true">{absence.tone === "warn" ? "⚠" : "◌"}</span>
  <p>{absence.text}</p>
  {#if absence.detail}<p class="detail">{absence.detail}</p>{/if}
  {#if absence.action}<button disabled={absence.action.disabled} onclick={absence.action.run}>{absence.action.label}</button>{/if}
</div>

<style>
  .area { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 8px; text-align: center; border: 1px dashed var(--line); padding: 14px; min-height: var(--absent-min, 120px); overflow-wrap: anywhere; }
  .area[data-tone="warn"] { border-color: color-mix(in srgb, var(--warn) 55%, transparent); }
  .icon { font-size: 26px; line-height: 1; color: var(--muted); }
  p { color: var(--muted); max-width: 32ch; }
  .detail { font-size: .88em; }
  .area[data-tone="warn"] :is(.icon, p:not(.detail)) { color: var(--warn); }
  button { color: var(--accent); border: 1px solid var(--control-line); padding: 4px 12px; border-radius: 14px; }
  button:hover:not(:disabled) { background: var(--surface-2); }
  button:disabled { opacity: .45; cursor: default; }
  button:focus-visible { outline: 2px solid var(--accent); outline-offset: 3px; }
</style>
