import type { AstNode, Scope, Variable } from "../ir/model.js";
import { collectBindingIdentifiers } from "./declare.js";
import { resolveInScopeChain } from "./resolve.js";
import type { PathEntry } from "./walk.js";

export function findReferenceOwner(
  path: ReadonlyArray<PathEntry>,
  scope: Scope,
): Variable | null {
  for (let i = path.length - 1; i >= 0; i--) {
    const entry = path[i];
    if (!entry) {
      break;
    }
    const t = entry.node.type;
    if (t === "VariableDeclarator") {
      const id = entry.node["id"];
      if (isAstNode(id)) {
        return firstBindingVariable(id, scope);
      }
      return null;
    }
    if (t === "AssignmentExpression") {
      const left = entry.node["left"];
      if (isAstNode(left)) {
        if (left.type === "Identifier") {
          const name = left["name"];
          if (typeof name === "string") {
            return resolveInScopeChain(scope, name);
          }
        }
        return firstBindingVariable(left, scope);
      }
      return null;
    }
    if (
      t === "FunctionDeclaration" ||
      t === "FunctionExpression" ||
      t === "ArrowFunctionExpression" ||
      t === "ClassDeclaration" ||
      t === "ClassExpression" ||
      t === "MethodDefinition" ||
      t === "PropertyDefinition" ||
      t === "AccessorProperty"
    ) {
      return null;
    }
  }
  return null;
}

function firstBindingVariable(pattern: AstNode, scope: Scope): Variable | null {
  const idents = collectBindingIdentifiers(pattern);
  const head = idents[0];
  if (!head) {
    return null;
  }
  return resolveInScopeChain(scope, head.name);
}

function isAstNode(value: unknown): value is AstNode {
  return (
    value !== null &&
    typeof value === "object" &&
    "type" in value &&
    typeof (value as { type: unknown }).type === "string"
  );
}
