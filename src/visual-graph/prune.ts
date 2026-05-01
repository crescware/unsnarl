import type { ParsedRootQuery } from "../cli/root-query/parsed-root-query.js";
import type {
  NodeKind,
  VisualBoundaryEdge,
  VisualEdge,
  VisualElement,
  VisualGraph,
  VisualNode,
  VisualSubgraph,
} from "./model.js";

// Every visible node that carries a meaningful source line is eligible as a
// root, including "use" nodes (ReturnUse for JSX/ownerless reads inside a
// return, WriteOp for assignments). `-r N` should pin the root at whatever
// is actually at line N; surrounding declarations are reached via the
// ancestors BFS, not auto-attached as a separate root.
const ROOT_CANDIDATE_KINDS: ReadonlySet<NodeKind> = new Set<NodeKind>([
  "Variable",
  "FunctionName",
  "ClassName",
  "Parameter",
  "CatchClause",
  "ImportBinding",
  "ImplicitGlobalVariable",
  "ReturnUse",
  "WriteOp",
]);

export interface PruneOptions {
  readonly roots: readonly ParsedRootQuery[];
  readonly descendants: number;
  readonly ancestors: number;
}

export interface PruneResult {
  readonly graph: VisualGraph;
  readonly perQuery: ReadonlyArray<{
    readonly query: ParsedRootQuery;
    readonly matched: number;
  }>;
}

export function pruneVisualGraph(
  graph: VisualGraph,
  options: PruneOptions,
): PruneResult {
  if (options.roots.length === 0) {
    return { graph, perQuery: [] };
  }

  const perQuery = options.roots.map((q) => ({
    query: q,
    matched: 0,
  }));
  const rootIds = new Set<string>();
  for (const node of iterateVisualNodes(graph.elements)) {
    for (let i = 0; i < options.roots.length; i++) {
      const q = options.roots[i];
      if (q !== undefined && nodeMatchesQuery(node, q)) {
        rootIds.add(node.id);
        const entry = perQuery[i];
        if (entry !== undefined) {
          perQuery[i] = { query: entry.query, matched: entry.matched + 1 };
        }
      }
    }
  }

  // A bare line query whose number is the start line of a subgraph (e.g.
  // `-r 10` pointing at the `return (` line) sweeps every node in that
  // subgraph into the root set. Range queries deliberately stay narrow:
  // selecting "lines 10-12" should not implicitly drag the whole return
  // body in just because the return subgraph happens to start at L10.
  for (let i = 0; i < options.roots.length; i++) {
    const q = options.roots[i];
    if (q === undefined || q.kind !== "line") {
      continue;
    }
    for (const sg of iterateVisualSubgraphs(graph.elements)) {
      if (sg.line !== q.line) {
        continue;
      }
      let added = 0;
      for (const id of collectNodeIds(sg.elements)) {
        if (rootIds.has(id)) {
          continue;
        }
        rootIds.add(id);
        added += 1;
      }
      if (added > 0) {
        const entry = perQuery[i];
        if (entry !== undefined) {
          perQuery[i] = {
            query: entry.query,
            matched: entry.matched + added,
          };
        }
      }
    }
  }

  const { outEdges, inEdges } = buildAdjacency(graph.edges);
  // BFS only as far as the user asked. The outermost edges that point
  // beyond this radius are surfaced separately as boundaryEdges so
  // renderers can hint at "more context exists in this direction" without
  // pulling the next generation of nodes into the diagram.
  const innerD = bfs(rootIds, outEdges, options.descendants);
  const innerA = bfs(rootIds, inEdges, options.ancestors);
  const reachable = new Set<string>([...rootIds, ...innerD, ...innerA]);

  // Boundary edges are "more graph beyond here" hints. They are not
  // about counting individual outgoing edges -- one inside node with
  // 100 cut neighbors should still produce a single hint, not 100. So
  // collapse on (inside, direction); for "in" we additionally union the
  // labels (split by comma so "read,call" + "read" yields {call, read}).
  type Bucket =
    | { kind: "out"; inside: string }
    | { kind: "in"; inside: string; labels: Set<string> };
  const buckets = new Map<string, Bucket>();
  const key = (inside: string, dir: "out" | "in") => `${dir}|${inside}`;

  if (options.descendants > 0) {
    for (const e of graph.edges) {
      if (!reachable.has(e.from) || reachable.has(e.to)) {
        continue;
      }
      const k = key(e.from, "out");
      if (!buckets.has(k)) {
        buckets.set(k, { kind: "out", inside: e.from });
      }
    }
  }
  if (options.ancestors > 0) {
    for (const e of graph.edges) {
      if (!reachable.has(e.to) || reachable.has(e.from)) {
        continue;
      }
      const k = key(e.to, "in");
      let bucket = buckets.get(k);
      if (bucket === undefined) {
        bucket = { kind: "in", inside: e.to, labels: new Set<string>() };
        buckets.set(k, bucket);
      }
      if (bucket.kind === "in") {
        for (const part of e.label.split(",")) {
          bucket.labels.add(part);
        }
      }
    }
  }

  const boundaryEdges: VisualBoundaryEdge[] = [];
  for (const bucket of buckets.values()) {
    if (bucket.kind === "out") {
      boundaryEdges.push({ inside: bucket.inside, direction: "out" });
    } else {
      const label = [...bucket.labels].sort().join(",");
      boundaryEdges.push({ inside: bucket.inside, direction: "in", label });
    }
  }

  const parentOf = buildParentMap(graph.elements);
  const initial = Array.from(reachable);
  for (const id of initial) {
    let cur = parentOf.get(id);
    while (cur !== undefined && !reachable.has(cur)) {
      reachable.add(cur);
      cur = parentOf.get(cur);
    }
  }

  const newElements = rebuildElements(graph.elements, reachable);
  const survivors = collectIds(newElements);
  const newEdges = graph.edges.filter(
    (e) => survivors.has(e.from) && survivors.has(e.to),
  );
  const survivingBoundary = boundaryEdges.filter((b) =>
    survivors.has(b.inside),
  );

  const pruned: VisualGraph = {
    version: graph.version,
    source: graph.source,
    direction: graph.direction,
    elements: newElements,
    edges: newEdges,
    boundaryEdges: survivingBoundary,
    pruning: {
      roots: perQuery.map(({ query, matched }) => ({
        query: query.raw,
        matched,
      })),
      descendants: options.descendants,
      ancestors: options.ancestors,
    },
  };

  return { graph: pruned, perQuery };
}

function* iterateVisualNodes(
  elements: readonly VisualElement[],
): Generator<VisualNode> {
  for (const e of elements) {
    if (e.type === "node") {
      if (ROOT_CANDIDATE_KINDS.has(e.kind)) {
        yield e;
      }
    } else {
      yield* iterateVisualNodes(e.elements);
    }
  }
}

function* iterateVisualSubgraphs(
  elements: readonly VisualElement[],
): Generator<VisualSubgraph> {
  for (const e of elements) {
    if (e.type === "subgraph") {
      yield e;
      yield* iterateVisualSubgraphs(e.elements);
    }
  }
}

function collectNodeIds(elements: readonly VisualElement[]): string[] {
  const out: string[] = [];
  walk(elements);
  return out;

  function walk(items: readonly VisualElement[]): void {
    for (const item of items) {
      if (item.type === "node") {
        out.push(item.id);
      } else {
        walk(item.elements);
      }
    }
  }
}

// Use-site nodes (WriteOp, ReturnUse) are positional: they make sense as
// roots when the user pinpoints a line, but a bare `-r counter` should
// stay declaration-scoped so it does not light up every assignment and
// every JSX usage of `counter`.
const NAME_QUERY_EXCLUDED: ReadonlySet<NodeKind> = new Set<NodeKind>([
  "WriteOp",
  "ReturnUse",
]);

function nodeMatchesQuery(node: VisualNode, q: ParsedRootQuery): boolean {
  const startLine = node.line;
  const endLine = node.endLine ?? node.line;
  switch (q.kind) {
    case "line":
      return q.line >= startLine && q.line <= endLine;
    case "line-name":
      return q.line >= startLine && q.line <= endLine && node.name === q.name;
    case "range":
      return startLine <= q.end && endLine >= q.start;
    case "range-name":
      return startLine <= q.end && endLine >= q.start && node.name === q.name;
    case "name":
      return !NAME_QUERY_EXCLUDED.has(node.kind) && node.name === q.name;
  }
}

function buildAdjacency(edges: readonly VisualEdge[]): {
  outEdges: Map<string, string[]>;
  inEdges: Map<string, string[]>;
} {
  const outEdges = new Map<string, string[]>();
  const inEdges = new Map<string, string[]>();
  for (const e of edges) {
    pushTo(outEdges, e.from, e.to);
    pushTo(inEdges, e.to, e.from);
  }
  return { outEdges, inEdges };
}

function pushTo(map: Map<string, string[]>, key: string, value: string): void {
  const arr = map.get(key);
  if (arr === undefined) {
    map.set(key, [value]);
  } else {
    arr.push(value);
  }
}

function bfs(
  starts: ReadonlySet<string>,
  adj: ReadonlyMap<string, readonly string[]>,
  maxDepth: number,
): Set<string> {
  const reached = new Set<string>(starts);
  if (maxDepth <= 0) {
    return reached;
  }
  let frontier = new Set<string>(starts);
  for (let depth = 0; depth < maxDepth && frontier.size > 0; depth++) {
    const next = new Set<string>();
    for (const id of frontier) {
      const neighbors = adj.get(id);
      if (neighbors === undefined) {
        continue;
      }
      for (const n of neighbors) {
        if (!reached.has(n)) {
          reached.add(n);
          next.add(n);
        }
      }
    }
    frontier = next;
  }
  return reached;
}

function buildParentMap(
  elements: readonly VisualElement[],
): Map<string, string> {
  const parent = new Map<string, string>();
  walk(elements, undefined);
  return parent;

  function walk(
    items: readonly VisualElement[],
    parentId: string | undefined,
  ): void {
    for (const item of items) {
      if (parentId !== undefined) {
        parent.set(item.id, parentId);
      }
      if (item.type === "subgraph") {
        walk(item.elements, item.id);
      }
    }
  }
}

function rebuildElements(
  elements: readonly VisualElement[],
  keep: ReadonlySet<string>,
): VisualElement[] {
  const result: VisualElement[] = [];
  for (const item of elements) {
    if (item.type === "node") {
      if (keep.has(item.id)) {
        result.push({ ...item });
      }
    } else {
      const children = rebuildElements(item.elements, keep);
      // Subgraphs only survive when at least one descendant survived.
      // Keeping an empty subgraph -- even if it appeared as an edge
      // endpoint during BFS -- crashes Mermaid's elk layout because the
      // cluster has no labels[0] for the renderer to size against. The
      // edges that pointed at this subgraph are filtered out below by
      // the `survivors` check, so dropping the cluster is consistent.
      if (children.length > 0) {
        const cloned: VisualSubgraph = { ...item, elements: children };
        result.push(cloned);
      }
    }
  }
  return result;
}

function collectIds(elements: readonly VisualElement[]): Set<string> {
  const ids = new Set<string>();
  walk(elements);
  return ids;

  function walk(items: readonly VisualElement[]): void {
    for (const item of items) {
      ids.add(item.id);
      if (item.type === "subgraph") {
        walk(item.elements);
      }
    }
  }
}
