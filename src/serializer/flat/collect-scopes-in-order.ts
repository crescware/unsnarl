import type { Scope } from "../../ir/model.js";

export function collectScopesInOrder(root: Scope): Scope[] {
  const out: Scope[] = [];
  function visit(s: Scope) {
    out.push(s);
    for (const c of s.childScopes) {
      visit(c);
    }
  }
  visit(root);
  return out;
}
