import { tick } from "svelte";
import { act, render, screen } from "@testing-library/svelte";
import { afterEach, expect, it, vi } from "vitest";
import OperationsStrip from "./OperationsStrip.svelte";
import type { Snapshot } from "./core";

const snapshot = (id: number, state: string, origin = "user"): Snapshot => ({ module: "github", updatedAt: null, data: { operation: { id, state, origin }, title: "SECRET" } });
afterEach(() => vi.useRealTimers());

it("keeps a requested operation until its actual result and expires only the result", async () => {
  vi.useFakeTimers();
  const { rerender } = render(OperationsStrip, { snapshots: {}, ready: true });
  await rerender({ snapshots: { github: snapshot(1, "running") }, ready: true });
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("GitHub · Estabelecendo enlace…");
  await act(() => vi.advanceTimersByTime(60_000));
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Estabelecendo enlace");
  await rerender({ snapshots: { github: snapshot(1, "succeeded") }, ready: true });
  await act(() => tick());
  expect(screen.getByRole("status")).toHaveTextContent("GitHub · Telemetria atualizada.");
  expect(screen.getByLabelText("Faixa de bordo")).not.toHaveTextContent("SECRET");
  await act(() => vi.advanceTimersByTime(3999));
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Telemetria atualizada");
  await act(() => vi.advanceTimersByTime(1));
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Bordo em repouso");
});

it("establishes an initial reference and never replays repeated snapshots or timestamps", async () => {
  vi.useFakeTimers();
  const existing = snapshot(10, "failed", "automatic");
  const { rerender } = render(OperationsStrip, { snapshots: { github: existing }, ready: false });
  await rerender({ snapshots: { github: existing }, ready: true });
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Bordo em repouso");
  expect(screen.getByRole("status").textContent).toBe("");
  await rerender({ snapshots: { github: { ...existing, updatedAt: 99999 } }, ready: true });
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Bordo em repouso");
  await rerender({ snapshots: { github: snapshot(11, "recovered", "automatic") }, ready: true });
  await act(() => tick());
  expect(screen.getByRole("status")).toHaveTextContent("Atualização restabelecida");
  await act(() => vi.advanceTimersByTime(4000));
  await rerender({ snapshots: { github: snapshot(11, "recovered", "automatic") }, ready: true });
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Bordo em repouso");
});

it("keeps automatic refreshes silent, shows resume visually, and announces its real failure", async () => {
  const { rerender } = render(OperationsStrip, { snapshots: {}, ready: true });
  for (const state of ["running", "succeeded"]) {
    await rerender({ snapshots: { github: snapshot(1, state, "automatic") }, ready: true });
    expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Bordo em repouso");
    expect(screen.getByRole("status").textContent).toBe("");
  }
  await rerender({ snapshots: { github: snapshot(2, "running", "resume") }, ready: true });
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Retomando atualização");
  expect(screen.getByRole("status").textContent).toBe("");
  await rerender({ snapshots: { github: snapshot(2, "failed", "resume") }, ready: true });
  await act(() => tick());
  expect(screen.getByRole("status")).toHaveTextContent("Falha ao atualizar");
});

it("does not infer a result from refreshing or updatedAt and does not extend repeated results", async () => {
  vi.useFakeTimers();
  const { rerender } = render(OperationsStrip, { snapshots: {}, ready: true });
  await rerender({ snapshots: { github: snapshot(1, "running") }, ready: true });
  await rerender({ snapshots: { github: { ...snapshot(1, "running"), data: { operation: { id: 1, origin: "user", state: "running" }, refreshing: false }, updatedAt: 123 } }, ready: true });
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Estabelecendo enlace");
  await rerender({ snapshots: { github: snapshot(1, "cancelled") }, ready: true });
  await act(() => tick());
  expect(screen.getByRole("status")).toHaveTextContent("Operação cancelada");
  expect(screen.getByLabelText("Faixa de bordo")).not.toHaveTextContent("Telemetria atualizada");
  await act(() => vi.advanceTimersByTime(3000));
  await rerender({ snapshots: { github: snapshot(1, "cancelled") }, ready: true });
  await act(() => vi.advanceTimersByTime(1000));
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Bordo em repouso");
});

it("groups six simultaneous sources, prioritizes failure and pause, and sanitizes announcements", async () => {
  const { default: fixture } = await import("./fixtures/operations/simultaneous.json");
  const { rerender } = render(OperationsStrip, { snapshots: {}, ready: true });
  const snapshots = Object.fromEntries(fixture.map(s => [s.module, s]));
  await rerender({ snapshots, ready: true });
  const strip = screen.getByLabelText("Faixa de bordo");
  expect(strip).toHaveTextContent("GitHub · Falha ao atualizar. Confira o Módulo.Trello · Atualização pausada. Confira o Módulo.Agentes · Atenção. Confira o Módulo. +3 Módulos");
  await act(() => tick());
  expect(screen.getByRole("status")).toHaveTextContent("Sistema · Atualização restabelecida");
  expect(screen.getByRole("status")).not.toHaveTextContent("Estabelecendo enlace");
  expect(document.body).not.toHaveTextContent("SENTINEL");
});

it("announces identical results of distinct requested operations again", async () => {
  const { rerender } = render(OperationsStrip, { snapshots: {}, ready: true });
  await rerender({ snapshots: { github: snapshot(1, "succeeded") }, ready: true });
  await act(() => tick());
  const live = screen.getByRole("status");
  const delivered: string[] = [];
  const observer = new MutationObserver(() => delivered.push(live.textContent ?? ""));
  observer.observe(live, { subtree: true, childList: true, characterData: true });
  await rerender({ snapshots: { github: snapshot(2, "succeeded") }, ready: true });
  await act(() => tick());
  observer.disconnect();
  expect(delivered).toContain("GitHub · Telemetria atualizada.");
});

it("keeps concurrent AI providers independent before grouping their summaries", async () => {
  vi.useFakeTimers();
  const { rerender } = render(OperationsStrip, { snapshots: {}, ready: true });
  const { default: transitions } = await import("./fixtures/operations/ai-transitions.json");
  const first: Snapshot = { ...transitions[0], module: "ai" };
  const second: Snapshot = { ...transitions[1], module: "ai" };
  await rerender({ snapshots: { ai: first }, ready: true });
  const strip = screen.getByLabelText("Faixa de bordo");
  expect(strip).toHaveTextContent("Claude · Falha ao atualizar");
  expect(strip).toHaveTextContent("Codex · Estabelecendo enlace");
  await rerender({ snapshots: { ai: second }, ready: true });
  expect(strip).toHaveTextContent("Claude · Falha ao atualizar");
  expect(strip).toHaveTextContent("Codex · Telemetria atualizada");
  expect(document.body).not.toHaveTextContent("SENTINEL");
  await act(() => vi.advanceTimersByTime(4000));
  expect(strip).toHaveTextContent("Bordo em repouso");
});


it("uses AI provider identities for recovery, pause, resume, repetition and reopening", async () => {
  vi.useFakeTimers();
  const { default: transitions } = await import("./fixtures/operations/ai-transitions.json");
  const state = (index: number): Snapshot => ({ ...transitions[index], module: "ai" });
  const { rerender, unmount } = render(OperationsStrip, { snapshots: {}, ready: true });
  await rerender({ snapshots: { ai: state(2) }, ready: true });
  await act(() => tick());
  expect(screen.getByRole("status")).toHaveTextContent("Claude · Atualização restabelecida");
  expect(screen.getByRole("status")).not.toHaveTextContent("Codex");
  await rerender({ snapshots: { ai: state(3) }, ready: true });
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Codex · Atualização pausada");
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Claude · Atualização pausada");
  await act(() => vi.advanceTimersByTime(3000));
  await rerender({ snapshots: { ai: state(3) }, ready: true });
  await act(() => vi.advanceTimersByTime(1000));
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Bordo em repouso");
  await rerender({ snapshots: { ai: state(4) }, ready: true });
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Codex · Retomando atualização");
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Claude · Retomando atualização");
  await act(() => vi.advanceTimersByTime(60_000));
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Retomando atualização");
  // A discarded request has no fabricated result or accessible announcement.
  await rerender({ snapshots: { ai: { module: "ai", updatedAt: null, data: { codex: {}, claude: {} } } }, ready: true });
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Bordo em repouso");
  expect(screen.getByRole("status")).not.toHaveTextContent(/Telemetria|cancelada/);
  unmount();
  render(OperationsStrip, { snapshots: { ai: state(2) }, ready: true });
  expect(screen.getByLabelText("Faixa de bordo")).toHaveTextContent("Bordo em repouso");
  expect(screen.getByRole("status").textContent).toBe("");
});
