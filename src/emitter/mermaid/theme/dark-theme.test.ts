import { describe, expect, test } from "vitest";

import { darkTheme } from "./dark-theme.js";

describe("darkTheme", () => {
  test("boundaryStub keeps the original dark-mode literals", () => {
    expect(darkTheme.boundaryStub.fill).toEqual("transparent");
    expect(darkTheme.boundaryStub.stroke).toEqual("#888");
    expect(darkTheme.boundaryStub.strokeDasharray).toEqual("3 3");
    expect(darkTheme.boundaryStub.color).toEqual("#888");
  });

  test("varNode keeps the original dash pattern", () => {
    expect(darkTheme.varNode.strokeDasharray).toEqual("5 5");
  });

  test("elkEmptyPlaceholder keeps the all-transparent literals", () => {
    expect(darkTheme.elkEmptyPlaceholder.fill).toEqual("transparent");
    expect(darkTheme.elkEmptyPlaceholder.stroke).toEqual("transparent");
    expect(darkTheme.elkEmptyPlaceholder.color).toEqual("transparent");
  });

  // Each function consumes two adjacent slots (wrapper at N, body at
  // N+1) so the wrapper and body read as distinct brightness levels.
  // Six entries trade cycle-distance for per-step contrast; this test
  // guards against accidental shrinkage that would lose contrast.
  test("nestPalette has at least six entries to keep wrap / body brightness distinct", () => {
    expect(darkTheme.nestPalette.length >= 6).toEqual(true);
  });
});
