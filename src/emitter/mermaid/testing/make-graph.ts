import { DIRECTION } from "../../../direction.js";
import { LANGUAGE } from "../../../language.js";
import { SERIALIZED_IR_VERSION } from "../../../serialized-ir-version.js";
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
