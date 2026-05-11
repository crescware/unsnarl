import { describe, expect, test } from "vitest";

import { darkTheme } from "./dark-theme.js";

describe("darkTheme", () => {
  // The dark theme carries the values that were hard-coded before the
  // theme abstraction landed. These literals are pinned so the existing
  // fixtures and snapshot-style tests keep matching when the default
  // theme is dark.
  test("fnWrap keeps the original dark-mode literals", () => {
    expect(darkTheme.fnWrap.fill).toEqual("#1a2030");
    expect(darkTheme.fnWrap.stroke).toEqual("#5a7d99");
  });

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

  test("nestPalette is non-empty so the cycle has a target", () => {
    expect(darkTheme.nestPalette.length >= 1).toEqual(true);
  });
});
