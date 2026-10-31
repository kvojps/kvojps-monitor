import { render, screen } from "@testing-library/svelte";
import { expect, it } from "vitest";
import ModuloRede from "./ModuloRede.svelte";
import type { DadosRede } from "../nucleo";
import boa from "./fixtures/boa.json";
import ruim from "./fixtures/ruim.json";
import semConexao from "./fixtures/sem-conexao.json";
import vpn from "./fixtures/vpn.json";
const dados = boa.dados as DadosRede;
it("mostra qualidade, medições dos dois destinos e tráfego", () => {
  render(ModuloRede, { dados });
  expect(screen.getByText("Boa")).toBeInTheDocument();
  expect(screen.getByText("↓ 1 KB/s")).toBeInTheDocument();
  expect(screen.getByText("↑ 512 B/s")).toBeInTheDocument();
  expect(screen.getByRole("region", { name: "Gateway" })).toHaveTextContent("2 ms");
  expect(screen.getByRole("region", { name: "Internet" })).toHaveTextContent("20 ms");
  expect(screen.getByRole("img", { name: "Tráfego total nos últimos 60 s" })).toBeInTheDocument();
});

it("mostra qualidade ruim sem confundir com Segurança da rede", () => {
  render(ModuloRede, { dados: ruim.dados as DadosRede });
  expect(screen.getByText("Ruim")).toBeInTheDocument();
  expect(screen.getByRole("region", { name: "Internet" })).toHaveTextContent("301 ms");
});
it("mostra Sem conexão e não exibe medidas antigas", () => {
  render(ModuloRede, { dados: semConexao.dados as DadosRede });
  expect(screen.getAllByText("Sem conexão")).toHaveLength(2);
  expect(screen.queryByRole("region", { name: "Internet" })).not.toBeInTheDocument();
});
it("mostra as etiquetas Cabeada e VPN ativa", () => {
  render(ModuloRede, { dados: vpn.dados as DadosRede });
  expect(screen.getByText("VPN ativa")).toBeInTheDocument();
  expect(screen.getByText("Cabeada")).toBeInTheDocument();
});

it("mostra Segurança da rede independente da qualidade", () => {
  render(ModuloRede, { dados });
  expect(screen.getByRole("region", { name: "Segurança da rede" })).toHaveTextContent("Sem alertas");
});

it.each([
  ["Atenção", "Portal cativo detectado", "atencao"],
  ["Risco", "Rede aberta sem OWE", "risco"],
])("mostra %s com o motivo vindo do núcleo", async (estado, motivo, nome) => {
  const fixture = await import(`./fixtures/${nome}.json`);
  render(ModuloRede, { dados: fixture.default.dados as DadosRede });
  expect(screen.getByRole("region", { name: "Segurança da rede" })).toHaveTextContent(estado);
  expect(screen.getByText(motivo)).toBeInTheDocument();
  expect(screen.getByText("Boa")).toBeInTheDocument();
});
