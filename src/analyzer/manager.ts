import type { AstNode } from "../ir/primitive/ast-node.js";
import type { BlockContext } from "../ir/scope/block-context.js";
import type { Scope } from "../ir/scope/scope.js";
import { ScopeImpl } from "./scope-impl.js";
import type { ScopeType } from "./scope-type.js";

export class ScopeManager {
  readonly globalScope: Scope;
  readonly allScopes: /* mutable */ Scope[];
  private readonly stack: /* mutable */ Scope[];

  constructor(rootKind: "module" | "global", block: AstNode) {
    const root = new ScopeImpl({
      type: rootKind,
      isStrict: rootKind === "module",
      upper: null,
      block,
      blockContext: null,
    });
    this.globalScope = root;
    this.allScopes = [root];
    this.stack = [root];
  }

  current(): Scope {
    const top = this.stack[this.stack.length - 1];
    if (!top) {
      throw new Error("Scope stack is empty");
    }
    return top;
  }

  push(
    type: ScopeType,
    block: AstNode,
    blockContext: BlockContext | null,
  ): Scope {
    const scope = new ScopeImpl({
      type,
      isStrict: this.current().isStrict,
      upper: this.current(),
      block,
      blockContext,
    });
    this.stack.push(scope);
    this.allScopes.push(scope);
    return scope;
  }

  pop(): void {
    if (this.stack.length <= 1) {
      throw new Error("Cannot pop the root scope");
    }
    this.stack.pop();
  }
}
