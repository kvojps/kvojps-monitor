import { render, screen, fireEvent } from "@testing-library/svelte";
import { expect, it, vi } from "vitest";
import ModuloGithub from "./ModuloGithub.svelte";
import cheio from "./fixtures/cheio.json";
import vazio from "./fixtures/vazio.json";
import semGh from "./fixtures/sem-gh.json";
it("mostra calendário, contagens e último commit", () => {
  render(ModuloGithub, { dados: cheio, atualizar: vi.fn() });
  expect(screen.getByText("exemplo/repo-1")).toBeInTheDocument();
  expect(screen.getByText("27 commits em 7 dias · 27 no total")).toBeInTheDocument();
  expect(screen.getByText("Atualiza documentação")).toBeInTheDocument();
});
it("janela vazia mostra explicação", () => {
  render(ModuloGithub, { dados: vazio, atualizar: vi.fn() });
  expect(screen.getByText("Sem commits nos últimos 7 dias")).toBeInTheDocument();
});
it("sem gh oferece instrução de login e emite atualização", async () => {
  const atualizar = vi.fn();
  render(ModuloGithub, { dados: semGh, atualizar });
  await fireEvent.click(screen.getByText("Configurar"));
  expect(screen.getByText("Rodar `gh auth login`")).toBeInTheDocument();
  await fireEvent.click(screen.getByRole("button", {name:"Atualizar GitHub"}));
  expect(atualizar).toHaveBeenCalledOnce();
});
it("mostra falha de atualização por fixture", async () => {
  const { default: erro } = await import("./fixtures/erro.json");
  render(ModuloGithub, { dados: erro, atualizar: vi.fn() });
  expect(screen.getByText("Não foi possível atualizar o GitHub")).toBeInTheDocument();
});
it("desabilita o botão durante a espera de dez segundos", async () => {
  vi.useFakeTimers();
  try {
    const atualizar = vi.fn();
    render(ModuloGithub, { dados: cheio, atualizar });
    const botao = screen.getByRole("button", {name:"Atualizar GitHub"});
    await fireEvent.click(botao);
    expect(botao).toBeDisabled();
    await vi.advanceTimersByTimeAsync(10_000);
    expect(botao).toBeEnabled();
  } finally { vi.useRealTimers(); }
});
