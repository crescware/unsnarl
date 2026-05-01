import type { VisualGraph } from "../../../visual-graph/model.js";

export function makeGraph(overrides: Partial<VisualGraph> = {}): VisualGraph {
  return {
    version: 1,
    source: { path: "input.ts", language: "ts" },
    direction: "RL",
    elements: [],
    edges: [],
    ...overrides,
  };
}
