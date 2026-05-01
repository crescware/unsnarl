import type { SerializedIR } from "../../ir/model.js";
import type { EmitOptions, Emitter } from "../../pipeline/types.js";
import { buildVisualGraph } from "../../visual-graph/builder.js";
import type { VisualNode } from "../../visual-graph/model.js";
import { BOUNDARY_EDGE_DIRECTION } from "../../visual-graph/prune/boundary-edge-direction.js";
import { collectNodes } from "./collect-nodes.js";
import { formatLabel } from "./format-label.js";

/**
 * Emits a wc-like, tab-separated table of per-node edge counts so the
 * output can be fed straight into shell tooling (sort, awk, ...). One
 * line per node, then a `<N> total` summary.
 *
 * Columns:
 *   1. descendants edge count (this node is the `from` side)
 *   2. ancestors  edge count (this node is the `to`   side)
 *   3. "<path>:<line> <label>" -- the leading `path:line` lets editors
 *      pick the row up as a clickable jump-to-source target.
 *
 * When pruning is in effect and a node touches a boundary edge, the
 * count in that direction is unknowable from inside the kept graph
 * (more graph exists past the cut) and is rendered as "?". A "?" in
 * any row also propagates into the corresponding column of the total
 * line, since the sum is no longer fully determined.
 */
export class StatsEmitter implements Emitter {
  readonly format = "stats";
  readonly contentType = "text/tab-separated-values";
  readonly extension = "tsv";

  emit(ir: SerializedIR, opts: EmitOptions): string {
    const graph = opts.prunedGraph ?? buildVisualGraph(ir);
    // Source-order sort so the rows read top-to-bottom like the file
    // itself: editors that pick up `path:line` jump targets land on
    // the right place, and same-line ties keep their original order
    // via Array.prototype.sort being stable.
    // Copy before sort: collectNodes returns a readonly view, so we
    // build a /* mutable */ slice locally for in-place sort.
    const nodes: /* mutable */ VisualNode[] = [...collectNodes(graph.elements)];
    nodes.sort((a, b) => a.line - b.line);

    const outCounts = new Map<string, number>();
    const inCounts = new Map<string, number>();
    for (const e of graph.edges) {
      outCounts.set(e.from, (outCounts.get(e.from) ?? 0) + 1);
      inCounts.set(e.to, (inCounts.get(e.to) ?? 0) + 1);
    }

    const boundaryOut = new Set<string>();
    const boundaryIn = new Set<string>();
    for (const be of graph.boundaryEdges ?? []) {
      if (be.direction === BOUNDARY_EDGE_DIRECTION.Out) {
        boundaryOut.add(be.inside);
      } else {
        boundaryIn.add(be.inside);
      }
    }

    const lines: /* mutable */ string[] = [];
    const path = graph.source.path;
    let sumDesc = 0;
    let sumAnc = 0;
    let descUnknown = false;
    let ancUnknown = false;

    for (const n of nodes) {
      const descNum = outCounts.get(n.id) ?? 0;
      const ancNum = inCounts.get(n.id) ?? 0;
      const descCell = boundaryOut.has(n.id) ? "?" : String(descNum);
      const ancCell = boundaryIn.has(n.id) ? "?" : String(ancNum);

      if (boundaryOut.has(n.id)) {
        descUnknown = true;
      } else {
        sumDesc += descNum;
      }
      if (boundaryIn.has(n.id)) {
        ancUnknown = true;
      } else {
        sumAnc += ancNum;
      }

      lines.push(`${descCell}\t${ancCell}\t${formatLabel(path, n)}`);
    }

    const totalDesc = descUnknown ? "?" : String(sumDesc);
    const totalAnc = ancUnknown ? "?" : String(sumAnc);
    lines.push(`${totalDesc}\t${totalAnc}\t${nodes.length} total`);

    return `${lines.join("\n")}\n`;
  }
}
