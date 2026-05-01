import { describe, expect, test } from "vitest";

import type { SerializedScope } from "../../ir/model.js";
import { findEnclosingSubgraphScope } from "./find-enclosing-subgraph-scope.js";
import { baseScope } from "./testing/make-scope.js";

const grand = { ...baseScope(), id: "grand" };
const parent = { ...baseScope(), id: "parent", upper: "grand" };
const child = { ...baseScope(), id: "child", upper: "parent" };
const ancestorChain = new Map<string, SerializedScope>(
  [grand, parent, child].map((s) => [s.id, s]),
);

describe("findEnclosingSubgraphScope", () => {
  test.each<{
    name: string;
    map: Map<string, SerializedScope>;
    owners: Map<string, string>;
    start: string;
    expected: string | null;
  }>([
    {
      name: "starting scope itself is owner -> returns start",
      map: ancestorChain,
      owners: new Map([["child", "v"]]),
      start: "child",
      expected: "child",
    },
    {
      name: "walks up multiple levels to find owner",
      map: ancestorChain,
      owners: new Map([["grand", "v"]]),
      start: "child",
      expected: "grand",
    },
    {
      name: "finds owner one level up",
      map: ancestorChain,
      owners: new Map([["parent", "v"]]),
      start: "child",
      expected: "parent",
    },
    {
      name: "no owner anywhere -> null",
      map: ancestorChain,
      owners: new Map(),
      start: "child",
      expected: null,
    },
    {
      name: "starting scope id not in map -> null",
      map: new Map(),
      owners: new Map([["x", "v"]]),
      start: "missing",
      expected: null,
    },
    {
      name: "broken upper chain (referenced scope missing) -> null",
      map: new Map([
        ["child", { ...baseScope(), id: "child", upper: "missing" }],
      ]),
      owners: new Map(),
      start: "child",
      expected: null,
    },
  ])("$name", ({ map, owners, start, expected }) => {
    expect(findEnclosingSubgraphScope(start, map, owners)).toBe(expected);
  });
});
