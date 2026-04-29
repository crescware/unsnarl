import { visitorKeys } from "oxc-parser";

import type { AstNode } from "../ir/model.js";

export interface WalkVisitor {
  enter?(node: AstNode, parent: AstNode | null, key: string | null): void;
  leave?(node: AstNode, parent: AstNode | null, key: string | null): void;
}

export function walk(root: AstNode, visitor: WalkVisitor): void {
  walkNode(root, null, null, visitor);
}

function walkNode(
  node: AstNode,
  parent: AstNode | null,
  key: string | null,
  visitor: WalkVisitor,
): void {
  visitor.enter?.(node, parent, key);
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
            walkNode(c, node, k, visitor);
          }
        }
      } else if (isAstNode(child)) {
        walkNode(child, node, k, visitor);
      }
    }
  }
  visitor.leave?.(node, parent, key);
}

function isAstNode(value: unknown): value is AstNode {
  return (
    value !== null &&
    typeof value === "object" &&
    "type" in value &&
    typeof (value as { type: unknown }).type === "string"
  );
}
