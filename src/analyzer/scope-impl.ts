import type { AstNode } from "../ir/primitive/ast-node.js";
import type { Reference } from "../ir/reference/reference.js";
import type { BlockContext } from "../ir/scope/block-context.js";
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
