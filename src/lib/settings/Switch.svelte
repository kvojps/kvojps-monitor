<script lang="ts">
  import FieldError from "./FieldError.svelte";

  /**
   * Controlled switch: the box shows `checked` until the owner confirms a new value,
   * with an optional hint and an error right below it, both announced as its description.
   */
  let { label, checked, disabled = false, hint, error, onchange }: {
    label: string;
    checked: boolean;
    disabled?: boolean;
    hint?: string;
    error?: string;
    onchange: (enabled: boolean) => void;
  } = $props();

  const uid = $props.id();
  const hintId = `${uid}-hint`;
  const errorId = `${uid}-error`;
  const describedBy = $derived([hint && hintId, error && errorId].filter(Boolean).join(" ") || undefined);

  function change(event: Event & { currentTarget: HTMLInputElement }) {
    const enabled = event.currentTarget.checked;
    event.currentTarget.checked = checked;
    onchange(enabled);
  }
</script>

<label class="switch">
  <input type="checkbox" {checked} {disabled} aria-describedby={describedBy} onchange={change} />
  {label}
</label>
{#if hint}<p id={hintId} class="hint">{hint}</p>{/if}
{#if error}<FieldError id={errorId} message={error} />{/if}

<style>
  .switch {
    display: flex;
    align-items: center;
    gap: 10px;
    min-height: 32px;
    margin: 6px 0;
    color: var(--text);
    cursor: pointer;
  }
  .switch:has(:disabled) { cursor: default; opacity: 0.6; }
  input { width: 18px; height: 18px; margin: 0; accent-color: var(--accent); }
  input:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .hint { margin: 0 0 8px; color: var(--muted); }
</style>
