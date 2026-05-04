import type { AstIdentifier } from "../ir/primitive/ast-identifier.js";
import type { AstNode } from "../ir/primitive/ast-node.js";
import type { ExpressionStatementContainer } from "../ir/reference/expression-statement-container.js";
import type { JsxElementContainer } from "../ir/reference/jsx-element-container.js";
import type { PredicateContainer } from "../ir/reference/predicate-container.js";
import { ReferenceFlags } from "../ir/reference/reference-flags.js";
import type { ReferenceFlagBits } from "../ir/reference/reference-flags.js";
import type { Reference } from "../ir/reference/reference.js";
import type { ReturnContainer } from "../ir/reference/return-container.js";
import type { BlockContext } from "../ir/scope/block-context.js";
import type { Definition } from "../ir/scope/definition.js";
import type { Scope } from "../ir/scope/scope.js";
import type { Variable } from "../ir/scope/variable.js";
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
