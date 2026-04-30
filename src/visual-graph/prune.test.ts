import { describe, expect, test } from "vitest";

import type { ParsedRootQuery } from "../cli/root-query.js";
import type { SerializedIR, SerializedReference } from "../ir/model.js";
import type {
  VisualEdge,
  VisualElement,
  VisualGraph,
  VisualNode,
  VisualSubgraph,
} from "./model.js";
import { pruneVisualGraph } from "./prune.js";

function makeRef(
  refId: string,
  name: string,
  line: number,
  resolved: string | null,
): SerializedReference {
  return {
    id: refId,
    identifier: { name, span: { line, column: 0, offset: 0 } },
    from: "scope#0",
    resolved,
    owners: [],
    writeExpr: null,
    init: false,
    flags: { read: true, write: false, call: false, receiver: false },
    predicateContainer: null,
    returnContainer: null,
  };
}

function makeIr(references: SerializedReference[]): SerializedIR {
  return {
    version: 1,
    source: { path: "x.ts", language: "ts" },
    raw: "",
    scopes: [],
    variables: [],
    references,
    unusedVariableIds: [],
    diagnostics: [],
  };
}

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

  test("collapses repeated boundary entries on the same inside+direction (out)", () => {
    // root c with descendants=1: from c the BFS reaches d, but d also
    // has TWO outgoing cut edges (to e and to f). The boundary hint is
    // about "more graph past d", not about counting the cut edges, so
    // only one entry survives.
    const g = graph(
      [node("c", "c", 1), node("d", "d", 2)],
      [
        { from: "c", to: "d", label: "read" },
        { from: "d", to: "e", label: "read" },
        { from: "d", to: "f", label: "read" },
      ],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawLineName(1, "c")],
      descendants: 1,
      ancestors: 0,
    });
    expect(r.graph.boundaryEdges).toEqual([{ inside: "d", direction: "out" }]);
  });

  test("merges in-direction labels into a sorted, deduplicated comma list", () => {
    // Three different cut neighbors feed into M with overlapping label
    // sets. With ancestors=1 from A, the BFS reaches M (inside) but
    // stops there, so X/Y/Z are past the cut. Their three inbound edges
    // collapse onto a single boundary entry whose label is the union of
    // the comma-split parts (deduped, sorted, re-joined).
    const g = graph(
      [node("A", "a", 5), node("M", "m", 4)],
      [
        { from: "M", to: "A", label: "read" },
        { from: "X", to: "M", label: "read,call" },
        { from: "Y", to: "M", label: "read" },
        { from: "Z", to: "M", label: "set" },
      ],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawLineName(5, "a")],
      descendants: 0,
      ancestors: 1,
    });
    expect(r.graph.boundaryEdges).toEqual([
      { inside: "M", direction: "in", label: "call,read,set" },
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

  test("subgraph ids serve as BFS endpoints, but the cluster is only kept when something inside survives", () => {
    // BFS from flag with descendants=1 reaches the subgraph cont_if as
    // an edge endpoint. wr1 is one more hop in (descendants=2), so with
    // descendants=1 the cont_if cluster ends up empty -- it must NOT
    // appear in the final output, otherwise Mermaid's elk renderer
    // crashes on the empty cluster.
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
        { from: "cont_if", to: "wr1", label: "branch" },
        { from: "wr1", to: "result", label: "read" },
      ],
    );

    const tight = pruneVisualGraph(g, {
      roots: [rawLineName(1, "flag")],
      descendants: 1,
      ancestors: 0,
    });
    expect(tight.graph.elements.map((e) => e.id)).toEqual(["flag"]);

    const wider = pruneVisualGraph(g, {
      roots: [rawLineName(1, "flag")],
      descendants: 2,
      ancestors: 0,
    });
    const widerIds = new Set(flatten(wider.graph.elements).map((e) => e.id));
    expect(widerIds.has("flag")).toBe(true);
    expect(widerIds.has("cont_if")).toBe(true);
    expect(widerIds.has("wr1")).toBe(true);
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

describe("pruneVisualGraph: reference-aware line/range queries", () => {
  // Simulate a file with one declaration per line and a return-block-style
  // line where only references appear: builder.ts maps Variable id
  // "scope#0:foo@<offset>" to VisualNode id "n_scope_0_foo_<offset>".
  const declA = node("n_scope_0_a_6", "a", 1);
  const declB = node("n_scope_0_b_19", "b", 2);
  const declC = node("n_scope_0_c_32", "c", 3);

  test("a line query reaches identifiers that only appear as references on that line", () => {
    const g = graph([declA, declB, declC], []);
    const ir = makeIr([
      // Line 50 references a and c only — neither has a declaration there.
      makeRef("ref#0", "a", 50, "scope#0:a@6"),
      makeRef("ref#1", "c", 50, "scope#0:c@32"),
    ]);
    const r = pruneVisualGraph(
      g,
      {
        roots: [rawLine(50)],
        descendants: 0,
        ancestors: 0,
      },
      ir,
    );
    expect(r.graph.elements.map((e) => e.id).sort()).toEqual([
      "n_scope_0_a_6",
      "n_scope_0_c_32",
    ]);
    expect(r.perQuery[0]?.matched).toBe(2);
  });

  test("range queries pick up references anywhere inside the range", () => {
    const g = graph([declA, declB, declC], []);
    const ir = makeIr([
      makeRef("ref#0", "a", 49, "scope#0:a@6"),
      makeRef("ref#1", "b", 51, "scope#0:b@19"),
      makeRef("ref#2", "c", 60, "scope#0:c@32"), // outside the 49-51 range
    ]);
    const r = pruneVisualGraph(
      g,
      { roots: [rawRange(49, 51)], descendants: 0, ancestors: 0 },
      ir,
    );
    expect(r.graph.elements.map((e) => e.id).sort()).toEqual([
      "n_scope_0_a_6",
      "n_scope_0_b_19",
    ]);
  });

  test("line-name narrows the reference sweep by identifier as well", () => {
    const g = graph([declA, declB], []);
    const ir = makeIr([
      makeRef("ref#0", "a", 10, "scope#0:a@6"),
      makeRef("ref#1", "b", 10, "scope#0:b@19"),
    ]);
    const r = pruneVisualGraph(
      g,
      {
        roots: [
          {
            kind: "line-name",
            line: 10,
            name: "a",
            raw: "10:a",
          },
        ],
        descendants: 0,
        ancestors: 0,
      },
      ir,
    );
    expect(r.graph.elements.map((e) => e.id)).toEqual(["n_scope_0_a_6"]);
  });

  test("name-only queries deliberately ignore reference lines (declaration-scoped)", () => {
    const g = graph([declA, declB], []);
    const ir = makeIr([
      // A reference whose resolved Variable is `a`, but with a different name.
      // A name query for "a" must not be confused into adding b just because b
      // happens to be referenced somewhere too.
      makeRef("ref#0", "b", 99, "scope#0:b@19"),
    ]);
    const r = pruneVisualGraph(
      g,
      { roots: [rawName("a")], descendants: 0, ancestors: 0 },
      ir,
    );
    expect(r.graph.elements.map((e) => e.id)).toEqual(["n_scope_0_a_6"]);
  });

  test("references whose resolved Variable has no VisualNode are quietly dropped", () => {
    const g = graph([declA], []);
    const ir = makeIr([
      // Pretend the resolved Variable doesn't exist in the visual graph
      // (e.g. it was synthesized away). The query must not crash.
      makeRef("ref#0", "ghost", 5, "scope#9:ghost@999"),
    ]);
    const r = pruneVisualGraph(
      g,
      { roots: [rawLine(5)], descendants: 0, ancestors: 0 },
      ir,
    );
    expect(r.graph.elements).toEqual([]);
    expect(r.perQuery[0]?.matched).toBe(0);
  });

  test("works without ir (existing API path) — references are NOT folded in", () => {
    const g = graph([declA], []);
    // No ir argument: the reference-driven expansion is skipped entirely.
    const r = pruneVisualGraph(g, {
      roots: [rawLine(99)],
      descendants: 0,
      ancestors: 0,
    });
    expect(r.graph.elements).toEqual([]);
    expect(r.perQuery[0]?.matched).toBe(0);
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
