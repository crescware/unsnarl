import { describe, expect, test } from "vitest";

import type {
  Direction,
  VisualElement,
  VisualNode,
  VisualSubgraph,
} from "../model.js";
import { collectIds } from "./collect-ids.js";

const node = (id: string): VisualNode => ({
  type: "node",
  id,
  kind: "Variable",
  name: id,
  line: 1,
  isJsxElement: false,
});

const sg = (
  id: string,
  elements: VisualElement[],
  direction: Direction = "TB",
): VisualSubgraph => ({
  type: "subgraph",
  id,
  kind: "scope",
  line: 1,
  direction,
  elements,
});

describe("collectIds", () => {
  test("includes both node ids and subgraph ids (unlike collectNodeIds)", () => {
    const ids = collectIds([
      sg("outer", [node("x"), sg("inner", [node("y")])]),
      node("top"),
    ]);
    expect([...ids].sort()).toEqual(["inner", "outer", "top", "x", "y"]);
  });

  test("empty input → empty set", () => {
    expect(collectIds([]).size).toBe(0);
  });

  test("returns a Set (deduped already by Map semantics)", () => {
    expect(collectIds([node("a")])).toBeInstanceOf(Set);
  });
});
