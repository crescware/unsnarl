import { describe, expect, test } from "vitest";

import type { SerializedScope } from "../../ir/model.js";
import { isAncestorScope } from "./is-ancestor-scope.js";
import { makeScope } from "./testing/make-scope.js";

const root = makeScope({ id: "root" });
const mid = makeScope({ id: "mid", upper: "root" });
const leaf = makeScope({ id: "leaf", upper: "mid" });
const sibling = makeScope({ id: "sibling", upper: "root" });
const map = new Map<string, SerializedScope>(
  [root, mid, leaf, sibling].map((s) => [s.id, s]),
);

describe("isAncestorScope", () => {
  test.each<{
    name: string;
    ancestor: string;
    descendant: string;
    expected: boolean;
  }>([
    {
      name: "self is its own ancestor",
      ancestor: "leaf",
      descendant: "leaf",
      expected: true,
    },
    {
      name: "direct parent is ancestor",
      ancestor: "mid",
      descendant: "leaf",
      expected: true,
    },
    {
      name: "grandparent is ancestor",
      ancestor: "root",
      descendant: "leaf",
      expected: true,
    },
    {
      name: "child is not ancestor of its parent",
      ancestor: "leaf",
      descendant: "mid",
      expected: false,
    },
    {
      name: "sibling is not ancestor",
      ancestor: "sibling",
      descendant: "leaf",
      expected: false,
    },
    {
      name: "missing descendant returns false",
      ancestor: "root",
      descendant: "missing",
      expected: false,
    },
  ])("$name", ({ ancestor, descendant, expected }) => {
    expect(isAncestorScope(ancestor, descendant, map)).toBe(expected);
  });

  test("broken upper chain returns false at the break", () => {
    const orphan = makeScope({ id: "orphan", upper: "missing" });
    const broken = new Map([[orphan.id, orphan]]);
    expect(isAncestorScope("any", "orphan", broken)).toBe(false);
  });
});
