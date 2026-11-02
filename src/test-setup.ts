import "./opening.css";
import "@testing-library/jest-dom/vitest";
import { vi } from "vitest";

// jsdom has no layout: scrolling is observed through this spy.
Element.prototype.scrollIntoView = vi.fn();

// jsdom does not implement the native dialog lifecycle or top-layer focus.
HTMLDialogElement.prototype.showModal = function () {
  this.open = true;
  this.querySelector<HTMLElement>("button")?.focus();
};
HTMLDialogElement.prototype.close = function () {
  this.open = false;
  this.dispatchEvent(new Event("close"));
};
