import { fireEvent, render, screen } from "@testing-library/svelte";
import { expect, it, vi } from "vitest";
import AiSettings from "./AiSettings.svelte";
import type { AiData } from "../core";
import off from "./fixtures/claude-off.json";
import connected from "./fixtures/claude-connected.json";
import replaced from "./fixtures/claude-replaced.json";
import usage from "./fixtures/claude-usage.json";
import expired from "./fixtures/claude-expired.json";

it("connects to Claude Code only on click", async () => {
  const connect = vi.fn().mockResolvedValue(undefined);
  render(AiSettings, { data: off as AiData, connect, disconnect: vi.fn(), setUsage: vi.fn() });
  expect(connect).not.toHaveBeenCalled();
  await fireEvent.click(screen.getByRole("button", { name: "Conectar ao Claude Code" }));
  expect(connect).toHaveBeenCalledOnce();
});

it("disconnects when connected", async () => {
  const disconnect = vi.fn().mockResolvedValue(undefined);
  render(AiSettings, { data: connected as AiData, connect: vi.fn(), disconnect, setUsage: vi.fn() });
  expect(screen.getByText("Conectado ao Claude Code pela status line")).toBeInTheDocument();
  await fireEvent.click(screen.getByRole("button", { name: "Desconectar" }));
  expect(disconnect).toHaveBeenCalledOnce();
});

it("offers to reconnect after another tool replaced the status line", async () => {
  const connect = vi.fn().mockResolvedValue(undefined);
  render(AiSettings, { data: replaced as AiData, connect, disconnect: vi.fn(), setUsage: vi.fn() });
  expect(screen.getByRole("status")).toHaveTextContent("Status line desconectada");
  await fireEvent.click(screen.getByRole("button", { name: "Reconectar" }));
  expect(connect).toHaveBeenCalledOnce();
});

it("explains settings it refused to change", async () => {
  render(AiSettings, { data: off as AiData, connect: vi.fn().mockRejectedValue("invalidSettings"), disconnect: vi.fn(), setUsage: vi.fn() });
  await fireEvent.click(screen.getByRole("button", { name: "Conectar ao Claude Code" }));
  expect(await screen.findByRole("alert")).toHaveTextContent("~/.claude/settings.json não é um JSON válido; nada foi alterado");
});

it("turns oauth/usage on only after the warning is confirmed", async () => {
  const setUsage = vi.fn().mockResolvedValue(undefined);
  render(AiSettings, { data: off as AiData, connect: vi.fn(), disconnect: vi.fn(), setUsage });
  const toggle = () => screen.getByRole("button", { name: "Ler via oauth/usage…" });
  expect(screen.queryByRole("alertdialog")).not.toBeInTheDocument();
  await fireEvent.click(toggle());
  const warning = screen.getByRole("alertdialog", { name: "Endpoint não documentado" });
  expect(warning).toHaveTextContent("não documentado");
  expect(warning).toHaveTextContent("zona cinzenta nos termos da Anthropic");
  expect(warning).toHaveTextContent("nunca renova");
  expect(setUsage).not.toHaveBeenCalled();
  await fireEvent.click(screen.getByRole("button", { name: "Cancelar" }));
  expect(screen.queryByRole("alertdialog")).not.toBeInTheDocument();
  expect(setUsage).not.toHaveBeenCalled();
  await fireEvent.click(toggle());
  await fireEvent.click(screen.getByRole("button", { name: "Entendi o risco, ligar" }));
  expect(setUsage).toHaveBeenCalledExactlyOnceWith(true);
});

it("turns oauth/usage off at once", async () => {
  const setUsage = vi.fn().mockResolvedValue(undefined);
  render(AiSettings, { data: usage as AiData, connect: vi.fn(), disconnect: vi.fn(), setUsage });
  expect(screen.getByText("Leitura via oauth/usage ligada · a cada 5 min")).toBeInTheDocument();
  await fireEvent.click(screen.getByRole("button", { name: "Desligar oauth/usage" }));
  expect(setUsage).toHaveBeenCalledExactlyOnceWith(false);
});

it("explains an expired token in Settings", () => {
  render(AiSettings, { data: expired as AiData, connect: vi.fn(), disconnect: vi.fn(), setUsage: vi.fn() });
  expect(screen.getByText("Token do Claude expirado · abra o Claude Code")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Desligar oauth/usage" })).toBeInTheDocument();
});
