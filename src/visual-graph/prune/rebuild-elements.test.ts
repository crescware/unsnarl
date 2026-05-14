import { describe, expect, test } from "vitest";

import { DIRECTION } from "../direction.js";
import type { Direction } from "../direction.js";
import { NODE_KIND } from "../node-kind.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualElement } from "../visual-element.js";
import type { VisualNode } from "../visual-node.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import { rebuildElements } from "./rebuild-elements.js";

const node = (id: string): VisualNode => ({
  type: VISUAL_ELEMENT_TYPE.Node,
  id,
  kind: NODE_KIND.ConstBinding,
  name: id,
  line: 1,
  isJsxElement: false,
  endLine: null,
  unused: false,
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

describe("rebuildElements", () => {
  test("keeps only nodes whose id is in the keep set", () => {
    const out = rebuildElements(
      [node("a"), node("b"), node("c")],
      new Set(["a", "c"]),
    );
    expect(out.map((v) => v.id)).toEqual(["a", "c"]);
  });

  test("subgraph survives only when at least one descendant survives", () => {
    const out = rebuildElements(
      [sg("s", [node("x"), node("y")]), sg("t", [node("z")])],
      new Set(["x"]),
    );
    expect(out.map((v) => v.id)).toEqual(["s"]);
    const survivingSubgraph = out[0] as VisualSubgraph;
    expect(survivingSubgraph.elements.map((v) => v.id)).toEqual(["x"]);
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
    expect(inner.id).toEqual("inner");
    expect(inner.elements.map((v) => v.id)).toEqual(["deep"]);
  });

  test("returned nodes are clones (immutability)", () => {
    const original = node("a");
    const [clone] = rebuildElements([original], new Set(["a"]));
    expect(clone !== original).toEqual(true);
    expect(clone).toEqual(original);
  });
});
