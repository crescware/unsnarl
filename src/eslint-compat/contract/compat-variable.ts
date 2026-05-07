import type { AstIdentifier } from "../../ir/primitive/ast-identifier.js";
import type { CompatDefinition } from "./compat-definition.js";
import type { CompatReference } from "./compat-reference.js";
import type { CompatScope } from "./compat-scope.js";

// Mirrors the non-deprecated public API of eslint-scope's Variable class
// (see node_modules/eslint-scope/lib/index.d.cts). Any unsnarl Variable
// type must remain structurally assignable to this contract; otherwise
// parity will (or already does) regress.
//
// Deliberately excluded:
// - `tainted` / `stack` — deprecated in eslint-scope; not part of the
//   contract surface unsnarl commits to.
// - `unsnarlIsUnused()` — unsnarl-only extension method; not in
//   eslint-scope.
export type CompatVariable = {
  name: string;
  scope: CompatScope;
  identifiers: AstIdentifier[];
  references: CompatReference[];
  defs: CompatDefinition[];
};
