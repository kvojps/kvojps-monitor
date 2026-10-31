import { render, screen, within } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";
import SystemModule from "./SystemModule.svelte";
import type { SystemData, Snapshot } from "../core";
// Snapshots published by the core; regenerate with UPDATE_FIXTURES=1 cargo test.
import notebook from "./fixtures/notebook.json";
import desktop from "./fixtures/desktop.json";
import noSensor from "./fixtures/no-sensor.json";
import warning from "./fixtures/warning.json";

function renderSnapshot(snapshot: Snapshot) {
  render(SystemModule, { data: snapshot.data as SystemData });
}

const region = (name: string) => screen.getByRole("region", { name: name });

describe("System module", () => {
  it("shows CPU usage, history and temperature", () => {
    renderSnapshot(notebook as Snapshot);

    const cpu = region("CPU");
    expect(within(cpu).getByText("2%")).toBeInTheDocument();
    expect(within(cpu).getByRole("img", { name: "Uso de CPU nos últimos 60 s" })).toBeInTheDocument();
    expect(within(cpu).getByText("47 °C")).toBeInTheDocument();
  });

  it("shows used and total RAM with swap on a secondary line", () => {
    renderSnapshot(notebook as Snapshot);

    const ram = region("RAM");
    expect(within(ram).getByText("5,7 / 15 GB")).toBeInTheDocument();
    expect(within(ram).getByText("Swap")).toBeInTheDocument();
    expect(within(ram).getByText("0,4 / 4,0 GB")).toBeInTheDocument();
  });

  it("shows one bar per disk with used, total and free space", () => {
    renderSnapshot(notebook as Snapshot);

    const disks = region("Discos");
    expect(within(disks).getByText("/")).toBeInTheDocument();
    expect(within(disks).getByText("70 / 147 GB · 69 GB livres")).toBeInTheDocument();
    expect(within(disks).getAllByRole("meter")).toHaveLength(1);
  });

  it("shows notebook battery percentage and state", () => {
    renderSnapshot(notebook as Snapshot);

    const battery = region("Bateria");
    expect(within(battery).getByText("100%")).toBeInTheDocument();
    expect(within(battery).getByText("carregada")).toBeInTheDocument();
  });

  it("hides the battery block on a desktop without a battery", () => {
    renderSnapshot(desktop as Snapshot);

    expect(screen.queryByRole("region", { name: "Bateria" })).not.toBeInTheDocument();
    expect(region("CPU")).toBeInTheDocument();
  });

  it("hides temperature when there is no sensor", () => {
    renderSnapshot(noSensor as Snapshot);

    expect(within(region("CPU")).queryByText(/°C/)).not.toBeInTheDocument();
  });

  it("shows remaining time and warns above thresholds", () => {
    renderSnapshot(warning as Snapshot);

    const battery = region("Bateria");
    expect(within(battery).getByText("na bateria · 30 min restantes")).toBeInTheDocument();
    expect(within(battery).getByText("12%")).toHaveClass("warning");
    expect(within(region("CPU")).getByText("92 °C")).toHaveClass("warning");

    const [root, data] = within(region("Discos")).getAllByRole("meter");
    expect(root).not.toHaveClass("warning");
    expect(data).toHaveClass("warning");
  });

  it("shows empty CPU usage until two samples exist", () => {
    const data = structuredClone(notebook.data) as SystemData;
    data.cpu = { ...data.cpu, usage: null, history: [] };
    render(SystemModule, { data });

    expect(within(region("CPU")).getByText("—")).toBeInTheDocument();
  });
});
