import { describe, expect, test } from "vitest";

import { NODE_KIND, VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type {
  Direction,
  VisualElement,
  VisualNode,
  VisualSubgraph,
} from "../model.js";
import { collectNodeIds } from "./collect-node-ids.js";

const node = (id: string): VisualNode => ({
  type: VISUAL_ELEMENT_TYPE.Node,
  id,
  kind: NODE_KIND.Variable,
  name: id,
  line: 1,
  isJsxElement: false,
});

const sg = (
  id: string,
  elements: VisualElement[],
  direction: Direction = "TB",
): VisualSubgraph => ({
  type: VISUAL_ELEMENT_TYPE.Subgraph,
  id,
  kind: "scope",
  line: 1,
  direction,
  elements,
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
