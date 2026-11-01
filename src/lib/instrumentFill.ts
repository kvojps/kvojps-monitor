/** Dial diameter of the B prototype, in px: the size with no free height. */
export const BASE_DIAL = 94;
/** Largest dial: past it the rings stop reading as the B instruments. */
export const MAX_DIAL = 180;

/**
 * Dial diameter that turns the free height of the "IA e Agentes" card into
 * instrument instead of empty strips. The AI pair sits side by side and the
 * Agents ring sits below it, so each dial takes half of the free height.
 */
export function dialSize(freeHeight: number): number {
  return Math.min(BASE_DIAL + Math.max(0, freeHeight) / 2, MAX_DIAL);
}
