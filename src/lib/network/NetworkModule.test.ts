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

const status = () => document.querySelector(".network-status");
const lines = () => [...document.querySelectorAll(".network > .line")].map((l) => l.textContent!.replace(/\s+/g, " "));
const latency = () => screen.getByRole("group", { name: "Internet" });

it("shows B's status line with the connection kind, quality and security", () => {
  render(NetworkModule, { data: dataOf(full) });
  expect(screen.getByRole("heading", { level: 2, name: "Rede" })).toBeInTheDocument();
  expect(status()).toHaveTextContent("Wi-Fi Qualidade: Regular Segurança: Sem alertas");
  expect(screen.queryByText(/\bsegura\b/i)).not.toBeInTheDocument();
});

it("shows nothing in the normal state beyond what B shows", () => {
  after(full, 60_000);
  render(NetworkModule, { data: dataOf(full) });
  expect(lines()).toEqual([]);
  expect(screen.queryByText(/NetworkManager|atualizado|VPN|Fixture|capacidade/i)).not.toBeInTheDocument();
  expect(screen.queryAllByRole("group")).toHaveLength(1);
});

it("highlights Internet latency next to the last speed test in Mbps with its time", () => {
  after(full, 60_000);
  render(NetworkModule, { data: dataOf(full) });
  expect(latency()).toHaveTextContent("20ms");
  const test = screen.getByRole("button", { name: "Testar velocidade" });
  expect(test).toHaveTextContent(/^↓ 8 · ↑ 3 Mbps · testar ↻ Teste · \d{2}:\d{2}$/);
});

it("shows Internet jitter and loss and gateway latency in one row, gateway jitter and loss below", () => {
  render(NetworkModule, { data: dataOf(full) });
  expect(document.querySelector(".metrics")).toHaveTextContent("Jitter 0 ms Perda 0% Gateway 2 ms");
  expect(screen.getByText("Gateway · jitter 0 ms · perda 0%")).toBeInTheDocument();
  expect(screen.queryByRole("table")).not.toBeInTheDocument();
});

it("explains a missing measurement instead of showing zero", () => {
  render(NetworkModule, { data: dataOf(running) });
  expect(document.querySelector(".metrics")).toHaveTextContent("Jitter sem medição");
  expect(screen.getByText("Gateway · jitter sem medição · perda 0%")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Testando velocidade…" })).toHaveTextContent("↓ — · ↑ — Mbps · testar ↻ Sem teste ainda");
  const traffic = screen.getByRole("region", { name: "Tráfego total" });
  expect(traffic).toHaveTextContent("Aguardando amostras de tráfego");
  expect(traffic.querySelector(".traffic-values .upload")).toHaveTextContent("sem medição");
  expect(within(traffic).queryByText(/0 KB\/s/)).not.toBeInTheDocument();
});

it("draws the two traffic series across the full width with current rates in KB/s", () => {
  render(NetworkModule, { data: dataOf(full) });
  const traffic = screen.getByRole("region", { name: "Tráfego total" });
  expect(within(traffic).getByRole("img")).toHaveAccessibleName(
    "Tráfego total por segundo. 5 amostras de 1 s. Descida: atual 177,7 KB/s, pico 205,1 KB/s. Subida: atual 45,9 KB/s, pico 62,5 KB/s.",
  );
  expect(traffic.querySelector(".traffic-values")).toHaveTextContent("Descida: ↓ 177,7 KB/s Subida: ↑ 45,9 KB/s");
  const paths = traffic.querySelectorAll("path");
  expect(paths).toHaveLength(2);
  expect(paths[0].getAttribute("d")).toMatch(/^M0,.*L300,[\d.]+$/);
  expect(paths[1]).toHaveClass("upload");
});

it("plots a single sample as two marks instead of inventing a trend", () => {
  render(NetworkModule, { data: dataOf(processesEmpty) });
  const chart = within(screen.getByRole("region", { name: "Tráfego total" })).getByRole("img");
  expect(chart).toHaveAccessibleName("Tráfego total por segundo. 1 amostra de 1 s. Descida 7 KB/s, subida 3,5 KB/s.");
  expect(chart.querySelectorAll("path")).toHaveLength(0);
});

/** The apps live in the Aplicativos de rede detail, opened from "N aplicativos + Outros ↗". */
const appsTrigger = () => screen.getByRole("button", { name: /aplicativos? \+ Outros|^Aplicativos de rede/ });
async function openApps() {
  await userEvent.click(appsTrigger());
  return screen.getByRole("dialog", { name: "Aplicativos de rede" });
}

it("keeps the apps out of the summary until the detail opens", async () => {
  render(NetworkModule, { data: dataOf(full) });
  expect(appsTrigger()).toHaveTextContent("5 aplicativos + Outros ↗");
  expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  expect(screen.queryByText("firefox")).not.toBeInTheDocument();
  const apps = await openApps();
  expect(within(apps).getByText("firefox")).toBeInTheDocument();
});

it("lists five apps and Others with paired bars on a common local scale", async () => {
  render(NetworkModule, { data: dataOf(full) });
  const apps = await openApps();
  const rows = within(apps).getAllByRole("listitem");
  expect(rows.map((r) => r.querySelector(".name")!.textContent)).toEqual(["firefox", "codex", "claude", "code", "slack", "Outros"]);
  expect(rows[0]).toHaveTextContent("↓ 6 KB/s · ↑ 3 KB/s");
  expect(rows[5]).toHaveTextContent("↓ 104,5 KB/s · ↑ 20,5 KB/s");
  // Others carries the largest value, so it sets the scale for every bar.
  expect(rows[5].querySelector("i")).toHaveStyle({ width: "100%" });
  expect(rows[0].querySelector("i")!.style.width).toMatch(/^5\.7\d*%$/);
  expect(within(apps).getByText("Outros")).toHaveAttribute("title", "UDP/QUIC, processos do sistema e conexões curtas");
  expect(apps).toHaveTextContent("Outros: UDP/QUIC, processos do sistema e conexões curtas");
  expect(within(apps).getByRole("list")).toHaveAccessibleDescription("Barras em escala comum entre aplicativos, não a capacidade do link");
});

it("shows an empty TCP window with the remaining interface traffic", async () => {
  render(NetworkModule, { data: dataOf(processesEmpty) });
  expect(appsTrigger()).toHaveTextContent("Aplicativos de rede ↗");
  const apps = await openApps();
  expect(apps).toHaveTextContent("Nenhum aplicativo com tráfego TCP nesta janela");
  expect(within(apps).getAllByRole("listitem")).toHaveLength(1);
});

it("waits for attribution until diagnostics provide a baseline", async () => {
  render(NetworkModule, { data: dataOf(good) });
  expect(await openApps()).toHaveTextContent("Aguardando medição");
  expect(screen.queryByText("Outros")).not.toBeInTheDocument();
});

it("names a single app in the singular", () => {
  const one = dataOf(full);
  render(NetworkModule, { data: { ...one, processTraffic: { ...one.processTraffic!, apps: one.processTraffic!.apps.slice(0, 1) } } });
  expect(appsTrigger()).toHaveTextContent("1 aplicativo + Outros ↗");
});

it("shows an old test as an attention line only once it is old", async () => {
  vi.useFakeTimers({ now: dataOf(full).speed!.measuredAt + 60_000, toFake: ["Date", "setInterval", "clearInterval"] });
  try {
    render(NetworkModule, { data: dataOf(full) });
    expect(lines()).toEqual([]);
    vi.setSystemTime(dataOf(full).speed!.measuredAt + 7 * 3600_000);
    await act(() => vi.advanceTimersByTime(1000));
    expect(screen.getByText(/Teste antigo/).closest(".line")).toHaveClass("alert");
    expect(lines()).toEqual([expect.stringMatching(/^⚠ Teste antigo · há 7 h · \d{2}\/\d{2}/)]);
  } finally {
    vi.useRealTimers();
  }
});

it("keeps the last speed offline with its time, disables a new test and drops live measurements", () => {
  after(speedOffline, 2 * 3600_000);
  render(NetworkModule, { data: dataOf(speedOffline) });
  expect(lines()).toEqual(["⊘ Sem conexão"]);
  expect(status()).toBeNull();
  expect(latency()).toHaveTextContent("—");
  expect(document.querySelector(".metrics")).toBeNull();
  const test = screen.getByRole("button", { name: "Testar velocidade" });
  expect(test).toBeDisabled();
  expect(test).toHaveTextContent(/↓ 8 · ↑ 3.*Teste · \d{2}:\d{2}/);
});

it("shows a disconnection without measurements, apps or connection labels", () => {
  render(NetworkModule, { data: dataOf(disconnected) });
  expect(lines()).toEqual(["⊘ Sem conexão"]);
  expect(screen.queryByText("Wi-Fi")).not.toBeInTheDocument();
  expect(screen.queryByText(/^Gateway ·/)).not.toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Testar velocidade" })).toHaveTextContent("Sem teste ainda");
});

it("shows a wired connection on the status line and an active VPN as a line", () => {
  render(NetworkModule, { data: dataOf(vpn) });
  expect(status()).toHaveTextContent(/^Cabeada/);
  expect(lines()).toContain("VPN ativa");
});

it("keeps poor quality apart from security", () => {
  render(NetworkModule, { data: dataOf(poor) });
  expect(status()).toHaveTextContent("Qualidade: Ruim Segurança: Sem alertas");
  expect(latency()).toHaveTextContent("301ms");
});

it.each([
  ["Atenção", "⚠ Portal cativo detectado", warning, "warning"],
  ["Risco", "⚠ Rede aberta sem OWE", risk, "risk"],
])("shows %s with the reason supplied by the core as a coloured line", (state, reason, fixture, tone) => {
  render(NetworkModule, { data: dataOf(fixture) });
  expect(status()).toHaveTextContent(`Qualidade: Boa Segurança: ${state}`);
  const line = screen.getByText(reason.slice(2)).closest(".line")!;
  expect(line).toHaveTextContent(reason);
  expect(line).toHaveClass(tone);
});

it("names the security state when the core gives no reason", () => {
  render(NetworkModule, { data: { ...dataOf(risk), reasons: [] } });
  expect(lines()).toContain("⚠ Risco");
});

it("shows test progress and a metadata-only failure as lines", async () => {
  const view = render(NetworkModule, { data: dataOf(running) });
  expect(screen.getByRole("button", { name: "Testando velocidade…" })).toBeDisabled();
  expect(screen.getByText("Testando velocidade…")).toBeInTheDocument();
  expect(screen.queryByRole("status")).not.toBeInTheDocument();
  await view.rerender({ data: dataOf(failed) });
  expect(screen.getByRole("status")).toHaveTextContent("⚠ Não foi possível medir a velocidade");
  expect(screen.getByRole("button", { name: "Testar velocidade" })).toBeEnabled();
});

it("runs a new test from the keyboard", async () => {
  const runSpeed = vi.fn();
  render(NetworkModule, { data: dataOf(full), runSpeed });
  await userEvent.tab();
  const test = screen.getByRole("button", { name: "Testar velocidade" });
  expect(test).toHaveFocus();
  await userEvent.keyboard("{Enter}");
  expect(runSpeed).toHaveBeenCalledOnce();
});

it("does not offer a test that the core cannot run, and says why", async () => {
  const runSpeed = vi.fn();
  render(NetworkModule, { data: dataOf(good), runSpeed });
  const test = screen.getByRole("button", { name: "Testar velocidade" });
  expect(test).toBeDisabled();
  await userEvent.click(test);
  expect(runSpeed).not.toHaveBeenCalled();
  expect(lines()).toContain("⊘ Teste indisponível sem internet ou em conexão limitada");
});

it("explains that nothing was read before the first snapshot", () => {
  render(NetworkModule, { data: null });
  expect(screen.getByRole("heading", { level: 2, name: "Rede" })).toBeInTheDocument();
  expect(lines()).toEqual([]);
  const area = document.querySelector(".network .area");
  expect(area).toHaveAttribute("data-tone", "neutral");
  expect(area).toHaveTextContent("◌ Sem leitura da rede ainda");
  expect(screen.queryByRole("group", { name: "Internet" })).not.toBeInTheDocument();
});

it("replaces the apps with the disconnection, not a pending measurement", async () => {
  render(NetworkModule, { data: dataOf(disconnected) });
  const apps = await openApps();
  expect(apps).toHaveTextContent("Sem conexão");
  expect(apps).not.toHaveTextContent("Aguardando medição");
});

it("exposes both measured speed values, units and test time to assistive technology", () => {
  render(NetworkModule, { data: dataOf(full) });
  expect(screen.getByRole("button", { name: "Testar velocidade" }))
    .toHaveAccessibleDescription(/Descida: 8 Mbps.*Subida: 3 Mbps.*Teste · \d{2}\/\d{2}/);
});

it("describes the absence of a speed measurement when the action name replaces its contents", () => {
  render(NetworkModule, { data: dataOf(running) });
  expect(screen.getByRole("button", { name: "Testando velocidade…" }))
    .toHaveAccessibleDescription("Sem teste de velocidade");
});

it("closes the apps detail with Close or Escape and returns focus to its trigger", async () => {
  render(NetworkModule, { data: dataOf(full) });
  const trigger = appsTrigger();
  const apps = await openApps();
  await userEvent.click(within(apps).getByRole("button", { name: "Fechar" }));
  expect(apps).not.toHaveAttribute("open");
  expect(trigger).toHaveFocus();
  trigger.focus();
  await userEvent.keyboard("{Enter}");
  expect(apps).toHaveAttribute("open");
  const escape = new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true });
  window.dispatchEvent(escape);
  await act(() => {});
  expect(escape.defaultPrevented).toBe(true);
  expect(apps).not.toHaveAttribute("open");
  expect(trigger).toHaveFocus();
});

it("leaves Escape to the Panel when the detail is closed", () => {
  render(NetworkModule, { data: dataOf(full) });
  const escape = new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true });
  window.dispatchEvent(escape);
  expect(escape.defaultPrevented).toBe(false);
});

it("keeps the detail open and focused while new snapshots arrive", async () => {
  const view = render(NetworkModule, { data: dataOf(full) });
  const apps = await openApps();
  const close = within(apps).getByRole("button", { name: "Fechar" });
  expect(close).toHaveFocus();
  await view.rerender({ data: dataOf(processesEmpty) });
  expect(apps).toHaveAttribute("open");
  expect(apps).toHaveTextContent("Nenhum aplicativo com tráfego TCP nesta janela");
  expect(close).toHaveFocus();
  await view.rerender({ data: dataOf(disconnected) });
  expect(apps).toHaveTextContent("Sem conexão");
});

it("keeps the apps button with the reason while per-app traffic has no measurement", () => {
  render(NetworkModule, { data: dataOf(good) });
  expect(appsTrigger()).toHaveTextContent("Aplicativos de rede · sem medição ↗");
});

it("keeps the speed test slot with dashes and the reason before the first test", () => {
  render(NetworkModule, { data: { ...dataOf(full), speed: null } });
  const speed = screen.getByRole("button", { name: "Testar velocidade" });
  expect(speed).toHaveTextContent("↓ — · ↑ — Mbps · testar ↻ Sem teste ainda");
  expect(speed).toHaveAccessibleDescription("Sem teste de velocidade");
});

it("keeps the cancellation explanation and retry action until the next speed operation", async () => {
  const { default: transitions } = await import("../fixtures/operations/network-speed-transitions.json");
  const cancelled = dataOf(transitions[7]);
  const { rerender } = render(NetworkModule, { data: cancelled });
  expect(screen.getByText("Teste cancelado após suspensão do sistema")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Testar velocidade" })).toBeEnabled();
  await rerender({ data: dataOf(transitions[6]) });
  expect(screen.queryByText("Teste cancelado após suspensão do sistema")).not.toBeInTheDocument();
});
