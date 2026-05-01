import { DIRECTION, LANGUAGE } from "../../../constants.js";
import type { VisualGraph } from "../../../visual-graph/model.js";

export function makeGraph(overrides: Partial<VisualGraph> = {}): VisualGraph {
  return {
    version: 1,
    source: { path: "input.ts", language: LANGUAGE.Ts },
    direction: DIRECTION.RL,
    elements: [],
    edges: [],
    ...overrides,
  };
}
