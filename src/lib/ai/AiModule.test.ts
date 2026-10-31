import { render, screen, fireEvent, within } from "@testing-library/svelte";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import AiModule from "./AiModule.svelte";
import type { AiData } from "../core";
import full from "./fixtures/full.json";
import stale from "./fixtures/stale.json";
import reset from "./fixtures/reset.json";
import missing from "./fixtures/missing.json";
import empty from "./fixtures/empty.json";

/** Instant the fixtures were published (12:34 in São Paulo). */
const NOW = 1793460871000;

beforeEach(() => {
  vi.stubEnv("TZ", "America/Sao_Paulo");
  vi.useFakeTimers();
  vi.setSystemTime(NOW);
});
afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllEnvs();
});

it("shows plan, source, both windows and their resets", () => {
  render(AiModule, { data: full as AiData, refresh: vi.fn() });
  expect(screen.getByText("ChatGPT / Codex")).toBeInTheDocument();
  expect(screen.getByText("Plus")).toBeInTheDocument();
  expect(screen.getByText("app-server · atualizado agora")).toBeInTheDocument();
  expect(screen.getByRole("meter", { name: "Sessão (5 h)" })).toHaveAttribute("aria-valuenow", "91");
  expect(screen.getByRole("meter", { name: "Semanal" })).toHaveAttribute("aria-valuenow", "48");
  expect(screen.getByText("reinicia em 1 h 48 min")).toBeInTheDocument();
  expect(screen.getByText("reinicia qua. 14:43")).toBeInTheDocument();
});

it("keeps an old reading with a valid window and shows its age", () => {
  render(AiModule, { data: stale as AiData, refresh: vi.fn() });
  expect(screen.getByText("rollout · atualizado há 1 hora")).toBeInTheDocument();
  expect(screen.getByRole("meter", { name: "Sessão (5 h)" })).toHaveAttribute("aria-valuenow", "91");
});

it("shows a restarted window without a bar and never as 0%", () => {
  vi.setSystemTime(1793467355000 + 60_000);
  render(AiModule, { data: reset as AiData, refresh: vi.fn() });
  expect(screen.getByText("Reiniciado às 14:22 · sem leitura nova")).toBeInTheDocument();
  expect(screen.queryByRole("meter", { name: "Sessão (5 h)" })).not.toBeInTheDocument();
  expect(screen.queryByText("0%")).not.toBeInTheDocument();
  expect(screen.getByRole("meter", { name: "Semanal" })).toBeInTheDocument();
});

it("explains a missing CLI", () => {
  render(AiModule, { data: missing as AiData, refresh: vi.fn() });
  expect(screen.getByText("Codex não encontrado · instale e faça login no CLI")).toBeInTheDocument();
  expect(screen.queryByRole("meter")).not.toBeInTheDocument();
});

it("explains when there is no reading yet", () => {
  render(AiModule, { data: empty as AiData, refresh: vi.fn() });
  expect(screen.getByText("Sem leitura dos limites do Codex ainda · use o Codex ou atualize com ↻")).toBeInTheDocument();
  expect(screen.getByText("Plus")).toBeInTheDocument();
});

it("refreshes on demand and waits ten seconds between clicks", async () => {
  const refresh = vi.fn();
  render(AiModule, { data: full as AiData, refresh });
  const button = screen.getByRole("button", { name: "Atualizar IA" });
  await fireEvent.click(button);
  expect(refresh).toHaveBeenCalledOnce();
  expect(button).toBeDisabled();
  await vi.advanceTimersByTimeAsync(10_000);
  expect(button).toBeEnabled();
});

it("disables refresh while offline and shows the reading age", () => {
  render(AiModule, { data: { ...(full as AiData), paused: true }, refresh: vi.fn() });
  expect(screen.getByRole("button", { name: "Atualizar IA" })).toBeDisabled();
  expect(screen.getByText("app-server · atualizado agora")).toBeInTheDocument();
});

it("shows sustained failures and keeps the fallback reading", async () => {
  const { default: failure } = await import("./fixtures/failure.json");
  vi.setSystemTime(NOW + 90_000);
  render(AiModule, { data: failure as AiData, refresh: vi.fn() });
  expect(screen.getByText("Falha ao atualizar há 1 minuto")).toBeInTheDocument();
  expect(screen.getByText("rollout · atualizado há 1 hora")).toBeInTheDocument();
});

/** Instant the Claude fixtures were published, before both windows reset. */
const CLAUDE_NOW = 1738420000000;

it("shows Claude's plan and both windows from the status line", async () => {
  const { default: connected } = await import("./fixtures/claude-connected.json");
  vi.setSystemTime(CLAUDE_NOW);
  render(AiModule, { data: connected as AiData, refresh: vi.fn() });
  const claude = screen.getByRole("region", { name: "Claude" });
  expect(within(claude).getByText("Pro")).toBeInTheDocument();
  expect(within(claude).getByText("status line · atualizado agora")).toBeInTheDocument();
  expect(within(claude).getByRole("meter", { name: "Sessão (5 h)" })).toHaveAttribute("aria-valuenow", "24");
  expect(within(claude).getByRole("meter", { name: "Semanal" })).toHaveAttribute("aria-valuenow", "41");
  expect(within(claude).queryByText(/Status line desconectada/)).not.toBeInTheDocument();
});

it("warns when another tool replaced the status line and reconnects only on click", async () => {
  const { default: replaced } = await import("./fixtures/claude-replaced.json");
  vi.setSystemTime(CLAUDE_NOW + 3_600_000);
  const reconnect = vi.fn();
  render(AiModule, { data: replaced as AiData, refresh: vi.fn(), reconnect });
  const claude = screen.getByRole("region", { name: "Claude" });
  expect(within(claude).getByRole("status")).toHaveTextContent("Status line desconectada · Reconectar");
  expect(within(claude).getByText("status line · atualizado há 1 hora")).toBeInTheDocument();
  expect(reconnect).not.toHaveBeenCalled();
  await fireEvent.click(within(claude).getByRole("button", { name: "Reconectar" }));
  expect(reconnect).toHaveBeenCalledOnce();
});

it("asks for a Claude Code session when connected without a reading", async () => {
  const { default: claudeEmpty } = await import("./fixtures/claude-empty.json");
  render(AiModule, { data: claudeEmpty as AiData, refresh: vi.fn() });
  const claude = screen.getByRole("region", { name: "Claude" });
  expect(within(claude).getByText("Abra uma sessão do Claude Code para ler os limites")).toBeInTheDocument();
  expect(within(claude).queryByRole("meter")).not.toBeInTheDocument();
});

it("points to Settings before connecting to Claude Code", async () => {
  const { default: off } = await import("./fixtures/claude-off.json");
  const configure = vi.fn();
  render(AiModule, { data: off as AiData, refresh: vi.fn(), configure });
  const claude = screen.getByRole("region", { name: "Claude" });
  expect(within(claude).getByText(/Conecte o app ao Claude Code para ler os limites/)).toBeInTheDocument();
  await fireEvent.click(within(claude).getByRole("button", { name: "Configurar" }));
  expect(configure).toHaveBeenCalledOnce();
});

it("explains a missing Claude Code", async () => {
  const { default: claudeMissing } = await import("./fixtures/claude-missing.json");
  render(AiModule, { data: claudeMissing as AiData, refresh: vi.fn() });
  expect(screen.getByText("Claude Code não encontrado · instale e faça login no CLI")).toBeInTheDocument();
  expect(screen.queryByRole("button", { name: "Configurar" })).not.toBeInTheDocument();
});
