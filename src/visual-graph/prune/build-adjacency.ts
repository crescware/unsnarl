import type { VisualEdge } from "../visual-edge.js";
import { pushTo } from "./push-to.js";

export function buildAdjacency(edges: readonly VisualEdge[]): {
  outEdges: Map<string, /* mutable */ string[]>;
  inEdges: Map<string, /* mutable */ string[]>;
} {
  const outEdges = new Map<string, /* mutable */ string[]>();
  const inEdges = new Map<string, /* mutable */ string[]>();
  for (const e of edges) {
    pushTo(outEdges, e.from, e.to);
    pushTo(inEdges, e.to, e.from);
  }
  return { outEdges, inEdges };
}
