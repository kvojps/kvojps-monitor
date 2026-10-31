import { render, screen } from "@testing-library/svelte";
import { expect, it } from "vitest";
import userEvent from "@testing-library/user-event";
import AgentsModule from "./AgentsModule.svelte";
import type { AgentsData } from "../core";
import empty from "./fixtures/empty.json";
import waiting from "./fixtures/waiting.json";
import closed from "./fixtures/closed.json";

it("explains when no sessions are active", () => {
  render(AgentsModule, { data: empty as AgentsData });
  expect(screen.getByText("Nenhuma sessão de agente ativa")).toBeInTheDocument();
});
it("shows waiting state, project, name, detail and elapsed time", () => {
  render(AgentsModule, { data: waiting as AgentsData });
  for (const text of ["Aguardando você", "project", "Minha sessão", "Aguardando aprovação", "1 min 40 s"])
    expect(screen.getByText(text)).toBeInTheDocument();
});
it("keeps closed sessions collapsed until expanded", async () => {
  const { container } = render(AgentsModule, { data: closed as AgentsData });
  const details = container.querySelector("details")!;
  expect(details.open).toBe(false);
  await userEvent.click(screen.getByText("Encerradas (1)"));
  expect(details.open).toBe(true);
  expect(screen.getByText("Minha sessão")).toBeVisible();
});

it("preserves a failed background outcome in the closed section", async () => {
  render(AgentsModule, { data: {
    ...closed,
    closed: [{ ...closed.closed[0], state: "closed", lastState: "failed" }],
  } as AgentsData });
  await userEvent.click(screen.getByText("Encerradas (1)"));
  expect(screen.getByText("Com falha")).toBeVisible();
});
