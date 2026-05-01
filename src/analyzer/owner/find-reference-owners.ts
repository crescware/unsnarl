import { AST_TYPE } from "../../constants.js";
import type { Scope, Variable } from "../../ir/model.js";
import { resolveInScopeChain } from "../resolve.js";
import type { PathEntry } from "../walk/walk.js";
import { allBindingVariables } from "./all-binding-variables.js";
import { isAstNode } from "./is-ast-node.js";

export function findReferenceOwners(
  path: readonly PathEntry[],
  scope: Scope,
): /* mutable */ Variable[] {
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
        if (left.type === AST_TYPE.Identifier) {
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
