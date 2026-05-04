import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { PathEntry } from "./path-entry.js";
import type { WalkAction } from "./walk-action.js";

export type WalkVisitor = Readonly<{
  enter?(
    node: AstNode,
    parent: AstNode | null,
    key: string | null,
    path: readonly PathEntry[],
  ): WalkAction;
  leave?(
    node: AstNode,
    parent: AstNode | null,
    key: string | null,
    path: readonly PathEntry[],
  ): void;
}>;
