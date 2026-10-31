import { fireEvent, render, screen } from "@testing-library/svelte";
import { expect, it, vi } from "vitest";
import GithubSettings from "./GithubSettings.svelte";
import missing from "./fixtures/missing-gh.json";

it("validates and saves a pasted token and clears the secret on success", async () => {
  const save = vi.fn().mockResolvedValue("exemplo");
  render(GithubSettings, { data: missing, save });
  const field = screen.getByLabelText("Token clássico (escopo repo)");
  expect(field).toHaveAttribute("type", "password");
  await fireEvent.input(field, { target: { value: "private-token" } });
  await fireEvent.click(screen.getByRole("button", { name: "Validar e salvar" }));
  expect(save).toHaveBeenCalledWith("private-token");
  expect(await screen.findByText("Conectado como exemplo")).toBeInTheDocument();
  expect(field).toHaveValue("");
});

it.each([
  ["invalidToken", "Token inválido"],
  ["missingRepo", "Falta o escopo repo"],
  ["offline", "Sem conexão, tente de novo"],
  ["keyringUnavailable", "GNOME Keyring indisponível. Ative ou desbloqueie o chaveiro para salvar; o token não foi salvo."],
])("shows %s beside the token field", async (error, message) => {
  render(GithubSettings, { data: missing, save: vi.fn().mockRejectedValue(error) });
  const field = screen.getByLabelText("Token clássico (escopo repo)");
  await fireEvent.input(field, { target: { value: "private-token" } });
  await fireEvent.click(screen.getByRole("button", { name: "Validar e salvar" }));
  expect(await screen.findByRole("alert")).toHaveTextContent(message);
  expect(field).toHaveAccessibleDescription(message);
  expect(field).toHaveValue("");
  expect(screen.queryByText(/Conectado como/)).not.toBeInTheDocument();
});

it.each([
  ["full", "via gh · exemplo"],
  ["own-token", "token próprio"],
])("shows the credential source from the %s snapshot", async (name, label) => {
  const data = name === "full" ? (await import("./fixtures/full.json")).default : (await import("./fixtures/own-token.json")).default;
  render(GithubSettings, { data, save: vi.fn() });
  expect(screen.getByText(label)).toBeInTheDocument();
});

it("prevents duplicate submissions while validation is pending", async () => {
  let complete!: (login: string) => void;
  const save = vi.fn(() => new Promise<string>((resolve) => complete = resolve));
  render(GithubSettings, { data: missing, save });
  await fireEvent.input(screen.getByLabelText("Token clássico (escopo repo)"), { target: { value: "private-token" } });
  await fireEvent.click(screen.getByRole("button", { name: "Validar e salvar" }));
  expect(screen.getByRole("button", { name: "Validando…" })).toBeDisabled();
  expect(screen.getByLabelText("Token clássico (escopo repo)")).toBeDisabled();
  complete("exemplo");
  expect(await screen.findByText("Conectado como exemplo")).toBeInTheDocument();
  expect(save).toHaveBeenCalledOnce();
});

it("keeps the own-token account visible when settings are reopened", async () => {
  const { default: data } = await import("./fixtures/own-token.json");
  const save = vi.fn();
  const first = render(GithubSettings, { data, save });
  expect(screen.getByText("Conectado como exemplo")).toBeInTheDocument();
  first.unmount();
  render(GithubSettings, { data, save });
  expect(screen.getByText("token próprio")).toBeInTheDocument();
  expect(screen.getByText("Conectado como exemplo")).toBeInTheDocument();
  expect(save).not.toHaveBeenCalled();
});
