import { LANGUAGE } from "../../../cli/language.js";
import { SERIALIZED_IR_VERSION } from "../../../serializer/serialized-ir-version.js";
import { DIRECTION } from "../../../visual-graph/direction.js";
import type { VisualGraph } from "../../../visual-graph/model.js";

export function baseGraph(): VisualGraph {
  return {
    version: SERIALIZED_IR_VERSION,
    source: { path: "input.ts", language: LANGUAGE.Ts },
    direction: DIRECTION.RL,
    elements: [],
    edges: [],
    boundaryEdges: [],
    pruning: null,
  };
}
