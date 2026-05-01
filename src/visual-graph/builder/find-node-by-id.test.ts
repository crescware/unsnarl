import { describe, expect, test } from "vitest";

import {
  DIRECTION,
  SUBGRAPH_KIND,
  VISUAL_ELEMENT_TYPE,
} from "../../constants.js";
import type { VisualElement, VisualNode } from "../model.js";
import { findNodeById } from "./find-node-by-id.js";

function leafNode(id: string, name = id): VisualNode {
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    id,
    kind: "Variable",
    name,
    line: 1,
    isJsxElement: false,
  };
}

const elements: VisualElement[] = [
  leafNode("a"),
  {
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id: "s1",
    kind: SUBGRAPH_KIND.Function,
    line: 1,
    direction: DIRECTION.RL,
    elements: [
      leafNode("b"),
      {
        type: VISUAL_ELEMENT_TYPE.Subgraph,
        id: "s2",
        kind: SUBGRAPH_KIND.If,
        line: 1,
        direction: DIRECTION.RL,
        elements: [leafNode("c")],
      },
    ],
  },
  leafNode("d"),
];

describe("findNodeById", () => {
  test.each([
    { id: "a", expected: "a" },
    { id: "b", expected: "b" },
    { id: "c", expected: "c" },
    { id: "d", expected: "d" },
  ])("finds top-level and nested node $id", ({ id, expected }) => {
    expect(findNodeById(elements, id)?.id).toBe(expected);
  });

  test("returns null when id is absent", () => {
    expect(findNodeById(elements, "missing")).toBeNull();
  });

  test("returns null on an empty element list", () => {
    expect(findNodeById([], "a")).toBeNull();
  });

  test("ignores subgraph ids (only matches node ids)", () => {
    expect(findNodeById(elements, "s1")).toBeNull();
    expect(findNodeById(elements, "s2")).toBeNull();
  });
});
