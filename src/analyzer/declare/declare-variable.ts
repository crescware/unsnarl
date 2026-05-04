import type {
  AstIdentifier,
  AstNode,
  Definition,
  Scope,
  Variable,
} from "../../ir/model.js";
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
