import type { AstNode } from "../../ir/model.js";
import { walkNode } from "./walk-node.js";

export interface PathEntry {
  readonly node: AstNode;
  readonly key: string | null;
}

export type WalkAction = "skip" | undefined | void;

export interface WalkVisitor {
  enter?(
    node: AstNode,
    parent: AstNode | null,
    key: string | null,
    path: ReadonlyArray<PathEntry>,
  ): WalkAction;
  leave?(
    node: AstNode,
    parent: AstNode | null,
    key: string | null,
    path: ReadonlyArray<PathEntry>,
  ): void;
}

export function walk(root: AstNode, visitor: WalkVisitor): void {
  walkNode(root, null, null, visitor, []);
}
