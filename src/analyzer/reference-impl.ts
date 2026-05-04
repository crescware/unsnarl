import type { AstIdentifier } from "../ir/primitive/ast-identifier.js";
import type { ExpressionStatementContainer } from "../ir/reference/expression-statement-container.js";
import type { JsxElementContainer } from "../ir/reference/jsx-element-container.js";
import type { PredicateContainer } from "../ir/reference/predicate-container.js";
import { ReferenceFlags } from "../ir/reference/reference-flags.js";
import type { ReferenceFlagBits } from "../ir/reference/reference-flags.js";
import type { Reference } from "../ir/reference/reference.js";
import type { ReturnContainer } from "../ir/reference/return-container.js";
import type { Scope } from "../ir/scope/scope.js";
import type { Variable } from "../ir/scope/variable.js";

export class ReferenceImpl implements Reference {
  readonly identifier: AstIdentifier;
  readonly from: Scope;
  resolved: Variable | null = null;
  unsnarlOwners: /* mutable */ Variable[] = [];
  unsnarlPredicateContainer: PredicateContainer | null = null;
  unsnarlReturnContainer: ReturnContainer | null = null;
  unsnarlJsxElement: JsxElementContainer | null = null;
  unsnarlExpressionStatementContainer: ExpressionStatementContainer | null =
    null;
  readonly init: boolean;
  readonly unsnarlFlags: ReferenceFlagBits;

  constructor(opts: {
    identifier: AstIdentifier;
    from: Scope;
    flags: ReferenceFlagBits;
    init: boolean;
  }) {
    this.identifier = opts.identifier;
    this.from = opts.from;
    this.unsnarlFlags = opts.flags;
    this.init = opts.init;
  }

  isRead(): boolean {
    return (this.unsnarlFlags & ReferenceFlags.Read) !== 0;
  }

  isWrite(): boolean {
    return (this.unsnarlFlags & ReferenceFlags.Write) !== 0;
  }

  isReadOnly(): boolean {
    return this.isRead() && !this.isWrite();
  }

  isWriteOnly(): boolean {
    return this.isWrite() && !this.isRead();
  }

  isReadWrite(): boolean {
    return this.isRead() && this.isWrite();
  }

  isCall(): boolean {
    return (this.unsnarlFlags & ReferenceFlags.Call) !== 0;
  }

  isReceiver(): boolean {
    return (this.unsnarlFlags & ReferenceFlags.Receiver) !== 0;
  }
}
