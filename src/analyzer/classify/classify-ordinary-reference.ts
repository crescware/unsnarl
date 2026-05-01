import { ReferenceFlags } from "../../ir/model.js";
import type { AstNode } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { ClassifyResult } from "./classify-result.js";
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
    return reference(flags, false);
  }
  if (t === AST_TYPE.UpdateExpression && key === "argument") {
    return reference(ReferenceFlags.Read | ReferenceFlags.Write, false);
  }
  if (t === AST_TYPE.CallExpression && key === "callee") {
    return reference(ReferenceFlags.Read | ReferenceFlags.Call, false);
  }
  if (t === AST_TYPE.NewExpression && key === "callee") {
    return reference(ReferenceFlags.Read | ReferenceFlags.Call, false);
  }
  if (t === AST_TYPE.MemberExpression && key === "object") {
    return reference(ReferenceFlags.Read | ReferenceFlags.Receiver, false);
  }
  let init = false;
  if (t === AST_TYPE.VariableDeclarator && key === "init") {
    init = true;
  }
  return reference(ReferenceFlags.Read, init);
}
