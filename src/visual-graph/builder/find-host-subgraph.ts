import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import type { BuildState } from "./build-state.js";

export function findHostSubgraph(
  ref: SerializedReference,
  enclosingFnVarId: string,
  scopeMap: ReadonlyMap<string, SerializedScope>,
  state: BuildState,
): VisualSubgraph | null {
  let cur: SerializedScope | null = scopeMap.get(ref.from) ?? null;
  while (cur) {
    const sg = state.subgraphByScope.get(cur.id);
    if (sg) {
      return sg;
    }
    if (!cur.upper) {
      break;
    }
    cur = scopeMap.get(cur.upper) ?? null;
  }
  return state.functionSubgraphByFn.get(enclosingFnVarId) ?? null;
}
