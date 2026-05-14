import type { ParsedRootQuery } from "../../root-query/parsed-root-query.js";
import { iterateVisualNodes } from "../prune/iterate-visual-nodes.js";
import type { VisualGraph } from "../visual-graph.js";
import { nodeMatchesHighlightQuery } from "./node-matches-highlight-query.js";

// Returns the ids of every visible node that satisfies at least one of
// the supplied queries. The query grammar matches `-r/--roots`, but
// the matching predicate is the highlight-specific
// `nodeMatchesHighlightQuery`, which deliberately includes use-site
// nodes (WriteReference, ReturnArgumentReference) on bare name queries so `-H counter`
// paints every place `counter` appears. The caller is responsible for
// already having resolved any LineOrName ambiguity (commonly via
// `resolveAmbiguousQueries`).
export function collectHighlightIds(
  graph: VisualGraph,
  queries: readonly ParsedRootQuery[],
): ReadonlySet<string> {
  const ids = new Set<string>();
  if (queries.length === 0) {
    return ids;
  }
  for (const node of iterateVisualNodes(graph.elements)) {
    for (const q of queries) {
      if (nodeMatchesHighlightQuery(node, q)) {
        ids.add(node.id);
        break;
      }
    }
  }
  return ids;
}
