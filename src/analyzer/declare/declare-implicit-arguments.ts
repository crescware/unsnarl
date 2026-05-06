import type { Scope } from "../../ir/scope/scope.js";
import { VariableImpl } from "../variable-impl.js";

// eslint-scope's FunctionScope.__defineArguments registers an `arguments`
// Variable with no identifiers and no defs (CreateUnmappedArgumentsObject /
// CreateMappedArgumentsObject from FunctionDeclarationInstantiation, ES spec
// 9.2.13). Arrow functions inherit `arguments` from the enclosing scope and
// must not call this helper.
export function declareImplicitArguments(scope: Scope): void {
  if (scope.set.has("arguments")) {
    return;
  }
  const variable = new VariableImpl("arguments", scope);
  scope.set.set("arguments", variable);
  scope.variables.push(variable);
}
