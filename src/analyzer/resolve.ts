import type { AstNode, Reference, Scope, Variable } from "../ir/model.js";
import { DEFINITION_TYPE } from "./definition-type.js";
import { VariableImpl } from "./scope.js";

export function resolveInScopeChain(
  scope: Scope,
  name: string,
): Variable | null {
  let cur: Scope | null = scope;
  while (cur) {
    const v = cur.set.get(name);
    if (v) {
      return v;
    }
    cur = cur.upper;
  }
  return null;
}

export function bindReference(
  scope: Scope,
  reference: Reference,
  globalScope: Scope,
): void {
  scope.references.push(reference);
  const resolved = resolveInScopeChain(scope, reference.identifier.name);
  if (resolved) {
    (reference as { resolved: Variable | null }).resolved = resolved;
    resolved.references.push(reference);
    return;
  }

  let implicit = globalScope.set.get(reference.identifier.name);
  if (!implicit) {
    implicit = new VariableImpl(reference.identifier.name, globalScope);
    globalScope.set.set(reference.identifier.name, implicit);
    globalScope.variables.push(implicit);
    implicit.identifiers.push(reference.identifier);
    implicit.defs.push({
      type: DEFINITION_TYPE.ImplicitGlobalVariable,
      name: reference.identifier,
      node: reference.identifier as unknown as AstNode,
      parent: null,
    });
  }
  (reference as { resolved: Variable | null }).resolved = implicit;
  implicit.references.push(reference);

  let cur: Scope | null = scope;
  while (cur && cur !== globalScope) {
    cur.through.push(reference);
    cur = cur.upper;
  }
  globalScope.through.push(reference);
}
