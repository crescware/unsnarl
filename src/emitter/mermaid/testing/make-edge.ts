import type { VisualEdge } from "../../../visual-graph/visual-edge.js";

export function baseEdge(): VisualEdge {
  return { from: "a", to: "b", label: "read" };
}
