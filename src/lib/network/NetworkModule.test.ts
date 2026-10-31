import { act, render, screen, within } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { afterEach, expect, it, vi } from "vitest";
import NetworkModule from "./NetworkModule.svelte";
import type { NetworkData } from "../core";
import full from "./fixtures/full.json";
import good from "./fixtures/good.json";
import poor from "./fixtures/poor.json";
import vpn from "./fixtures/vpn.json";
import warning from "./fixtures/warning.json";
import risk from "./fixtures/risk.json";
import disconnected from "./fixtures/disconnected.json";
import speedOffline from "./fixtures/speed-offline.json";
import running from "./fixtures/speed-running.json";
import failed from "./fixtures/speed-failed.json";
import processesEmpty from "./fixtures/processes-empty.json";

afterEach(() => vi.restoreAllMocks());

const dataOf = (fixture: { data: unknown }) => fixture.data as NetworkData;
/** Freeze the clock this long after the fixture's speed test. */
function after(fixture: { data: unknown }, ms: number) {
  vi.spyOn(Date, "now").mockReturnValue(dataOf(fixture).speed!.measuredAt + ms);
}

it("separates network quality and security, with the connection kind", () => {
  render(NetworkModule, { data: dataOf(full) });
  expect(screen.getByRole("group", { name: "Qualidade da rede" })).toHaveTextContent("Regular");
  expect(screen.getByRole("group", { name: "Segurança da rede" })).toHaveTextContent("Sem alertas");
  expect(screen.getByText("Sem fio")).toBeInTheDocument();
  expect(screen.getByText("VPN inativa")).toBeInTheDocument();
  expect(screen.queryByText(/\bsegura\b/i)).not.toBeInTheDocument();
});

it("highlights Internet latency next to the last speed test in Mbps", () => {
  after(full, 60_000);
  render(NetworkModule, { data: dataOf(full) });
  expect(screen.getByRole("group", { name: "Latência da internet" })).toHaveTextContent("20 ms");
  const test = screen.getByRole("button", { name: "Testar velocidade" });
  expect(test).toHaveTextContent("↓ 8");
  expect(test).toHaveTextContent("↑ 3");
  expect(test).toHaveTextContent("Mbps");
  expect(test).toHaveAccessibleDescription(/Capacidade medida há 1 min/);
});

it("shows Internet jitter and loss and gateway latency as small indicators, gateway jitter and loss in one band", () => {
  render(NetworkModule, { data: dataOf(full) });
  const term = (name: string) => screen.getByText(name, { selector: "dt" }).nextElementSibling;
  expect(term("Jitter")).toHaveTextContent("0 ms");
  expect(term("Perda")).toHaveTextContent("0 %");
  expect(term("Latência do gateway")).toHaveTextContent("2 ms");
  expect(screen.getByText("Gateway · jitter 0 ms · perda 0 %")).toBeInTheDocument();
  expect(screen.queryByRole("table")).not.toBeInTheDocument();
});

it("explains a missing measurement instead of showing zero", () => {
  render(NetworkModule, { data: dataOf(running) });
  expect(screen.getByText("Jitter", { selector: "dt" }).nextElementSibling).toHaveTextContent("sem medição");
  expect(screen.getByText("Gateway · jitter sem medição · perda 0 %")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Testando velocidade…" })).toHaveTextContent("Sem teste de velocidade");
  const traffic = screen.getByRole("region", { name: "Tráfego total" });
  expect(traffic).toHaveTextContent("Aguardando amostras de tráfego");
  expect(within(traffic).getByText("sem medição", { selector: ".upload dd" })).toBeInTheDocument();
  expect(within(traffic).queryByText(/0 KB\/s/)).not.toBeInTheDocument();
});

it("summarizes the two traffic series and their current rates in KB/s", () => {
  render(NetworkModule, { data: dataOf(full) });
  const traffic = screen.getByRole("region", { name: "Tráfego total" });
  expect(within(traffic).getByRole("img")).toHaveAccessibleName(
    "Tráfego total por segundo. 5 amostras de 1 s. Descida: atual 177,7 KB/s, pico 205,1 KB/s. Subida: atual 45,9 KB/s, pico 62,5 KB/s.",
  );
  expect(within(traffic).getByText("Descida").closest("div")).toHaveTextContent("177,7 KB/s");
  expect(within(traffic).getByText("Subida").closest("div")).toHaveTextContent("45,9 KB/s");
  expect(traffic.querySelectorAll("polyline")).toHaveLength(2);
});

it("plots a single sample as two marks instead of inventing a trend", () => {
  render(NetworkModule, { data: dataOf(processesEmpty) });
  const chart = within(screen.getByRole("region", { name: "Tráfego total" })).getByRole("img");
  expect(chart).toHaveAccessibleName("Tráfego total por segundo. 1 amostra de 1 s. Descida 7 KB/s, subida 3,5 KB/s.");
  expect(chart.querySelectorAll("polyline")).toHaveLength(0);
});

it("lists five apps and Others with paired bars on a common local scale", () => {
  render(NetworkModule, { data: dataOf(full) });
  const apps = screen.getByRole("region", { name: "Tráfego por aplicativo" });
  const rows = within(apps).getAllByRole("listitem");
  expect(rows.map((r) => r.querySelector(".name")!.textContent)).toEqual(["firefox", "codex", "claude", "code", "slack", "Outros"]);
  expect(rows[0]).toHaveTextContent("↓ 6 KB/s ↑ 3 KB/s");
  expect(rows[5]).toHaveTextContent("↓ 104,5 KB/s ↑ 20,5 KB/s");
  // Others carries the largest value, so it sets the scale for every bar.
  expect(rows[5].querySelector("i")).toHaveStyle({ width: "100%" });
  expect(rows[0].querySelector("i")!.style.width).toMatch(/^5\.7\d*%$/);
  expect(within(apps).getByText("Outros")).toHaveAttribute("title", "UDP/QUIC, processos do sistema e conexões curtas");
  expect(apps).toHaveTextContent("Outros: UDP/QUIC, processos do sistema e conexões curtas");
  expect(within(apps).getByRole("list")).toHaveAccessibleDescription("Barras em escala comum entre aplicativos, não a capacidade do link");
});

it("shows an empty TCP window with the remaining interface traffic", () => {
  render(NetworkModule, { data: dataOf(processesEmpty) });
  const apps = screen.getByRole("region", { name: "Tráfego por aplicativo" });
  expect(apps).toHaveTextContent("Nenhum aplicativo com tráfego TCP nesta janela");
  expect(within(apps).getAllByRole("listitem")).toHaveLength(1);
});

it("waits for attribution until diagnostics provide a baseline", () => {
  render(NetworkModule, { data: dataOf(good) });
  expect(screen.getByRole("region", { name: "Tráfego por aplicativo" })).toHaveTextContent("Aguardando medição");
  expect(screen.queryByText("Outros")).not.toBeInTheDocument();
});

it("keeps the test age on focus and pointer while recent, and shows it directly once old", async () => {
  vi.useFakeTimers({ now: dataOf(full).speed!.measuredAt + 60_000, toFake: ["Date", "setInterval", "clearInterval"] });
  try {
    render(NetworkModule, { data: dataOf(full) });
    const age = () => screen.getByText(/^(Capacidade medida|Teste antigo)/);
    expect(age()).toHaveTextContent(/^Capacidade medida há 1 min · \d{2}\/\d{2}/);
    expect(age()).not.toHaveClass("shown");
    vi.setSystemTime(dataOf(full).speed!.measuredAt + 7 * 3600_000);
    await act(() => vi.advanceTimersByTime(1000));
    expect(age()).toHaveTextContent(/^Teste antigo · há 7 h/);
    expect(age()).toHaveClass("shown");
  } finally {
    vi.useRealTimers();
  }
});

it("keeps the last speed offline with its age, disables a new test and drops live measurements", () => {
  after(speedOffline, 2 * 3600_000);
  render(NetworkModule, { data: dataOf(speedOffline) });
  expect(screen.getByRole("group", { name: "Qualidade da rede" })).toHaveTextContent("Sem conexão");
  expect(screen.getByRole("group", { name: "Segurança da rede" })).toHaveTextContent("Sem conexão");
  expect(screen.getByRole("group", { name: "Latência da internet" })).toHaveTextContent("Sem conexão");
  const test = screen.getByRole("button", { name: "Testar velocidade" });
  expect(test).toBeDisabled();
  expect(test).toHaveTextContent("↓ 8");
  expect(screen.getByText(/^Capacidade medida há 2 h/)).toHaveClass("shown");
  expect(screen.getByText("Teste indisponível sem internet ou em conexão limitada")).toBeInTheDocument();
});

it("shows a disconnection without measurements, apps or connection labels", () => {
  render(NetworkModule, { data: dataOf(disconnected) });
  expect(screen.getByRole("group", { name: "Qualidade da rede" })).toHaveTextContent("Sem conexão");
  expect(screen.getByRole("group", { name: "Segurança da rede" })).toHaveTextContent("Sem conexão");
  expect(screen.queryByText("Sem fio")).not.toBeInTheDocument();
  expect(screen.queryByText(/^Gateway ·/)).not.toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Testar velocidade" })).toHaveTextContent("Sem teste de velocidade");
});

it("shows wired and VPN labels from the snapshot", () => {
  render(NetworkModule, { data: dataOf(vpn) });
  expect(screen.getByText("Cabeada")).toBeInTheDocument();
  expect(screen.getByText("VPN ativa")).toBeInTheDocument();
});

it("keeps poor quality apart from security", () => {
  render(NetworkModule, { data: dataOf(poor) });
  expect(screen.getByRole("group", { name: "Qualidade da rede" })).toHaveTextContent("✕ Ruim");
  expect(screen.getByRole("group", { name: "Segurança da rede" })).toHaveTextContent("✓ Sem alertas");
  expect(screen.getByRole("group", { name: "Latência da internet" })).toHaveTextContent("301 ms");
});

it.each([
  ["Atenção", "⚠ Portal cativo detectado", warning],
  ["Risco", "⚠ Rede aberta sem OWE", risk],
])("shows %s with the reason supplied by the core", (state, reason, fixture) => {
  render(NetworkModule, { data: dataOf(fixture) });
  expect(screen.getByRole("group", { name: "Segurança da rede" })).toHaveTextContent(state);
  expect(screen.getByRole("listitem")).toHaveTextContent(reason);
  expect(screen.getByRole("group", { name: "Qualidade da rede" })).toHaveTextContent("Boa");
});

it("shows test progress and a metadata-only failure", async () => {
  const view = render(NetworkModule, { data: dataOf(running) });
  expect(screen.getByRole("button", { name: "Testando velocidade…" })).toBeDisabled();
  await view.rerender({ data: dataOf(failed) });
  expect(screen.getByRole("status")).toHaveTextContent("⚠ Não foi possível medir a velocidade");
  expect(screen.getByRole("button", { name: "Testar velocidade" })).toBeEnabled();
});

it("runs a new test from the keyboard and describes the test age on focus", async () => {
  after(full, 60_000);
  const runSpeed = vi.fn();
  render(NetworkModule, { data: dataOf(full), runSpeed });
  await userEvent.tab();
  const test = screen.getByRole("button", { name: "Testar velocidade" });
  expect(test).toHaveFocus();
  expect(test).toHaveAccessibleDescription(/^Capacidade medida há 1 min/);
  await userEvent.keyboard("{Enter}");
  expect(runSpeed).toHaveBeenCalledOnce();
});

it("does not offer a test that the core cannot run", async () => {
  const runSpeed = vi.fn();
  render(NetworkModule, { data: dataOf(good), runSpeed });
  const test = screen.getByRole("button", { name: "Testar velocidade" });
  expect(test).toBeDisabled();
  await userEvent.click(test);
  expect(runSpeed).not.toHaveBeenCalled();
  expect(screen.getByText("Teste indisponível sem internet ou em conexão limitada")).toBeInTheDocument();
});

it("explains that nothing was read before the first snapshot", () => {
  render(NetworkModule, { data: null });
  expect(screen.getByText("Sem leitura da rede ainda")).toBeInTheDocument();
});

it("replaces the latency instrument and the apps with the disconnection, not a pending measurement", () => {
  render(NetworkModule, { data: dataOf(disconnected) });
  expect(screen.getByRole("group", { name: "Latência da internet" })).toHaveTextContent("Sem conexão");
  const apps = screen.getByRole("region", { name: "Tráfego por aplicativo" });
  expect(apps).toHaveTextContent("Sem conexão");
  expect(apps).not.toHaveTextContent("Aguardando medição");
});

it("shows the date of an old test directly, in the attention tone only when it is old", async () => {
  after(speedOffline, 7 * 3600_000);
  render(NetworkModule, { data: dataOf(speedOffline) });
  const age = screen.getByText(/^Teste antigo/);
  expect(age).toHaveTextContent(/^Teste antigo · há 7 h · \d{2}\/\d{2}/);
  expect(age).toHaveClass("shown", "old");
});

it("shows a recent test's age directly when a new test cannot run, without calling it old", () => {
  after(speedOffline, 60_000);
  render(NetworkModule, { data: dataOf(speedOffline) });
  const age = screen.getByText(/^Capacidade medida há 1 min/);
  expect(age).toHaveClass("shown");
  expect(age).not.toHaveClass("old");
});

it("keeps the last test's age readable while a new test runs and the button is disabled", () => {
  after(full, 60_000);
  render(NetworkModule, { data: { ...dataOf(full), speedRunning: true } });
  expect(screen.getByRole("button", { name: "Testando velocidade…" })).toBeDisabled();
  expect(screen.getByText(/^Capacidade medida há 1 min/)).toHaveClass("shown");
});

it("labels each chart series with its own line style next to the chart", () => {
  render(NetworkModule, { data: dataOf(full) });
  const traffic = screen.getByRole("region", { name: "Tráfego total" });
  expect(within(traffic).getByText("Descida").closest("div")!.querySelector("svg line.download")).not.toBeNull();
  expect(within(traffic).getByText("Subida").closest("div")!.querySelector("svg line.upload")).not.toBeNull();
});
