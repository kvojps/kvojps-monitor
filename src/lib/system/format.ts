// System size formatting, using binary units like free and df -h.

const GIB = 1024 ** 3;
const TIB = 1024 ** 4;

const number = (places: number) =>
  new Intl.NumberFormat("pt-BR", { minimumFractionDigits: places, maximumFractionDigits: places });

/** Always one decimal, as prototype B shows: "5,7", "14,5". */
const value = (n: number) => number(1).format(n);

function unit(reference: number): [number, string] {
  return reference >= 1000 * GIB ? [TIB, "TB"] : [GIB, "GB"];
}

/** "5,7 / 14,5 GB": choose the unit from the total. */
export function pair(used: number, total: number): string {
  const [base, name] = unit(total);
  return `${value(used / base)} / ${value(total / base)} ${name}`;
}

