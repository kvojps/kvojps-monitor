<script lang="ts">
  import { onMount } from "svelte";
  import type { DadosGithub } from "../nucleo";
  import { t, preencher } from "../strings";
  let { dados, atualizar }: { dados: DadosGithub; atualizar: () => void } = $props();
  let configurar = $state(false);
  let agora = $state(Date.now());
  let liberadoEm = $state(0);
  onMount(() => { const timer = setInterval(() => agora = Date.now(), 1000); return () => clearInterval(timer); });
  const relativo = new Intl.RelativeTimeFormat("pt-BR", { numeric: "auto" });
  function idade(quando: number) {
    const segundos = Math.max(0, Math.floor((agora - quando) / 1000));
    if (segundos < 3600) return relativo.format(-Math.floor(segundos / 60), "minute");
    if (segundos < 86400) return relativo.format(-Math.floor(segundos / 3600), "hour");
    return relativo.format(-Math.floor(segundos / 86400), "day");
  }
  function atualizarAgora() { liberadoEm = Date.now() + 10000; atualizar(); }
</script>
<section class="github">
  <button aria-label={t.github.atualizar} disabled={dados.atualizando || agora < liberadoEm} onclick={atualizarAgora}>↻</button>
  {#if dados.atualizando}<span role="status">{t.github.atualizando}</span>{/if}
  {#if dados.estado === "semGh"}
    <p>{t.github.semGh}</p>
    <button onclick={() => configurar = true}>{t.github.configurar}</button>
    {#if configurar}<p>{t.github.login}</p><p>{t.github.instrucao}</p>{/if}
  {:else}
    {#if dados.estado === "erro"}<p role="status">{t.github.erro}</p>{/if}
    {#if dados.calendario}
      <h2>{t.github.calendario}</h2>
      <p>{preencher(t.github.contribuicoes, {total:String(dados.calendario.contributionCalendar.totalContributions)})}</p>
      <div class="calendario" role="img" aria-label={t.github.calendario}>
        {#each dados.calendario.contributionCalendar.weeks as semana}
          <div class="semana">{#each semana.contributionDays as dia}<span style:background={dia.color} title={`${dia.date}: ${dia.contributionCount}`}></span>{/each}</div>
        {/each}
      </div>
      <p class="secundario">{preencher(t.github.privadas, {total:String(dados.calendario.restrictedContributionsCount)})}</p>
    {/if}
    <h2>{t.github.destaque}</h2>
    {#if dados.destaque}
      <p>{dados.destaque.nome}</p>
      <p class="num">{preencher(t.github.contagem, {commits:String(dados.destaque.commits),total:String(dados.destaque.total)})}</p>
      <p class="secundario">{idade(dados.destaque.quando)}</p>
      <p class="titulo">{dados.destaque.titulo}</p>
    {:else}<p>{t.github.vazio}</p>{/if}
  {/if}
</section>
<style>
  .github { padding: 0 16px 16px; min-width: 0; overflow: auto; }
  h2 { font-size: 13px; margin-top: 18px; }
  p { margin: 8px 0; font-size: 12px; }
  button { padding: 5px 8px; border: 1px solid var(--line); border-radius: 5px; }
  button:disabled { opacity: .4; }
  .secundario { color: var(--muted); }
  .titulo { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .calendario { display: flex; gap: 2px; margin: 12px 0; overflow-x: auto; }
  .semana { display: flex; flex-direction: column; gap: 2px; }
  .semana span { width: 6px; height: 6px; border-radius: 1px; }
</style>
