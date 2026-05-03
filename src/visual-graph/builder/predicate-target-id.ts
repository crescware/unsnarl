import { PREDICATE_CONTAINER_TYPE } from "../../analyzer/predicate-container-type.js";
import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedReference, SerializedScope } from "../../ir/model.js";
import type { BuildState } from "./build-state.js";
import { sanitize } from "./sanitize.js";

export function predicateTargetId(
  r: SerializedReference,
  scopeMap: ReadonlyMap<string, SerializedScope>,
  state: BuildState,
): string | null {
  const pc = r.predicateContainer;
  if (!pc) {
    return null;
  }
  if (pc.type === PREDICATE_CONTAINER_TYPE.SwitchStatement) {
    let cur = scopeMap.get(r.from);
    while (cur) {
      if (
        cur.type === SCOPE_TYPE.Switch &&
        cur.block.span.offset === pc.offset
      ) {
        return `s_${sanitize(cur.id)}`;
      }
      if (!cur.upper) {
        break;
      }
      cur = scopeMap.get(cur.upper);
    }
    return null;
  }
  return state.ifTestAnchorByOffset.get(pc.offset) ?? null;
}
