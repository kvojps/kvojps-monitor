import { render, screen, fireEvent } from "@testing-library/svelte";
import { expect, it, vi } from "vitest";
import GithubModule from "./GithubModule.svelte";
import full from "./fixtures/full.json";
import empty from "./fixtures/empty.json";
import missingGh from "./fixtures/missing-gh.json";
it("shows calendar, counts and latest commit", () => {
  render(GithubModule, { data: full, refresh: vi.fn() });
  expect(screen.getByText("exemplo/repo-1")).toBeInTheDocument();
  expect(screen.getByText("27 commits em 7 dias · 27 no total")).toBeInTheDocument();
  expect(screen.getByText("Atualiza documentação")).toBeInTheDocument();
});
it("explains an empty contribution window", () => {
  render(GithubModule, { data: empty, refresh: vi.fn() });
  expect(screen.getByText("Sem commits nos últimos 7 dias")).toBeInTheDocument();
});
it("offers login instructions without gh and requests refresh", async () => {
  const refresh = vi.fn();
  render(GithubModule, { data: missingGh, refresh });
  await fireEvent.click(screen.getByText("Configurar"));
  expect(screen.getByText("Rodar `gh auth login`")).toBeInTheDocument();
  await fireEvent.click(screen.getByRole("button", {name:"Atualizar GitHub"}));
  expect(refresh).toHaveBeenCalledOnce();
});
it("shows refresh failure from fixture", async () => {
  const { default: error } = await import("./fixtures/error.json");
  render(GithubModule, { data: error, refresh: vi.fn() });
  expect(screen.getByText(/Falha ao atualizar/)).toBeInTheDocument();
});
it("disables refresh during the ten second cooldown", async () => {
  vi.useFakeTimers();
  try {
    const refresh = vi.fn();
    render(GithubModule, { data: full, refresh });
    const button = screen.getByRole("button", {name:"Atualizar GitHub"});
    await fireEvent.click(button);
    expect(button).toBeDisabled();
    await vi.advanceTimersByTimeAsync(10_000);
    expect(button).toBeEnabled();
  } finally { vi.useRealTimers(); }
});

it("dims cached data and offers reconnection for refused credentials", async () => {
  const { default: data } = await import("./fixtures/invalid-credential.json");
  render(GithubModule, { data, refresh: vi.fn() });
  expect(screen.getByText("Credencial inválida · Reconectar")).toBeInTheDocument();
  expect(screen.getByText("exemplo/repo-1").closest("[data-stale]" )).toHaveAttribute("data-stale", "true");
  expect(screen.getByText(/Atualizado/)).toBeInTheDocument();
  await fireEvent.click(screen.getByText("Credencial inválida · Reconectar"));
  expect(screen.getByText("Rodar `gh auth login`")).toBeInTheDocument();
});
it("shows stale snapshot age without a failure banner", async () => {
  const { default: data } = await import("./fixtures/stale.json");
  render(GithubModule, { data, refresh: vi.fn() });
  expect(screen.getByText(/Atualizado/)).toBeInTheDocument();
  expect(screen.queryByText(/Falha ao atualizar/)).not.toBeInTheDocument();
});
it("shows sustained failure age and keeps the last repository", async () => {
  const { default: data } = await import("./fixtures/failure.json");
  render(GithubModule, { data, refresh: vi.fn() });
  expect(screen.getByText(/Falha ao atualizar/)).toBeInTheDocument();
  expect(screen.getByText("exemplo/repo-1")).toBeInTheDocument();
});

it("keeps offline cached data and age without an error banner", async () => {
  const { default: data } = await import("./fixtures/offline.json");
  render(GithubModule, { data, refresh: vi.fn() });
  expect(screen.getByText("exemplo/repo-1")).toBeInTheDocument();
  expect(screen.getByText(/Atualizado/)).toBeInTheDocument();
  expect(screen.queryByText(/Falha ao atualizar/)).not.toBeInTheDocument();
  expect(screen.getByRole("button", {name:"Atualizar GitHub"})).toBeDisabled();
});
