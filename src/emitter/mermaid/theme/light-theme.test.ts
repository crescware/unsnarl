import { describe, expect, test } from "vitest";

import { lightTheme } from "./light-theme.js";

describe("lightTheme", () => {
  test("every classDef slot is populated", () => {
    expect(lightTheme.boundaryStub.stroke).not.toEqual("");
    expect(lightTheme.boundaryStub.strokeDasharray).not.toEqual("");
    expect(lightTheme.boundaryStub.color).not.toEqual("");
    expect(lightTheme.varNode.strokeDasharray).not.toEqual("");
    expect(lightTheme.elkEmptyPlaceholder.fill).not.toEqual("");
    expect(lightTheme.elkEmptyPlaceholder.stroke).not.toEqual("");
  });

  test("nestPalette has at least six entries to keep wrap / body brightness distinct", () => {
    expect(lightTheme.nestPalette.length >= 6).toEqual(true);
  });

  // The placeholder is a layout-only hack -- it is not a node. fill and
  // stroke stay transparent so no rectangle is drawn around the label.
  // Text color is left to Mermaid's default so the "No nodes" label
  // stays readable against whichever subgraph background it lands on.
  test("elkEmptyPlaceholder has transparent fill and stroke (same workaround as the dark theme)", () => {
    expect(lightTheme.elkEmptyPlaceholder.fill).toEqual("transparent");
    expect(lightTheme.elkEmptyPlaceholder.stroke).toEqual("transparent");
  });
});
