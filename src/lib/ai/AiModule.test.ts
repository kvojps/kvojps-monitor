import { render, screen, fireEvent, within } from "@testing-library/svelte";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import AiModule from "./AiModule.svelte";
import type { AiData } from "../core";
import usage from "./fixtures/claude-usage.json";

/** Instant the oauth/usage fixtures were published (São Paulo). */
const USAGE_NOW = 1793401200000;

beforeEach(() => {
  vi.stubEnv("TZ", "America/Sao_Paulo");
  vi.useFakeTimers();
  vi.setSystemTime(USAGE_NOW);
});
afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllEnvs();
});

const provider = (name: string) => screen.getByRole("region", { name });

it("shows a five hour instrument per provider with usage and reset", () => {
  render(AiModule, { data: usage as AiData, refresh: vi.fn() });
  const codex = within(provider("Codex")).getByRole("meter", { name: "Codex · janela de 5 h" });
  expect(codex).toHaveAttribute("aria-valuenow", "91");
  expect(codex).toHaveTextContent("91%");
  expect(within(provider("Codex")).getByText("reinicia em 18 h 23 min")).toBeInTheDocument();
  const claude = within(provider("Claude")).getByRole("meter", { name: "Claude · janela de 5 h" });
  expect(claude).toHaveAttribute("aria-valuenow", "15");
  expect(within(provider("Claude")).getByText("reinicia em 3 h 30 min")).toBeInTheDocument();
});

it("shows a weekly bar per provider from the all models bucket only", () => {
  render(AiModule, { data: usage as AiData, refresh: vi.fn() });
  const codex = within(provider("Codex")).getByRole("meter", { name: "Codex · semana" });
  expect(codex).toHaveAttribute("aria-valuenow", "48");
  expect(within(provider("Codex")).getByText("48%")).toBeInTheDocument();
  // Claude's per model weekly limits (Sonnet 33%) stay out of the panel.
  expect(within(provider("Claude")).getAllByRole("meter", { name: "Claude · semana" })
    .map((meter) => meter.getAttribute("aria-valuenow"))).toEqual(["6"]);
  expect(screen.queryByText("Sonnet")).not.toBeInTheDocument();
  expect(screen.queryByText("33%")).not.toBeInTheDocument();
});

it("identifies each reading by source and age, without plans", () => {
  vi.setSystemTime(USAGE_NOW + 420_000);
  render(AiModule, { data: usage as AiData, refresh: vi.fn() });
  expect(within(provider("Claude")).getByText("oauth/usage · atualizado há 7 minutos")).toBeInTheDocument();
  expect(within(provider("Codex")).getByText("app-server · atualizado agora")).toBeInTheDocument();
  expect(screen.queryByText("Pro")).not.toBeInTheDocument();
  expect(screen.queryByText("Plus")).not.toBeInTheDocument();
});

it("removes a restarted window's arc and never shows it as 0%", async () => {
  const { default: reset } = await import("./fixtures/reset.json");
  vi.setSystemTime(1793467355000 + 60_000);
  render(AiModule, { data: reset as AiData, refresh: vi.fn() });
  const codex = provider("Codex");
  expect(within(codex).getByText("Reiniciado às 14:22 · sem leitura nova")).toBeInTheDocument();
  expect(within(codex).queryByRole("meter", { name: "Codex · janela de 5 h" })).not.toBeInTheDocument();
  expect(within(codex).getByRole("img", { name: "Codex · janela de 5 h: Reiniciado às 14:22 · sem leitura nova" })).toBeInTheDocument();
  expect(within(codex).queryByText(/^0/)).not.toBeInTheDocument();
  expect(within(codex).getByRole("meter", { name: "Codex · semana" })).toHaveAttribute("aria-valuenow", "48");
});

it("removes a restarted weekly bar and keeps the five hour reading", async () => {
  const { default: weeklyReset } = await import("./fixtures/weekly-reset.json");
  render(AiModule, { data: weeklyReset as AiData, refresh: vi.fn() });
  const codex = provider("Codex");
  expect(within(codex).queryByRole("meter", { name: "Codex · semana" })).not.toBeInTheDocument();
  expect(within(codex).getByText("Reiniciado às 14:00 · sem leitura nova")).toBeInTheDocument();
  expect(within(codex).getByRole("meter", { name: "Codex · janela de 5 h" })).toHaveAttribute("aria-valuenow", "91");
});

it("keeps an old reading with its age", async () => {
  const { default: stale } = await import("./fixtures/stale.json");
  vi.setSystemTime(1793460871000);
  render(AiModule, { data: stale as AiData, refresh: vi.fn() });
  expect(within(provider("Codex")).getByText("rollout · atualizado há 1 hora")).toBeInTheDocument();
  expect(within(provider("Codex")).getByRole("meter", { name: "Codex · janela de 5 h" })).toHaveAttribute("aria-valuenow", "91");
});

it("replaces a missing CLI's instruments with an explanation, keeping the other provider", async () => {
  const { default: missing } = await import("./fixtures/missing.json");
  const data = { ...(missing as AiData), claude: usage.claude as AiData["claude"] };
  render(AiModule, { data, refresh: vi.fn() });
  const codex = provider("Codex");
  expect(within(codex).getByText("Codex não encontrado · instale e faça login no CLI")).toBeInTheDocument();
  expect(within(codex).queryByRole("meter")).not.toBeInTheDocument();
  expect(within(provider("Claude")).getByRole("meter", { name: "Claude · janela de 5 h" })).toBeInTheDocument();
});

it("explains when Codex has no reading yet", async () => {
  const { default: empty } = await import("./fixtures/empty.json");
  render(AiModule, { data: empty as AiData, refresh: vi.fn() });
  expect(within(provider("Codex")).getByText("Sem leitura dos limites do Codex ainda · use o Codex ou atualize com ↻")).toBeInTheDocument();
  expect(within(provider("Codex")).queryByRole("meter")).not.toBeInTheDocument();
});

it("shows sustained Codex failures and keeps the last reading with its age", async () => {
  const { default: failure } = await import("./fixtures/failure.json");
  vi.setSystemTime(1793460871000 + 90_000);
  render(AiModule, { data: failure as AiData, refresh: vi.fn() });
  const codex = provider("Codex");
  expect(within(codex).getByRole("status")).toHaveTextContent("Falha ao atualizar há 1 minuto");
  expect(within(codex).getByText("rollout · atualizado há 1 hora")).toBeInTheDocument();
  expect(within(codex).getByRole("meter", { name: "Codex · janela de 5 h" })).toBeInTheDocument();
});

it("hides Codex failures while paused offline", async () => {
  const { default: failure } = await import("./fixtures/failure.json");
  render(AiModule, { data: { ...(failure as AiData), paused: true }, refresh: vi.fn() });
  expect(screen.queryByText(/Falha ao atualizar/)).not.toBeInTheDocument();
});

/** Instant the status line fixtures were published, before both windows reset. */
const STATUS_LINE_NOW = 1738420000000;

it("shows Claude's windows from the status line", async () => {
  const { default: connected } = await import("./fixtures/claude-connected.json");
  vi.setSystemTime(STATUS_LINE_NOW);
  render(AiModule, { data: connected as AiData, refresh: vi.fn() });
  const claude = provider("Claude");
  expect(within(claude).getByText("status line · atualizado agora")).toBeInTheDocument();
  expect(within(claude).getByRole("meter", { name: "Claude · janela de 5 h" })).toHaveAttribute("aria-valuenow", "24");
  expect(within(claude).getByRole("meter", { name: "Claude · semana" })).toHaveAttribute("aria-valuenow", "41");
  expect(within(claude).queryByRole("status")).not.toBeInTheDocument();
});

it("warns when another tool replaced the status line and reconnects only on click", async () => {
  const { default: replaced } = await import("./fixtures/claude-replaced.json");
  vi.setSystemTime(STATUS_LINE_NOW + 3_600_000);
  const reconnect = vi.fn();
  render(AiModule, { data: replaced as AiData, refresh: vi.fn(), reconnect });
  const claude = provider("Claude");
  expect(within(claude).getByRole("status")).toHaveTextContent("Status line desconectada · Reconectar");
  expect(within(claude).getByText("status line · atualizado há 1 hora")).toBeInTheDocument();
  expect(reconnect).not.toHaveBeenCalled();
  await fireEvent.click(within(claude).getByRole("button", { name: "Reconectar" }));
  expect(reconnect).toHaveBeenCalledOnce();
});

it("asks for a Claude Code session when connected without a reading", async () => {
  const { default: claudeEmpty } = await import("./fixtures/claude-empty.json");
  render(AiModule, { data: claudeEmpty as AiData, refresh: vi.fn() });
  const claude = provider("Claude");
  expect(within(claude).getByText("Abra uma sessão do Claude Code para ler os limites")).toBeInTheDocument();
  expect(within(claude).queryByRole("meter")).not.toBeInTheDocument();
  expect(within(provider("Codex")).getByRole("meter", { name: "Codex · janela de 5 h" })).toBeInTheDocument();
});

it("points to Settings before connecting to Claude Code", async () => {
  const { default: off } = await import("./fixtures/claude-off.json");
  const configure = vi.fn();
  render(AiModule, { data: off as AiData, refresh: vi.fn(), configure });
  const claude = provider("Claude");
  expect(within(claude).getByText(/Conecte o app ao Claude Code para ler os limites/)).toBeInTheDocument();
  expect(within(claude).queryByRole("meter")).not.toBeInTheDocument();
  await fireEvent.click(within(claude).getByRole("button", { name: "Configurar" }));
  expect(configure).toHaveBeenCalledOnce();
});

it("explains a missing Claude Code without a way to configure it", async () => {
  const { default: claudeMissing } = await import("./fixtures/claude-missing.json");
  render(AiModule, { data: claudeMissing as AiData, refresh: vi.fn() });
  const claude = provider("Claude");
  expect(within(claude).getByText("Claude Code não encontrado · instale e faça login no CLI")).toBeInTheDocument();
  expect(within(claude).queryByRole("button")).not.toBeInTheDocument();
});

it("shows an expired Claude token and keeps the status line reading", async () => {
  const { default: expired } = await import("./fixtures/claude-expired.json");
  vi.setSystemTime(STATUS_LINE_NOW);
  render(AiModule, { data: expired as AiData, refresh: vi.fn() });
  const claude = provider("Claude");
  expect(within(claude).getByRole("status")).toHaveTextContent("Token do Claude expirado · abra o Claude Code");
  expect(within(claude).getByText("status line · atualizado agora")).toBeInTheDocument();
  expect(within(claude).getByRole("meter", { name: "Claude · janela de 5 h" })).toHaveAttribute("aria-valuenow", "20");
});

it("shows sustained oauth/usage failures and keeps the last reading", async () => {
  const { default: failure } = await import("./fixtures/claude-failure.json");
  vi.setSystemTime(USAGE_NOW + 420_000);
  render(AiModule, { data: failure as AiData, refresh: vi.fn() });
  const claude = provider("Claude");
  expect(within(claude).getByRole("status")).toHaveTextContent("Falha ao atualizar há 2 minutos");
  expect(within(claude).getByText("oauth/usage · atualizado há 7 minutos")).toBeInTheDocument();
  expect(within(provider("Codex")).queryByRole("status")).not.toBeInTheDocument();
});

it("hides oauth/usage failures while Claude is paused offline", async () => {
  const { default: failure } = await import("./fixtures/claude-failure.json");
  const paused = { ...(failure as AiData), claude: { ...(failure as AiData).claude!, paused: true } };
  render(AiModule, { data: paused, refresh: vi.fn() });
  expect(screen.queryByText(/Falha ao atualizar/)).not.toBeInTheDocument();
});

it("refreshes on demand and waits ten seconds between clicks", async () => {
  const refresh = vi.fn();
  render(AiModule, { data: usage as AiData, refresh });
  const button = screen.getByRole("button", { name: "Atualizar IA" });
  await fireEvent.click(button);
  expect(refresh).toHaveBeenCalledOnce();
  expect(button).toBeDisabled();
  await vi.advanceTimersByTimeAsync(9_000);
  expect(button).toBeDisabled();
  await vi.advanceTimersByTimeAsync(1_000);
  expect(button).toBeEnabled();
});

it("disables refresh while refreshing or offline and keeps the reading age", () => {
  const { unmount } = render(AiModule, { data: { ...(usage as AiData), refreshing: true }, refresh: vi.fn() });
  expect(screen.getByRole("button", { name: "Atualizar IA" })).toBeDisabled();
  expect(screen.getByRole("status")).toHaveTextContent("Atualizando…");
  unmount();
  render(AiModule, { data: { ...(usage as AiData), paused: true }, refresh: vi.fn() });
  expect(screen.getByRole("button", { name: "Atualizar IA" })).toBeDisabled();
  expect(within(provider("Claude")).getByText("oauth/usage · atualizado agora")).toBeInTheDocument();
});

it("opens Settings from the module", async () => {
  const configure = vi.fn();
  render(AiModule, { data: usage as AiData, refresh: vi.fn(), configure });
  await fireEvent.click(screen.getByRole("button", { name: "Configurar IA" }));
  expect(configure).toHaveBeenCalledOnce();
});

it("names high usage in text, not only by colour", () => {
  render(AiModule, { data: usage as AiData, refresh: vi.fn() });
  const codex = within(provider("Codex")).getByRole("meter", { name: "Codex · janela de 5 h" });
  expect(codex).toHaveAttribute("aria-valuetext", "91% · uso alto");
  expect(codex).toHaveTextContent("uso alto");
  expect(within(provider("Codex")).getByRole("meter", { name: "Codex · semana" })).toHaveAttribute("aria-valuetext", "48%");
});

it("explains a window missing from the reading instead of guessing it", () => {
  const onlyWeekly = structuredClone(usage) as AiData;
  // A reading whose only window is weekly: no five hour value to show.
  onlyWeekly.codex.reading!.buckets[0].primary = null;
  render(AiModule, { data: onlyWeekly, refresh: vi.fn() });
  const codex = provider("Codex");
  expect(within(codex).getByRole("img", { name: "Codex · janela de 5 h: Sem leitura dos limites" })).toBeInTheDocument();
  expect(within(codex).getByRole("meter", { name: "Codex · semana" })).toHaveAttribute("aria-valuenow", "48");
});

it("never labels a window of another duration as five hours or weekly", () => {
  const other = structuredClone(usage) as AiData;
  other.codex.reading!.buckets[0].primary = { usedPercent: 40, durationMins: 60, resetsAt: null, reset: false };
  render(AiModule, { data: other, refresh: vi.fn() });
  expect(within(provider("Codex")).queryByRole("meter", { name: "Codex · janela de 5 h" })).not.toBeInTheDocument();
  expect(within(provider("Codex")).queryByText("40%")).not.toBeInTheDocument();
});

it("describes compact title actions on focus and pointer without changing focus on update", async () => {
  const { rerender } = render(AiModule, { data: usage as AiData, refresh: vi.fn() });
  const refresh = screen.getByRole("button", { name: "Atualizar IA" });
  expect(refresh).toHaveAccessibleDescription("Consultar os limites de uso de Claude e Codex");
  expect(screen.getByRole("button", { name: "Configurar IA" })).toHaveAccessibleDescription("Abrir conexões e consentimentos de IA nas Configurações");
  refresh.focus();
  expect(refresh).toHaveFocus();
  await rerender({ data: { ...(usage as AiData), refreshing: true } });
  expect(refresh).toHaveFocus();
});

it("identifies offline providers while preserving cached instruments and reading time", async () => {
  const { default: offline } = await import("./fixtures/offline.json");
  render(AiModule, { data: offline as AiData, refresh: vi.fn() });
  for (const name of ["Codex", "Claude"]) {
    const region = within(provider(name));
    expect(region.getByRole("status")).toHaveTextContent("Coleta pausada · offline ou aguardando retomada");
    expect(region.getByRole("meter", { name: `${name} · janela de 5 h` })).toBeInTheDocument();
    expect(region.getByText(/atualizado/)).toBeInTheDocument();
    expect(region.getByText(/Leitura às/)).toBeInTheDocument();
  }
});

it("distinguishes loading without a reading from a completed empty snapshot", async () => {
  const { default: loading } = await import("./fixtures/loading.json");
  const { rerender } = render(AiModule, { data: loading as AiData, refresh: vi.fn() });
  expect(screen.getByRole("status")).toHaveTextContent("Atualizando…");
  expect(screen.queryByRole("meter")).not.toBeInTheDocument();
  await rerender({ data: { ...(loading as AiData), refreshing: false } });
  expect(screen.queryByText("Atualizando…")).not.toBeInTheDocument();
  expect(within(provider("Codex")).getByText(/Sem leitura/)).toBeInTheDocument();
});

it("explains a missing weekly value without claiming a restart", async () => {
  const { default: unavailable } = await import("./fixtures/weekly-unavailable.json");
  render(AiModule, { data: unavailable as AiData, refresh: vi.fn() });
  expect(within(provider("Codex")).queryByRole("meter", { name: "Codex · semana" })).not.toBeInTheDocument();
  expect(within(provider("Codex")).getByText("Sem leitura dos limites")).toBeInTheDocument();
  expect(within(provider("Codex")).queryByText(/Reiniciado/)).not.toBeInTheDocument();
});


it("removes only the elapsed window while awaiting a new snapshot", async () => {
  vi.setSystemTime(1793413799000);
  render(AiModule, { data: usage as AiData, refresh: vi.fn() });
  expect(within(provider("Claude")).getByRole("meter", { name: "Claude · janela de 5 h" })).toHaveAttribute("aria-valuenow", "15");
  await vi.advanceTimersByTimeAsync(1000);
  expect(within(provider("Claude")).queryByRole("meter", { name: "Claude · janela de 5 h" })).not.toBeInTheDocument();
  expect(within(provider("Claude")).getByText("Reiniciado às 23:30 · sem leitura nova")).toBeInTheDocument();
  expect(within(provider("Claude")).getByRole("meter", { name: "Claude · semana" })).toHaveAttribute("aria-valuenow", "6");
  expect(within(provider("Codex")).getByRole("meter", { name: "Codex · janela de 5 h" })).toHaveAttribute("aria-valuenow", "91");
});


it("dismisses action descriptions with Escape before the panel handles it", async () => {
  render(AiModule, { data: usage as AiData, refresh: vi.fn() });
  const configure = screen.getByRole("button", { name: "Configurar IA" });
  configure.focus();
  const escape = new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true });
  configure.dispatchEvent(escape);
  await fireEvent.keyUp(configure, { key: "Escape" });
  expect(escape.defaultPrevented).toBe(true);
  expect(screen.queryByRole("tooltip")).not.toBeInTheDocument();
  expect(configure).toHaveFocus();
});
