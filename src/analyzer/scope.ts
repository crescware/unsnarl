import { ReferenceFlags } from "../ir/model.js";
import type {
  AstExpression,
  AstIdentifier,
  AstNode,
  BlockContext,
  Definition,
  PredicateContainer,
  Reference,
  ReferenceFlagBits,
  Scope,
  ScopeType,
  Variable,
} from "../ir/model.js";

export class ScopeImpl implements Scope {
  readonly type: ScopeType;
  readonly isStrict: boolean;
  readonly upper: Scope | null;
  readonly childScopes: Scope[] = [];
  readonly variableScope: Scope;
  readonly block: AstNode;
  readonly variables: Variable[] = [];
  readonly set: Map<string, Variable> = new Map();
  readonly references: Reference[] = [];
  readonly through: Reference[] = [];
  readonly functionExpressionScope: boolean = false;
  unsnarlBlockContext: BlockContext | null = null;

  constructor(opts: {
    type: ScopeType;
    isStrict: boolean;
    upper: Scope | null;
    block: AstNode;
    blockContext?: BlockContext | null;
  }) {
    this.type = opts.type;
    this.isStrict = opts.isStrict;
    this.upper = opts.upper;
    this.block = opts.block;
    this.unsnarlBlockContext = opts.blockContext ?? null;
    if (opts.upper) {
      opts.upper.childScopes.push(this);
    }
    if (
      opts.type === "function" ||
      opts.type === "module" ||
      opts.type === "global"
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
  readonly identifiers: AstIdentifier[] = [];
  readonly references: Reference[] = [];
  readonly defs: Definition[] = [];

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
  unsnarlOwners: Variable[] = [];
  unsnarlPredicateContainer: PredicateContainer | null = null;
  readonly writeExpr: AstExpression | null;
  readonly init: boolean;
  readonly unsnarlFlags: ReferenceFlagBits;

  constructor(opts: {
    identifier: AstIdentifier;
    from: Scope;
    flags: ReferenceFlagBits;
    init: boolean;
    writeExpr?: AstExpression | null;
  }) {
    this.identifier = opts.identifier;
    this.from = opts.from;
    this.unsnarlFlags = opts.flags;
    this.init = opts.init;
    this.writeExpr = opts.writeExpr ?? null;
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
