import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import Painel from "./Painel.svelte";
import type { Modulo, Nucleo, Snapshot } from "./nucleo";
import notebook from "./sistema/fixtures/notebook.json";

function nucleoFalso(inicial: { ultimoModulo?: Modulo; snapshots?: Snapshot[] } = {}) {
  let ouvinte: ((s: Snapshot) => void) | undefined;
  const nucleo = {
    estadoInicial: vi.fn(async () => ({
      ultimoModulo: inicial.ultimoModulo ?? "ia",
      snapshots: inicial.snapshots ?? [],
    })),
    ouvirSnapshots: vi.fn(async (cb: (s: Snapshot) => void) => {
      ouvinte = cb;
      return () => {};
    }),
    informarModuloVisivel: vi.fn(),
    fecharPainel: vi.fn(),
  } satisfies Nucleo;
  return { nucleo, publicar: (s: Snapshot) => ouvinte?.(s) };
}

describe("trilho do Painel", () => {
  it("lista os seis Módulos e Configurações", async () => {
    const { nucleo } = nucleoFalso();
    render(Painel, { nucleo });

    const trilho = await screen.findByRole("navigation", { name: "Módulos" });
    const nomes = Array.from(trilho.querySelectorAll("button")).map((b) => b.getAttribute("aria-label"));
    expect(nomes).toEqual(["IA", "Agentes", "GitHub", "Trello", "Rede", "Sistema", "Configurações"]);
  });

  it("abre no último Módulo visitado, com o nome no cabeçalho", async () => {
    const { nucleo } = nucleoFalso({ ultimoModulo: "rede" });
    render(Painel, { nucleo });

    expect(await screen.findByRole("heading", { name: "Rede" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Rede" })).toHaveAttribute("aria-current", "page");
  });

  it("troca de Módulo pelo trilho e informa o núcleo do Módulo visível", async () => {
    const { nucleo } = nucleoFalso({ ultimoModulo: "ia" });
    render(Painel, { nucleo });
    await screen.findByRole("heading", { name: "IA" });

    await userEvent.click(screen.getByRole("button", { name: "Sistema" }));

    expect(screen.getByRole("heading", { name: "Sistema" })).toBeInTheDocument();
    expect(nucleo.informarModuloVisivel).toHaveBeenLastCalledWith("sistema");
  });

  it("Configurações não contam como Módulo visível", async () => {
    const { nucleo } = nucleoFalso();
    render(Painel, { nucleo });
    await screen.findByRole("heading", { name: "IA" });

    await userEvent.click(screen.getByRole("button", { name: "Configurações" }));

    expect(screen.getByRole("heading", { name: "Configurações" })).toBeInTheDocument();
    expect(nucleo.informarModuloVisivel).toHaveBeenLastCalledWith(null);
  });

  it("mostra no cabeçalho a hora da última atualização do Módulo", async () => {
    const quatorzeETrinta = new Date(2026, 9, 30, 14, 32).getTime();
    const { nucleo } = nucleoFalso({
      ultimoModulo: "ia",
      snapshots: [{ modulo: "ia", atualizadoEm: quatorzeETrinta, dados: null }],
    });
    render(Painel, { nucleo });

    expect(await screen.findByText("atualizado 14:32")).toBeInTheDocument();
  });

  it("atualiza o cabeçalho quando o núcleo empurra um snapshot novo", async () => {
    const { nucleo, publicar } = nucleoFalso({ ultimoModulo: "rede" });
    render(Painel, { nucleo });
    expect(await screen.findByText("sem dados ainda")).toBeInTheDocument();

    publicar({ modulo: "rede", atualizadoEm: new Date(2026, 9, 30, 9, 5).getTime(), dados: null });

    expect(await screen.findByText("atualizado 09:05")).toBeInTheDocument();
  });

  it("Esc pede ao núcleo para fechar o Painel", async () => {
    const { nucleo } = nucleoFalso();
    render(Painel, { nucleo });
    await screen.findByRole("heading", { name: "IA" });

    await userEvent.keyboard("{Escape}");

    expect(nucleo.fecharPainel).toHaveBeenCalledOnce();
  });

  it("mostra o Módulo Sistema com o snapshot empurrado pelo núcleo", async () => {
    const { nucleo, publicar } = nucleoFalso({ ultimoModulo: "sistema" });
    render(Painel, { nucleo });
    await screen.findByRole("heading", { name: "Sistema" });
    expect(screen.queryByRole("region", { name: "CPU" })).not.toBeInTheDocument();

    publicar(notebook as Snapshot);

    expect(await screen.findByRole("region", { name: "CPU" })).toBeInTheDocument();
  });
});
