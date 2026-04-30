import { describe, expect, test } from "vitest";

import type { ParsedRootQuery } from "../cli/root-query.js";
import type {
  VisualEdge,
  VisualElement,
  VisualGraph,
  VisualNode,
  VisualSubgraph,
} from "./model.js";
import { pruneVisualGraph } from "./prune.js";

function node(
  id: string,
  name: string,
  line: number,
  extra: Partial<VisualNode> = {},
): VisualNode {
  return {
    type: "node",
    id,
    kind: "Variable",
    name,
    line,
    ...extra,
  };
}

function subgraph(
  id: string,
  line: number,
  elements: VisualElement[],
  extra: Partial<VisualSubgraph> = {},
): VisualSubgraph {
  return {
    type: "subgraph",
    id,
    kind: "function",
    line,
    direction: "RL",
    elements,
    ...extra,
  };
}

function graph(elements: VisualElement[], edges: VisualEdge[]): VisualGraph {
  return {
    version: 1,
    source: { path: "x.ts", language: "ts" },
    direction: "RL",
    elements,
    edges,
  };
}

const rawLine = (n: number): ParsedRootQuery => ({
  kind: "line",
  line: n,
  raw: String(n),
});
const rawLineName = (n: number, name: string): ParsedRootQuery => ({
  kind: "line-name",
  line: n,
  name,
  raw: `${n}:${name}`,
});
const rawName = (name: string): ParsedRootQuery => ({
  kind: "name",
  name,
  raw: name,
});
const rawRange = (s: number, e: number): ParsedRootQuery => ({
  kind: "range",
  start: s,
  end: e,
  raw: `${s}-${e}`,
});

describe("pruneVisualGraph", () => {
  test("returns the graph unchanged when no roots are provided", () => {
    const g = graph(
      [node("a", "a", 1), node("b", "b", 2)],
      [{ from: "a", to: "b", label: "read" }],
    );
    const r = pruneVisualGraph(g, {
      roots: [],
      descendants: 5,
      ancestors: 5,
    });
    expect(r.graph).toBe(g);
    expect(r.perQuery).toEqual([]);
  });

  test("matches by line and keeps only the root when N=0", () => {
    const g = graph(
      [node("a", "a", 1), node("b", "b", 2), node("c", "c", 3)],
      [
        { from: "a", to: "b", label: "read" },
        { from: "b", to: "c", label: "read" },
      ],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawLine(2)],
      descendants: 0,
      ancestors: 0,
    });
    expect(r.graph.elements.map((e) => e.id)).toEqual(["b"]);
    expect(r.graph.edges).toEqual([]);
    expect(r.perQuery[0]?.matched).toBe(1);
  });

  test("expands descendants by N hops; the outbound boundary hint carries no label", () => {
    const g = graph(
      [
        node("a", "a", 1),
        node("b", "b", 2),
        node("c", "c", 3),
        node("d", "d", 4),
      ],
      [
        { from: "a", to: "b", label: "read" },
        { from: "b", to: "c", label: "read" },
        { from: "c", to: "d", label: "read" },
      ],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawLine(1)],
      descendants: 2,
      ancestors: 0,
    });
    expect(r.graph.elements.map((e) => e.id).sort()).toEqual(["a", "b", "c"]);
    expect(r.graph.boundaryEdges).toEqual([{ inside: "c", direction: "out" }]);
  });

  test("expands ancestors by N hops; the inbound boundary hint keeps the label", () => {
    const g = graph(
      [
        node("a", "a", 1),
        node("b", "b", 2),
        node("c", "c", 3),
        node("d", "d", 4),
      ],
      [
        { from: "a", to: "b", label: "read" },
        { from: "b", to: "c", label: "read" },
        { from: "c", to: "d", label: "read" },
      ],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawLine(4)],
      descendants: 0,
      ancestors: 2,
    });
    expect(r.graph.elements.map((e) => e.id).sort()).toEqual(["b", "c", "d"]);
    expect(r.graph.boundaryEdges).toEqual([
      { inside: "b", direction: "in", label: "read" },
    ]);
  });

  test("context expands in both directions and emits both-side boundary hints", () => {
    const g = graph(
      [
        node("a", "a", 1),
        node("b", "b", 2),
        node("c", "c", 3),
        node("d", "d", 4),
        node("e", "e", 5),
      ],
      [
        { from: "a", to: "b", label: "read" },
        { from: "b", to: "c", label: "read" },
        { from: "c", to: "d", label: "read" },
        { from: "d", to: "e", label: "read" },
      ],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawLine(3)],
      descendants: 1,
      ancestors: 1,
    });
    expect(r.graph.elements.map((e) => e.id).sort()).toEqual(["b", "c", "d"]);
    expect(r.graph.boundaryEdges).toEqual([
      { inside: "d", direction: "out" },
      { inside: "b", direction: "in", label: "read" },
    ]);
  });

  test("descendants=0 stays strict (no boundary peek emitted)", () => {
    const g = graph(
      [node("a", "a", 1), node("b", "b", 2)],
      [{ from: "a", to: "b", label: "read" }],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawLine(1)],
      descendants: 0,
      ancestors: 0,
    });
    expect(r.graph.elements.map((e) => e.id)).toEqual(["a"]);
    expect(r.graph.boundaryEdges).toEqual([]);
  });

  test("retains the parent subgraph wrapping a kept node", () => {
    const inner = node("inner", "x", 5);
    const sg = subgraph("sg1", 4, [inner]);
    const outer = node("outer", "y", 1);
    const g = graph([outer, sg], []);
    const r = pruneVisualGraph(g, {
      roots: [rawLineName(5, "x")],
      descendants: 0,
      ancestors: 0,
    });
    expect(r.graph.elements.map((e) => e.id)).toEqual(["sg1"]);
    expect((r.graph.elements[0] as VisualSubgraph).elements).toHaveLength(1);
    expect((r.graph.elements[0] as VisualSubgraph).elements[0]?.id).toBe(
      "inner",
    );
  });

  test("drops empty subgraphs", () => {
    const lonely = subgraph("sg2", 10, [node("z", "z", 11)]);
    const g = graph([node("a", "a", 1), lonely], []);
    const r = pruneVisualGraph(g, {
      roots: [rawLine(1)],
      descendants: 0,
      ancestors: 0,
    });
    expect(r.graph.elements.map((e) => e.id)).toEqual(["a"]);
  });

  test("treats subgraph ids as legitimate edge endpoints during BFS", () => {
    const g = graph(
      [
        node("flag", "flag", 1),
        subgraph("cont_if", 3, [node("wr1", "set", 4)], {
          kind: "if-else-container",
        }),
        node("result", "result", 10),
      ],
      [
        { from: "flag", to: "cont_if", label: "read" },
        { from: "wr1", to: "result", label: "read" },
      ],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawLineName(1, "flag")],
      descendants: 1,
      ancestors: 0,
    });
    const ids = new Set(flatten(r.graph.elements).map((e) => e.id));
    expect(ids.has("flag")).toBe(true);
    expect(ids.has("cont_if")).toBe(true);
  });

  test("counts per-query matches and reports zero when nothing matches", () => {
    const g = graph([node("a", "foo", 1), node("b", "bar", 2)], []);
    const r = pruneVisualGraph(g, {
      roots: [rawName("foo"), rawName("nope")],
      descendants: 0,
      ancestors: 0,
    });
    expect(r.perQuery[0]?.matched).toBe(1);
    expect(r.perQuery[1]?.matched).toBe(0);
    expect(r.graph.pruning?.roots).toEqual([
      { query: "foo", matched: 1 },
      { query: "nope", matched: 0 },
    ]);
  });

  test("emits an empty graph when every query misses", () => {
    const g = graph(
      [node("a", "foo", 1), node("b", "bar", 2)],
      [{ from: "a", to: "b", label: "read" }],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawName("nope")],
      descendants: 5,
      ancestors: 5,
    });
    expect(r.graph.elements).toEqual([]);
    expect(r.graph.edges).toEqual([]);
    expect(r.graph.pruning?.roots).toEqual([{ query: "nope", matched: 0 }]);
  });

  test("name-only query matches across scopes (multiple hits)", () => {
    const g = graph(
      [
        node("outer:counter", "counter", 1),
        subgraph("fn", 5, [node("inner:counter", "counter", 6)]),
      ],
      [],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawName("counter")],
      descendants: 0,
      ancestors: 0,
    });
    const ids = new Set(flatten(r.graph.elements).map((e) => e.id));
    expect(ids.has("outer:counter")).toBe(true);
    expect(ids.has("inner:counter")).toBe(true);
    expect(ids.has("fn")).toBe(true);
    expect(r.perQuery[0]?.matched).toBe(2);
  });

  test("range query covers all lines in the inclusive range", () => {
    const g = graph(
      [
        node("a", "a", 9),
        node("b", "b", 11),
        node("c", "c", 13),
        node("d", "d", 14),
      ],
      [],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawRange(9, 13)],
      descendants: 0,
      ancestors: 0,
    });
    const ids = r.graph.elements.map((e) => e.id);
    expect(ids).toEqual(["a", "b", "c"]);
  });
});

function flatten(elements: readonly VisualElement[]): VisualElement[] {
  const out: VisualElement[] = [];
  for (const e of elements) {
    out.push(e);
    if (e.type === "subgraph") {
      out.push(...flatten(e.elements));
    }
  }
  return out;
}
