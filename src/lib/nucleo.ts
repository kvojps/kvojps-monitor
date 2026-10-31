// Contrato núcleo → UI: o núcleo empurra um snapshot por Módulo; a UI renderiza e envia comandos pontuais.
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const MODULOS = ["ia", "agentes", "github", "trello", "rede", "sistema"] as const;
export type Modulo = (typeof MODULOS)[number];

export interface Snapshot {
  modulo: Modulo;
  /** Epoch em ms da última atualização; null enquanto não há dado. */
  atualizadoEm: number | null;
  dados: unknown;
}

export interface EstadoInicial {
  ultimoModulo: Modulo;
  snapshots: Snapshot[];
}

export interface Nucleo {
  estadoInicial(): Promise<EstadoInicial>;
  ouvirSnapshots(cb: (s: Snapshot) => void): Promise<() => void>;
  /** null quando a vista atual não é um Módulo (Configurações). */
  informarModuloVisivel(modulo: Modulo | null): void;
  fecharPainel(): void;
}

export const nucleoTauri: Nucleo = {
  estadoInicial: () => invoke<EstadoInicial>("estado_inicial"),
  async ouvirSnapshots(cb) {
    const paradas = await Promise.all(MODULOS.map((m) => listen<Snapshot>(`snapshot:${m}`, (e) => cb(e.payload))));
    return () => paradas.forEach((parar) => parar());
  },
  informarModuloVisivel: (modulo) => void invoke("informar_modulo_visivel", { modulo }),
  fecharPainel: () => void invoke("fechar_painel"),
};
