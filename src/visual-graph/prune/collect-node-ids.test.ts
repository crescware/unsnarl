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
import { collectNodeIds } from "./collect-node-ids.js";

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
  ownerNodeId: "n_owner",
  ownerName: "owner",
});

describe("collectNodeIds", () => {
  test("returns top-level node ids", () => {
    expect(collectNodeIds([node("a"), node("b")])).toEqual(["a", "b"]);
  });

  test("recurses into subgraphs but does NOT include subgraph ids", () => {
    expect(
      collectNodeIds([sg("s", [node("x"), sg("inner", [node("y")])])]),
    ).toEqual(["x", "y"]);
  });

  test("empty input → empty output", () => {
    expect(collectNodeIds([])).toEqual([]);
  });
});
