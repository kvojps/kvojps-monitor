<script lang="ts">
  import type { NetworkData } from "../core";
  import StatusLine, { type Tone } from "../settings/StatusLine.svelte";
  import Switch from "../settings/Switch.svelte";
  import { t } from "../strings";

  let { data, setAutomaticSpeed }: {
    data?: NetworkData;
    setAutomaticSpeed: (enabled: boolean) => void;
  } = $props();

  // The core answers with the next snapshot; until then the switch shows the choice just made.
  let automatic = $derived(data?.automaticSpeed ?? false);

  const status = $derived.by((): { tone: Tone; text: string } => {
    if (!data) return { tone: "neutral", text: t.network.noReading };
    if (!data.connected) return { tone: "bad", text: t.network.disconnected };
    const link = data.wired ? t.network.wired : t.network.wireless;
    return { tone: "ok", text: data.vpn ? `${link} · ${t.network.vpn}` : link };
  });

  function change(enabled: boolean) {
    automatic = enabled;
    setAutomaticSpeed(enabled);
  }
</script>

<section aria-labelledby="settings-network-title">
  <h3 id="settings-network-title">{t.modules.network}</h3>
  <StatusLine tone={status.tone} text={status.text} />
  <Switch label={t.network.automaticSpeed} checked={automatic} disabled={!data} onchange={change} />
</section>
