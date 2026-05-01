import { describe, expect, test } from "vitest";

import type {
  Direction,
  NodeKind,
  VisualElement,
  VisualNode,
  VisualSubgraph,
} from "../model.js";
import { iterateVisualNodes } from "./iterate-visual-nodes.js";

const node = (id: string, kind: NodeKind = "Variable"): VisualNode => ({
  type: "node",
  id,
  kind,
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

describe("iterateVisualNodes", () => {
  test("yields only ROOT_CANDIDATE_KINDS nodes", () => {
    const out = [
      ...iterateVisualNodes([node("a"), node("b", "PropertyKey" as NodeKind)]),
    ] satisfies VisualNode[];
    expect(out.map((n) => n.id)).toEqual(["a"]);
  });

  test("recurses into subgraphs", () => {
    const out = [
      ...iterateVisualNodes([
        sg("s", [node("inner"), sg("s2", [node("deep")])]),
        node("top"),
      ]),
    ] satisfies VisualNode[];
    expect(out.map((n) => n.id)).toEqual(["inner", "deep", "top"]);
  });

  test("empty input → empty output", () => {
    expect([...iterateVisualNodes([])]).toEqual([]);
  });
});
