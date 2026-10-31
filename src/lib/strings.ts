// Shared pt-BR UI text. Rust reads the same JSON for the indicator menu.
import strings from "./strings.json";

export const t = strings;

export function interpolate(template: string, values: Record<string, string>): string {
  return template.replace(/\{(\w+)\}/g, (_, key: string) => values[key] ?? "");
}
