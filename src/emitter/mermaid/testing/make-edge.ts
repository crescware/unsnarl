import type { VisualEdge } from "../../../visual-graph/model.js";

export function makeEdge(overrides: Partial<VisualEdge> = {}): VisualEdge {
  return { from: "a", to: "b", label: "read", ...overrides };
}
