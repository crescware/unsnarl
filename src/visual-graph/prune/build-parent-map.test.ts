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
import { buildParentMap } from "./build-parent-map.js";

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

describe("buildParentMap", () => {
  test("top-level elements have no parent entry", () => {
    expect(buildParentMap([node("a"), node("b")]).size).toBe(0);
  });

  test("each child of a subgraph maps to that subgraph id", () => {
    const map = buildParentMap([sg("s", [node("x"), node("y")])]);
    expect(map.get("x")).toBe("s");
    expect(map.get("y")).toBe("s");
    expect(map.has("s")).toBe(false);
  });

  test("nested subgraphs chain correctly", () => {
    const map = buildParentMap([sg("outer", [sg("inner", [node("deep")])])]);
    expect(map.get("inner")).toBe("outer");
    expect(map.get("deep")).toBe("inner");
  });
});
