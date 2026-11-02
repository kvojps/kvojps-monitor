<script>
  import { onMount } from 'svelte';
  import OperationsStrip from '/src/lib/OperationsStrip.svelte';
  import simultaneous from '/src/lib/fixtures/operations/simultaneous.json';
  let { scenario, font } = $props();
  let snapshots = $state({});
  const callbacks = [];
  const clock = { after: (_, callback) => { callbacks.push(callback); return () => {}; } };
  onMount(() => {
    window.expire = () => callbacks.forEach(callback => callback());
    snapshots = scenario === 'many' || scenario === 'long'
      ? Object.fromEntries(simultaneous.map(snapshot => [snapshot.module, snapshot]))
      : Object.fromEntries(['github', 'trello'].slice(0, scenario === 'one' ? 1 : 2).map(module => [module, { module, updatedAt: null, data: { operation: { id: 1, state: 'cancelled', origin: 'user' } } }]));
    if (scenario === 'long') snapshots.ai = { module: 'ai', updatedAt: null, data: { codex: { operation: { id: 1, state: 'failed', origin: 'user' } }, claude: { operation: { id: 1, state: 'failed', origin: 'user' } } } };
  });
</script>
<div style={`--validation-font: ${font}px`}><OperationsStrip {snapshots} ready={true} {clock} /></div>
<style>:global(.operations) { font-size: var(--validation-font) !important; }</style>
