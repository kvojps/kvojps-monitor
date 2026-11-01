<script lang="ts">
  /** Error shown right below the control that failed; the control points to it by `id`, when given. */
  let { id, message }: { id?: string; message: string } = $props();
</script>

<p {id} class="field-error" role="alert">{message}</p>

<style>
  .field-error { margin: 4px 0 8px; color: var(--bad); }
</style>
