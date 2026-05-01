import { ReferenceFlags } from "../../ir/model.js";
import type { AstNode } from "../../ir/model.js";
import type { ClassifyResult } from "./classify-result.js";
import { isAstExpression } from "./is-ast-expression.js";
import { reference } from "./reference.js";

export function classifyOrdinaryReference(
  t: string,
  key: string | null,
  parent: AstNode,
): ClassifyResult {
  if (t === "AssignmentExpression" && key === "left") {
    const op = (parent as { operator?: string }).operator ?? "=";
    const flags =
      op === "="
        ? ReferenceFlags.Write
        : ReferenceFlags.Read | ReferenceFlags.Write;
    const right = parent["right"];
    return reference(flags, false, isAstExpression(right) ? right : null);
  }
  if (t === "UpdateExpression" && key === "argument") {
    return reference(ReferenceFlags.Read | ReferenceFlags.Write, false, null);
  }
  if (t === "CallExpression" && key === "callee") {
    return reference(ReferenceFlags.Read | ReferenceFlags.Call, false, null);
  }
  if (t === "NewExpression" && key === "callee") {
    return reference(ReferenceFlags.Read | ReferenceFlags.Call, false, null);
  }
  if (t === "MemberExpression" && key === "object") {
    return reference(
      ReferenceFlags.Read | ReferenceFlags.Receiver,
      false,
      null,
    );
  }
  let init = false;
  if (t === "VariableDeclarator" && key === "init") {
    init = true;
  }
  return reference(ReferenceFlags.Read, init, null);
}
