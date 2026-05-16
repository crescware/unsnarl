import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { ensureReturnUseNode } from "./ensure-return-use-node.js";
import { MODULE_ROOT_ID } from "./module-root-id.js";

type ReadTarget = Readonly<{
  targetId: string;
  needsModuleRoot: boolean;
}>;

export function resolveReadTargetId(
  exprStmtId: string | null,
  enclosingFn: string | null,
  ref: SerializedReference,
  ctx: BuilderContext,
  state: BuildState,
): ReadTarget {
  let targetId: string | null = exprStmtId;
  if (targetId === null && enclosingFn !== null) {
    targetId = ensureReturnUseNode(enclosingFn, ref, ctx, state);
  }
  if (targetId === null) {
    return { targetId: MODULE_ROOT_ID, needsModuleRoot: true };
  }
  return { targetId, needsModuleRoot: false };
}
