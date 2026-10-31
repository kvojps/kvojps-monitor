// Development-only surface: no native commands, credentials, or real collection.
import { mount } from "svelte";
import Panel from "../../../src/lib/Panel.svelte";
import "../../../src/tokens.css";
import type { Core, Module, Snapshot, Theme } from "../../../src/lib/core";

const params = new URLSearchParams(location.search);
const fixtures = import.meta.glob("../../../src/lib/*/fixtures/*.json", { eager: true, import: "default" });
const defaults: Record<Module, string> = {
  ai: "full", system: "notebook", agents: "states", trello: "full", github: "full", network: "full",
};
const snapshots = Object.entries(defaults).map(([module, fallback]) => {
  const name = params.get(module) ?? fallback;
  const fixture = fixtures[`../../../src/lib/${module}/fixtures/${name}.json`];
  if (!fixture) throw new Error(`Unknown fixture: ${module}/${name}`);
  return module === "system" || module === "network"
    ? fixture as Snapshot
    : { module: module as Module, updatedAt: 1793450000000, data: fixture };
});
const core: Core = {
  initialState: async () => ({
    lastAnchor: "ai", snapshots, panelVisible: true,
    appearance: { theme: (params.get("theme") ?? "dark") as Theme, reduceMotion: true },
  }),
  listenSnapshots: async () => () => {},
  reportVisibleModules: () => {},
  chooseAnchor: () => {},
  closePanel: () => {},
};
mount(Panel, { target: document.getElementById("app")!, props: { core } });
