import type { VisualGraphPruning } from "../../visual-graph/visual-graph-pruning.js";

export function formatPruningQuery(pruning: VisualGraphPruning): string {
  const roots = pruning.roots.map((v) => v.query).join(",");
  if (pruning.descendants === pruning.ancestors) {
    return `-r ${roots} -C ${pruning.descendants}`;
  }
  return `-r ${roots} -A ${pruning.descendants} -B ${pruning.ancestors}`;
}
