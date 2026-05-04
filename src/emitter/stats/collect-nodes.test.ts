import { describe, expect, test } from "vitest";

import { DIRECTION } from "../../visual-graph/direction.js";
import type { Direction } from "../../visual-graph/direction.js";
import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { SUBGRAPH_KIND } from "../../visual-graph/subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../../visual-graph/visual-element-type.js";
import type { VisualElement } from "../../visual-graph/visual-element.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";
import type { VisualSubgraph } from "../../visual-graph/visual-subgraph.js";
import { collectNodes } from "./collect-nodes.js";

const node = (id: string, line = 1): VisualNode => ({
  type: VISUAL_ELEMENT_TYPE.Node,
  id,
  kind: NODE_KIND.Variable,
  name: id,
  line,
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

describe("collectNodes", () => {
  test("returns top-level nodes verbatim", () => {
    const out = collectNodes([node("a", 1), node("b", 2)]);
    expect(out.map((n) => n.id)).toEqual(["a", "b"]);
  });

  test("flattens one level of subgraph nesting", () => {
    const out = collectNodes([
      node("a"),
      sg("s", [node("b"), node("c")]),
      node("d"),
    ]);
    expect(out.map((n) => n.id)).toEqual(["a", "b", "c", "d"]);
  });

  test("flattens recursively across multiple levels", () => {
    const out = collectNodes([
      sg("s1", [sg("s2", [node("deep")]), node("mid")]),
      node("top"),
    ]);
    expect(out.map((n) => n.id)).toEqual(["deep", "mid", "top"]);
  });

  test("empty input → empty output", () => {
    expect(collectNodes([])).toEqual([]);
  });

  test("subgraph with no node descendants contributes nothing", () => {
    expect(collectNodes([sg("s", [sg("inner", [])])])).toEqual([]);
  });
});
