import type { VisualEdge } from "../model.js";
import { pushTo } from "./push-to.js";

export function buildAdjacency(edges: readonly VisualEdge[]): {
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
