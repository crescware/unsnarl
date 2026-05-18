import type { VisualBoundaryEdge } from "../../visual-graph/visual-boundary-edge.js";
import type { VisualEdge } from "../../visual-graph/visual-edge.js";

// Mermaid identifies edges via `linkStyle <index>` where `index` reflects
// the order edges appear in the diagram source. mermaid.ts emits body
// edges first, then import edges, then one synthetic edge per
// `boundaryEdge`, so this function walks the three buckets in that same
// order and returns the indices of any edge whose endpoint is in
// `highlightIds`. For boundary edges only the `inside` end is a real
// node (the other end is a generated stub), so we test that side alone.
export function collectHighlightEdgeIndices(
  bodyEdges: readonly VisualEdge[],
  importEdges: readonly VisualEdge[],
  boundaryEdges: readonly VisualBoundaryEdge[],
  highlightIds: ReadonlySet<string>,
): readonly number[] {
  if (highlightIds.size === 0) {
    return [];
  }
  const out: /* mutable */ number[] = [];
  let i = 0;
  for (const e of bodyEdges) {
    if (highlightIds.has(e.from) || highlightIds.has(e.to)) {
      out.push(i);
    }
    i += 1;
  }
  for (const e of importEdges) {
    if (highlightIds.has(e.from) || highlightIds.has(e.to)) {
      out.push(i);
    }
    i += 1;
  }
  for (const be of boundaryEdges) {
    if (highlightIds.has(be.inside)) {
      out.push(i);
    }
    i += 1;
  }
  return out;
}
