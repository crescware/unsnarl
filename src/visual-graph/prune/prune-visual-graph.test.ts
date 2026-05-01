import { describe, expect, test } from "vitest";

import { LANGUAGE } from "../../cli/language.js";
import type { ParsedRootQuery } from "../../cli/root-query/parsed-root-query.js";
import { ROOT_QUERY_KIND } from "../../cli/root-query/root-query-kind.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import { DIRECTION } from "../direction.js";
import type {
  VisualEdge,
  VisualElement,
  VisualGraph,
  VisualNode,
  VisualSubgraph,
} from "../model.js";
import { NODE_KIND } from "../node-kind.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import { BOUNDARY_EDGE_DIRECTION } from "./boundary-edge-direction.js";
import { pruneVisualGraph } from "./prune-visual-graph.js";

function node(
  id: string,
  name: string,
  line: number,
  extra: Partial<VisualNode> = {},
): VisualNode {
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    id,
    kind: NODE_KIND.Variable,
    name,
    line,
    endLine: null,
    isJsxElement: false,
    unused: false,
    declarationKind: null,
    initIsFunction: false,
    importKind: null,
    importedName: null,
    importSource: null,
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
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id,
    kind: SUBGRAPH_KIND.Function,
    line,
    endLine: null,
    direction: DIRECTION.RL,
    caseTest: null,
    hasElse: false,
    ownerNodeId: null,
    ownerName: null,
    elements,
    ...extra,
  };
}

function graph(elements: VisualElement[], edges: VisualEdge[]): VisualGraph {
  return {
    version: SERIALIZED_IR_VERSION,
    source: { path: "x.ts", language: LANGUAGE.Ts },
    direction: DIRECTION.RL,
    elements,
    edges,
    boundaryEdges: [],
    pruning: null,
  };
}

const rawLine = (n: number): ParsedRootQuery => ({
  kind: ROOT_QUERY_KIND.Line,
  line: n,
  raw: String(n),
});
const rawLineName = (n: number, name: string): ParsedRootQuery => ({
  kind: ROOT_QUERY_KIND.LineName,
  line: n,
  name,
  raw: `${n}:${name}`,
});
const rawName = (name: string): ParsedRootQuery => ({
  kind: ROOT_QUERY_KIND.Name,
  name,
  raw: name,
});
const rawRange = (s: number, e: number): ParsedRootQuery => ({
  kind: ROOT_QUERY_KIND.Range,
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
    expect(r.graph.boundaryEdges).toEqual([
      { inside: "c", direction: BOUNDARY_EDGE_DIRECTION.Out },
    ]);
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
      { inside: "b", direction: BOUNDARY_EDGE_DIRECTION.In, label: "read" },
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
      { inside: "d", direction: BOUNDARY_EDGE_DIRECTION.Out },
      { inside: "b", direction: BOUNDARY_EDGE_DIRECTION.In, label: "read" },
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
    expect(r.graph.boundaryEdges).toEqual([
      { inside: "d", direction: BOUNDARY_EDGE_DIRECTION.Out },
    ]);
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
      {
        inside: "M",
        direction: BOUNDARY_EDGE_DIRECTION.In,
        label: "call,read,set",
      },
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
          kind: SUBGRAPH_KIND.IfElseContainer,
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

describe("pruneVisualGraph: ReturnUse / WriteOp as direct roots", () => {
  test("a line query matches a ReturnUse at that line directly (no longer routed through the resolved declaration)", () => {
    const declA = node("n_scope_0_a_6", "a", 1);
    const useA = node("ret_use_ref_0", "a", 11, { kind: NODE_KIND.ReturnUse });
    const ret = subgraph("sg_return", 10, [useA], {
      kind: SUBGRAPH_KIND.Return,
    });
    const g = graph(
      [declA, ret],
      [{ from: declA.id, to: useA.id, label: "read" }],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawLine(11)],
      descendants: 0,
      ancestors: 0,
    });
    // Only the use is a root; the declaration is reachable via ancestors=N>0.
    expect(
      flatten(r.graph.elements)
        .map((e) => e.id)
        .sort(),
    ).toEqual(["ret_use_ref_0", "sg_return"]);
    expect(r.perQuery[0]?.matched).toBe(1);
  });

  test("ancestors=1 reaches the declaration from a ReturnUse root", () => {
    const declA = node("n_scope_0_a_6", "a", 1);
    const useA = node("ret_use_ref_0", "a", 11, { kind: NODE_KIND.ReturnUse });
    const ret = subgraph("sg_return", 10, [useA], {
      kind: SUBGRAPH_KIND.Return,
    });
    const g = graph(
      [declA, ret],
      [{ from: declA.id, to: useA.id, label: "read" }],
    );
    const r = pruneVisualGraph(g, {
      roots: [rawLine(11)],
      descendants: 0,
      ancestors: 1,
    });
    const ids = flatten(r.graph.elements).map((e) => e.id);
    expect(ids).toContain("n_scope_0_a_6");
    expect(ids).toContain("ret_use_ref_0");
  });

  test("a JSX ReturnUse spanning multiple lines is matched anywhere within [line, endLine]", () => {
    const useA = node("ret_use_ref_0", "a", 11, {
      kind: NODE_KIND.ReturnUse,
      endLine: 23,
    });
    const ret = subgraph("sg_return", 10, [useA], {
      kind: SUBGRAPH_KIND.Return,
    });
    const g = graph([ret], []);
    const r = pruneVisualGraph(g, {
      roots: [rawLine(23)],
      descendants: 0,
      ancestors: 0,
    });
    expect(flatten(r.graph.elements).map((e) => e.id)).toContain(
      "ret_use_ref_0",
    );
    expect(r.perQuery[0]?.matched).toBe(1);
  });

  test("a WriteOp is also a root candidate", () => {
    const writeOp = node("wr_ref_0", "x", 5, { kind: NODE_KIND.WriteOp });
    const g = graph([writeOp], []);
    const r = pruneVisualGraph(g, {
      roots: [rawLine(5)],
      descendants: 0,
      ancestors: 0,
    });
    expect(r.graph.elements.map((e) => e.id)).toEqual(["wr_ref_0"]);
    expect(r.perQuery[0]?.matched).toBe(1);
  });

  test("name queries skip WriteOp / ReturnUse — `-r foo` stays declaration-scoped", () => {
    // Three "foo"-named nodes share a name. A bare name query should pin
    // the root on the declaration only; the assignment site and the JSX
    // usage are reachable via descendants/ancestors but never auto-rooted.
    const decl = node("n_decl_foo", "foo", 1);
    const writeOp = node("wr_foo", "foo", 5, { kind: NODE_KIND.WriteOp });
    const ret = node("ret_foo", "foo", 11, { kind: NODE_KIND.ReturnUse });
    const g = graph([decl, writeOp, ret], []);
    const r = pruneVisualGraph(g, {
      roots: [rawName("foo")],
      descendants: 0,
      ancestors: 0,
    });
    expect(r.graph.elements.map((e) => e.id)).toEqual(["n_decl_foo"]);
    expect(r.perQuery[0]?.matched).toBe(1);
  });

  test("line-name still matches WriteOp / ReturnUse at the requested line", () => {
    // A line-name query is still positional, so the use-site nodes remain
    // valid roots. This protects the "use line + name disambiguator" case.
    const writeOp = node("wr_foo", "foo", 5, { kind: NODE_KIND.WriteOp });
    const ret = node("ret_foo", "foo", 11, { kind: NODE_KIND.ReturnUse });
    const g = graph([writeOp, ret], []);
    const r = pruneVisualGraph(g, {
      roots: [
        {
          kind: ROOT_QUERY_KIND.LineName,
          line: 11,
          name: "foo",
          raw: "11:foo",
        },
      ],
      descendants: 0,
      ancestors: 0,
    });
    expect(r.graph.elements.map((e) => e.id)).toEqual(["ret_foo"]);
    expect(r.perQuery[0]?.matched).toBe(1);
  });
});

describe("pruneVisualGraph: subgraph line matching", () => {
  test("a bare line query equal to a subgraph's start line sweeps every node it contains", () => {
    const inner = node("inner_a", "a", 11);
    const outerOnly = node("outside", "z", 50);
    const sg = subgraph("sg_return", 10, [inner], {
      kind: SUBGRAPH_KIND.Return,
    });
    const g = graph([sg, outerOnly], []);
    const r = pruneVisualGraph(g, {
      roots: [rawLine(10)],
      descendants: 0,
      ancestors: 0,
    });
    // The subgraph survives because at least one descendant did.
    const flat = collectIds(r.graph.elements);
    expect(flat).toContain("inner_a");
    expect(flat).not.toContain("outside");
    expect(r.perQuery[0]?.matched).toBe(1);
  });

  test("a range query never auto-pulls a subgraph's body, even if its start line falls inside the range", () => {
    const inner = node("inner_a", "a", 11);
    const sg = subgraph("sg_return", 10, [inner], {
      kind: SUBGRAPH_KIND.Return,
    });
    const g = graph([sg], []);
    const r = pruneVisualGraph(g, {
      // Range [10..11] would contain both the subgraph's start and the inner
      // node's line; only the inner-node match should keep it, not the
      // subgraph match (which is line-only by design).
      roots: [rawRange(10, 11)],
      descendants: 0,
      ancestors: 0,
    });
    const flat = collectIds(r.graph.elements);
    expect(flat).toContain("inner_a");
    expect(r.perQuery[0]?.matched).toBe(1);
  });

  test("a line query that is not a subgraph's start line falls back to per-node matching", () => {
    const inner = node("inner_a", "a", 11);
    const sg = subgraph("sg_return", 10, [inner], {
      kind: SUBGRAPH_KIND.Return,
    });
    const g = graph([sg], []);
    const r = pruneVisualGraph(g, {
      // 11 is the inner node's line, not the subgraph's start line.
      roots: [rawLine(11)],
      descendants: 0,
      ancestors: 0,
    });
    // The subgraph survives only because its descendant did; the match
    // count is for the per-node hit, not a subgraph hit.
    const flat = collectIds(r.graph.elements);
    expect(flat).toContain("inner_a");
    expect(flat).toContain("sg_return");
    expect(r.perQuery[0]?.matched).toBe(1);
  });
});

describe("pruneVisualGraph: VisualNode endLine matching", () => {
  test("a line inside a node's [line, endLine] window matches it", () => {
    const ranged = node("a", "a", 11, { endLine: 23 });
    const g = graph([ranged], []);
    const r = pruneVisualGraph(g, {
      roots: [rawLine(20)],
      descendants: 0,
      ancestors: 0,
    });
    expect(r.graph.elements.map((e) => e.id)).toEqual(["a"]);
  });

  test("a line just past endLine does not match", () => {
    const ranged = node("a", "a", 11, { endLine: 23 });
    const g = graph([ranged], []);
    const r = pruneVisualGraph(g, {
      roots: [rawLine(24)],
      descendants: 0,
      ancestors: 0,
    });
    expect(r.graph.elements).toEqual([]);
  });

  test("a range query overlapping the node's window matches it once", () => {
    const ranged = node("a", "a", 11, { endLine: 23 });
    const g = graph([ranged], []);
    const r = pruneVisualGraph(g, {
      roots: [rawRange(20, 30)],
      descendants: 0,
      ancestors: 0,
    });
    expect(r.graph.elements.map((e) => e.id)).toEqual(["a"]);
    expect(r.perQuery[0]?.matched).toBe(1);
  });
});

function collectIds(elements: VisualElement[]): readonly string[] {
  const out: /* mutable */ string[] = [];
  walk(elements);
  return out;

  function walk(items: VisualElement[]): void {
    for (const item of items) {
      out.push(item.id);
      if (item.type === VISUAL_ELEMENT_TYPE.Subgraph) {
        walk(item.elements);
      }
    }
  }
}

function flatten(elements: VisualElement[]): VisualElement[] {
  const out: /* mutable */ VisualElement[] = [];
  for (const e of elements) {
    out.push(e);
    if (e.type === VISUAL_ELEMENT_TYPE.Subgraph) {
      out.push(...flatten(e.elements));
    }
  }
  return out;
}
