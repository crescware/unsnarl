import type { VisualGraph } from "../../visual-graph/model.js";

export function renderPruningComment(
  graph: VisualGraph,
  lines: /* mutable */ string[],
): void {
  if (graph.pruning === undefined) {
    return;
  }
  // Avoid `[ ]` in the comment payload because some Mermaid versions
  // misread a comment line that contains shape-like brackets.
  const summary = graph.pruning.roots
    .map((r) => `${r.query}=${r.matched}`)
    .join(" ");
  lines.push(
    `  %% pruning roots ${summary} ancestors=${graph.pruning.ancestors} descendants=${graph.pruning.descendants}`,
  );
  for (const r of graph.pruning.roots) {
    if (r.matched === 0) {
      lines.push(`  %% pruning warning query ${r.query} matched 0 roots`);
    }
  }
}
