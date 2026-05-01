import type { VisualNode } from "../../../visual-graph/model.js";
import type { RenderState } from "../render-state.js";
import { baseStrategy } from "./make-strategy.js";

export function baseRenderState(): RenderState {
  return {
    lines: [],
    nodeMap: new Map<string, VisualNode>(),
    wrappedOwnerIds: new Set<string>(),
    edgeEndpointIds: new Set<string>(),
    placeholderIds: [],
    wrapperIds: [],
    strategy: baseStrategy(),
  };
}
