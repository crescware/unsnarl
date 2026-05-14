import { describe, expect, test } from "vitest";

import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { asFilledString } from "../../util/filled-string.js";
import { isAncestorScope } from "./is-ancestor-scope.js";
import { baseScope } from "./testing/make-scope.js";

const root = { ...baseScope(), id: asScopeId("root") };
const mid = { ...baseScope(), id: asScopeId("mid"), upper: asScopeId("root") };
const leaf = { ...baseScope(), id: asScopeId("leaf"), upper: asScopeId("mid") };
const sibling = {
  ...baseScope(),
  id: asScopeId("sibling"),
  upper: asScopeId("root"),
};
const map = new Map<string, SerializedScope>(
  [root, mid, leaf, sibling].map((v) => [v.id, v]),
);

describe("isAncestorScope", () => {
  test.each<{
    name: string;
    ancestor: string;
    descendant: string;
    expected: boolean;
  }>([
    {
      name: asFilledString("self is its own ancestor"),
      ancestor: "leaf",
      descendant: "leaf",
      expected: true,
    },
    {
      name: asFilledString("direct parent is ancestor"),
      ancestor: "mid",
      descendant: "leaf",
      expected: true,
    },
    {
      name: asFilledString("grandparent is ancestor"),
      ancestor: "root",
      descendant: "leaf",
      expected: true,
    },
    {
      name: asFilledString("child is not ancestor of its parent"),
      ancestor: "leaf",
      descendant: "mid",
      expected: false,
    },
    {
      name: asFilledString("sibling is not ancestor"),
      ancestor: "sibling",
      descendant: "leaf",
      expected: false,
    },
    {
      name: asFilledString("missing descendant returns false"),
      ancestor: "root",
      descendant: "missing",
      expected: false,
    },
  ])("$name", ({ ancestor, descendant, expected }) => {
    expect(isAncestorScope(ancestor, descendant, map)).toEqual(expected);
  });

  test("broken upper chain returns false at the break", () => {
    const orphan = {
      ...baseScope(),
      id: asScopeId("orphan"),
      upper: asScopeId("missing"),
    };
    const broken = new Map([[orphan.id, orphan]]);
    expect(isAncestorScope("any", "orphan", broken)).toEqual(false);
  });
});
