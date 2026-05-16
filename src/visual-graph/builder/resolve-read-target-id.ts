import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { ensureReturnUseNode } from "./ensure-return-use-node.js";
import { MODULE_ROOT_ID } from "./module-root-id.js";

export function resolveReadTargetId(
  exprStmtId: string | null,
  enclosingFn: string | null,
  ref: SerializedReference,
  ctx: BuilderContext,
  state: BuildState,
): string {
  let targetId: string | null = exprStmtId;
  if (targetId === null && enclosingFn !== null) {
    targetId = ensureReturnUseNode(enclosingFn, ref, ctx, state);
  }
  return targetId ?? MODULE_ROOT_ID;
}
