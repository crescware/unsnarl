import { describe, expect, test } from "vitest";

import type {
  Direction,
  VisualElement,
  VisualNode,
  VisualSubgraph,
} from "../model.js";
import { iterateVisualSubgraphs } from "./iterate-visual-subgraphs.js";

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

describe("iterateVisualSubgraphs", () => {
  test("yields nothing for plain nodes", () => {
    expect([...iterateVisualSubgraphs([node("a")])]).toEqual([]);
  });

  test("yields each subgraph in pre-order (parent before children)", () => {
    const out = [...iterateVisualSubgraphs([
      sg("outer", [node("x"), sg("inner", [node("y")])]),
      sg("sibling", []),
    ])];
    expect(out.map((s) => s.id)).toEqual(["outer", "inner", "sibling"]);
  });

  test("empty input → empty output", () => {
    expect([...iterateVisualSubgraphs([])]).toEqual([]);
  });
});
