// Core to UI contract: push one snapshot per module; the UI renders and sends commands.
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const MODULES = ["ai", "agents", "github", "trello", "network", "system"] as const;
export type Module = (typeof MODULES)[number];

export interface Snapshot {
  module: Module;
  /** Last update in epoch milliseconds; null until data exists. */
  updatedAt: number | null;
  data: unknown;
}

/** Bytes. */
export interface Memory {
  used: number;
  total: number;
}

/** System snapshot data, including warning decisions made by the core. */
export interface SystemData {
  cpu: {
    /** Percentage; null until two samples exist. */
    usage: number | null;
    /** One sample every two seconds, oldest to newest. */
    history: number[];
    temperature: { celsius: number; warning: boolean } | null;
  };
  ram: Memory;
  swap: Memory | null;
  disks: { mount: string; used: number; total: number; free: number; warning: boolean }[];
  battery: {
    percentage: number;
    state: "charging" | "discharging" | "full" | "empty" | "pluggedIn" | "unknown";
    secondsRemaining: number | null;
    warning: boolean;
  } | null;
}

export interface InitialState {
  lastModule: Module;
  snapshots: Snapshot[];
}

export interface Core {
  initialState(): Promise<InitialState>;
  listenSnapshots(cb: (s: Snapshot) => void): Promise<() => void>;
  /** null when the current view is Settings. */
  reportModuleVisible(module: Module | null): void;
  runSpeedTest?(): void;
  setAutomaticSpeed?(enabled: boolean): void;
  refreshGithub?(): void;
  saveGithubToken?(token: string): Promise<string>;
  refreshTrello?(): void;
  /** Resolves with the account login; rejects with a credential error code. */
  connectTrello?(key: string, token: string): Promise<string>;
  selectTrelloBoard?(board: string): Promise<void>;
  disconnectTrello?(): Promise<void>;
  openTrelloAdmin?(): void;
  openTrelloAuthorization?(key: string): void;
  openTrelloCard?(url: string): void;
  /** Rejects when Trello refuses or cannot be reached. */
  moveTrelloCard?(card: string, list: string): Promise<void>;
  closePanel(): void;
}

export const tauriCore: Core = {
  initialState: () => invoke<InitialState>("initial_state"),
  async listenSnapshots(cb) {
    const stops = await Promise.all(MODULES.map((m) => listen<Snapshot>(`snapshot:${m}`, (e) => cb(e.payload))));
    return () => stops.forEach((stop) => stop());
  },
  reportModuleVisible: (module) => void invoke("report_module_visible", { module }),
  runSpeedTest: () => void invoke("run_speed_test"),
  setAutomaticSpeed: (enabled) => void invoke("set_automatic_speed", { enabled }),
  saveGithubToken: (token) => invoke<string>("save_github_token", { token }),
  refreshGithub: () => void invoke("refresh_github"),
  refreshTrello: () => void invoke("refresh_trello"),
  connectTrello: (key, token) => invoke<string>("connect_trello", { key, token }),
  selectTrelloBoard: (board) => invoke<void>("select_trello_board", { board }),
  disconnectTrello: () => invoke<void>("disconnect_trello"),
  openTrelloAdmin: () => void invoke("open_trello_admin"),
  openTrelloAuthorization: (key) => void invoke("open_trello_authorization", { key }),
  openTrelloCard: (url) => void invoke("open_trello_card", { url }),
  moveTrelloCard: (card, list) => invoke<void>("move_trello_card", { card, list }),
  closePanel: () => void invoke("close_panel"),
};

export interface NetworkMeasurements { latency: number | null; jitter: number | null; loss: number | null }
export interface NetworkTraffic { download: number; upload: number }
export interface NetworkData {
  processTraffic: { apps: { name: string; download: number; upload: number }[]; others: NetworkTraffic } | null;
  speed: { download: number; upload: number; measuredAt: number; networkId: string } | null;
  speedAvailable: boolean;
  speedRunning: boolean;
  speedFailed: boolean;
  automaticSpeed: boolean;
  connected: boolean;
  wired: boolean;
  vpn: boolean;
  quality: "good" | "regular" | "poor" | null;
  security: "noAlerts" | "warning" | "risk" | null;
  reasons: string[];
  gateway: NetworkMeasurements;
  internet: NetworkMeasurements;
  traffic: NetworkTraffic | null;
  history: NetworkTraffic[];
}

export interface GithubData {
  /** Credential metadata only; source is gh or token. */
  credential?: { source: string; login: string };
  paused?: boolean;
  lastSuccessAt?: number | null;
  failureSince?: number | null;
  state: string;
  refreshing: boolean;
  calendar: { restrictedContributionsCount: number; contributionCalendar: {totalContributions:number; weeks: {contributionDays: {date:string; contributionCount:number; color:string}[]}[]} } | null;
  featured: {name:string; commits:number; total:number; title:string; when:number} | null;
}

export interface TrelloCard {
  id: string;
  /** Empty when restored from the disk cache, which never stores titles. */
  name: string;
  url: string;
  /** Trello color names, e.g. green or sky_light. */
  labels: string[];
  due: number | null;
  dueComplete: boolean;
  overdue: boolean;
}
export interface TrelloData {
  state: "missingCredential" | "chooseBoard" | "ready" | "stale" | "error" | "invalidCredential";
  refreshing: boolean;
  paused?: boolean;
  lastSuccessAt?: number | null;
  failureSince?: number | null;
  account: { login: string } | null;
  boards: { id: string; name: string }[];
  board: { id: string; name: string } | null;
  lists: { id: string; name: string; cards: TrelloCard[] }[] | null;
  /** Key typed before generating the token; kept in core memory only. */
  pendingKey?: string;
}
