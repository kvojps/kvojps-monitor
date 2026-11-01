// Development-only surface for ticket 07: real Panel, recorded fixtures, no native
// commands, credentials or collection. Not part of the production build.
//   ?theme=light&ai=stale&agents=empty&long=1&dense=1&move=fail&target=s-closed
// long/dense derive stress variants from the recorded fixtures; they are not new readings.
import { mount } from "svelte";
import Panel from "../../../../src/lib/Panel.svelte";
import "../../../../src/tokens.css";
import type { Core, Module, Snapshot, Theme } from "../../../../src/lib/core";

const params = new URLSearchParams(location.search);
const fixtures = import.meta.glob("../../../../src/lib/*/fixtures/*.json", { eager: true, import: "default" });
const defaults: Record<Module, string> = {
  ai: "full", system: "notebook", agents: "states", trello: "full", github: "full", network: "full",
};
const LONG = " com um nome propositalmente muito longo para validação";
const clone = <T>(v: T): T => structuredClone(v);

function stress(module: Module, data: any) {
  const long = params.has("long"), dense = params.has("dense");
  if (!data || typeof data !== "object") return data;
  if (module === "agents" && Array.isArray(data.sessions)) {
    if (long) for (const s of [...data.sessions, ...(data.closed ?? [])]) { s.project += "-workspace-extremamente-longo"; if (s.name) s.name += LONG; }
    if (dense) data.sessions = Array.from({ length: 40 }, (_, i) => ({ ...data.sessions[i % data.sessions.length], id: `dense-${i}` }));
  }
  if (module === "trello" && Array.isArray(data.lists)) {
    if (long) { if (data.board) data.board.name += LONG; for (const l of data.lists) { l.name += LONG; for (const c of l.cards) c.name += LONG; } }
    if (dense) for (const l of data.lists) l.cards = Array.from({ length: 60 }, (_, i) => ({ ...l.cards[i % Math.max(1, l.cards.length)] ?? { due: null, dueComplete: false, labels: [], overdue: false, url: "https://trello.com/c/x" }, id: `${l.id}-dense-${i}`, name: `Card denso ${i + 1}` }));
  }
  if (module === "network" && data.processTraffic?.apps) {
    if (long) for (const a of data.processTraffic.apps) a.name += "-processo-com-nome-muito-longo";
    if (dense) data.processTraffic.apps = Array.from({ length: 50 }, (_, i) => ({ name: `app-${i + 1}`, download: 1000 * (50 - i), upload: 500 * (50 - i) }));
  }
  if (module === "github" && long && data.featured) { data.featured.name += "-repositorio-extremamente-longo"; if (data.featured.lastCommit) data.featured.lastCommit.message += LONG; }
  return data;
}

const snapshots = Object.entries(defaults).map(([module, fallback]) => {
  const name = params.get(module) ?? fallback;
  const fixture = fixtures[`../../../../src/lib/${module}/fixtures/${name}.json`];
  if (name === "none") return null;
  if (!fixture) throw new Error(`Unknown fixture: ${module}/${name}`);
  const snap: Snapshot = module === "system" || module === "network"
    ? clone(fixture) as Snapshot
    : { module: module as Module, updatedAt: 1793450000000, data: clone(fixture) };
  snap.data = stress(module as Module, snap.data);
  return snap;
}).filter((s): s is Snapshot => s !== null);

const log: string[] = ((window as any).__coreCalls = []);
const target = params.get("target");
const core: Core = {
  initialState: async () => ({
    lastAnchor: "ai", snapshots, panelVisible: true, agentTarget: target,
    appearance: { theme: (params.get("theme") ?? "dark") as Theme, reduceMotion: params.get("motion") !== "full" },
  }),
  listenSnapshots: async () => () => {},
  // Lets the audit emulate the native hide/show event; no native window is involved.
  listenPanelVisibility: async (cb) => { (window as any).__setVisible = cb; return () => {}; },
  reportVisibleModules: (m) => log.push(`visible:${m.join()}`),
  chooseAnchor: (m) => log.push(`anchor:${m}`),
  closePanel: () => log.push("close"),
  refreshAi: () => log.push("refresh:ai"),
  refreshGithub: () => log.push("refresh:github"),
  refreshTrello: () => log.push("refresh:trello"),
  runSpeedTest: () => log.push("speed"),
  openTrelloCard: (url) => log.push(`open:${url}`),
  moveTrelloCard: (card, list) => {
    log.push(`move:${card}->${list}`);
    return params.get("move") === "fail" ? new Promise((_, reject) => setTimeout(() => reject(new Error("refused")), 50)) : Promise.resolve();
  },
};
mount(Panel, { target: document.getElementById("app")!, props: { core } });
