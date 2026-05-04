import type { AstNode } from "../../ir/primitive/ast-node.js";
import { walkNode } from "./walk-node.js";
import type { WalkVisitor } from "./walk-visitor.js";

export function walk(root: AstNode, visitor: WalkVisitor): void {
  walkNode(root, null, null, visitor, []);
}
