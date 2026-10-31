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
  atualizarGithub?(): void;
  fecharPainel(): void;
}

export const nucleoTauri: Nucleo = {
  estadoInicial: () => invoke<EstadoInicial>("estado_inicial"),
  async ouvirSnapshots(cb) {
    const paradas = await Promise.all(MODULOS.map((m) => listen<Snapshot>(`snapshot:${m}`, (e) => cb(e.payload))));
    return () => paradas.forEach((parar) => parar());
  },
  informarModuloVisivel: (modulo) => void invoke("informar_modulo_visivel", { modulo }),
  atualizarGithub: () => void invoke("atualizar_github"),
  fecharPainel: () => void invoke("fechar_painel"),
};

export interface MedidasRede { latencia: number | null; jitter: number | null; perda: number | null }
export interface TrafegoRede { descida: number; subida: number }
export interface DadosRede {
  conectada: boolean;
  cabeada: boolean;
  vpn: boolean;
  qualidade: "boa" | "regular" | "ruim" | null;
  seguranca: "semAlertas" | "atencao" | "risco" | null;
  motivos: string[];
  gateway: MedidasRede;
  internet: MedidasRede;
  trafego: TrafegoRede | null;
  historico: TrafegoRede[];
}

export interface DadosGithub {
  estado: string;
  atualizando: boolean;
  calendario: { restrictedContributionsCount: number; contributionCalendar: {totalContributions:number; weeks: {contributionDays: {date:string; contributionCount:number; color:string}[]}[]} } | null;
  destaque: {nome:string; commits:number; total:number; titulo:string; quando:number} | null;
}
