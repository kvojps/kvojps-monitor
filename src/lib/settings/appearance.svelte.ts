import type { Appearance, SystemTheme } from "../core";

const LIGHT = "(prefers-color-scheme: light)";
const REDUCED = "(prefers-reduced-motion: reduce)";

/**
 * Effective theme and motion: the saved preference, the desktop theme reported by
 * the core (or the webview color scheme without it) and the system reduced motion,
 * which always prevails.
 */
export class AppearanceState {
  preference = $state<Appearance>({ theme: "system", reduceMotion: false });
  /** Desktop color scheme from the core; null falls back to the webview. */
  desktop = $state<SystemTheme | null>(null);
  #mediaLight = $state(false);
  systemReduced = $state(false);

  theme = $derived<SystemTheme>(
    this.preference.theme !== "system" ? this.preference.theme : this.desktop ?? (this.#mediaLight ? "light" : "dark"),
  );
  reduced = $derived(this.preference.reduceMotion || this.systemReduced);

  /** Follow the webview media queries; returns the cleanup. */
  watch(): () => void {
    const stops = [this.#follow(LIGHT, (v) => (this.#mediaLight = v)), this.#follow(REDUCED, (v) => (this.systemReduced = v))];
    return () => stops.forEach((stop) => stop());
  }

  #follow(query: string, set: (matches: boolean) => void): () => void {
    const list = typeof window.matchMedia === "function" ? window.matchMedia(query) : null;
    if (!list) return () => {};
    set(list.matches);
    const listener = (e: { matches: boolean }) => set(e.matches);
    list.addEventListener?.("change", listener);
    return () => list.removeEventListener?.("change", listener);
  }
}
