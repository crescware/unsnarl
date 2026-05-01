import { describe, expect, test } from "vitest";

import { DIRECTION } from "../../direction.js";
import { NODE_KIND } from "../../node-kind.js";
import { SUBGRAPH_KIND } from "../../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../../visual-element-type.js";
import type {
  Direction,
  NodeKind,
  VisualElement,
  VisualNode,
  VisualSubgraph,
} from "../model.js";
import { iterateVisualNodes } from "./iterate-visual-nodes.js";

const node = (id: string, kind: NodeKind = NODE_KIND.Variable): VisualNode => ({
  type: VISUAL_ELEMENT_TYPE.Node,
  id,
  kind,
  name: id,
  line: 1,
  isJsxElement: false,
});

const sg = (
  id: string,
  elements: VisualElement[],
  direction: Direction = DIRECTION.TB,
): VisualSubgraph => ({
  type: VISUAL_ELEMENT_TYPE.Subgraph,
  id,
  kind: SUBGRAPH_KIND.Function,
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
