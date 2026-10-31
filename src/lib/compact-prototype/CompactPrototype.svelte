<script lang="ts">
  // Three compositions on the same prototype surface, selected by ?variant=A|B|C.
  import { onMount } from "svelte";
  import Icon from "../Icon.svelte";
  import ai from "../ai/fixtures/claude-usage.json";
  import agents from "../agents/fixtures/states.json";
  import github from "../github/fixtures/full.json";
  import trello from "../trello/fixtures/full.json";
  import networkSnapshot from "../network/fixtures/full.json";
  import systemSnapshot from "../system/fixtures/notebook.json";

  const net = networkSnapshot.data;
  const system = systemSnapshot.data;
  const keys = ["A", "B", "C"];
  const names = ["Ponte horizontal", "Núcleo lateral", "Faixa de instrumentos"];
  const query = new URLSearchParams(location.search);
  let variant = $state(keys.includes(query.get("variant") ?? "") ? query.get("variant")! : "A");
  let light = $state(query.get("theme") === "light");
  let reduced = $state(query.get("motion") === "reduced");
  let enlarged = $state(false);
  let longText = $state(false);
  let feedback = $state("");
  let dialog: HTMLDialogElement;
  let details = $state("");
  let width = $state(1100);
  let height = $state(700);
  const states = [
    { key: "working", name: "Em andamento", color: "var(--accent)" },
    { key: "waiting", name: "Aguardando você", color: "var(--warn)" },
    { key: "done", name: "Turno concluído", color: "var(--ok)" },
    { key: "failed", name: "Com falha", color: "var(--bad)" },
    { key: "closed", name: "Encerrada", color: "var(--band-lavender-text)" },
  ];
  const sessions = [...agents.sessions, ...agents.closed];
  const count = (key: string) => sessions.filter(s => s.state === key).length;
  const segmentTotal = sessions.length;
  const waiting = count("waiting");
  const time = (at: number) => new Date(at).toLocaleTimeString("pt-BR", { hour: "2-digit", minute: "2-digit", timeZone: "America/Recife" });
  const rate = (value: number) => `${(value / 1024).toLocaleString("pt-BR", { maximumFractionDigits: 1 })} KB/s`;
  const percent = (used: number, total: number) => Math.round(used / total * 100);
  const weeks = github.calendar.contributionCalendar.weeks.slice(-8);
  const days = weeks.flatMap(w => w.contributionDays).slice(-14);
  const maxDay = Math.max(...days.map(d => d.contributionCount), 1);
  const maxTraffic = Math.max(...net.history.flatMap(s => [s.download, s.upload]), 1);
  function path(key: "download" | "upload") {
    return net.history.map((s, i) => `${i ? "L" : "M"}${i / Math.max(net.history.length - 1, 1) * 300},${66 - s[key] / maxTraffic * 60}`).join(" ");
  }
  function switchVariant(direction: number) {
    variant = keys[(keys.indexOf(variant) + direction + keys.length) % keys.length];
    const url = new URL(location.href);
    url.searchParams.set("variant", variant);
    history.replaceState(null, "", url);
  }
  function onKey(event: KeyboardEvent) {
    if (dialog?.open || event.target instanceof HTMLInputElement || event.target instanceof HTMLSelectElement || (event.target instanceof HTMLElement && event.target.isContentEditable)) return;
    if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
      event.preventDefault(); switchVariant(event.key === "ArrowRight" ? 1 : -1);
    }
  }
  function openDetails(name: string) { details = name; dialog.showModal(); }
  $effect(() => {
    document.documentElement.dataset.theme = light ? "light" : "dark";
    document.documentElement.dataset.motion = reduced ? "reduced" : "full";
  });
  onMount(() => {
    const sync = () => { document.documentElement.dataset.decoration = document.hidden ? "paused" : "running"; };
    sync(); document.addEventListener("visibilitychange", sync);
    return () => document.removeEventListener("visibilitychange", sync);
  });
</script>

<svelte:window bind:innerWidth={width} bind:innerHeight={height} onkeydown={onKey}/>

{#snippet action(label: string, symbol: string, run: () => void)}
  <button class="icon" aria-label={label} onclick={run}><span aria-hidden="true">{symbol}</span><span class="tip">{label}</span></button>
{/snippet}

{#snippet heading(name: string, refresh = false, configure = false)}
  <header><h2>{name}</h2><span class="actions">
    {#if refresh}{@render action(`Atualizar ${name}`, "↻", () => feedback = `${name}: atualização simulada; fixtures mantidas.`)}{/if}
    {#if configure}{@render action(`Configurar ${name}`, "⚙", () => openDetails(`Configurações — ${name}`))}{/if}
  </span></header>
{/snippet}

{#snippet meter(label: string, value: number, note: string)}
  <div class="meter"><div class="meter-label"><span>{label}</span><b class="num">{value}%</b></div><div class="track"><i style:width={`${value}%`}></i></div><small>{note}</small></div>
{/snippet}

{#snippet ring(label: string, value: number, caption: string, tone = "var(--accent)")}
  <div class="instrument" style:--instrument={tone}>
    <div class="dial"><svg viewBox="0 0 120 120" aria-hidden="true"><circle class="orbit" cx="60" cy="60" r="55"/><circle class="base" cx="60" cy="60" r="47"/><circle class="arc" cx="60" cy="60" r="47" pathLength="100" stroke-dasharray={`${value} 100`} transform="rotate(-90 60 60)"/></svg><span class="reading num">{value}<small>%</small></span></div>
    <h3>{label}</h3><small>{caption}</small>
  </div>
{/snippet}

<main class="prototype" class:enlarged class:long-text={longText} data-variant={variant}>
  <div class="bridge" aria-hidden="true"><span></span><span></span><span></span></div>
  <div class="utility"><small>Demonstração · leituras independentes</small><button class="settings" onclick={() => openDetails("Configurações")}><Icon name="settings" size={16}/> Configurações</button></div>
  <div class="composition">
    <section class="intelligence" aria-label="IA e Agentes">
      {@render heading("IA e Agentes", true, true)}
      <div class="instruments">
        {#each [{name:"Codex", bucket:ai.codex.reading.buckets[0], tone:"var(--accent)"}, {name:"Claude", bucket:ai.claude.reading.buckets[0], tone:"var(--band-lavender-text)"}] as provider}
          <div class="provider">
            {@render ring(provider.name, provider.bucket.primary!.usedPercent, `5 h · reinicia ${time(provider.bucket.primary!.resetsAt)}`, provider.tone)}
            {@render meter("Semana", provider.bucket.secondary.usedPercent, `Reinício ${new Date(provider.bucket.secondary.resetsAt).toLocaleDateString("pt-BR")}`)}
            <small class="source">Leitura {time(provider.name === "Codex" ? ai.codex.reading.readAt : ai.claude.reading.readAt)}</small>
          </div>
        {/each}
        <div class="agent-instrument">
          <button class="session-dial" onclick={() => openDetails("Sessões de agente")} aria-label={`${count("working")} sessões em andamento; abrir todas as sessões`}>
            <svg viewBox="0 0 120 120" aria-hidden="true"><circle class="orbit" cx="60" cy="60" r="55"/>
              {#each states as state, index}
                <circle cx="60" cy="60" r="47" fill="none" stroke={state.color} stroke-width="8" pathLength="100" stroke-dasharray={`${Math.max(0,count(state.key)/segmentTotal*100-2)} 100`} stroke-dashoffset={-states.slice(0,index).reduce((sum,s)=>sum+count(s.key)/segmentTotal*100,0)} transform="rotate(-90 60 60)"/>
              {/each}
            </svg><span class="reading num">{count("working")}</span>
          </button><h3>Trabalhando</h3><small>{sessions.length} sessões · inclui encerradas</small>
        </div>
        <div class="session-summary">
          {#each states as state}
            <button class="legend" style:--state-color={state.color} onclick={() => openDetails("Sessões de agente")}>
              <i></i><span>{state.name}</span><b class="num">{count(state.key)}</b>
              <span class="session-tip">{sessions.filter(s=>s.state===state.key).map(s=>`${s.project} · ${s.name ?? "Sem nome"}`).join(" / ") || "Nenhuma sessão"}</span>
            </button>
          {/each}
        </div>
      </div>
      <button class="waiting" onclick={() => openDetails("Sessões de agente")}>⚠ {waiting} aguardando você <span>Ver sessões ↗</span></button>
    </section>

    <section class="github" aria-label="GitHub">
      {@render heading("GitHub", true, true)}
      <div class="github-top"><div><strong class="num">{github.calendar.contributionCalendar.totalContributions.toLocaleString("pt-BR")}</strong><small>contribuições no ano</small></div>
        <div><div class="heatmap" role="img" aria-label="Contribuições das últimas oito semanas">
          {#each weeks as week}<div>{#each week.contributionDays as day}<i title={`${day.date}: ${day.contributionCount}`} style:opacity={day.contributionCount ? Math.max(.3,day.contributionCount/63) : .1}></i>{/each}</div>{/each}
        </div><small>Últimas 8 semanas</small></div>
      </div>
      <div class="activity" role="img" aria-label={`Últimos 14 dias: ${days.map(d=>`${d.date}: ${d.contributionCount}`).join("; ")}`}>
        {#each days as day}<i style:height={`${Math.max(2,day.contributionCount/maxDay*100)}%`}></i>{/each}
      </div><small>Atividade · últimos 14 dias</small>
      <div class="featured"><small>Repo em destaque · {github.featured.commits} commits / 7 dias</small><b>{longText ? "exemplo/repositorio-com-nome-muito-longo-para-verificacao" : github.featured.name}</b><span>{longText ? "Atualiza documentação de configuração e orientações para contribuidores do projeto" : github.featured.title}</span></div>
    </section>

    <section class="system" aria-label="Sistema">
      {@render heading("Sistema")}
      <div class="system-top"><div><strong class="num">{system.cpu.usage}<small>%</small></strong><small>CPU</small></div><div><strong class="num temperature">{system.cpu.temperature.celsius.toLocaleString("pt-BR")}<small>°C</small></strong><small>Temperatura</small></div></div>
      {@render meter("RAM", percent(system.ram.used,system.ram.total), `${(system.ram.used/1073741824).toFixed(1)} / ${(system.ram.total/1073741824).toFixed(1)} GB`)}
      {@render meter("Swap", percent(system.swap.used,system.swap.total), "")}
      {@render meter("Disco /", percent(system.disks[0].used,system.disks[0].total), "")}
      <p class="battery">▰ Bateria {system.battery.percentage}% · Carregada</p>
    </section>

    <section class="trello" aria-label="Trello">
      {@render heading("Trello", true, true)}
      <div class="board-summary"><b>{trello.board.name}</b><span>{trello.lists.reduce((n,l)=>n+l.cards.length,0)} cards · {trello.lists.length} listas</span></div>
      <div class="list-counts">{#each trello.lists as list}<div><span>{list.name}</span><b class="num">{list.cards.length}</b></div>{/each}</div>
      <button class="card-preview" onclick={() => openDetails("Quadro Trello")}><small>Primeiro card · {trello.lists[0].name}</small><b>{longText ? "Reorganizar tarefas e documentação para próxima entrega do projeto" : trello.lists[0].cards[0].name}</b><span class="labels"><i></i><i></i></span></button>
      <button class="detail-button" onclick={() => openDetails("Quadro Trello")}>Abrir quadro completo ↗</button>
    </section>

    <section class="network" aria-label="Rede">
      {@render heading("Rede")}
      <div class="network-status"><span>Wi-Fi</span><span>Qualidade: Regular</span><span>Segurança: Sem alertas</span></div>
      <div class="network-top"><div><strong class="num">{net.internet.latency}<small>ms</small></strong><small>Internet</small></div>
        <button class="speed" onclick={() => feedback="Teste de velocidade simulado; nenhuma conexão externa."}><b class="num">↓ {net.speed.download} · ↑ {net.speed.upload}</b><span>Mbps · testar ↻</span><small>Fixture · {time(net.speed.measuredAt)}</small></button>
      </div>
      <div class="metrics"><span>Jitter <b>{net.internet.jitter} ms</b></span><span>Perda <b>{net.internet.loss}%</b></span><span>Gateway <b>{net.gateway.latency} ms</b></span></div>
      <small>Gateway · jitter {net.gateway.jitter} ms · perda {net.gateway.loss}%</small>
      <svg class="traffic" viewBox="0 0 300 72" preserveAspectRatio="none" role="img" aria-label={`Tráfego: download ${rate(net.traffic.download)}, upload ${rate(net.traffic.upload)}`}><path d={path("download")} fill="none" stroke="var(--accent)" stroke-width="2"/><path d={path("upload")} fill="none" stroke="var(--chart-secondary)" stroke-width="2" stroke-dasharray="5 3"/></svg>
      <div class="traffic-values num"><span>↓ {rate(net.traffic.download)}</span><span>↑ {rate(net.traffic.upload)}</span></div>
      <button class="detail-button" onclick={() => openDetails("Aplicativos de rede")}>{net.processTraffic.apps.length} aplicativos + Outros ↗</button>
    </section>
  </div>
  <footer><span>Fixtures · {width} × {height} · {variant} · {enlarged ? "texto 125%" : "texto 100%"} · {longText ? "nomes longos" : "nomes originais"}</span><span role="status">{feedback}</span></footer>
</main>

{#if import.meta.env.DEV}
  <aside class="switcher" aria-label="Controles do protótipo">
    <button onclick={() => switchVariant(-1)} aria-label="Composição anterior">←</button><b>{variant} · {names[keys.indexOf(variant)]}</b><button onclick={() => switchVariant(1)} aria-label="Próxima composição">→</button>
    <label><input type="checkbox" bind:checked={light}/> Claro</label><label><input type="checkbox" bind:checked={reduced}/> Reduzir movimento</label><label><input type="checkbox" bind:checked={enlarged}/> Texto 125%</label><label><input type="checkbox" bind:checked={longText}/> Nomes longos</label>
  </aside>
{/if}

<dialog bind:this={dialog}>
  <header><h2>{details}</h2><button class="detail-button" onclick={() => dialog.close()}>Fechar ×</button></header>
  <p class="source">Detalhe ilustrativo; localização e comportamento serão validados na próxima decisão.</p>
  {#if details === "Sessões de agente"}
    {#each sessions as session}<article><b>{session.project} · {session.name ?? "Sem nome"}</b><p>{session.provider} · {states.find(s=>s.key===session.state)?.name}</p><small>{session.waitingFor ?? "Sem intervenção pendente"} · início {time(session.startedAt)}</small></article>{/each}
  {:else if details === "Quadro Trello"}
    {#each trello.lists as list}<article><h3>{list.name}</h3>{#each list.cards as card}<p>{card.name} · {card.labels.join(", ") || "Sem etiquetas"} · {card.due ?? "Sem vencimento"}</p>{/each}</article>{/each}
  {:else if details === "Aplicativos de rede"}
    {#each [...net.processTraffic.apps,{name:"Outros",...net.processTraffic.others}] as app}<article><b>{app.name}</b><p class="num">↓ {rate(app.download)} · ↑ {rate(app.upload)}</p></article>{/each}
    <small>Outros inclui UDP/QUIC, processos do sistema e conexões curtas.</small>
  {:else}<p>Configurações simuladas. Este protótipo avalia somente a composição dos resumos.</p>{/if}
</dialog>

<style>
  :global(body) { overflow: auto; }
  :global(button:focus-visible), :global(input:focus-visible) { outline: 2px solid var(--accent); outline-offset: 3px; }
  .prototype { padding: 8px 18px 52px; min-height: 100vh; font-size: 13px; }
  .prototype.enlarged { font-size: 16.25px; }
  .bridge { display: flex; gap: 7px; height: 6px; margin-bottom: 5px; }
  .bridge span { background: var(--band-lavender); flex: 2; border-radius: 15px 0 0 0; }
  .bridge span:nth-child(2) { background: var(--band-cyan); flex: 5; }
  .bridge span:last-child { background: var(--band-amber); flex: 1; border-radius: 0 12px 12px 0; }
  .utility { display: flex; justify-content: space-between; align-items: center; margin-bottom: 6px; min-height: 24px; }
  .settings { display: flex; align-items: center; gap: 7px; }
  .composition { display: grid; gap: 12px; align-items: stretch; }
  [data-variant="A"] .composition { grid-template-columns: 1fr 1fr 1.15fr; grid-template-areas: "intelligence intelligence github" "system trello network"; }
  [data-variant="B"] .composition { grid-template-columns: 1.2fr 1fr 1fr; grid-template-areas: "intelligence github network" "intelligence trello system"; }
  [data-variant="C"] .composition { grid-template-columns: 1fr 1fr 1.1fr; grid-template-areas: "intelligence intelligence intelligence" "github trello network" "system system system"; }
  section { min-width: 0; padding: 8px 14px 8px; background: var(--module-bg); border-top: 4px solid var(--band-cyan); position: relative; }
  section::after { content: ""; position: absolute; inset: 0; border-right: 1px solid var(--line); pointer-events: none; animation: border-glow 7s ease-in-out infinite; }
  @keyframes border-glow { 50% { border-color: var(--accent); } }
  :global([data-decoration="paused"]) section::after, :global([data-decoration="paused"]) .orbit { animation-play-state: paused; }
  .intelligence { grid-area: intelligence; border-color: var(--band-lavender); }
  .github { grid-area: github; border-color: var(--band-steel); }
  .system { grid-area: system; border-color: var(--band-lavender); }
  .trello { grid-area: trello; border-color: var(--band-amber); }
  .network { grid-area: network; border-color: var(--band-blue); }
  header { display: flex; align-items: center; gap: 10px; justify-content: space-between; min-height: 28px; margin-bottom: 6px; }
  h2 { font-size: 1.25em; font-weight: 500; text-transform: uppercase; letter-spacing: 1px; }
  h3 { font-size: 1em; font-weight: 500; }
  small, .source { font-size: .88em; color: var(--muted); }
  .actions { display: flex; gap: 5px; }
  .icon { position: relative; width: 28px; height: 28px; border: 1px solid var(--line); border-radius: 50%; font-size: 19px; }
  .tip, .session-tip { display: none; position: absolute; z-index: 4; background: var(--surface-2); color: var(--text); padding: 9px; border: 1px solid var(--control-line); box-shadow: 0 4px 20px #0006; font-size: 12px; font-weight: 400; }
  .tip { right: 0; top: 100%; min-width: 150px; }
  .icon:hover .tip, .icon:focus .tip, .legend:hover .session-tip, .legend:focus .session-tip { display: block; }
  .instruments { display: grid; grid-template-columns: 1fr 1fr 1fr 1.3fr; gap: 10px; align-items: center; }
  .instrument, .agent-instrument { text-align: center; min-width: 0; }
  .dial, .session-dial { position: relative; width: 94px; height: 94px; margin: auto; display: block; }
  .dial svg, .session-dial svg { width: 100%; height: 100%; }
  .reading { position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-size: 2.55em; gap: 2px; }
  .reading small { font-size: .4em; color: var(--muted); }
  .base { fill: none; stroke: var(--line); stroke-width: 8; }
  .arc { fill: none; stroke: var(--instrument); stroke-width: 8; }
  .orbit { fill: none; stroke: var(--control-line); stroke-width: 1; stroke-dasharray: 24 12 6 30; transform-origin: center; animation: orbit 48s linear infinite; }
  @keyframes orbit { to { transform: rotate(360deg); } }
  .provider .meter { margin-top: 6px; }
  .provider>.source { display: block; margin-top: 4px; }
  .meter { margin-top: 8px; }
  .meter-label { display: flex; justify-content: space-between; gap: 8px; margin-bottom: 4px; }
  .track { height: 5px; background: var(--line); border-radius: 5px; }
  .track i { display: block; height: 100%; background: var(--accent); border-radius: inherit; }
  .meter>small:empty { display: none; }
  .legend { position: relative; width: 100%; display: flex; gap: 7px; align-items: center; padding: 4px 0; text-align: left; font-size: .92em; }
  .legend i { width: 6px; height: 6px; border-radius: 50%; background: var(--state-color); flex-shrink: 0; }
  .legend b { margin-left: auto; }
  .session-tip { top: 100%; right: 0; width: 240px; overflow-wrap: anywhere; }
  .waiting { color: var(--warn); border: 1px solid var(--warn); display: flex; justify-content: space-between; gap: 12px; padding: 8px 10px; width: 100%; margin: 8px 0 4px; text-align: left; }
  .waiting span { font-size: .9em; }
  .github-top, .system-top, .network-top { display: flex; align-items: center; justify-content: space-between; gap: 12px; }
  strong { display: block; font-size: 2.8em; font-weight: 400; line-height: 1.15; }
  strong small { font-size: .35em; margin-left: 3px; }
  .temperature { font-size: 2.3em; }
  .heatmap { display: flex; gap: 3px; }
  .heatmap>div { display: flex; flex-direction: column; gap: 3px; }
  .heatmap i { width: 7px; height: 7px; background: var(--accent); }
  .github-top small { display: block; margin-top: 5px; }
  .activity { height: 26px; display: flex; align-items: flex-end; gap: 5px; margin: 6px 0 4px; }
  .activity i { flex: 1; background: var(--band-steel); border-radius: 3px 3px 0 0; }
  .featured { border-top: 1px solid var(--line); padding-top: 8px; margin-top: 9px; display: grid; gap: 4px; overflow-wrap: anywhere; }
  .featured span { font-size: .92em; }
  .battery { margin-top: 12px; color: var(--ok); }
  .board-summary { display: flex; justify-content: space-between; gap: 10px; font-size: .93em; margin: 2px 0 12px; }
  .board-summary span { color: var(--muted); }
  .list-counts { display: grid; grid-template-columns: 1fr 1fr; gap: 4px 16px; }
  .list-counts>div { display: flex; justify-content: space-between; border-bottom: 1px solid var(--line); padding-bottom: 4px; }
  .card-preview { text-align: left; display: grid; gap: 5px; background: var(--surface-2); width: 100%; margin-top: 8px; padding: 7px; border-left: 3px solid var(--band-amber); overflow-wrap: anywhere; }
  .labels { display: flex; gap: 4px; }
  .labels i { background: var(--ok); width: 20px; height: 3px; }
  .labels i:last-child { background: var(--band-blue); }
  .detail-button { color: var(--accent); border-bottom: 1px solid var(--line); padding: 7px 0; text-align: left; margin-top: 5px; }
  .network-status { display: flex; flex-wrap: wrap; gap: 4px 10px; font-size: .88em; color: var(--muted); margin-bottom: 6px; }
  .speed { text-align: right; display: grid; gap: 4px; }
  .speed span { font-size: .9em; color: var(--muted); }
  .metrics { display: flex; flex-wrap: wrap; justify-content: space-between; gap: 6px; margin: 6px 0 5px; font-size: .88em; }
  .metrics b { font-weight: 500; }
  .traffic { width: 100%; height: 40px; display: block; margin-top: 8px; background: linear-gradient(transparent 95%, var(--line) 95%); background-size: 100% 20px; }
  .traffic-values { display: flex; justify-content: space-between; font-size: .9em; margin: 5px 0 2px; }
  .traffic-values span:first-child { color: var(--accent); }
  .traffic-values span:last-child { color: var(--chart-secondary); }
  footer { display: flex; justify-content: space-between; flex-wrap: wrap; gap: 8px; font-size: 11px; color: var(--muted); padding-top: 6px; }
  .switcher { position: fixed; z-index: 8; bottom: 10px; left: 50%; transform: translateX(-50%); display: flex; align-items: center; gap: 12px; border: 1px solid var(--control-line); padding: 10px 14px; background: var(--surface-2); border-radius: 28px; box-shadow: 0 5px 25px #0008; white-space: nowrap; font-size: 12px; }
  .switcher button { padding: 4px 7px; background: var(--bg); border-radius: 50%; }
  .switcher label { display: flex; align-items: center; gap: 4px; }
  dialog { margin: auto; width: min(680px,90vw); max-height: 80vh; overflow: auto; background: var(--surface); color: var(--text); border: 1px solid var(--control-line); padding: 20px; }
  dialog::backdrop { background: #000a; }
  dialog article { margin-top: 12px; border-top: 1px solid var(--line); padding-top: 10px; overflow-wrap: anywhere; }
  dialog p { margin-top: 8px; }
  [data-variant="B"] .instruments { grid-template-columns: 1fr 1fr; gap: 18px 12px; }
  [data-variant="B"] .session-summary { align-self: center; }
  [data-variant="B"] .waiting { margin-top: 25px; flex-direction: column; gap: 5px; }
  [data-variant="C"] .intelligence { display: grid; grid-template-columns: 1fr 1fr; gap: 6px 20px; }
  [data-variant="C"] .intelligence header { grid-row: 1; grid-column: 1; margin: 0; }
  [data-variant="C"] .intelligence .waiting { grid-row: 1; grid-column: 2; margin: 0; }
  [data-variant="C"] .instruments { grid-column: 1 / -1; grid-row: 2; grid-template-columns: 1fr 1fr .8fr 1.1fr; }
  [data-variant="C"] .provider { display: grid; grid-template-columns: 105px 1fr; gap: 5px 12px; align-items: center; }
  [data-variant="C"] .provider>.source { grid-column: 1 / -1; }
  [data-variant="C"] .dial, [data-variant="C"] .session-dial { width: 92px; height: 92px; }
  [data-variant="C"] .instrument>small { display: block; }
  [data-variant="C"] .waiting { margin-top: 8px; padding: 6px 10px; }
  [data-variant="C"] .system { display: grid; grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr; gap: 14px; align-items: center; }
  [data-variant="C"] .system header { margin: 0; }
  [data-variant="C"] .system-top { gap: 10px; }
  [data-variant="C"] .system strong { font-size: 1.9em; }
  [data-variant="C"] .system .meter, [data-variant="C"] .battery { margin-top: 0; }
  @media (prefers-reduced-motion: reduce) { .orbit, section::after { animation: none; } }
  @media (max-width: 950px) {
    .prototype { padding: 10px 12px 112px; }
    [data-variant] .composition { grid-template-columns: 1fr 1fr; grid-template-areas: "intelligence intelligence" "github network" "system trello"; }
    [data-variant] .instruments { grid-template-columns: 1fr 1fr 1fr 1.2fr; }
    [data-variant="C"] .intelligence { display: block; }
    [data-variant="C"] .provider { display: block; }
    [data-variant="C"] .system { display: block; }
    .switcher { max-width: 96vw; white-space: normal; flex-wrap: wrap; justify-content: center; width: max-content; gap: 8px; }
  }
  @media (max-width: 650px) {
    [data-variant] .composition { display: flex; flex-direction: column; }
    [data-variant] .instruments { grid-template-columns: 1fr 1fr; }
  }
</style>
