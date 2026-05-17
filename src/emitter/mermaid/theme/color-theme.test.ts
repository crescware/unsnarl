import { describe, expect, test } from "vitest";

import type { ColorTheme } from "./color-theme.js";

describe("ColorTheme", () => {
  // A minimal concrete value compiles only if the structural shape is
  // satisfied; this test exists so a future field rename surfaces here.
  test("a hand-built theme value type-checks against ColorTheme", () => {
    const t: ColorTheme = {
      boundaryStub: {
        stroke: "#888",
        strokeDasharray: "3 3",
        color: "#888",
      },
      varNode: { strokeDasharray: "5 5" },
      elkEmptyPlaceholder: {
        fill: "transparent",
        stroke: "transparent",
      },
      nestPalette: [{ fill: "#111", stroke: "#222" }],
      highlight: {
        fill: "#ff0",
        stroke: "#cc0",
        color: "#000",
        edgeStroke: "#cc0",
        edgeStrokeWidth: "2px",
      },
    };
    expect(t.nestPalette.length).toEqual(1);
  });
});
