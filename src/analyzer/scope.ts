import type {
  AstIdentifier,
  AstNode,
  Definition,
  Reference,
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

  constructor(opts: {
    type: ScopeType;
    isStrict: boolean;
    upper: Scope | null;
    block: AstNode;
  }) {
    this.type = opts.type;
    this.isStrict = opts.isStrict;
    this.upper = opts.upper;
    this.block = opts.block;
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
