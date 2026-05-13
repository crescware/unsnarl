import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualNode } from "../visual-node.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import type { BuildState } from "./build-state.js";
import { sanitize } from "./sanitize.js";

// Return (creating on first call) the single BeyondDepth stub for a
// given visible ancestor subgraph. Multiple anonymous collapsed children
// of the same surviving outer container funnel into the same placeholder,
// so the rendered graph shows one `((...))` boundary marker per visible
// parent instead of one per hidden child.
export function ensureBeyondDepthStub(
  parentSubgraph: VisualSubgraph,
  state: BuildState,
): string {
  const existing = state.beyondDepthStubByParent?.get(parentSubgraph.id);
  if (existing !== undefined) {
    return existing;
  }
  const stubId = `beyond_depth_${sanitize(parentSubgraph.id)}`;
  const node = {
    type: VISUAL_ELEMENT_TYPE.Node,
    id: stubId,
    kind: NODE_KIND.LegacyBeyondDepth,
    name: "...",
    line: parentSubgraph.line,
    endLine: parentSubgraph.endLine,
    isJsxElement: false,
    unused: false,
  } satisfies VisualNode;
  parentSubgraph.elements.push(node);
  state.beyondDepthStubByParent?.set(parentSubgraph.id, stubId);
  return stubId;
}
