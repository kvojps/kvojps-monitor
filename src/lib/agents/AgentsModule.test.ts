import { render, screen, within } from "@testing-library/svelte";
import { expect, it, vi } from "vitest";
import AgentsModule from "./AgentsModule.svelte";
import type { AgentsData } from "../core";
import empty from "./fixtures/empty.json";
import waiting from "./fixtures/waiting.json";
import mixed from "./fixtures/mixed.json";
import states from "./fixtures/states.json";

const list = () => screen.getByRole("list", { name: "Sessões de agente" });
const row = (name: string) => within(list()).getAllByRole("listitem").find((item) => within(item).queryByText(name))!;
const counter = (label: string) => screen.getByText(label, { selector: "dt" }).nextElementSibling?.textContent;

it("explains when no sessions are active and shows no intervention", () => {
  render(AgentsModule, { data: empty as AgentsData });
  expect(screen.getByText("Nenhuma sessão de agente ativa")).toBeInTheDocument();
  const banner = screen.getByRole("group", { name: "Intervenção" });
  expect(banner).toHaveTextContent("Nenhuma intervenção pendente");
  expect(banner).not.toHaveTextContent("Aguardando você");
  expect(screen.queryByRole("list", { name: "Sessões de agente" })).not.toBeInTheDocument();
});

it("explains there is no reading yet before the first snapshot", () => {
  render(AgentsModule, { data: null });
  expect(screen.getByText("Sem leitura das sessões de agente ainda")).toBeInTheDocument();
  expect(screen.queryByText("Nenhuma sessão de agente ativa")).not.toBeInTheDocument();
});

it("highlights the interventions counted from the sessions", () => {
  render(AgentsModule, { data: states as AgentsData });
  const banner = screen.getByRole("group", { name: "Intervenção" });
  expect(banner).toHaveTextContent("Aguardando você");
  expect(banner).toHaveTextContent("01");
});

it("shows project, name, provider, state, reason and duration of each session", () => {
  render(AgentsModule, { data: states as AgentsData });
  const first = row("example-session");
  for (const text of ["example-project · Claude Code", "Aguardando você", "Aguardando aprovação", "1 min 40 s"])
    expect(within(first).getByText(text)).toBeInTheDocument();
  const codex = row("Sessão Codex");
  for (const text of ["kvojps-monitor · Codex", "Em andamento", "1 h 01 min"])
    expect(within(codex).getByText(text)).toBeInTheDocument();
  expect(within(row("notes · Claude Code")).getByText("Sem nome")).toBeInTheDocument();
});

it("names each of the five session states in text", () => {
  render(AgentsModule, { data: states as AgentsData, notified: "s-closed" });
  for (const [name, state] of [["example-session", "Aguardando você"], ["Sessão Codex", "Em andamento"],
    ["notes · Claude Code", "Turno concluído"], ["deploy", "Com falha"], ["Sessão encerrada", "Encerrada"]])
    expect(within(row(name)).getByText(state)).toBeInTheDocument();
  expect(within(row("Sessão encerrada")).getByText("Última situação: Com falha")).toBeInTheDocument();
});

it("summarizes closed sessions in the counters without listing or expanding them", () => {
  const { container } = render(AgentsModule, { data: states as AgentsData });
  expect(container.querySelector("details")).toBeNull();
  expect(screen.queryByText("Sessão encerrada")).not.toBeInTheDocument();
  expect(counter("Em andamento")).toBe("1");
  expect(counter("Aguardando você")).toBe("1");
  expect(counter("Turno concluído")).toBe("1");
  expect(counter("Com falha")).toBe("1");
  expect(counter("Encerradas")).toBe("2");
});

it("derives the counters from the snapshot, without fixed values", () => {
  render(AgentsModule, { data: waiting as AgentsData });
  expect(counter("Aguardando você")).toBe("1");
  expect(counter("Em andamento")).toBe("0");
  expect(counter("Encerradas")).toBe("0");
});

it("identifies each provider and explains unavailable Codex waiting detection", () => {
  render(AgentsModule, { data: mixed as AgentsData });
  expect(within(row("Sessão Codex")).getByText("project · Codex")).toBeInTheDocument();
  expect(within(row("Minha sessão")).getByText("project · Claude Code")).toBeInTheDocument();
  expect(screen.getByText("Aguardando indisponível sem Detecção instantânea")).toBeVisible();
  expect(screen.getByText("Nenhuma intervenção pendente")).toBeInTheDocument();
});

it("keeps the sessions with the age of the last good read when collection fails", () => {
  render(AgentsModule, { data: { ...states, failed: true, observedAt: 1800000120000, lastSuccessAt: 1800000000000 } as AgentsData });
  const time = new Intl.DateTimeFormat("pt-BR", { hour: "2-digit", minute: "2-digit" }).format(1800000000000);
  expect(screen.getByText(/Não foi possível consultar/)).toHaveTextContent(`Não foi possível consultar as sessões de agente · última leitura às ${time}`);
  expect(row("example-session")).toBeInTheDocument();
});

it("reaches and highlights the session targeted by a notification", async () => {
  const scroll = vi.mocked(Element.prototype.scrollIntoView);
  scroll.mockClear();
  render(AgentsModule, { data: states as AgentsData, notified: "s-failed" });
  const targeted = row("deploy");
  expect(targeted).toHaveAttribute("aria-current", "true");
  expect(within(targeted).getByText("Da notificação")).toBeInTheDocument();
  expect(row("example-session")).not.toHaveAttribute("aria-current");
  await vi.waitFor(() => expect(scroll.mock.contexts).toContain(targeted));
});

it("reaches a targeted session that has closed since the notification", () => {
  render(AgentsModule, { data: states as AgentsData, notified: "s-closed" });
  expect(row("Sessão encerrada")).toHaveAttribute("aria-current", "true");
  expect(screen.queryByText("Outra encerrada")).not.toBeInTheDocument();
});

it("names the tool awaiting approval only when it is a known tool, never free-form detail", () => {
  const session = states.sessions[0];
  render(AgentsModule, { data: { ...states, sessions: [
    { ...session, id: "a", name: "com ferramenta", waitingFor: "Bash" },
    { ...session, id: "b", name: "livre", waitingFor: "PRIVATE detail" },
  ] } as AgentsData });
  expect(within(row("com ferramenta")).getByText("Aguardando aprovação: Bash")).toBeInTheDocument();
  expect(within(row("livre")).getByText("Aguardando sua resposta")).toBeInTheDocument();
  expect(screen.queryByText(/PRIVATE/)).not.toBeInTheDocument();
});
