import { describe, expect, test } from "vitest";

import type { ColorTheme } from "./color-theme.js";

describe("ColorTheme", () => {
  // A minimal concrete value compiles only if the structural shape is
  // satisfied; this test exists so a future field rename surfaces here.
  test("a hand-built theme value type-checks against ColorTheme", () => {
    const t: ColorTheme = {
      boundaryStub: {
        fill: "transparent",
        stroke: "#888",
        strokeDasharray: "3 3",
        color: "#888",
      },
      varNode: { strokeDasharray: "5 5" },
      elkEmptyPlaceholder: {
        fill: "transparent",
        stroke: "transparent",
        color: "transparent",
      },
      nestPalette: [{ fill: "#111", stroke: "#222" }],
    };
    expect(t.nestPalette.length).toEqual(1);
  });
});
