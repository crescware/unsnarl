import type {
  AstIdentifier,
  AstNode,
  Definition,
  DefinitionType,
  Scope,
  Variable,
} from "../../ir/model.js";
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
  const def: Definition = {
    type: defType,
    name: identifier,
    node: defNode,
    parent,
  };
  variable.defs.push(def);
  return variable;
}
