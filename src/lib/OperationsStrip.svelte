<script lang="ts">
  import { onDestroy, onMount, tick, untrack } from "svelte";
  import { MODULES, type Module, type Snapshot, type Operation } from "./core";
  import { t } from "./strings";

  interface Clock {
    after(delay: number, callback: () => void): () => void;
  }
  let { snapshots, ready, clock = { after: (delay, callback) => {
    const timer = setTimeout(callback, delay);
    return () => clearTimeout(timer);
  } } }: { snapshots: Partial<Record<Module, Snapshot>>; ready: boolean; clock?: Clock } = $props();
  type Entry = { module: Module; operation: Operation; text: string; priority: number };
  let entries = $state<Record<string, Entry | undefined>>({});
  let announcement = $state("");
  let capacity = $state(3);
  let host: HTMLElement;
  let initialized = false;
  const seen = new Map<string, string>();
  const timers = new Map<string, () => void>();
  const pendingAnnouncements = new Map<string, string>();
  let announcing = false;
  let disposed = false;

  async function announce() {
    if (announcing) return;
    announcing = true;
    // A new operation may have exactly the same result text as the previous one.
    announcement = "";
    await tick();
    if (!disposed) announcement = [...pendingAnnouncements.values()].join(" · ");
    pendingAnnouncements.clear();
    announcing = false;
  }
  onDestroy(() => { disposed = true; timers.forEach(cancel => cancel()); });

  // Only the closed operational vocabulary reaches the strip; source payload content is never rendered.
  function describe(module: Module, operation: Operation, provider?: "codex" | "claude"): Entry | null {
    const state = operation.state;
    if (state === "running" && operation.origin === "automatic") return null;
    if (state === "succeeded" && operation.origin !== "user") return null;
    const messages = t.operations.messages;
    if (!Object.hasOwn(messages, state)) return null;
    const text = state === "running" && operation.origin === "resume" ? t.operations.resume : messages[state];
    const priority = state === "failed" ? 0 : state === "paused" ? 1 : state === "attention" || state === "unavailable" ? 2 : state === "running" ? 3 : 4;
    return { module, operation, text: `${provider ? t.operations.providers[provider] : t.modules[module]} · ${text}`, priority };
  }

  $effect(() => {
    if (!ready) return;
    const current = MODULES.flatMap<{ module: Module; source: string; provider?: "codex" | "claude"; operation?: Operation }>(module => {
      const data = snapshots[module]?.data as { operation?: Operation; codex?: { operation?: Operation }; claude?: { operation?: Operation } } | undefined;
      if (module === "ai" && (data?.codex || data?.claude)) {
        return (["codex", "claude"] as const).map(provider => ({ module, source: `ai:${provider}`, provider, operation: data[provider]?.operation }));
      }
      return [{ module, source: module, provider: undefined, operation: data?.operation }];
    });
    untrack(() => {
      for (const { module, source, provider, operation } of current) {
        if (!operation) {
          timers.get(source)?.();
          timers.delete(source);
          entries[source] = undefined;
          continue;
        }
        const key = `${operation.id}:${operation.state}`;
        if (seen.get(source) === key) continue;
        seen.set(source, key);
        if (!initialized) continue;
        timers.get(source)?.();
        timers.delete(source);
        const entry = describe(module, operation, provider);
        entries[source] = entry ?? undefined;
        if (!entry) continue;
        if (operation.state !== "running") {
          timers.set(source, clock.after(4000, () => {
            entries[source] = undefined;
            timers.delete(source);
          }));
          if (operation.origin === "user" || ["failed", "recovered", "paused", "attention", "unavailable"].includes(operation.state)) {
            pendingAnnouncements.set(source, entry.text);
          }
        }
      }
      initialized = true;
      if (pendingAnnouncements.size) void announce();
    });
  });
  const ordered = $derived(MODULES.flatMap(module => {
    const sources = Object.values(entries).filter((entry): entry is Entry => entry?.module === module).sort((a, b) => a.priority - b.priority);
    return sources.length ? [{ text: sources.map(entry => entry.text).join("; "), priority: sources[0].priority }] : [];
  }).sort((a, b) => a.priority - b.priority));
  const visible = $derived(ordered.slice(0, capacity));
  onMount(() => {
    if (typeof ResizeObserver !== "function") return;
    const observer = new ResizeObserver(() => {
      // Reserve room for the overflow count; larger text naturally reduces the number of summaries.
      const size = parseFloat(getComputedStyle(host).fontSize) || 12;
      capacity = Math.max(1, Math.min(3, Math.floor((host.clientWidth - 9 * size) / (24 * size))));
    });
    observer.observe(host);
    return () => observer.disconnect();
  });
</script>

<div class="operations" aria-label="Faixa de bordo" bind:this={host}>
  <div class="summaries">
    {#each visible as entry}<span>{entry.text}</span>{:else}<span>{t.operations.rest}</span>{/each}
  </div>
  {#if ordered.length > visible.length}<span class="overflow">+{ordered.length - visible.length} Módulos</span>{/if}
</div>
<div class="sr-only" role="status" aria-live="polite" aria-atomic="true">{announcement}</div>

<style>
  .operations { display: flex; align-items: center; gap: 12px; flex: none; min-width: 0; min-height: 40px; margin: 6px -18px -10px; padding: 8px 18px; border-top: 1px solid var(--accent); background: var(--surface); color: var(--text); font: 12px monospace; }
  .summaries { display: flex; flex: 1; gap: 16px; min-width: 0; }
  .summaries span { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .overflow { flex: none; }
  .sr-only { position: absolute; width: 1px; height: 1px; padding: 0; overflow: hidden; clip-path: inset(50%); white-space: nowrap; }
  @media (max-width: 650px) { .operations { margin: 6px -4px -4px; padding: 8px; } }
</style>
