<script lang="ts">
  import type { DadosRede } from "../nucleo";
  import { t } from "../strings";
  let { dados }: { dados: DadosRede } = $props();
  const numero = new Intl.NumberFormat("pt-BR", { maximumFractionDigits: 1 });
  const medida = (v: number | null, unidade: string) => v === null ? t.rede.semValor : `${numero.format(v)} ${unidade}`;
  function taxa(v: number) {
    return v >= 1048576 ? `${numero.format(v / 1048576)} MB/s` : v >= 1024 ? `${numero.format(v / 1024)} KB/s` : `${numero.format(v)} B/s`;
  }
  const maximo = $derived(Math.max(1, ...dados.historico.flatMap((p) => [p.descida, p.subida])));
  function pontos(direcao: "descida" | "subida") {
    return dados.historico.map((p, i) => `${i * 300 / 59},${60 - p[direcao] / maximo * 56}`).join(" ");
  }
</script>

<div class="rede">
  <section aria-label={t.rede.qualidade}>
    <h2>{t.rede.qualidade}</h2>
    <strong class:boa={dados.qualidade === "boa"} class:regular={dados.qualidade === "regular"} class:ruim={dados.qualidade === "ruim"}>
      {!dados.conectada ? t.rede.semConexao : dados.qualidade ? t.rede[dados.qualidade] : t.rede.semMedicao}
    </strong>
    <div class="etiquetas">
      {#if dados.conectada && dados.cabeada}<span>{t.rede.cabeada}</span>{/if}
      {#if dados.conectada && dados.vpn}<span>{t.rede.vpn}</span>{/if}
    </div>
  </section>
  {#if dados.conectada}
    {#each [{ nome: t.rede.gateway, medidas: dados.gateway }, { nome: t.rede.internet, medidas: dados.internet }] as destino}
      <section aria-label={destino.nome}>
        <h2>{destino.nome}</h2>
        <dl>
          <div><dt>{t.rede.latencia}</dt><dd class="num">{medida(destino.medidas.latencia, "ms")}</dd></div>
          <div><dt>{t.rede.jitter}</dt><dd class="num">{medida(destino.medidas.jitter, "ms")}</dd></div>
          <div><dt>{t.rede.perda}</dt><dd class="num">{medida(destino.medidas.perda, "%")}</dd></div>
        </dl>
      </section>
    {/each}
  {/if}
  <section aria-label={t.rede.trafego}>
    <h2>{t.rede.trafego}</h2>
    <svg role="img" aria-label={t.rede.grafico} viewBox="0 0 300 64">
      <polyline points={pontos("descida")} class="descida" />
      <polyline points={pontos("subida")} class="subida" />
    </svg>
    <dl>
      <div><dt>{t.rede.descida}</dt><dd class="num">↓ {dados.trafego ? taxa(dados.trafego.descida) : t.rede.semValor}</dd></div>
      <div><dt>{t.rede.subida}</dt><dd class="num">↑ {dados.trafego ? taxa(dados.trafego.subida) : t.rede.semValor}</dd></div>
    </dl>
  </section>
</div>
<style>
  .rede { padding: 4px 16px 16px; overflow-y: auto; }
  section { padding: 12px 0; border-bottom: 1px solid var(--line); }
  h2 { font-size: 12px; color: var(--muted); font-weight: 500; margin-bottom: 8px; }
  strong { font-size: 22px; }
  .boa { color: var(--ok); } .regular { color: var(--warn); } .ruim { color: var(--bad); }
  .etiquetas { display: flex; gap: 6px; margin-top: 6px; color: var(--muted); font-size: 11px; }
  .etiquetas span { background: var(--surface-2); padding: 3px 6px; border-radius: 4px; }
  dl { display: flex; justify-content: space-between; gap: 12px; }
  dt { font-size: 11px; color: var(--muted); margin-bottom: 4px; }
  dd { font-size: 12px; }
  svg { width: 100%; height: 64px; background: var(--surface); margin-bottom: 8px; }
  polyline { fill: none; stroke-width: 1.5; }
  .descida { stroke: var(--accent); } .subida { stroke: var(--muted); }
</style>
