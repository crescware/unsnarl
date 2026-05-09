import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";

export function referenceCallReceiverFlags(
  parent: AstNode | null,
  key: string | null,
): Readonly<{ call: boolean; receiver: boolean }> {
  if (parent === null) {
    return { call: false, receiver: false };
  }
  const t = parent.type;
  if (t === AST_TYPE.CallExpression && key === "callee") {
    return { call: true, receiver: false };
  }
  if (t === AST_TYPE.NewExpression && key === "callee") {
    return { call: true, receiver: false };
  }
  if (t === AST_TYPE.MemberExpression && key === "object") {
    return { call: false, receiver: true };
  }
  return { call: false, receiver: false };
}
