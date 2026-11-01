import { describe, expect, it } from "vitest";
import { BASE_DIAL, MAX_DIAL, dialSize } from "./instrumentFill";

describe("dialSize", () => {
  it("keeps the B dial when there is no free height", () => {
    expect(dialSize(0)).toBe(BASE_DIAL);
    expect(dialSize(-40)).toBe(BASE_DIAL);
  });

  it("gives each dial half of the free height", () => {
    expect(dialSize(140)).toBe(BASE_DIAL + 70);
  });

  it("stops growing at the largest dial", () => {
    expect(dialSize(1000)).toBe(MAX_DIAL);
  });
});
