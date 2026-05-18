import { describe, expect, test } from "vitest";

import { nestPaletteIndex } from "./nest-palette-index.js";

describe("nestPaletteIndex", () => {
  test("depth 1 with a non-empty palette maps to index 0", () => {
    expect(nestPaletteIndex(1, 4)).toEqual(0);
  });

  test("depths within palette length map 1:1 onto 0-based indices", () => {
    expect(nestPaletteIndex(2, 4)).toEqual(1);
    expect(nestPaletteIndex(3, 4)).toEqual(2);
    expect(nestPaletteIndex(4, 4)).toEqual(3);
  });

  test("depth beyond palette length wraps back to the start", () => {
    expect(nestPaletteIndex(5, 4)).toEqual(0);
    expect(nestPaletteIndex(6, 4)).toEqual(1);
    expect(nestPaletteIndex(9, 4)).toEqual(0);
  });

  test("palette of length 1 always maps to index 0", () => {
    expect(nestPaletteIndex(1, 1)).toEqual(0);
    expect(nestPaletteIndex(7, 1)).toEqual(0);
  });

  test("rejects a non-positive palette length", () => {
    expect(() => nestPaletteIndex(1, 0)).toThrow();
    expect(() => nestPaletteIndex(1, -1)).toThrow();
  });

  test("rejects a depth less than 1", () => {
    expect(() => nestPaletteIndex(0, 4)).toThrow();
    expect(() => nestPaletteIndex(-1, 4)).toThrow();
  });
});
