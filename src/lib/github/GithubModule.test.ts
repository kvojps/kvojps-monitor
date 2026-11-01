import { render, screen, fireEvent, within } from "@testing-library/svelte";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import GithubModule from "./GithubModule.svelte";
import type { GithubData } from "../core";
import full from "./fixtures/full.json";
import empty from "./fixtures/empty.json";
import missingGh from "./fixtures/missing-gh.json";

/** Instant the GitHub fixtures were published (São Paulo, 31/10/2026 09:00). */
const FIXTURE_NOW = 1793448000000;

beforeEach(() => {
  vi.stubEnv("TZ", "America/Sao_Paulo");
  vi.useFakeTimers();
  vi.setSystemTime(FIXTURE_NOW);
});
afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllEnvs();
});

const data = (fixture: unknown) => fixture as GithubData;

/** The same snapshot with every day's contributions set to zero. */
function withoutActivity(fixture: GithubData): GithubData {
  const calendar = structuredClone(fixture.calendar!);
  calendar.contributionCalendar.totalContributions = 0;
  calendar.restrictedContributionsCount = 0;
  for (const week of calendar.contributionCalendar.weeks) for (const day of week.contributionDays) day.contributionCount = 0;
  return { ...fixture, calendar, featured: null };
}

it("shows the B summary: annual total, eight weeks, 14 days and featured repository", () => {
  const { container } = render(GithubModule, { data: data(full), refresh: vi.fn() });
  expect(screen.getByRole("heading", { name: "GitHub" })).toBeInTheDocument();
  expect(container).toHaveTextContent(
    /^GitHub ↻ Consultar novamente a atividade do GitHub ⚙ Abrir as configurações de conexão do GitHub 1\.608\s*contribuições no ano\s*Últimas 8 semanas\s*Atividade · últimos 14 dias\s*Repo em destaque · 27 commits \/ 7 dias\s*exemplo\/repo-1\s*Atualiza documentação$/);
});

it("shows nothing beyond B in the normal state", () => {
  render(GithubModule, { data: data(full), refresh: vi.fn() });
  expect(screen.queryByText(/privadas|pico|Atualizado|Falha|Fonte|atualizado/)).not.toBeInTheDocument();
  expect(screen.queryByRole("status")).not.toBeInTheDocument();
});

it("summarises the last eight weeks as a map, without the full year calendar", () => {
  render(GithubModule, { data: data(full), refresh: vi.fn() });
  const map = screen.getByRole("img", { name: /^Últimas 8 semanas/ });
  expect(map).toHaveAccessibleName("Últimas 8 semanas: 1.219 contribuições de 06/09 a 31/10, máximo de 63 em um dia");
  // 8 weeks of 7 days, never the 53 weeks of the snapshot.
  expect(map.querySelectorAll("[data-date]")).toHaveLength(56);
  expect(screen.getByText("Últimas 8 semanas")).toBeInTheDocument();
});

it("names each of the last 14 days' bars with its date and value", () => {
  render(GithubModule, { data: data(full), refresh: vi.fn() });
  const bars = screen.getByRole("list", { name: "Contribuições por dia nos últimos 14 dias" });
  const days = within(bars).getAllByRole("listitem");
  expect(days).toHaveLength(14);
  expect(days[0]).toHaveAccessibleName("18/10: 29 contribuições");
  expect(days[3]).toHaveAccessibleName("21/10: 35 contribuições");
  expect(days[13]).toHaveAccessibleName("31/10: 0 contribuições");
  expect(screen.getByText("Atividade · últimos 14 dias")).toBeInTheDocument();
});

it("leaves out calendar days after today", () => {
  const future = structuredClone(full) as GithubData;
  future.calendar!.contributionCalendar.weeks.push({
    contributionDays: [{ date: "2026-11-01", contributionCount: 0, color: "#ebedf0" }],
  });
  render(GithubModule, { data: future, refresh: vi.fn() });
  const days = within(screen.getByRole("list", { name: "Contribuições por dia nos últimos 14 dias" })).getAllByRole("listitem");
  expect(days.at(-1)).toHaveAccessibleName("31/10: 0 contribuições");
});

it("shows the featured repository with commits in 7 days and the last commit title", () => {
  render(GithubModule, { data: data(full), refresh: vi.fn() });
  const repo = screen.getByRole("region", { name: "Repo em destaque" });
  expect(within(repo).getByText("exemplo/repo-1")).toBeInTheDocument();
  expect(within(repo).getByText("Repo em destaque · 27 commits / 7 dias")).toBeInTheDocument();
  expect(within(repo).getByText("Atualiza documentação")).toBeInTheDocument();
});

it("omits the commit title restored from the disk cache, which never stores it", () => {
  const restored = structuredClone(full) as GithubData;
  restored.featured!.title = "";
  render(GithubModule, { data: restored, refresh: vi.fn() });
  const repo = screen.getByRole("region", { name: "Repo em destaque" });
  expect(repo).toHaveTextContent(/^Repo em destaque · 27 commits \/ 7 dias\s*exemplo\/repo-1$/);
});

it("explains an empty contribution window", () => {
  render(GithubModule, { data: data(empty), refresh: vi.fn() });
  expect(screen.getByText("Sem commits nos últimos 7 dias")).toBeInTheDocument();
});

it("shows zero activity as zero, with no demonstration numbers", () => {
  render(GithubModule, { data: withoutActivity(data(full)), refresh: vi.fn() });
  expect(screen.getByText("0")).toBeInTheDocument();
  expect(screen.getByRole("img", { name: /^Últimas 8 semanas/ })).toHaveAccessibleName("Últimas 8 semanas: sem contribuições de 06/09 a 31/10");
  expect(within(screen.getByRole("list")).getAllByRole("listitem")[0]).toHaveAccessibleName("18/10: 0 contribuições");
  expect(screen.getByText("Sem commits nos últimos 7 dias")).toBeInTheDocument();
});

it("explains a missing source without numbers and offers configuration", async () => {
  const configure = vi.fn();
  render(GithubModule, { data: data(missingGh), refresh: vi.fn(), configure });
  expect(screen.getByText("GitHub não conectado · rode gh auth login no terminal ou cole um token")).toBeInTheDocument();
  expect(screen.queryByRole("img")).not.toBeInTheDocument();
  expect(screen.queryByRole("list")).not.toBeInTheDocument();
  expect(screen.queryByText(/\d/)).not.toBeInTheDocument();
  await fireEvent.click(screen.getByRole("button", { name: "Configurar GitHub" }));
  expect(configure).toHaveBeenCalledOnce();
});

it("explains that nothing was read yet before the first snapshot", () => {
  render(GithubModule, { data: null, refresh: vi.fn() });
  expect(screen.getByText("Sem leitura da atividade do GitHub ainda")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Atualizar GitHub" })).toBeEnabled();
});

it("requests a refresh and waits ten seconds before the next one", async () => {
  const refresh = vi.fn();
  render(GithubModule, { data: data(full), refresh });
  const button = screen.getByRole("button", { name: "Atualizar GitHub" });
  await fireEvent.click(button);
  expect(refresh).toHaveBeenCalledOnce();
  expect(button).toBeDisabled();
  await vi.advanceTimersByTimeAsync(9_000);
  expect(button).toBeDisabled();
  await vi.advanceTimersByTimeAsync(1_000);
  expect(button).toBeEnabled();
});

it("shows refresh failure without data from fixture", async () => {
  const { default: error } = await import("./fixtures/error.json");
  render(GithubModule, { data: data(error), refresh: vi.fn() });
  expect(screen.getByText(/Falha ao atualizar/)).toBeInTheDocument();
  expect(screen.getByText("Sem leitura da atividade do GitHub ainda")).toBeInTheDocument();
  // Nothing was read, so it must not claim there were no commits.
  expect(screen.queryByText("Sem commits nos últimos 7 dias")).not.toBeInTheDocument();
  expect(screen.queryByRole("region", { name: "Repo em destaque" })).not.toBeInTheDocument();
});

it("keeps the annual total when the calendar has no days up to today yet", () => {
  vi.setSystemTime(new Date("2025-10-01T12:00:00-03:00"));
  const early = structuredClone(full) as GithubData;
  early.calendar!.contributionCalendar.weeks = early.calendar!.contributionCalendar.weeks.slice(-1);
  render(GithubModule, { data: early, refresh: vi.fn() });
  expect(screen.getByText("contribuições no ano")).toBeInTheDocument();
  expect(screen.queryByRole("img")).not.toBeInTheDocument();
  expect(screen.queryByText("Sem leitura da atividade do GitHub ainda")).not.toBeInTheDocument();
});

it("uses the singular for a single contribution or commit", () => {
  const one = structuredClone(full) as GithubData;
  one.calendar!.contributionCalendar.weeks.at(-1)!.contributionDays.at(-1)!.contributionCount = 1;
  one.featured!.commits = 1;
  render(GithubModule, { data: one, refresh: vi.fn() });
  const days = within(screen.getByRole("list", { name: "Contribuições por dia nos últimos 14 dias" })).getAllByRole("listitem");
  expect(days.at(-1)).toHaveAccessibleName("31/10: 1 contribuição");
  expect(screen.getByText("Repo em destaque · 1 commit / 7 dias")).toBeInTheDocument();
});

it("dims cached data and offers reconnection for refused credentials", async () => {
  const { default: invalid } = await import("./fixtures/invalid-credential.json");
  const configure = vi.fn();
  vi.setSystemTime(FIXTURE_NOW + 2 * 3_600_000);
  render(GithubModule, { data: data(invalid), refresh: vi.fn(), configure });
  expect(screen.getByText(/Credencial do GitHub inválida/)).toBeInTheDocument();
  expect(screen.getByText("exemplo/repo-1").closest("[data-stale]")).toHaveAttribute("data-stale", "true");
  expect(screen.getByText("Atualizado há 2 horas")).toBeInTheDocument();
  await fireEvent.click(screen.getByRole("button", { name: "Reconectar" }));
  expect(configure).toHaveBeenCalledOnce();
});

it("shows stale snapshot age without a failure banner", async () => {
  const { default: stale } = await import("./fixtures/stale.json");
  render(GithubModule, { data: data(stale), refresh: vi.fn() });
  expect(screen.getByText(/Atualizado/)).toBeInTheDocument();
  expect(screen.queryByText(/Falha ao atualizar/)).not.toBeInTheDocument();
});

it("shows sustained failure age and keeps the last activity", async () => {
  const { default: failure } = await import("./fixtures/failure.json");
  vi.setSystemTime(FIXTURE_NOW + 600_000);
  render(GithubModule, { data: data(failure), refresh: vi.fn() });
  expect(screen.getByText("Falha ao atualizar há 5 minutos")).toBeInTheDocument();
  expect(screen.getByText("exemplo/repo-1")).toBeInTheDocument();
  expect(screen.getByText("1.608")).toBeInTheDocument();
});

it("keeps offline cached data and age without an error banner", async () => {
  const { default: offline } = await import("./fixtures/offline.json");
  render(GithubModule, { data: data(offline), refresh: vi.fn() });
  expect(screen.getByText("exemplo/repo-1")).toBeInTheDocument();
  expect(screen.getByText(/Atualizado/)).toBeInTheDocument();
  expect(screen.queryByText(/Falha ao atualizar/)).not.toBeInTheDocument();
  expect(screen.getByRole("button", { name: "Atualizar GitHub" })).toBeDisabled();
  expect(screen.getByRole("status")).toHaveTextContent("Coleta pausada · offline ou aguardando retomada");
});


it("describes compact title actions and lets Escape dismiss their descriptions", async () => {
  render(GithubModule, { data: data(full), refresh: vi.fn() });
  const configure = screen.getByRole("button", { name: "Configurar GitHub" });
  expect(configure).toHaveAccessibleDescription("Abrir as configurações de conexão do GitHub");
  expect(screen.getByRole("button", { name: "Atualizar GitHub" })).toHaveAccessibleDescription("Consultar novamente a atividade do GitHub");
  configure.focus();
  const escape = new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true });
  configure.dispatchEvent(escape);
  await vi.advanceTimersByTimeAsync(0);
  expect(document.getElementById(configure.getAttribute("aria-describedby")!)).toHaveAttribute("hidden");
  expect(escape.defaultPrevented).toBe(true);
});
