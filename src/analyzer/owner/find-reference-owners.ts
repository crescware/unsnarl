import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import { AST_TYPE } from "../../parser/ast-type.js";
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
    if (t === AST_TYPE.VariableDeclarator) {
      const id = entry.node["id"];
      if (isAstNode(id)) {
        return allBindingVariables(id, scope);
      }
      return [];
    }
    if (t === AST_TYPE.AssignmentExpression) {
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
      t === AST_TYPE.FunctionDeclaration ||
      t === AST_TYPE.FunctionExpression ||
      t === AST_TYPE.ArrowFunctionExpression ||
      t === AST_TYPE.ClassDeclaration ||
      t === AST_TYPE.ClassExpression ||
      t === AST_TYPE.MethodDefinition ||
      t === AST_TYPE.PropertyDefinition ||
      t === AST_TYPE.AccessorProperty
    ) {
      return [];
    }
  }
  return [];
}
