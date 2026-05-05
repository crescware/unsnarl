import { PREDICATE_CONTAINER_TYPE } from "../../analyzer/predicate-container-type.js";
import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { BuildState } from "./build-state.js";

export function predicateTargetId(
  r: SerializedReference,
  _scopeMap: ReadonlyMap<string, SerializedScope>,
  state: BuildState,
): string | null {
  const pc = r.predicateContainer;
  if (!pc) {
    return null;
  }
  if (pc.type === PREDICATE_CONTAINER_TYPE.SwitchStatement) {
    return state.switchDiscriminantAnchorByOffset.get(pc.offset) ?? null;
  }
  if (pc.type === PREDICATE_CONTAINER_TYPE.WhileStatement) {
    return state.whileTestAnchorByOffset.get(pc.offset) ?? null;
  }
  if (pc.type === PREDICATE_CONTAINER_TYPE.DoWhileStatement) {
    return state.doWhileTestAnchorByOffset.get(pc.offset) ?? null;
  }
  if (
    pc.type === PREDICATE_CONTAINER_TYPE.ForStatement ||
    pc.type === PREDICATE_CONTAINER_TYPE.ForOfStatement ||
    pc.type === PREDICATE_CONTAINER_TYPE.ForInStatement
  ) {
    return state.forTestAnchorByOffset.get(pc.offset) ?? null;
  }
  return state.ifTestAnchorByOffset.get(pc.offset) ?? null;
}
