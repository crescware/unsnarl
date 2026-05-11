import type { VisualNode } from "../../../visual-graph/visual-node.js";
import type { RenderState } from "../render-state.js";
import { darkTheme } from "../theme/dark-theme.js";
import { baseStrategy } from "./make-strategy.js";

export function baseRenderState(): RenderState {
  return {
    lines: [],
    nodeMap: new Map<string, VisualNode>(),
    wrappedOwnerIds: new Set<string>(),
    edgeEndpointIds: new Set<string>(),
    placeholderIds: [],
    nestClassMap: new Map<number, string[]>(),
    strategy: baseStrategy(),
    theme: darkTheme,
    debug: false,
  };
}
