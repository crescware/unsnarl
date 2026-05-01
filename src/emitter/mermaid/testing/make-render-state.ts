import type { VisualNode } from "../../../visual-graph/model.js";
import type { RenderState } from "../render-state.js";
import { makeStrategy } from "./make-strategy.js";

export function makeRenderState(
  overrides: Partial<RenderState> = {},
): RenderState {
  return {
    lines: [],
    nodeMap: new Map<string, VisualNode>(),
    wrappedOwnerIds: new Set<string>(),
    edgeEndpointIds: new Set<string>(),
    placeholderIds: [],
    wrapperIds: [],
    strategy: makeStrategy(),
    ...overrides,
  };
}
