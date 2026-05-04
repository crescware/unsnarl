import { visitorKeys } from "oxc-parser";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { isAstNode } from "./is-ast-node.js";
import type { PathEntry, WalkVisitor } from "./walk.js";

export function walkNode(
  node: AstNode,
  parent: AstNode | null,
  key: string | null,
  visitor: WalkVisitor,
  path: /* mutable */ PathEntry[],
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
