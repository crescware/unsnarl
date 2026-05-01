import { describe, expect, test } from "vitest";

import { NODE_KIND, VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type {
  Direction,
  VisualElement,
  VisualNode,
  VisualSubgraph,
} from "../../visual-graph/model.js";
import { collectNodes } from "./collect-nodes.js";

const node = (id: string, line = 1): VisualNode => ({
  type: VISUAL_ELEMENT_TYPE.Node,
  id,
  kind: NODE_KIND.Variable,
  name: id,
  line,
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
