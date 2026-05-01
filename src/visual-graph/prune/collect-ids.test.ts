import { describe, expect, test } from "vitest";

import { DIRECTION, NODE_KIND, VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type {
  Direction,
  VisualElement,
  VisualNode,
  VisualSubgraph,
} from "../model.js";
import { collectIds } from "./collect-ids.js";

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
  direction: Direction = DIRECTION.TB,
): VisualSubgraph => ({
  type: VISUAL_ELEMENT_TYPE.Subgraph,
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
