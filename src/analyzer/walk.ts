import { visitorKeys } from "oxc-parser";

import type { AstNode } from "../ir/model.js";

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

function walkNode(
  node: AstNode,
  parent: AstNode | null,
  key: string | null,
  visitor: WalkVisitor,
  path: PathEntry[],
): void {
  const action = visitor.enter?.(node, parent, key, path);
  if (action === "skip") {
    visitor.leave?.(node, parent, key, path);
    return;
  }
  path.push({ node, key });
  const keys = visitorKeys[node.type];
  if (keys) {
    for (const k of keys) {
      const child = node[k];
      if (child === null || child === undefined) {
        continue;
      }
      if (Array.isArray(child)) {
        for (const c of child) {
          if (isAstNode(c)) {
            walkNode(c, node, k, visitor, path);
          }
        }
      } else if (isAstNode(child)) {
        walkNode(child, node, k, visitor, path);
      }
    }
  }
  path.pop();
  visitor.leave?.(node, parent, key, path);
}

function isAstNode(value: unknown): value is AstNode {
  return (
    value !== null &&
    typeof value === "object" &&
    "type" in value &&
    typeof (value as { type: unknown }).type === "string"
  );
}
