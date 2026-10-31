// System size formatting, using binary units like free and df -h.

const GIB = 1024 ** 3;
const TIB = 1024 ** 4;

const number = (places: number) =>
  new Intl.NumberFormat("pt-BR", { minimumFractionDigits: places, maximumFractionDigits: places });

/** One decimal below 10, integer otherwise: "5,7", "147". */
function value(n: number): string {
  return number(n < 10 ? 1 : 0).format(n);
}

function unit(reference: number): [number, string] {
  return reference >= 1000 * GIB ? [TIB, "TB"] : [GIB, "GB"];
}

/** "5,7 / 15 GB": choose the unit from the total. */
export function pair(used: number, total: number): string {
  const [base, name] = unit(total);
  return `${value(used / base)} / ${value(total / base)} ${name}`;
}

