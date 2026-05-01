import type {
  VisualBoundaryEdge,
  VisualGraph,
} from "../model.js";
import { bfs } from "./bfs.js";
import { buildAdjacency } from "./build-adjacency.js";
import { buildParentMap } from "./build-parent-map.js";
import { collectIds } from "./collect-ids.js";
import { collectNodeIds } from "./collect-node-ids.js";
import { iterateVisualNodes } from "./iterate-visual-nodes.js";
import { iterateVisualSubgraphs } from "./iterate-visual-subgraphs.js";
import { nodeMatchesQuery } from "./node-matches-query.js";
import type { PruneOptions, PruneResult } from "./prune-options.js";
import { rebuildElements } from "./rebuild-elements.js";

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
