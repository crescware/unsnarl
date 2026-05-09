import type { AstIdentifier } from "../../ir/primitive/ast-identifier.js";
import { ReferenceFlags } from "../../ir/reference/reference-flags.js";
import type { ReferenceFlagBits } from "../../ir/reference/reference-flags.js";
import type { Reference } from "../../ir/reference/reference.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";

export class ReferenceImpl implements Reference {
  readonly identifier: AstIdentifier;
  readonly from: Scope;
  resolved: Variable | null = null;
  readonly init: boolean;
  private readonly flags: ReferenceFlagBits;

  constructor(opts: {
    identifier: AstIdentifier;
    from: Scope;
    flags: ReferenceFlagBits;
    init: boolean;
  }) {
    this.identifier = opts.identifier;
    this.from = opts.from;
    this.flags = opts.flags;
    this.init = opts.init;
  }

  isRead(): boolean {
    return (this.flags & ReferenceFlags.Read) !== 0;
  }

  isWrite(): boolean {
    return (this.flags & ReferenceFlags.Write) !== 0;
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
    return (this.flags & ReferenceFlags.Call) !== 0;
  }

  isReceiver(): boolean {
    return (this.flags & ReferenceFlags.Receiver) !== 0;
  }
}
