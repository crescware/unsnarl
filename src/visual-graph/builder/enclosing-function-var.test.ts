import { describe, expect, test } from "vitest";

import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { enclosingFunctionVar } from "./enclosing-function-var.js";
import { baseScope } from "./testing/make-scope.js";

const grand = { ...baseScope(), id: "grand" };
const parent = { ...baseScope(), id: "parent", upper: "grand" };
const child = { ...baseScope(), id: "child", upper: "parent" };
const map = new Map<string, SerializedScope>(
  [grand, parent, child].map((s) => [s.id, s]),
);

describe("enclosingFunctionVar", () => {
  test.each<{
    name: string;
    owners: Map<string, string>;
    start: string;
    expected: string | null;
  }>([
    {
      name: "owner found at start scope returns its variable id",
      owners: new Map([["child", "varChild"]]),
      start: "child",
      expected: "varChild",
    },
    {
      name: "owner found higher up returns that ancestor's variable id",
      owners: new Map([["grand", "varGrand"]]),
      start: "child",
      expected: "varGrand",
    },
    {
      name: "no owner anywhere -> null",
      owners: new Map(),
      start: "child",
      expected: null,
    },
    {
      name: "starting scope missing from map -> null",
      owners: new Map([["x", "v"]]),
      start: "missing",
      expected: null,
    },
  ])("$name", ({ owners, start, expected }) => {
    expect(enclosingFunctionVar(start, map, owners)).toBe(expected);
  });
});
