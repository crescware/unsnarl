import type { VisualGraph } from "../../visual-graph/visual-graph.js";

export function renderPruningComment(
  graph: VisualGraph,
  lines: /* mutable */ string[],
): void {
  const pruning = graph.pruning;
  if (pruning === null) {
    return;
  }
  // Avoid `[ ]` in the comment payload because some Mermaid versions
  // misread a comment line that contains shape-like brackets.
  const summary = pruning.roots.map((r) => `${r.query}=${r.matched}`).join(" ");
  lines.push(
    `  %% pruning roots ${summary} ancestors=${pruning.ancestors} descendants=${pruning.descendants}`,
  );
  for (const r of pruning.roots) {
    if (r.matched === 0) {
      lines.push(`  %% pruning warning query ${r.query} matched 0 roots`);
    }
  }
}
