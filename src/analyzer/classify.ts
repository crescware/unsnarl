import { ReferenceFlags } from "../ir/model.js";
import type { AstExpression, AstNode, ReferenceFlagBits } from "../ir/model.js";
import type { PathEntry } from "./walk.js";

export type ClassifyResult =
  | { kind: "binding" }
  | { kind: "skip" }
  | {
      kind: "reference";
      flags: ReferenceFlagBits;
      init: boolean;
      writeExpr: AstExpression | null;
    };

export function classifyIdentifier(
  parent: AstNode | null,
  key: string | null,
  path: ReadonlyArray<PathEntry>,
): ClassifyResult {
  if (!parent) {
    return reference(ReferenceFlags.Read, false, null);
  }

  const t = parent.type;

  if (isSkipContext(t, key, parent)) {
    return { kind: "skip" };
  }

  if (isDirectBinding(t, key)) {
    return { kind: "binding" };
  }

  if (isPatternStep(parent, path, path.length - 1)) {
    const root = findBindingRootContext(parent, key, path);
    if (root === "var" || root === "param" || root === "catch") {
      return { kind: "binding" };
    }
    if (root === "assign") {
      return reference(ReferenceFlags.Write, false, null);
    }
  }

  return classifyOrdinaryReference(t, key, parent);
}

function isSkipContext(
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

function isDirectBinding(t: string, key: string | null): boolean {
  if (t === "VariableDeclarator" && key === "id") {
    return true;
  }
  if (
    (t === "FunctionDeclaration" || t === "FunctionExpression") &&
    key === "id"
  ) {
    return true;
  }
  if ((t === "ClassDeclaration" || t === "ClassExpression") && key === "id") {
    return true;
  }
  if (t === "CatchClause" && key === "param") {
    return true;
  }
  if (
    (t === "ImportSpecifier" ||
      t === "ImportDefaultSpecifier" ||
      t === "ImportNamespaceSpecifier") &&
    key === "local"
  ) {
    return true;
  }
  return false;
}

function classifyOrdinaryReference(
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
  let init = false;
  if (t === "VariableDeclarator" && key === "init") {
    init = true;
  }
  return reference(ReferenceFlags.Read, init, null);
}

function findBindingRootContext(
  parent: AstNode | null,
  key: string | null,
  path: ReadonlyArray<PathEntry>,
): "var" | "param" | "catch" | "assign" | null {
  let curParent: AstNode | null = parent;
  let curKey = key;
  let i = path.length - 1;
  while (curParent) {
    const t = curParent.type;
    const isPattern = isPatternStep(curParent, path, i);
    if (!isPattern) {
      switch (t) {
        case "VariableDeclarator":
          return curKey === "id" ? "var" : null;
        case "CatchClause":
          return curKey === "param" ? "catch" : null;
        case "FunctionDeclaration":
        case "FunctionExpression":
        case "ArrowFunctionExpression":
          return curKey === "params" ? "param" : null;
        case "AssignmentExpression":
          return curKey === "left" ? "assign" : null;
        default:
          return null;
      }
    }
    i -= 1;
    if (i < 0) {
      return null;
    }
    const next = path[i];
    if (!next) {
      return null;
    }
    curParent = next.node;
    curKey = path[i + 1]?.key ?? null;
  }
  return null;
}

function isPatternStep(
  node: AstNode,
  path: ReadonlyArray<PathEntry>,
  i: number,
): boolean {
  const t = node.type;
  if (
    t === "ObjectPattern" ||
    t === "ArrayPattern" ||
    t === "RestElement" ||
    t === "AssignmentPattern"
  ) {
    return true;
  }
  if (t === "Property") {
    return path[i - 1]?.node.type === "ObjectPattern";
  }
  return false;
}

function isComputed(node: AstNode): boolean {
  return (node as { computed?: boolean }).computed === true;
}

function isAstExpression(value: unknown): value is AstExpression {
  return (
    value !== null &&
    typeof value === "object" &&
    "type" in value &&
    typeof (value as { type: unknown }).type === "string"
  );
}

function reference(
  flags: ReferenceFlagBits,
  init: boolean,
  writeExpr: AstExpression | null,
): ClassifyResult {
  return { kind: "reference", flags, init, writeExpr };
}
