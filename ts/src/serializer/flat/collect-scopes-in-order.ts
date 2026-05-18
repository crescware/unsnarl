import type { Scope } from "../../ir/scope/scope.js";

export function collectScopesInOrder(root: Scope): readonly Scope[] {
  const out: /* mutable */ Scope[] = [];
  function visit(s: Scope) {
    out.push(s);
    for (const c of s.childScopes) {
      visit(c);
    }
  }
  visit(root);
  return out;
}
