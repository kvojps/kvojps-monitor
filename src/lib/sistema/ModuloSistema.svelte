<script lang="ts">
  import type { DadosSistema } from "../nucleo";
  import { preencher, t } from "../strings";
  import { bytes, duracao, par } from "./formato";

  let { dados }: { dados: DadosSistema } = $props();

  /** O mini‑gráfico tem lugar para ~60 s e cresce da esquerda. */
  const PONTOS = 30;
  const ALTURA = 34;

  const linha = $derived(
    dados.cpu.historico
      .map((v, i) => `${((i / (PONTOS - 1)) * 100).toFixed(1)},${(ALTURA - (v / 100) * (ALTURA - 2)).toFixed(1)}`)
      .join(" "),
  );

  const pct = (usado: number, total: number) => (total > 0 ? Math.round((usado / total) * 100) : 0);

  const bateria = $derived(dados.bateria);
  const detalheBateria = $derived.by(() => {
    if (!bateria) return "";
    const estado = t.sistema.estadosBateria[bateria.estado];
    const s = bateria.segundosRestantes;
    if (s === null) return estado;
    const modelo = bateria.estado === "carregando" ? t.sistema.paraCarregar : t.sistema.restantes;
    return `${estado} · ${preencher(modelo, { tempo: duracao(s) })}`;
  });
</script>

<div class="sistema">
  <section aria-label={t.sistema.cpu}>
    <div class="linha">
      <span class="rotulo">{t.sistema.cpu}</span>
      <span class="num">{dados.cpu.uso === null ? t.sistema.semValor : `${Math.round(dados.cpu.uso)}%`}</span>
    </div>
    <svg class="grafico" role="img" aria-label={t.sistema.graficoCpu} viewBox="0 0 100 {ALTURA}" preserveAspectRatio="none">
      {#if dados.cpu.historico.length > 1}
        <polyline class="area" points="0,{ALTURA} {linha} {(((dados.cpu.historico.length - 1) / (PONTOS - 1)) * 100).toFixed(1)},{ALTURA}" />
        <polyline class="traco" points={linha} />
      {/if}
    </svg>
    {#if dados.cpu.temperatura}
      <div class="linha secundaria">
        <span>{t.sistema.temperatura}</span>
        <span class="num" class:atencao={dados.cpu.temperatura.atencao}>{Math.round(dados.cpu.temperatura.celsius)} °C</span>
      </div>
    {/if}
  </section>

  <section aria-label={t.sistema.ram}>
    <div class="linha">
      <span class="rotulo">{t.sistema.ram}</span>
      <span class="num">{par(dados.ram.usada, dados.ram.total)}</span>
    </div>
    <div class="barra" role="meter" aria-label={t.sistema.ram} aria-valuemin={0} aria-valuemax={100} aria-valuenow={pct(dados.ram.usada, dados.ram.total)}>
      <i style:width="{pct(dados.ram.usada, dados.ram.total)}%"></i>
    </div>
    {#if dados.swap}
      <div class="linha secundaria">
        <span>{t.sistema.swap}</span>
        <span class="num">{par(dados.swap.usada, dados.swap.total)}</span>
      </div>
    {/if}
  </section>

  <section aria-label={t.sistema.discos}>
    <span class="rotulo">{t.sistema.discos}</span>
    {#each dados.discos as disco (disco.ponto)}
      <div class="disco">
        <div class="linha">
          <span class="ponto num">{disco.ponto}</span>
          <span class="num">{par(disco.usado, disco.total)} · {preencher(t.sistema.livres, { valor: bytes(disco.livre) })}</span>
        </div>
        <div
          class="barra"
          class:atencao={disco.atencao}
          role="meter"
          aria-label={disco.ponto}
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={pct(disco.usado, disco.usado + disco.livre)}
        >
          <i style:width="{pct(disco.usado, disco.usado + disco.livre)}%"></i>
        </div>
      </div>
    {/each}
  </section>

  {#if bateria}
    <section aria-label={t.sistema.bateria}>
      <div class="linha">
        <span class="rotulo">{t.sistema.bateria}</span>
        <span class="num" class:atencao={bateria.atencao}>{Math.round(bateria.percentual)}%</span>
      </div>
      <div
        class="barra"
        class:atencao={bateria.atencao}
        role="meter"
        aria-label={t.sistema.bateria}
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={Math.round(bateria.percentual)}
      >
        <i style:width="{bateria.percentual}%"></i>
      </div>
      <div class="linha secundaria"><span>{detalheBateria}</span></div>
    </section>
  {/if}
</div>

<style>
  .sistema {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 4px 16px 16px;
    min-height: 0;
    overflow-y: auto;
  }

  section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .linha {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 8px;
  }

  .rotulo {
    color: var(--muted);
  }

  .secundaria {
    font-size: 12px;
    color: var(--muted);
  }

  .disco {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .disco + .disco {
    margin-top: 6px;
  }

  .ponto {
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .num {
    font-size: 12px;
  }

  .atencao {
    color: var(--warn);
  }

  .barra {
    height: 4px;
    border-radius: 2px;
    background: var(--surface-2);
    overflow: hidden;
  }

  .barra > i {
    display: block;
    height: 100%;
    background: var(--accent);
    border-radius: 2px;
  }

  .barra.atencao > i {
    background: var(--warn);
  }

  .grafico {
    width: 100%;
    height: 34px;
    display: block;
  }

  .area {
    fill: var(--accent-dim);
    stroke: none;
  }

  .traco {
    fill: none;
    stroke: var(--accent);
    stroke-width: 1;
    vector-effect: non-scaling-stroke;
  }
</style>
