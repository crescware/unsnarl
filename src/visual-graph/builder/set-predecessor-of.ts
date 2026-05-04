import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { isAncestorScope } from "./is-ancestor-scope.js";
import { nodeId } from "./node-id.js";
import { writeOpNodeId } from "./write-op-node-id.js";
import type { WriteOp } from "./write-op.js";

export function setPredecessorOf(
  op: WriteOp,
  ops: readonly WriteOp[],
  scopeMap: ReadonlyMap<string, SerializedScope>,
): string {
  const i = ops.findIndex((candidate) => candidate.refId === op.refId);
  if (i < 0) {
    return nodeId(op.varId);
  }
  for (let j = i - 1; j >= 0; j--) {
    const candidate = ops[j];
    if (!candidate) {
      continue;
    }
    if (isAncestorScope(candidate.scopeId, op.scopeId, scopeMap)) {
      return writeOpNodeId(candidate.refId);
    }
  }
  return nodeId(op.varId);
}
