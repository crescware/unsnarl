import { describe, expect, test } from "vitest";

import { DIRECTION } from "../direction.js";
import { NODE_KIND } from "../node-kind.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualElement } from "../visual-element.js";
import type { VisualNode } from "../visual-node.js";
import { findNodeById } from "./find-node-by-id.js";

function leafNode(id: string, name = id): VisualNode {
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    id,
    kind: NODE_KIND.Variable,
    name,
    line: 1,
    endLine: null,
    isJsxElement: false,
    unused: false,
    declarationKind: null,
    initIsFunction: false,
  };
}

const elements: VisualElement[] = [
  leafNode("a"),
  {
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id: "s1",
    kind: SUBGRAPH_KIND.Function,
    line: 1,
    endLine: null,
    direction: DIRECTION.RL,
    ownerNodeId: "n_owner",
    ownerName: "owner",
    elements: [
      leafNode("b"),
      {
        type: VISUAL_ELEMENT_TYPE.Subgraph,
        id: "s2",
        kind: SUBGRAPH_KIND.If,
        line: 1,
        endLine: null,
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
