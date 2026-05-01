import type { AstNode } from "../../ir/model.js";
import { walkNode } from "./walk-node.js";

export type PathEntry = Readonly<{
  node: AstNode;
  key: string | null;
}>;

export type WalkAction = "skip" | undefined | void;

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

export function walk(root: AstNode, visitor: WalkVisitor): void {
  walkNode(root, null, null, visitor, []);
}
