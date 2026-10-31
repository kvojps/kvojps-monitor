import { fireEvent, render, screen } from "@testing-library/svelte";
import { expect, it, vi } from "vitest";
import AiSettings from "./AiSettings.svelte";
import type { AiData } from "../core";
import off from "./fixtures/claude-off.json";
import connected from "./fixtures/claude-connected.json";
import replaced from "./fixtures/claude-replaced.json";

it("connects to Claude Code only on click", async () => {
  const connect = vi.fn().mockResolvedValue(undefined);
  render(AiSettings, { data: off as AiData, connect, disconnect: vi.fn() });
  expect(connect).not.toHaveBeenCalled();
  await fireEvent.click(screen.getByRole("button", { name: "Conectar ao Claude Code" }));
  expect(connect).toHaveBeenCalledOnce();
});

it("disconnects when connected", async () => {
  const disconnect = vi.fn().mockResolvedValue(undefined);
  render(AiSettings, { data: connected as AiData, connect: vi.fn(), disconnect });
  expect(screen.getByText("Conectado ao Claude Code pela status line")).toBeInTheDocument();
  await fireEvent.click(screen.getByRole("button", { name: "Desconectar" }));
  expect(disconnect).toHaveBeenCalledOnce();
});

it("offers to reconnect after another tool replaced the status line", async () => {
  const connect = vi.fn().mockResolvedValue(undefined);
  render(AiSettings, { data: replaced as AiData, connect, disconnect: vi.fn() });
  expect(screen.getByRole("status")).toHaveTextContent("Status line desconectada");
  await fireEvent.click(screen.getByRole("button", { name: "Reconectar" }));
  expect(connect).toHaveBeenCalledOnce();
});

it("explains settings it refused to change", async () => {
  render(AiSettings, { data: off as AiData, connect: vi.fn().mockRejectedValue("invalidSettings"), disconnect: vi.fn() });
  await fireEvent.click(screen.getByRole("button", { name: "Conectar ao Claude Code" }));
  expect(await screen.findByRole("alert")).toHaveTextContent("~/.claude/settings.json não é um JSON válido; nada foi alterado");
});
