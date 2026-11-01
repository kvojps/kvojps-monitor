<script lang="ts" module>
  export type Tone = "ok" | "warn" | "bad" | "neutral";
</script>

<script lang="ts">
  /** First line of a settings section: a dot colored by tone and the state in words, announced when it changes. */
  let { tone, text }: { tone: Tone; text: string } = $props();
</script>

<p class="status-line {tone}" role="status"><span class="dot" aria-hidden="true"></span>{text}</p>

<style>
  .status-line {
    display: flex;
    align-items: center;
    gap: 8px;
    margin: 0 0 12px;
    color: var(--text);
  }
  .dot {
    flex: none;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--muted);
  }
  .ok .dot { background: var(--ok); }
  .warn .dot { background: var(--warn); }
  .bad .dot { background: var(--bad); }
</style>
