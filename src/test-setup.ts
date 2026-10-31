import "@testing-library/jest-dom/vitest";
import { vi } from "vitest";

// jsdom has no layout: scrolling is observed through this spy.
Element.prototype.scrollIntoView = vi.fn();
