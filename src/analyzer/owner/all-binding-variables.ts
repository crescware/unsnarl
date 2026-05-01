import type { AstNode, Scope, Variable } from "../../ir/model.js";
import { collectBindingIdentifiers } from "../declare/collect-binding-identifiers.js";
import { resolveInScopeChain } from "../resolve.js";

export function allBindingVariables(
  pattern: AstNode,
  scope: Scope,
): Variable[] {
  const idents = collectBindingIdentifiers(pattern);
  const out: Variable[] = [];
  for (const ident of idents) {
    const v = resolveInScopeChain(scope, ident.name);
    if (v && !out.includes(v)) {
      out.push(v);
    }
  }
  return out;
}
