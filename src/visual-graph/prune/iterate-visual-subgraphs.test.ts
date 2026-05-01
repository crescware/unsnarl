import { describe, expect, test } from "vitest";

import { DIRECTION } from "../direction.js";
import type {
  Direction,
  VisualElement,
  VisualNode,
  VisualSubgraph,
} from "../model.js";
import { NODE_KIND } from "../node-kind.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import { iterateVisualSubgraphs } from "./iterate-visual-subgraphs.js";

const node = (id: string): VisualNode => ({
  type: VISUAL_ELEMENT_TYPE.Node,
  id,
  kind: NODE_KIND.Variable,
  name: id,
  line: 1,
  isJsxElement: false,
  endLine: null,
  unused: false,
  declarationKind: null,
  initIsFunction: false,
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
  endLine: null,
  caseTest: null,
  hasElse: false,
  ownerNodeId: null,
  ownerName: null,
});

describe("iterateVisualSubgraphs", () => {
  test("yields nothing for plain nodes", () => {
    expect([...iterateVisualSubgraphs([node("a")])]).toEqual([]);
  });

  test("yields each subgraph in pre-order (parent before children)", () => {
    const out = [
      ...iterateVisualSubgraphs([
        sg("outer", [node("x"), sg("inner", [node("y")])]),
        sg("sibling", []),
      ]),
    ] satisfies VisualSubgraph[];
    expect(out.map((s) => s.id)).toEqual(["outer", "inner", "sibling"]);
  });

  test("empty input → empty output", () => {
    expect([...iterateVisualSubgraphs([])]).toEqual([]);
  });
});
