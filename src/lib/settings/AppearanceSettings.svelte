<script lang="ts">
  import type { Appearance, Theme } from "../core";
  import { t } from "../strings";
  import type { AppearanceState } from "./appearance.svelte";

  let { appearance, save }: { appearance: AppearanceState; save: (a: Appearance) => Promise<void> } = $props();
  const themes: Theme[] = ["dark", "light", "system"];

  let pending = $state(false);
  let failed = $state(false);

  async function update(change: Partial<Appearance>) {
    const previous = appearance.preference;
    appearance.preference = { ...previous, ...change };
    pending = true;
    failed = false;
    try {
      await save(appearance.preference);
    } catch {
      appearance.preference = previous;
      failed = true;
    } finally {
      pending = false;
    }
  }
</script>

<section aria-labelledby="settings-appearance-title">
  <h3 id="settings-appearance-title">{t.settings.appearance.title}</h3>
  <!-- Radios, not <select>: its GTK popup takes focus and the panel closes under it. -->
  <fieldset disabled={pending} aria-describedby={failed ? "appearance-error" : undefined}>
    <legend>{t.settings.appearance.theme}</legend>
    <div class="themes">
      {#each themes as theme (theme)}
        <label class="choice">
          <input type="radio" name="theme" value={theme} checked={appearance.preference.theme === theme}
            onchange={() => update({ theme })} />
          {t.settings.appearance[theme]}
        </label>
      {/each}
    </div>
  </fieldset>
  <label class="switch">
    <input type="checkbox" disabled={pending} checked={appearance.preference.reduceMotion}
      aria-describedby={appearance.systemReduced ? "system-reduced" : undefined}
      onchange={(e) => update({ reduceMotion: e.currentTarget.checked })} />
    {t.settings.appearance.reduceMotion}
  </label>
  {#if appearance.systemReduced}<p id="system-reduced" class="hint">{t.settings.appearance.systemReduced}</p>{/if}
  {#if failed}<p id="appearance-error" class="error" role="alert">{t.settings.appearance.saveFailed}</p>{/if}
</section>

<style>
  fieldset { border: 0; }
  legend { margin-bottom: 8px; color: var(--muted); }
  .themes { display: flex; flex-wrap: wrap; gap: 8px; }
  .choice {
    display: flex;
    align-items: center;
    gap: 8px;
    min-height: 36px;
    padding: 6px 16px 6px 12px;
    border: 1px solid var(--line);
    border-radius: 18px;
    background: var(--surface-2);
    cursor: pointer;
  }
  .choice:has(:checked) { border-color: var(--accent); }
  .hint { color: var(--muted); }
</style>
