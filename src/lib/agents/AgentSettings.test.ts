import { render, screen, waitFor } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { expect, it, vi } from "vitest";
import AgentSettings from "./AgentSettings.svelte";
import empty from "./fixtures/empty.json";

it("changes hooks only after a click and shows the manual Codex trust step", async () => {
  const setHooks = vi.fn().mockResolvedValue(undefined);
  render(AgentSettings, { data: empty, save: vi.fn(), setHooks });
  expect(setHooks).not.toHaveBeenCalled();
  await userEvent.click(screen.getByRole("checkbox", { name: "Detecção instantânea" }));
  expect(setHooks).toHaveBeenCalledWith(true);
  expect(screen.getByText(/\/hooks/)).toBeVisible();
  await userEvent.click(screen.getByRole("checkbox", { name: "Detecção instantânea" }));
  expect(setHooks).toHaveBeenLastCalledWith(false);
});
it("keeps the switch off and reports a configuration failure", async () => {
  const setHooks = vi.fn().mockRejectedValue("configurationFailed");
  render(AgentSettings, { data: empty, save: vi.fn(), setHooks });
  await userEvent.click(screen.getByRole("checkbox", { name: "Detecção instantânea" }));
  await waitFor(() => expect(screen.getByRole("alert")).toBeVisible());
  expect(screen.getByRole("checkbox", { name: "Detecção instantânea" })).not.toBeChecked();
});
