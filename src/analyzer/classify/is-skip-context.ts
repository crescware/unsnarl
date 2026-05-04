import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { isComputed } from "./is-computed.js";

export function isSkipContext(
  t: string,
  key: string | null,
  parent: AstNode,
): boolean {
  if (t === AST_TYPE.ImportSpecifier && key === "imported") {
    return true;
  }
  if (t === AST_TYPE.ExportSpecifier && key === "exported") {
    return true;
  }
  if (
    t === AST_TYPE.MemberExpression &&
    key === "property" &&
    !isComputed(parent)
  ) {
    return true;
  }
  if (
    (t === AST_TYPE.Property ||
      t === AST_TYPE.MethodDefinition ||
      t === AST_TYPE.PropertyDefinition ||
      t === AST_TYPE.AccessorProperty) &&
    key === "key" &&
    !isComputed(parent)
  ) {
    return true;
  }
  if (t === AST_TYPE.JSXAttribute && key === "name") {
    return true;
  }
  if (t === AST_TYPE.JSXMemberExpression && key === "property") {
    return true;
  }
  if (t === AST_TYPE.JSXClosingElement) {
    return true;
  }
  if (
    (t === "LabeledStatement" ||
      t === AST_TYPE.ContinueStatement ||
      t === AST_TYPE.BreakStatement) &&
    key === "label"
  ) {
    return true;
  }
  return false;
}
