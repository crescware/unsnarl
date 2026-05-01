import { SCOPE_TYPE } from "../../constants.js";
import type { SerializedReference, SerializedScope } from "../../ir/model.js";
import { branchContainerKey } from "./branch-container-key.js";
import { ifContainerSubgraphId } from "./if-container-subgraph-id.js";
import { sanitize } from "./sanitize.js";

export function predicateTargetId(
  r: SerializedReference,
  scopes: readonly SerializedScope[],
  scopeMap: ReadonlyMap<string, SerializedScope>,
): string | null {
  const pc = r.predicateContainer;
  if (!pc) {
    return null;
  }
  if (pc.type === "SwitchStatement") {
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
  const containerKey = `if:${r.from}:${pc.offset}`;
  const branches = scopes.filter((s) => branchContainerKey(s) === containerKey);
  if (branches.length >= 2) {
    return ifContainerSubgraphId(r.from, pc.offset);
  }
  const single = branches[0];
  if (single) {
    return `s_${sanitize(single.id)}`;
  }
  return null;
}
