import { render, screen } from "@testing-library/svelte";
import { afterEach, expect, it, vi } from "vitest";
import userEvent from "@testing-library/user-event";
import speedOffline from "./fixtures/speed-offline.json";
import speed from "./fixtures/speed.json";
import running from "./fixtures/speed-running.json";
import failed from "./fixtures/speed-failed.json";
afterEach(() => vi.restoreAllMocks());
import NetworkModule from "./NetworkModule.svelte";
import type { NetworkData } from "../core";
import good from "./fixtures/good.json";
import poor from "./fixtures/poor.json";
import disconnected from "./fixtures/disconnected.json";
import vpn from "./fixtures/vpn.json";
const data = good.data as NetworkData;
it("shows quality, measurements for both destinations and traffic", () => {
  render(NetworkModule, { data });
  expect(screen.getByText("Boa")).toBeInTheDocument();
  expect(screen.getByText("↓ 1 KB/s")).toBeInTheDocument();
  expect(screen.getByText("↑ 512 B/s")).toBeInTheDocument();
  expect(screen.getByRole("region", { name: "Gateway" })).toHaveTextContent("2 ms");
  expect(screen.getByRole("region", { name: "Internet" })).toHaveTextContent("20 ms");
  expect(screen.getByRole("img", { name: "Tráfego total nos últimos 60 s" })).toBeInTheDocument();
});

it("shows poor quality independently of network security", () => {
  render(NetworkModule, { data: poor.data as NetworkData });
  expect(screen.getByText("Ruim")).toBeInTheDocument();
  expect(screen.getByRole("region", { name: "Internet" })).toHaveTextContent("301 ms");
});
it("shows disconnected state and hides stale measurements", () => {
  render(NetworkModule, { data: disconnected.data as NetworkData });
  expect(screen.getAllByText("Sem conexão")).toHaveLength(2);
  expect(screen.queryByRole("region", { name: "Internet" })).not.toBeInTheDocument();
});
it("shows wired and VPN labels", () => {
  render(NetworkModule, { data: vpn.data as NetworkData });
  expect(screen.getByText("VPN ativa")).toBeInTheDocument();
  expect(screen.getByText("Cabeada")).toBeInTheDocument();
});

it("shows network security independently of quality", () => {
  render(NetworkModule, { data });
  expect(screen.getByRole("region", { name: "Segurança da rede" })).toHaveTextContent("Sem alertas");
});

it.each([
  ["Atenção", "Portal cativo detectado", "warning"],
  ["Risco", "Rede aberta sem OWE", "risk"],
])("shows %s with the reason supplied by the core", async (state, reason, name) => {
  const fixture = await import(`./fixtures/${name}.json`);
  render(NetworkModule, { data: fixture.default.data as NetworkData });
  expect(screen.getByRole("region", { name: "Segurança da rede" })).toHaveTextContent(state);
  expect(screen.getByText(reason)).toBeInTheDocument();
  expect(screen.getByText("Boa")).toBeInTheDocument();
});

it("keeps the last speed visible offline with its age and blocks a new test", () => {
  vi.spyOn(Date, "now").mockReturnValue(speedOffline.data.speed!.measuredAt + 7_200_000);
  render(NetworkModule, { data: speedOffline.data as NetworkData });
  expect(screen.getByRole("region", { name: "Última velocidade" })).toHaveTextContent("8 Mbps");
  expect(screen.getByText("há 2 h")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Testar velocidade" })).toBeDisabled();
});

it("requests a full test from the enabled button", async () => {
  const runSpeed = vi.fn();
  render(NetworkModule, { data: speed.data as NetworkData, runSpeed });
  await userEvent.click(screen.getByRole("button", { name: "Testar velocidade" }));
  expect(runSpeed).toHaveBeenCalledOnce();
});
it("shows test progress and a metadata-only failure", async () => {
  const view = render(NetworkModule, { data: running.data as NetworkData });
  expect(screen.getByRole("button", { name: "Testando velocidade…" })).toBeDisabled();
  await view.rerender({ data: failed.data as NetworkData });
  expect(screen.getByRole("status")).toHaveTextContent("Não foi possível medir a velocidade");
});

it("shows attributed apps and explains traffic that cannot be attributed", async () => {
  const fixture = await import("./fixtures/processes.json");
  render(NetworkModule, { data: fixture.default.data as NetworkData });
  const processes = screen.getByRole("region", { name: "Tráfego por aplicativo" });
  expect(processes).toHaveTextContent("firefox");
  expect(processes).toHaveTextContent("↓ 4 KB/s");
  expect(processes).toHaveTextContent("↑ 2 KB/s");
  expect(processes).toHaveTextContent("codex");
  expect(screen.getByText("Outros")).toHaveAttribute("title", "UDP/QUIC, processos do sistema e conexões curtas");
});

it("shows an empty TCP window with the remaining interface traffic", async () => {
  const fixture = await import("./fixtures/processes-empty.json");
  render(NetworkModule, { data: fixture.default.data as NetworkData });
  expect(screen.getByText("Nenhum aplicativo com tráfego TCP nesta janela")).toBeInTheDocument();
  expect(screen.getByText("Outros")).toBeInTheDocument();
});

it("shows pending attribution until diagnostics provide a baseline", () => {
  render(NetworkModule, { data });
  expect(screen.getByRole("region", { name: "Tráfego por aplicativo" })).toHaveTextContent("Aguardando medição");
  expect(screen.queryByText("Outros")).not.toBeInTheDocument();
});
