import { describe, expect, test } from "vitest";

import { LANGUAGE } from "../../language.js";
import { ROOT_QUERY_KIND } from "../../root-query/root-query-kind.js";
import { DIRECTION } from "../direction.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualGraph } from "../visual-graph.js";
import type { VisualNode } from "../visual-node.js";
import { collectHighlightIds } from "./collect-highlight-ids.js";

function variableNode(id: string, name: string, line: number): VisualNode {
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    kind: NODE_KIND.ConstBinding,
    id,
    name,
    line,
    endLine: null,
    isJsxElement: false,
    unused: false,
    initIsFunction: false,
  };
}

function returnUseNode(id: string, name: string, line: number): VisualNode {
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    kind: NODE_KIND.ReturnArgumentReference,
    id,
    name,
    line,
    endLine: null,
    isJsxElement: false,
    unused: false,
  };
}

function writeOpNode(id: string, name: string, line: number): VisualNode {
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    kind: NODE_KIND.WriteReference,
    id,
    name,
    line,
    endLine: null,
    isJsxElement: false,
    unused: false,
    declarationKind: null,
  };
}

function graphOf(...nodes: readonly VisualNode[]): VisualGraph {
  return {
    version: 1,
    source: { path: "x.ts", language: LANGUAGE.Ts },
    direction: DIRECTION.RL,
    elements: [...nodes],
    edges: [],
    boundaryEdges: [],
    pruning: null,
  };
}

describe("collectHighlightIds", () => {
  test("returns an empty set when no queries are supplied", () => {
    const g = graphOf(variableNode("n_a", "a", 1));
    expect(collectHighlightIds(g, []).size).toEqual(0);
  });

  test("name query collects every node carrying that source name", () => {
    const g = graphOf(
      variableNode("n_a_decl", "a", 1),
      variableNode("n_a_use", "a", 2),
      variableNode("n_b", "b", 3),
    );
    const ids = collectHighlightIds(g, [
      { kind: ROOT_QUERY_KIND.Name, name: "a", raw: "a" },
    ]);
    expect([...ids].sort()).toEqual(["n_a_decl", "n_a_use"]);
  });

  test("line query collects every node on that line", () => {
    const g = graphOf(
      variableNode("n_a", "a", 1),
      variableNode("n_b", "b", 2),
      variableNode("n_c", "c", 2),
    );
    const ids = collectHighlightIds(g, [
      { kind: ROOT_QUERY_KIND.Line, line: 2, raw: "2" },
    ]);
    expect([...ids].sort()).toEqual(["n_b", "n_c"]);
  });

  test("a query that matches nothing yields an empty set without throwing", () => {
    const g = graphOf(variableNode("n_a", "a", 1));
    const ids = collectHighlightIds(g, [
      { kind: ROOT_QUERY_KIND.Name, name: "nope", raw: "nope" },
    ]);
    expect(ids.size).toEqual(0);
  });

  // Highlight diverges from `-r/--roots` here on purpose: pruning's
  // `nodeMatchesQuery` would skip `WriteOp` / `ReturnUse` on a bare
  // name query, but for highlight the user benefit is "paint every
  // place this identifier appears". This test pins the divergence.
  test("name query matches WriteOp and ReturnUse nodes (unlike -r)", () => {
    const g = graphOf(
      variableNode("n_decl", "counter", 1),
      writeOpNode("n_write", "counter", 2),
      returnUseNode("n_return", "counter", 3),
      variableNode("n_other", "other", 4),
    );
    const ids = collectHighlightIds(g, [
      { kind: ROOT_QUERY_KIND.Name, name: "counter", raw: "counter" },
    ]);
    expect([...ids].sort()).toEqual(["n_decl", "n_return", "n_write"]);
  });
});
