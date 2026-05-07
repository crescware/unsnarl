import type { DefinitionType } from "../../../analyzer/definition-type.js";
import type { AstIdentifier } from "../../../ir/primitive/ast-identifier.js";
import type { AstNode } from "../../../ir/primitive/ast-node.js";

// Mirrors the non-deprecated public API of eslint-scope's Definition type
// (see node_modules/eslint-scope/lib/index.d.cts). Any unsnarl Definition
// type must remain structurally assignable to this contract; otherwise
// parity will (or already does) regress.
//
// eslint-scope models Definition as a discriminated union of seven
// variants, each pinning `node` and `parent` to specific ESTree node
// types. unsnarl collapses this into a single structural shape with the
// shared key surface (`type` / `name` / `node` / `parent`). The narrower
// per-variant constraints are not part of the contract; they would tie
// unsnarl to estree's exact node identities, which it does not commit to.
//
// Deliberately excluded:
// - `index` / `kind` — deprecated in eslint-scope; not part of the
//   contract surface unsnarl commits to.
export type CompatDefinition = {
  type: DefinitionType;
  name: AstIdentifier;
  node: AstNode;
  parent: AstNode | null;
};
