import { describe, expect, test } from "vitest";

import { DIRECTION } from "../direction.js";
import type { Direction } from "../direction.js";
import { NODE_KIND } from "../node-kind.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualElement } from "../visual-element.js";
import type { VisualNode } from "../visual-node.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import { buildParentMap } from "./build-parent-map.js";

const node = (id: string): VisualNode => ({
  type: VISUAL_ELEMENT_TYPE.Node,
  id,
  kind: NODE_KIND.LegacyVariable,
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
    expect(buildParentMap([node("a"), node("b")]).size).toEqual(0);
  });

  test("each child of a subgraph maps to that subgraph id", () => {
    const map = buildParentMap([sg("s", [node("x"), node("y")])]);
    expect(map.get("x")).toEqual("s");
    expect(map.get("y")).toEqual("s");
    expect(map.has("s")).toEqual(false);
  });

  test("nested subgraphs chain correctly", () => {
    const map = buildParentMap([sg("outer", [sg("inner", [node("deep")])])]);
    expect(map.get("inner")).toEqual("outer");
    expect(map.get("deep")).toEqual("inner");
  });
});
