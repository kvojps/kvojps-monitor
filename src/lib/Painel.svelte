<script lang="ts">
  import { onMount } from "svelte";
  import Icone from "./Icone.svelte";
  import { MODULOS, type DadosSistema, type Modulo, type Nucleo, type Snapshot } from "./nucleo";
  import ModuloSistema from "./sistema/ModuloSistema.svelte";
  import { preencher, t } from "./strings";
  import type { Vista } from "./vista";

  let { nucleo }: { nucleo: Nucleo } = $props();

  const itens: { vista: Vista; nome: string }[] = [
    ...MODULOS.map((m) => ({ vista: m, nome: t.modulos[m] })),
    { vista: "configuracoes", nome: t.configuracoes.titulo },
  ];

  let vista = $state<Vista | null>(null);
  const atual = $derived(itens.find((i) => i.vista === vista));

  let snapshots = $state<Partial<Record<Modulo, Snapshot>>>({});
  const atualizadoEm = $derived(vista && vista !== "configuracoes" ? (snapshots[vista]?.atualizadoEm ?? null) : undefined);

  function receber(s: Snapshot) {
    const anterior = snapshots[s.modulo];
    if (!anterior || (anterior.atualizadoEm ?? 0) <= (s.atualizadoEm ?? 0)) snapshots[s.modulo] = s;
  }

  const hora = new Intl.DateTimeFormat("pt-BR", { hour: "2-digit", minute: "2-digit" });

  onMount(() => {
    // Ouve antes de pedir o estado inicial para não perder snapshots empurrados no meio.
    const parar = nucleo.ouvirSnapshots(receber);
    nucleo.estadoInicial().then((inicial) => {
      inicial.snapshots.forEach(receber);
      vista = inicial.ultimoModulo;
    });
    return () => void parar.then((p) => p());
  });

  function selecionar(nova: Vista) {
    vista = nova;
    nucleo.informarModuloVisivel(nova === "configuracoes" ? null : nova);
  }
  function aoTeclar(e: KeyboardEvent) {
    if (e.key === "Escape") nucleo.fecharPainel();
  }
</script>

<svelte:window onkeydown={aoTeclar} />

<div class="painel">
  <nav class="trilho" aria-label={t.trilho.rotulo}>
    {#each itens as item (item.vista)}
      <button
        class="item"
        aria-label={item.nome}
        title={item.nome}
        aria-current={item.vista === vista ? "page" : undefined}
        onclick={() => selecionar(item.vista)}
      >
        <Icone nome={item.vista} />
      </button>
    {/each}
  </nav>
  {#if atual}
    <main class="detalhe">
      <header class="cabecalho">
        <h1>{atual.nome}</h1>
        {#if atualizadoEm !== undefined}
          <span class="atualizacao num">
            {atualizadoEm === null
              ? t.cabecalho.semDados
              : preencher(t.cabecalho.atualizado, { hora: hora.format(atualizadoEm) })}
          </span>
        {/if}
      </header>
      {#if vista === "sistema" && snapshots.sistema}
        <ModuloSistema dados={snapshots.sistema.dados as DadosSistema} />
      {/if}
    </main>
  {/if}
</div>

<style>
  .painel {
    display: flex;
    height: 100%;
    background: var(--bg);
    border: 1px solid var(--line);
    border-radius: var(--raio);
    overflow: hidden;
  }

  .trilho {
    width: 52px;
    flex: none;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 10px 0;
    background: var(--surface);
    border-right: 1px solid var(--line);
  }

  /* Configurações vão para o rodapé do trilho. */
  .item:last-child {
    margin-top: auto;
  }

  .item {
    width: 36px;
    height: 36px;
    border-radius: 8px;
    display: grid;
    place-items: center;
    color: var(--muted);
  }

  .item:hover {
    color: var(--text);
  }

  .item[aria-current="page"] {
    color: var(--accent);
    background: var(--accent-dim);
  }

  .item:focus-visible {
    outline: 1px solid var(--accent);
  }

  .detalhe {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .cabecalho {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 8px;
    padding: 14px 16px 10px;
  }

  h1 {
    font-size: 15px;
    font-weight: 600;
  }

  .atualizacao {
    font-size: 11px;
    color: var(--faint);
  }
</style>
