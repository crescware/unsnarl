import type { AstIdentifier } from "../../../ir/primitive/ast-identifier.js";
import type { CompatScope } from "./compat-scope.js";
import type { CompatVariable } from "./compat-variable.js";

// Mirrors the non-deprecated public API of eslint-scope's Reference class
// (see node_modules/eslint-scope/lib/index.d.cts). Any unsnarl Reference
// type must remain structurally assignable to this contract; otherwise
// parity will (or already does) regress.
//
// Deliberately excluded:
// - `isCall()` / `isReceiver()` — unsnarl-only extensions; eslint-scope
//   has no equivalent (parity stubs them to false on the eslint-scope
//   side).
// - `writeExpr` / `tainted` / `partial` / `flag` / `isStatic()` —
//   deprecated in eslint-scope; not part of the contract surface unsnarl
//   commits to.
export type CompatReference = {
  identifier: AstIdentifier;
  resolved: CompatVariable | null;
  from: CompatScope;
  init?: boolean;
  isWrite(): boolean;
  isRead(): boolean;
  isReadOnly(): boolean;
  isWriteOnly(): boolean;
  isReadWrite(): boolean;
};
