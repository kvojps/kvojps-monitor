<script lang="ts">
  import type { AgentsData, AgentNotificationPreferences } from "../core";
  import { t } from "../strings";
  let { data, save }: { data?: AgentsData; save: (preferences: AgentNotificationPreferences) => void } = $props();
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
