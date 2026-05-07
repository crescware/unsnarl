import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";

// Walk up scope.upper until we hit a scope that materialised a subgraph
// during the top-down build and return it. Collapsed ancestors are
// skipped (they have no subgraph). Returns null when the chain reaches
// the module / global root without ever crossing a surviving subgraph --
// callers treat null as "no visible host", which turns the corresponding
// edge into a drop signal.
export function visibleAncestorSubgraph(
  scope: SerializedScope,
  ctx: BuilderContext,
  state: BuildState,
): VisualSubgraph | null {
  let parentId = scope.upper;
  while (parentId !== null) {
    const sg = state.subgraphByScope.get(parentId);
    if (sg) {
      return sg;
    }
    const parent = ctx.scopeMap.get(parentId);
    if (!parent) {
      return null;
    }
    parentId = parent.upper;
  }
  return null;
}
