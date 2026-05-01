import { AST_TYPE } from "../../constants.js";
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
  if (t === AST_TYPE.AssignmentExpression && key === "left") {
    const op = (parent as { operator?: string }).operator ?? "=";
    const flags =
      op === "="
        ? ReferenceFlags.Write
        : ReferenceFlags.Read | ReferenceFlags.Write;
    const right = parent["right"];
    return reference(flags, false, isAstExpression(right) ? right : null);
  }
  if (t === AST_TYPE.UpdateExpression && key === "argument") {
    return reference(ReferenceFlags.Read | ReferenceFlags.Write, false, null);
  }
  if (t === AST_TYPE.CallExpression && key === "callee") {
    return reference(ReferenceFlags.Read | ReferenceFlags.Call, false, null);
  }
  if (t === AST_TYPE.NewExpression && key === "callee") {
    return reference(ReferenceFlags.Read | ReferenceFlags.Call, false, null);
  }
  if (t === AST_TYPE.MemberExpression && key === "object") {
    return reference(
      ReferenceFlags.Read | ReferenceFlags.Receiver,
      false,
      null,
    );
  }
  let init = false;
  if (t === AST_TYPE.VariableDeclarator && key === "init") {
    init = true;
  }
  return reference(ReferenceFlags.Read, init, null);
}
