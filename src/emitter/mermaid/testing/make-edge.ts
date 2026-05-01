import type { VisualEdge } from "../../../visual-graph/model.js";

export function baseEdge(): VisualEdge {
  return { from: "a", to: "b", label: "read" };
}
