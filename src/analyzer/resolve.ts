import type { Reference, Scope, Variable } from "../ir/model.js";

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

export function bindReference(scope: Scope, reference: Reference): void {
  scope.references.push(reference);
  const resolved = resolveInScopeChain(scope, reference.identifier.name);
  if (resolved) {
    (reference as { resolved: Variable | null }).resolved = resolved;
    resolved.references.push(reference);
  } else {
    let cur: Scope | null = scope;
    while (cur) {
      cur.through.push(reference);
      cur = cur.upper;
    }
  }
}
