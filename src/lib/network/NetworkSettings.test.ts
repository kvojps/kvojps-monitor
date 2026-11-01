import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { expect, it, vi } from "vitest";
import NetworkSettings from "./NetworkSettings.svelte";
import type { NetworkData } from "../core";
import speed from "./fixtures/speed.json";
import disconnected from "./fixtures/disconnected.json";

const toggle = () => screen.getByRole("checkbox", { name: "Teste de velocidade automático" });

it("stays present before the first snapshot with the automatic test disabled", async () => {
  const setAutomaticSpeed = vi.fn();
  render(NetworkSettings, { data: undefined, setAutomaticSpeed });

  expect(screen.getByRole("heading", { name: "Rede" })).toBeInTheDocument();
  expect(screen.getByText("Sem leitura da rede ainda")).toBeInTheDocument();
  expect(toggle()).toBeDisabled();
  await userEvent.click(toggle());
  expect(setAutomaticSpeed).not.toHaveBeenCalled();
});

it("reflects and changes the automatic speed test from the snapshot", async () => {
  const setAutomaticSpeed = vi.fn();
  render(NetworkSettings, { data: speed.data as NetworkData, setAutomaticSpeed });

  expect(screen.getByText("Wi-Fi")).toBeInTheDocument();
  expect(toggle()).toBeEnabled();
  expect(toggle()).toBeChecked();
  await userEvent.click(toggle());
  expect(setAutomaticSpeed).toHaveBeenCalledWith(false);
  expect(toggle()).not.toBeChecked();
  await userEvent.click(toggle());
  expect(setAutomaticSpeed).toHaveBeenLastCalledWith(true);
});

it("shows a lost connection in the status line", () => {
  render(NetworkSettings, { data: disconnected.data as NetworkData, setAutomaticSpeed: vi.fn() });
  expect(screen.getByText("Sem conexão")).toBeInTheDocument();
});
