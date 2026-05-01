import type { WriteOp } from "./write-op.js";

export function stateAt(
  varId: string,
  offset: number,
  writeOpsByVariable: ReadonlyMap<string, WriteOp[]>,
): string {
  const ops = writeOpsByVariable.get(varId) ?? [];
  let last: WriteOp | null = null;
  for (const op of ops) {
    if (op.offset >= offset) {
      break;
    }
    last = op;
  }
  return last ? last.refId : varId;
}
