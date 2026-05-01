import type { BuilderContext } from "./context.js";
import { nodeId } from "./node-id.js";
import { stateAt } from "./state-at.js";
import { writeOpNodeId } from "./write-op-node-id.js";

export function stateRefId(
  refId: string,
  varId: string,
  ctx: BuilderContext,
): string {
  const op = ctx.writeOpByRef.get(refId);
  if (op) {
    return writeOpNodeId(op.refId);
  }
  const ref = ctx.ir.references.find((r) => r.id === refId);
  if (!ref) {
    return nodeId(varId);
  }
  const stateRef = stateAt(
    varId,
    ref.identifier.span.offset,
    ctx.writeOpsByVariable,
  );
  return stateRef === varId ? nodeId(varId) : writeOpNodeId(stateRef);
}
