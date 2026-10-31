// Formatos de tamanho e duração do Módulo Sistema (unidades binárias, como o `free` e o `df -h`).

const GIB = 1024 ** 3;
const TIB = 1024 ** 4;

const numero = (casas: number) =>
  new Intl.NumberFormat("pt-BR", { minimumFractionDigits: casas, maximumFractionDigits: casas });

/** Uma casa abaixo de 10, inteiro acima: "5,7", "147". */
function valor(n: number): string {
  return numero(n < 10 ? 1 : 0).format(n);
}

function unidade(referencia: number): [number, string] {
  return referencia >= 1000 * GIB ? [TIB, "TB"] : [GIB, "GB"];
}

export function bytes(b: number): string {
  const [base, nome] = unidade(b);
  return `${valor(b / base)} ${nome}`;
}

/** "5,7 / 15 GB": a unidade vem do total. */
export function par(usado: number, total: number): string {
  const [base, nome] = unidade(total);
  return `${valor(usado / base)} / ${valor(total / base)} ${nome}`;
}

/** "30 min", "1 h 30 min", "2 h". */
export function duracao(segundos: number): string {
  const minutos = Math.round(segundos / 60);
  const h = Math.floor(minutos / 60);
  const min = minutos % 60;
  if (h === 0) return `${min} min`;
  return min === 0 ? `${h} h` : `${h} h ${min} min`;
}
