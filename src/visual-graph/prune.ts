import type { ParsedRootQuery } from "../cli/root-query.js";
import type {
  NodeKind,
  VisualBoundaryEdge,
  VisualEdge,
  VisualElement,
  VisualGraph,
  VisualNode,
  VisualSubgraph,
} from "./model.js";

const ROOT_CANDIDATE_KINDS: ReadonlySet<NodeKind> = new Set<NodeKind>([
  "Variable",
  "FunctionName",
  "ClassName",
  "Parameter",
  "CatchClause",
  "ImportBinding",
  "ImplicitGlobalVariable",
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

  const { outEdges, inEdges } = buildAdjacency(graph.edges);
  // BFS only as far as the user asked. The outermost edges that point
  // beyond this radius are surfaced separately as boundaryEdges so
  // renderers can hint at "more context exists in this direction" without
  // pulling the next generation of nodes into the diagram.
  const innerD = bfs(rootIds, outEdges, options.descendants);
  const innerA = bfs(rootIds, inEdges, options.ancestors);
  const reachable = new Set<string>([...rootIds, ...innerD, ...innerA]);

  const boundaryEdges: VisualBoundaryEdge[] = [];
  if (options.descendants > 0) {
    for (const e of graph.edges) {
      if (reachable.has(e.from) && !reachable.has(e.to)) {
        // inside -> beyond: the action's actor is the unseen `to`,
        // so the label is unknowable -- intentionally drop it.
        boundaryEdges.push({ inside: e.from, direction: "out" });
      }
    }
  }
  if (options.ancestors > 0) {
    for (const e of graph.edges) {
      if (reachable.has(e.to) && !reachable.has(e.from)) {
        // beyond -> inside: the actor is the visible `to` (= inside),
        // so the original edge label still applies.
        boundaryEdges.push({
          inside: e.to,
          direction: "in",
          label: e.label,
        });
      }
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

function nodeMatchesQuery(node: VisualNode, q: ParsedRootQuery): boolean {
  switch (q.kind) {
    case "line":
      return node.line === q.line;
    case "line-name":
      return node.line === q.line && node.name === q.name;
    case "range":
      return node.line >= q.start && node.line <= q.end;
    case "range-name":
      return node.line >= q.start && node.line <= q.end && node.name === q.name;
    case "name":
      return node.name === q.name;
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
      if (children.length > 0 || keep.has(item.id)) {
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
