<script lang="ts">
  import type { AgentsData, AgentNotificationPreferences } from "../core";
  import { t } from "../strings";
  let { data, save, setHooks }: {
    data?: AgentsData;
    save: (preferences: AgentNotificationPreferences) => void;
    setHooks?: (enabled: boolean) => Promise<void>;
  } = $props();
  let hooksEnabled = $state(false);
  let pending = $state(false);
  let error = $state(false);
  $effect(() => { hooksEnabled = data?.hooksEnabled ?? false; });
  async function changeHooks(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const enabled = input.checked;
    input.checked = hooksEnabled;
    pending = true;
    error = false;
    try {
      if (!setHooks) throw new Error("unavailable");
      await setHooks(enabled);
      hooksEnabled = enabled;
    } catch { error = true; }
    finally { pending = false; }
  }
  let preferences = $state<AgentNotificationPreferences>({ waiting: true, done: true, failed: true });
  $effect(() => { preferences = data?.notificationPreferences ?? { waiting: true, done: true, failed: true }; });
  const switches = [
    { key: "waiting", label: t.agentNotifications.waitingSetting },
    { key: "done", label: t.agentNotifications.doneSetting },
    { key: "failed", label: t.agentNotifications.failedSetting },
  ] as const;
  function change(key: keyof AgentNotificationPreferences, enabled: boolean) {
    preferences = { ...preferences, [key]: enabled };
    save(preferences);
  }
</script>
<section aria-label={t.modules.agents}>
  <h2>{t.modules.agents}</h2>
  <label><input type="checkbox" checked={hooksEnabled} disabled={pending} onchange={changeHooks} />{t.agentHooks.setting}</label>
  {#if hooksEnabled}<p>{t.agentHooks.trust}</p>{/if}
  {#if error}<p role="alert">{t.agentHooks.error}</p>{/if}
  {#each switches as item}
    <label><input type="checkbox" checked={preferences[item.key]}
      onchange={(event) => change(item.key, event.currentTarget.checked)} />{item.label}</label>
  {/each}
</section>
<style>
  section { padding: 16px; font-size: 13px; }
  h2 { font-size: 14px; margin-bottom: 12px; }
  label { display: flex; align-items: center; gap: 8px; margin: 10px 0; }
</style>
