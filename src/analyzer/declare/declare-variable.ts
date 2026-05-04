import type { AstIdentifier } from "../../ir/primitive/ast-identifier.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { Definition } from "../../ir/scope/definition.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import type { DefinitionType } from "../definition-type.js";
import { VariableImpl } from "../scope.js";

export function declareVariable(
  scope: Scope,
  identifier: AstIdentifier,
  defType: DefinitionType,
  defNode: AstNode,
  parent: AstNode | null,
): Variable {
  let variable = scope.set.get(identifier.name);
  if (!variable) {
    variable = new VariableImpl(identifier.name, scope);
    scope.set.set(identifier.name, variable);
    scope.variables.push(variable);
  }
  variable.identifiers.push(identifier);
  const def = {
    type: defType,
    name: identifier,
    node: defNode,
    parent,
  } as const satisfies Definition;
  variable.defs.push(def);
  return variable;
}
