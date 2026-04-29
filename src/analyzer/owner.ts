import type { AstNode, Scope, Variable } from "../ir/model.js";
import { collectBindingIdentifiers } from "./declare.js";
import { resolveInScopeChain } from "./resolve.js";
import type { PathEntry } from "./walk.js";

export function findReferenceOwners(
  path: ReadonlyArray<PathEntry>,
  scope: Scope,
): Variable[] {
  for (let i = path.length - 1; i >= 0; i--) {
    const entry = path[i];
    if (!entry) {
      break;
    }
    const t = entry.node.type;
    if (t === "VariableDeclarator") {
      const id = entry.node["id"];
      if (isAstNode(id)) {
        return allBindingVariables(id, scope);
      }
      return [];
    }
    if (t === "AssignmentExpression") {
      const left = entry.node["left"];
      if (isAstNode(left)) {
        if (left.type === "Identifier") {
          const name = left["name"];
          if (typeof name === "string") {
            const v = resolveInScopeChain(scope, name);
            return v ? [v] : [];
          }
        }
        return allBindingVariables(left, scope);
      }
      return [];
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
      return [];
    }
  }
  return [];
}

function allBindingVariables(pattern: AstNode, scope: Scope): Variable[] {
  const idents = collectBindingIdentifiers(pattern);
  const out: Variable[] = [];
  for (const ident of idents) {
    const v = resolveInScopeChain(scope, ident.name);
    if (v && !out.includes(v)) {
      out.push(v);
    }
  }
  return out;
}

function isAstNode(value: unknown): value is AstNode {
  return (
    value !== null &&
    typeof value === "object" &&
    "type" in value &&
    typeof (value as { type: unknown }).type === "string"
  );
}
