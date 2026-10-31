import { render, screen } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";
import SystemModule from "./SystemModule.svelte";
import type { SystemData, Snapshot } from "../core";
// Snapshots published by the core; regenerate with UPDATE_FIXTURES=1 cargo test.
import notebook from "./fixtures/notebook.json";
import desktop from "./fixtures/desktop.json";
import noSensor from "./fixtures/no-sensor.json";
import missing from "./fixtures/missing.json";
import warning from "./fixtures/warning.json";
import starting from "./fixtures/starting.json";

function renderSnapshot(snapshot: Snapshot) {
  render(SystemModule, { data: snapshot.data as SystemData, updatedAt: snapshot.updatedAt });
}

const group = (name: string) => screen.getByRole("group", { name });
const meter = (name: string) => screen.getByRole("meter", { name });

afterEach(() => vi.useRealTimers());

/** Render a snapshot as seen some time after the core published it. */
function renderLater(snapshot: Snapshot, elapsed: number) {
  vi.useFakeTimers({ now: snapshot.updatedAt! + elapsed, toFake: ["Date", "setInterval", "clearInterval"] });
  renderSnapshot(snapshot);
}

describe("System module", () => {
  it("highlights CPU and temperature and shows RAM, swap, disk and battery from a notebook", () => {
    renderSnapshot(notebook as Snapshot);

    expect(group("CPU")).toHaveTextContent("2%");
    expect(group("Temperatura")).toHaveTextContent("47 °C");
    expect(meter("RAM")).toHaveAttribute("aria-valuenow", "39");
    expect(meter("RAM")).toHaveAttribute("aria-valuetext", "5,7 / 15 GB · 39%");
    expect(screen.getByText("5,7 / 15 GB")).toBeInTheDocument();
    expect(meter("Swap")).toHaveAttribute("aria-valuetext", "0,4 / 4,0 GB · 9%");
    // The percentage agrees with the visible pair; the core still decides the warning.
    expect(meter("Disco /")).toHaveAttribute("aria-valuetext", "70 / 147 GB · 48%");
    expect(meter("Bateria")).toHaveAttribute("aria-valuenow", "100");
    expect(meter("Bateria")).toHaveAttribute("aria-valuetext", "100% · carregada");
    expect(group("Bateria")).toHaveTextContent("100%");
    expect(group("Bateria")).toHaveTextContent("carregada");
  });

  it("leaves out the CPU history and free space dismissed from the selection", () => {
    renderSnapshot(notebook as Snapshot);

    expect(screen.queryByRole("img")).not.toBeInTheDocument();
    expect(screen.queryByText(/livres/)).not.toBeInTheDocument();
  });

  it("says a desktop has no battery instead of showing a charge", () => {
    renderSnapshot(desktop as Snapshot);

    expect(group("Bateria")).toHaveTextContent("Sem bateria detectada");
    expect(screen.queryByRole("meter", { name: "Bateria" })).not.toBeInTheDocument();
    expect(group("CPU")).toHaveTextContent("2%");
  });

  it("explains a missing temperature sensor without inventing a value", () => {
    renderSnapshot(noSensor as Snapshot);

    expect(group("Temperatura")).toHaveTextContent("Sem sensor");
    expect(group("Temperatura")).not.toHaveTextContent("°C");
  });

  it("explains missing sensor, swap and battery together", () => {
    renderSnapshot(missing as Snapshot);

    expect(group("Temperatura")).toHaveTextContent("Sem sensor");
    expect(screen.queryByRole("meter", { name: "Swap" })).not.toBeInTheDocument();
    expect(screen.getByText("Sem swap configurada")).toBeInTheDocument();
    expect(group("Bateria")).toHaveTextContent("Sem bateria detectada");
    expect(meter("RAM")).toBeInTheDocument();
  });

  it("identifies temperature, disk and battery alerts by text, not only color", () => {
    renderSnapshot(warning as Snapshot);

    expect(group("Temperatura")).toHaveTextContent("92 °C");
    expect(group("Temperatura")).toHaveTextContent("Temperatura alta");
    expect(group("Bateria")).toHaveTextContent("12%");
    expect(group("Bateria")).toHaveTextContent("Bateria baixa");
    expect(group("Bateria")).toHaveTextContent("na bateria");
    expect(group("Bateria")).not.toHaveTextContent(/restantes|min/);
    // A warning disk other than the root one keeps its alert without listing every disk.
    expect(screen.getAllByRole("meter").map((m) => m.getAttribute("aria-label"))).toEqual(["RAM", "Swap", "Disco /", "Bateria"]);
    expect(screen.getByRole("alert")).toHaveTextContent("Pouco espaço em /mnt/data");
  });

  it("shows no alert on a normal reading", () => {
    renderSnapshot(notebook as Snapshot);

    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
    expect(screen.queryByText(/Temperatura alta|Bateria baixa|Pouco espaço/)).not.toBeInTheDocument();
  });

  it("flags the root disk itself when it is almost full", () => {
    const data = structuredClone(notebook.data) as SystemData;
    data.disks = [{ ...data.disks[0], free: 8_000_000_000, warning: true }];
    render(SystemModule, { data, updatedAt: notebook.updatedAt });

    expect(screen.getByRole("alert")).toHaveTextContent("Pouco espaço em /");
    expect(meter("Disco /")).toHaveClass("warning");
  });

  it("says CPU usage is being measured until two samples exist, never 0%", () => {
    renderSnapshot(starting as Snapshot);

    expect(group("CPU")).toHaveTextContent("Medindo…");
    expect(group("CPU")).not.toHaveTextContent("0%");
  });

  it("explains there is no reading before the first system snapshot", () => {
    render(SystemModule, { data: null, updatedAt: null });

    expect(screen.getByText("Sem leitura do sistema ainda")).toBeInTheDocument();
    expect(screen.queryByRole("meter")).not.toBeInTheDocument();
  });

  it("keeps the last reading with its age when collection stops arriving", () => {
    renderLater(notebook as Snapshot, 90_000);

    expect(screen.getByRole("status")).toHaveTextContent("Leitura antiga · sem atualização há 1 minuto");
    expect(group("CPU")).toHaveTextContent("2%");
    expect(meter("RAM")).toBeInTheDocument();
  });

  it("does not flag a reading that is still fresh", () => {
    renderLater(notebook as Snapshot, 3_000);

    expect(screen.queryByText(/Leitura antiga/)).not.toBeInTheDocument();
  });
});
