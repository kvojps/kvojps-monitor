// Único arquivo de textos da UI (pt-BR). O núcleo Rust lê o mesmo JSON para o menu do Indicador.
import strings from "./strings.json";

export const t = strings;

export function preencher(modelo: string, valores: Record<string, string>): string {
  return modelo.replace(/\{(\w+)\}/g, (_, chave: string) => valores[chave] ?? "");
}
