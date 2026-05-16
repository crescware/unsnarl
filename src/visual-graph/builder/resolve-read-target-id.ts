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
  if (ref.returnContainer !== null && ref.throwContainer !== null) {
    // Mutually exclusive per AST grammar: see issue #94. Asserted here so an
    // analyzer regression surfaces immediately instead of silently routing
    // via an implicit precedence rule.
    // @TODO https://github.com/crescware/unsnarl/issues/94
    throw new Error(
      "resolveReadTargetId: returnContainer and throwContainer are mutually exclusive",
    );
  }
  if (ref.returnContainer !== null) {
    return (
      ensureReturnUseNode(enclosingFnVarId, ref, ctx, state) ?? MODULE_ROOT_ID
    );
  }
  if (ref.throwContainer !== null) {
    return (
      ensureThrowUseNode(enclosingFnVarId, ref, ctx, state) ?? MODULE_ROOT_ID
    );
  }
  return MODULE_ROOT_ID;
}
