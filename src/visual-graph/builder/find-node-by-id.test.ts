import { describe, expect, test } from "vitest";

import { DIRECTION } from "../../constants.js";
import type { VisualElement, VisualNode } from "../model.js";
import { findNodeById } from "./find-node-by-id.js";

function leafNode(id: string, name = id): VisualNode {
  return {
    type: "node",
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
    type: "subgraph",
    id: "s1",
    kind: "function",
    line: 1,
    direction: DIRECTION.RL,
    elements: [
      leafNode("b"),
      {
        type: "subgraph",
        id: "s2",
        kind: "if",
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
