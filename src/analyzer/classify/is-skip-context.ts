import type { AstNode } from "../../ir/model.js";
import { isComputed } from "./is-computed.js";

export function isSkipContext(
  t: string,
  key: string | null,
  parent: AstNode,
): boolean {
  if (t === "ImportSpecifier" && key === "imported") {
    return true;
  }
  if (t === "ExportSpecifier" && key === "exported") {
    return true;
  }
  if (t === "MemberExpression" && key === "property" && !isComputed(parent)) {
    return true;
  }
  if (
    (t === "Property" ||
      t === "MethodDefinition" ||
      t === "PropertyDefinition" ||
      t === "AccessorProperty") &&
    key === "key" &&
    !isComputed(parent)
  ) {
    return true;
  }
  if (t === "JSXAttribute" && key === "name") {
    return true;
  }
  if (t === "JSXMemberExpression" && key === "property") {
    return true;
  }
  if (t === "JSXClosingElement") {
    return true;
  }
  if (
    (t === "LabeledStatement" ||
      t === "ContinueStatement" ||
      t === "BreakStatement") &&
    key === "label"
  ) {
    return true;
  }
  return false;
}
