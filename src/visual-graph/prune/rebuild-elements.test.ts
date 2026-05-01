import { describe, expect, test } from "vitest";

import { DIRECTION, NODE_KIND, VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type {
  Direction,
  VisualElement,
  VisualNode,
  VisualSubgraph,
} from "../model.js";
import { rebuildElements } from "./rebuild-elements.js";

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

describe("rebuildElements", () => {
  test("keeps only nodes whose id is in the keep set", () => {
    const out = rebuildElements(
      [node("a"), node("b"), node("c")],
      new Set(["a", "c"]),
    );
    expect(out.map((e) => e.id)).toEqual(["a", "c"]);
  });

  test("subgraph survives only when at least one descendant survives", () => {
    const out = rebuildElements(
      [sg("s", [node("x"), node("y")]), sg("t", [node("z")])],
      new Set(["x"]),
    );
    expect(out.map((e) => e.id)).toEqual(["s"]);
    const survivingSubgraph = out[0] as VisualSubgraph;
    expect(survivingSubgraph.elements.map((e) => e.id)).toEqual(["x"]);
  });

  test("subgraph with zero surviving descendants is dropped", () => {
    const out = rebuildElements([sg("empty", [node("x")])], new Set());
    expect(out).toEqual([]);
  });

  test("nested subgraphs are reconstructed independently", () => {
    const out = rebuildElements(
      [sg("outer", [sg("inner", [node("deep")]), node("mid")])],
      new Set(["deep"]),
    );
    expect(out).toHaveLength(1);
    const outer = out[0] as VisualSubgraph;
    expect(outer.elements).toHaveLength(1);
    const inner = outer.elements[0] as VisualSubgraph;
    expect(inner.id).toBe("inner");
    expect(inner.elements.map((e) => e.id)).toEqual(["deep"]);
  });

  test("returned nodes are clones (immutability)", () => {
    const original = node("a");
    const [clone] = rebuildElements([original], new Set(["a"]));
    expect(clone).not.toBe(original);
    expect(clone).toEqual(original);
  });
});
