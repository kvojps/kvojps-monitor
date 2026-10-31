import { render, screen, within } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";
import ModuloSistema from "./ModuloSistema.svelte";
import type { DadosSistema, Snapshot } from "../nucleo";
// Snapshots publicados pelo núcleo; regravados por `ATUALIZAR_FIXTURES=1 cargo test`.
import notebook from "./fixtures/notebook.json";
import desktop from "./fixtures/desktop.json";
import semSensor from "./fixtures/sem-sensor.json";
import atencao from "./fixtures/atencao.json";

function renderizar(snapshot: Snapshot) {
  render(ModuloSistema, { dados: snapshot.dados as DadosSistema });
}

const bloco = (nome: string) => screen.getByRole("region", { name: nome });

describe("Módulo Sistema", () => {
  it("mostra o uso de CPU, o mini‑gráfico e a temperatura", () => {
    renderizar(notebook as Snapshot);

    const cpu = bloco("CPU");
    expect(within(cpu).getByText("2%")).toBeInTheDocument();
    expect(within(cpu).getByRole("img", { name: "Uso de CPU nos últimos 60 s" })).toBeInTheDocument();
    expect(within(cpu).getByText("47 °C")).toBeInTheDocument();
  });

  it("mostra RAM usada/total e o swap como linha secundária", () => {
    renderizar(notebook as Snapshot);

    const ram = bloco("RAM");
    expect(within(ram).getByText("5,7 / 15 GB")).toBeInTheDocument();
    expect(within(ram).getByText("Swap")).toBeInTheDocument();
    expect(within(ram).getByText("0,4 / 4,0 GB")).toBeInTheDocument();
  });

  it("mostra uma barra por disco com usado/total e livre", () => {
    renderizar(notebook as Snapshot);

    const discos = bloco("Discos");
    expect(within(discos).getByText("/")).toBeInTheDocument();
    expect(within(discos).getByText("70 / 147 GB · 69 GB livres")).toBeInTheDocument();
    expect(within(discos).getAllByRole("meter")).toHaveLength(1);
  });

  it("mostra a bateria do notebook com % e estado", () => {
    renderizar(notebook as Snapshot);

    const bateria = bloco("Bateria");
    expect(within(bateria).getByText("100%")).toBeInTheDocument();
    expect(within(bateria).getByText("carregada")).toBeInTheDocument();
  });

  it("num desktop sem bateria, o bloco some", () => {
    renderizar(desktop as Snapshot);

    expect(screen.queryByRole("region", { name: "Bateria" })).not.toBeInTheDocument();
    expect(bloco("CPU")).toBeInTheDocument();
  });

  it("sem sensor, a linha de temperatura some", () => {
    renderizar(semSensor as Snapshot);

    expect(within(bloco("CPU")).queryByText(/°C/)).not.toBeInTheDocument();
  });

  it("mostra o tempo restante e marca com cor de atenção o que passou do limite", () => {
    renderizar(atencao as Snapshot);

    const bateria = bloco("Bateria");
    expect(within(bateria).getByText("na bateria · 30 min restantes")).toBeInTheDocument();
    expect(within(bateria).getByText("12%")).toHaveClass("atencao");
    expect(within(bloco("CPU")).getByText("92 °C")).toHaveClass("atencao");

    const [raiz, dados] = within(bloco("Discos")).getAllByRole("meter");
    expect(raiz).not.toHaveClass("atencao");
    expect(dados).toHaveClass("atencao");
  });

  it("ao abrir, sem duas amostras ainda, a CPU aparece vazia", () => {
    const dados = structuredClone(notebook.dados) as DadosSistema;
    dados.cpu = { ...dados.cpu, uso: null, historico: [] };
    render(ModuloSistema, { dados });

    expect(within(bloco("CPU")).getByText("—")).toBeInTheDocument();
  });
});
