import { nodeId } from "./node-id.js";
import { writeOpNodeId } from "./write-op-node-id.js";
import type { WriteOp } from "./write-op.js";

export function ownerTargetId(
  ownerVarId: string,
  offset: number,
  writeOpsByVariable: ReadonlyMap<string, WriteOp[]>,
): string {
  const ops = writeOpsByVariable.get(ownerVarId) ?? [];
  let last: WriteOp | null = null;
  for (const op of ops) {
    if (op.offset > offset) {
      break;
    }
    last = op;
  }
  return last ? writeOpNodeId(last.refId) : nodeId(ownerVarId);
}
