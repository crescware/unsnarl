import { describe, expect, test } from "vitest";

import { lightTheme } from "./light-theme.js";

describe("lightTheme", () => {
  test("every classDef slot is populated", () => {
    expect(lightTheme.fnWrap.fill).not.toEqual("");
    expect(lightTheme.fnWrap.stroke).not.toEqual("");
    expect(lightTheme.boundaryStub.fill).not.toEqual("");
    expect(lightTheme.boundaryStub.stroke).not.toEqual("");
    expect(lightTheme.boundaryStub.strokeDasharray).not.toEqual("");
    expect(lightTheme.boundaryStub.color).not.toEqual("");
    expect(lightTheme.varNode.strokeDasharray).not.toEqual("");
    expect(lightTheme.elkEmptyPlaceholder.fill).not.toEqual("");
    expect(lightTheme.elkEmptyPlaceholder.stroke).not.toEqual("");
    expect(lightTheme.elkEmptyPlaceholder.color).not.toEqual("");
  });

  test("nestPalette is non-empty so the cycle has a target", () => {
    expect(lightTheme.nestPalette.length >= 1).toEqual(true);
  });

  test("elkEmptyPlaceholder is invisible (same workaround as the dark theme)", () => {
    expect(lightTheme.elkEmptyPlaceholder.fill).toEqual("transparent");
    expect(lightTheme.elkEmptyPlaceholder.stroke).toEqual("transparent");
    expect(lightTheme.elkEmptyPlaceholder.color).toEqual("transparent");
  });
});
