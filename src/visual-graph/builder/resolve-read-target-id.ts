import {
  normal$,
  return$,
  throw$,
} from "../../ir/completion/completion-type.js";
import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { ensureReturnUseNode } from "./ensure-return-use-node.js";
import { ensureThrowUseNode } from "./ensure-throw-use-node.js";
import { MODULE_ROOT_ID } from "./module-root-id.js";

export function resolveReadTargetId(
  exprStmtId: string | null,
  enclosingFnVarId: string | null,
  ref: SerializedReference,
  ctx: BuilderContext,
  state: BuildState,
): string {
  if (exprStmtId !== null) {
    return exprStmtId;
  }
  if (enclosingFnVarId === null) {
    return MODULE_ROOT_ID;
  }
  switch (ref.completion.type) {
    case return$.literal:
      return (
        ensureReturnUseNode(enclosingFnVarId, ref, ctx, state) ?? MODULE_ROOT_ID
      );
    case throw$.literal:
      return (
        ensureThrowUseNode(enclosingFnVarId, ref, ctx, state) ?? MODULE_ROOT_ID
      );
    case normal$.literal:
      return MODULE_ROOT_ID;
  }
}
