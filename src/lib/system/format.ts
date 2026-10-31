// System size and duration formatting, using binary units like free and df -h.

const GIB = 1024 ** 3;
const TIB = 1024 ** 4;

const number = (places: number) =>
  new Intl.NumberFormat("pt-BR", { minimumFractionDigits: places, maximumFractionDigits: places });

/** One decimal below 10, integer otherwise: "5,7", "147". */
function value(n: number): string {
  return number(n < 10 ? 1 : 0).format(n);
}

function unit(referencia: number): [number, string] {
  return referencia >= 1000 * GIB ? [TIB, "TB"] : [GIB, "GB"];
}

export function bytes(b: number): string {
  const [base, name] = unit(b);
  return `${value(b / base)} ${name}`;
}

/** "5,7 / 15 GB": choose the unit from the total. */
export function pair(used: number, total: number): string {
  const [base, name] = unit(total);
  return `${value(used / base)} / ${value(total / base)} ${name}`;
}

/** "30 min", "1 h 30 min", "2 h". */
export function duration(seconds: number): string {
  const minutes = Math.round(seconds / 60);
  const h = Math.floor(minutes / 60);
  const min = minutes % 60;
  if (h === 0) return `${min} min`;
  return min === 0 ? `${h} h` : `${h} h ${min} min`;
}
