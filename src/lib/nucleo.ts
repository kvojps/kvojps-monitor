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

/** Bytes. */
export interface Memoria {
  usada: number;
  total: number;
}

/** `dados` do snapshot do Módulo Sistema. As regras de atenção vêm prontas do núcleo. */
export interface DadosSistema {
  cpu: {
    /** %; null até haver duas amostras. */
    uso: number | null;
    /** Uma amostra a cada 2 s, da mais antiga à mais recente. */
    historico: number[];
    temperatura: { celsius: number; atencao: boolean } | null;
  };
  ram: Memoria;
  swap: Memoria | null;
  discos: { ponto: string; usado: number; total: number; livre: number; atencao: boolean }[];
  bateria: {
    percentual: number;
    estado: "carregando" | "descarregando" | "cheia" | "vazia" | "naTomada" | "desconhecido";
    segundosRestantes: number | null;
    atencao: boolean;
  } | null;
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
