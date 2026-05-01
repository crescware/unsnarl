import type { WriteOp } from "../write-op.js";

export function baseWriteOp(): WriteOp {
  return {
    refId: "r",
    varId: "v",
    varName: "x",
    line: 1,
    offset: 0,
    scopeId: "s",
  };
}
