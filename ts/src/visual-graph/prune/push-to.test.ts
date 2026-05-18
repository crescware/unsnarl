import { describe, expect, test } from "vitest";

import { pushTo } from "./push-to.js";

describe("pushTo", () => {
  test("creates a single-element array on first push", () => {
    const m = new Map<string, string[]>();
    pushTo(m, "a", "x");
    expect(m.get("a")).toEqual(["x"]);
  });

  test("appends to an existing array on subsequent pushes (in order)", () => {
    const m = new Map<string, string[]>();
    pushTo(m, "a", "x");
    pushTo(m, "a", "y");
    pushTo(m, "a", "z");
    expect(m.get("a")).toEqual(["x", "y", "z"]);
  });

  test("does not affect other keys", () => {
    const m = new Map<string, string[]>();
    pushTo(m, "a", "x");
    pushTo(m, "b", "y");
    expect(m.get("a")).toEqual(["x"]);
    expect(m.get("b")).toEqual(["y"]);
  });

  test("preserves duplicate values verbatim", () => {
    const m = new Map<string, string[]>();
    pushTo(m, "k", "v");
    pushTo(m, "k", "v");
    expect(m.get("k")).toEqual(["v", "v"]);
  });
});
