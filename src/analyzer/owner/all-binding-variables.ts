import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import { collectBindingIdentifiers } from "../declare/collect-binding-identifiers.js";
import { resolveInScopeChain } from "../resolve.js";

export function allBindingVariables(
  pattern: AstNode,
  scope: Scope,
): /* mutable */ Variable[] {
  const idents = collectBindingIdentifiers(pattern);
  const out: /* mutable */ Variable[] = [];
  for (const ident of idents) {
    const v = resolveInScopeChain(scope, ident.name);
    if (v && !out.includes(v)) {
      out.push(v);
    }
  }
  return out;
}
