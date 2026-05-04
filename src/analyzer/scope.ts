import { ReferenceFlags } from "../ir/model.js";
import type {
  AstIdentifier,
  AstNode,
  BlockContext,
  Definition,
  ExpressionStatementContainer,
  JsxElementContainer,
  PredicateContainer,
  Reference,
  ReferenceFlagBits,
  ReturnContainer,
  Scope,
  Variable,
} from "../ir/model.js";
import { SCOPE_TYPE, type ScopeType } from "./scope-type.js";

export class ScopeImpl implements Scope {
  readonly type: ScopeType;
  readonly isStrict: boolean;
  readonly upper: Scope | null;
  readonly childScopes: /* mutable */ Scope[] = [];
  readonly variableScope: Scope;
  readonly block: AstNode;
  readonly variables: /* mutable */ Variable[] = [];
  readonly set: Map<string, Variable> = new Map();
  readonly references: /* mutable */ Reference[] = [];
  readonly through: /* mutable */ Reference[] = [];
  readonly functionExpressionScope: boolean = false;
  unsnarlBlockContext: BlockContext | null = null;
  unsnarlFallsThrough: boolean = false;
  unsnarlExitsFunction: boolean = false;

  constructor(opts: {
    type: ScopeType;
    isStrict: boolean;
    upper: Scope | null;
    block: AstNode;
    blockContext: BlockContext | null;
  }) {
    this.type = opts.type;
    this.isStrict = opts.isStrict;
    this.upper = opts.upper;
    this.block = opts.block;
    this.unsnarlBlockContext = opts.blockContext;
    if (opts.upper) {
      opts.upper.childScopes.push(this);
    }
    if (
      opts.type === SCOPE_TYPE.Function ||
      opts.type === SCOPE_TYPE.Module ||
      opts.type === SCOPE_TYPE.Global
    ) {
      this.variableScope = this;
    } else {
      this.variableScope = opts.upper?.variableScope ?? this;
    }
  }
}

export class VariableImpl implements Variable {
  readonly name: string;
  readonly scope: Scope;
  readonly identifiers: /* mutable */ AstIdentifier[] = [];
  readonly references: /* mutable */ Reference[] = [];
  readonly defs: /* mutable */ Definition[] = [];

  constructor(name: string, scope: Scope) {
    this.name = name;
    this.scope = scope;
  }

  unsnarlIsUnused(): boolean {
    return this.references.length === 0;
  }
}

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
