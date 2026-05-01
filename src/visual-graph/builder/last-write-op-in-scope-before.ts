import type { SerializedScope } from "../../ir/model.js";
import { isAncestorScope } from "./is-ancestor-scope.js";
import type { WriteOp } from "./write-op.js";

export function lastWriteOpInScopeBefore(
  varId: string,
  scopeId: string,
  offset: number,
  writeOpsByVariable: ReadonlyMap<string, WriteOp[]>,
  scopeMap: ReadonlyMap<string, SerializedScope>,
): WriteOp | null {
  const ops = writeOpsByVariable.get(varId) ?? [];
  let last: WriteOp | null = null;
  for (const op of ops) {
    if (op.offset >= offset) {
      break;
    }
    if (
      op.scopeId === scopeId ||
      isAncestorScope(scopeId, op.scopeId, scopeMap)
    ) {
      last = op;
    }
  }
  return last;
}
