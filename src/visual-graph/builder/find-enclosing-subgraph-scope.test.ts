import { describe, expect, test } from "vitest";

import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { asFilledString } from "../../util/filled-string.js";
import { findEnclosingSubgraphScope } from "./find-enclosing-subgraph-scope.js";
import { baseScope } from "./testing/make-scope.js";

const grand = { ...baseScope(), id: asScopeId("grand") };
const parent = {
  ...baseScope(),
  id: asScopeId("parent"),
  upper: asScopeId("grand"),
};
const child = {
  ...baseScope(),
  id: asScopeId("child"),
  upper: asScopeId("parent"),
};
const ancestorChain = new Map<string, SerializedScope>(
  [grand, parent, child].map((v) => [v.id, v]),
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
      name: asFilledString("starting scope itself is owner -> returns start"),
      map: ancestorChain,
      owners: new Map([["child", "v"]]),
      start: "child",
      expected: "child",
    },
    {
      name: asFilledString("walks up multiple levels to find owner"),
      map: ancestorChain,
      owners: new Map([["grand", "v"]]),
      start: "child",
      expected: "grand",
    },
    {
      name: asFilledString("finds owner one level up"),
      map: ancestorChain,
      owners: new Map([["parent", "v"]]),
      start: "child",
      expected: "parent",
    },
    {
      name: asFilledString("no owner anywhere -> null"),
      map: ancestorChain,
      owners: new Map(),
      start: "child",
      expected: null,
    },
    {
      name: asFilledString("starting scope id not in map -> null"),
      map: new Map(),
      owners: new Map([["x", "v"]]),
      start: "missing",
      expected: null,
    },
    {
      name: asFilledString(
        "broken upper chain (referenced scope missing) -> null",
      ),
      map: new Map([
        [
          "child",
          {
            ...baseScope(),
            id: asScopeId("child"),
            upper: asScopeId("missing"),
          },
        ],
      ]),
      owners: new Map(),
      start: "child",
      expected: null,
    },
  ])("$name", ({ map, owners, start, expected }) => {
    expect(findEnclosingSubgraphScope(start, map, owners)).toEqual(expected);
  });
});
