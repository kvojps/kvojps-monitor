import { render, screen } from "@testing-library/svelte";
import { expect, it } from "vitest";
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
