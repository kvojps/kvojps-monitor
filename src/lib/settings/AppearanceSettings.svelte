<script lang="ts">
  import type { Appearance, Theme } from "../core";
  import { t } from "../strings";
  import type { AppearanceState } from "./appearance.svelte";
  import FieldError from "./FieldError.svelte";
  import Switch from "./Switch.svelte";

  type Control = keyof Appearance;

  let { appearance, save }: { appearance: AppearanceState; save: (a: Appearance) => Promise<void> } = $props();
  const themes: Theme[] = ["dark", "light", "system"];
  const uid = $props.id();

  let pending = $state(false);
  /** The control whose change failed to save; its error shows right below it. */
  let failed = $state<Control | null>(null);

  async function update(change: Partial<Appearance>, control: Control) {
    const previous = appearance.preference;
    appearance.preference = { ...previous, ...change };
    pending = true;
    failed = null;
    try {
      await save(appearance.preference);
    } catch {
      appearance.preference = previous;
      failed = control;
    } finally {
      pending = false;
    }
  }
</script>

<section aria-labelledby="{uid}-title">
  <h3 id="{uid}-title">{t.settings.appearance.title}</h3>
  <!-- Radios, not <select>: its GTK popup takes focus and the panel closes under it. -->
  <fieldset disabled={pending} aria-describedby={failed === "theme" ? `${uid}-error` : undefined}>
    <legend>{t.settings.appearance.theme}</legend>
    <div class="themes">
      {#each themes as theme (theme)}
        <label class="choice">
          <input type="radio" name="theme" value={theme} checked={appearance.preference.theme === theme}
            onchange={() => update({ theme }, "theme")} />
          {t.settings.appearance[theme]}
        </label>
      {/each}
    </div>
  </fieldset>
  {#if failed === "theme"}<FieldError id="{uid}-error" message={t.settings.appearance.saveFailed} />{/if}
  <Switch label={t.settings.appearance.reduceMotion} checked={appearance.preference.reduceMotion} disabled={pending}
    hint={appearance.systemReduced ? t.settings.appearance.systemReduced : undefined}
    error={failed === "reduceMotion" ? t.settings.appearance.saveFailed : undefined}
    onchange={(reduceMotion) => update({ reduceMotion }, "reduceMotion")} />
</section>

<style>
  fieldset { margin-bottom: 12px; border: 0; }
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
  input { margin: 0; }
</style>
