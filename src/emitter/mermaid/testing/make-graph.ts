import {
  DIRECTION,
  LANGUAGE,
  SERIALIZED_IR_VERSION,
} from "../../../constants.js";
import type { VisualGraph } from "../../../visual-graph/model.js";

export function makeGraph(overrides: Partial<VisualGraph> = {}): VisualGraph {
  return {
    version: SERIALIZED_IR_VERSION,
    source: { path: "input.ts", language: LANGUAGE.Ts },
    direction: DIRECTION.RL,
    elements: [],
    edges: [],
    ...overrides,
  };
}
