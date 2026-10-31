import { render, screen, fireEvent } from "@testing-library/svelte";
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
