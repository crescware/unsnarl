import type { ScopeType } from "../../../analyzer/scope-type.js";
import type { AstNode } from "../../../ir/primitive/ast-node.js";
import type { CompatReference } from "./compat-reference.js";
import type { CompatVariable } from "./compat-variable.js";

// Mirrors the non-deprecated public API of eslint-scope's Scope class
// (see node_modules/eslint-scope/lib/index.d.cts). Any unsnarl Scope type
// must remain structurally assignable to this contract; otherwise parity
// will (or already does) regress.
//
// Deliberately excluded:
// - `taints` / `dynamic` / `directCallToEvalScope` / `thisFound` and the
//   methods `resolve()` / `isStatic()` / `isArgumentsMaterialized()` /
//   `isThisMaterialized()` / `isUsedName()` — all deprecated in
//   eslint-scope; not part of the contract surface unsnarl commits to.
export type CompatScope = {
  type: ScopeType;
  isStrict: boolean;
  upper: CompatScope | null;
  variableScope: CompatScope;
  variables: CompatVariable[];
  references: CompatReference[];
  childScopes: CompatScope[];
  block: AstNode;
  functionExpressionScope: boolean;
  set: Map<string, CompatVariable>;
  through: CompatReference[];
};
