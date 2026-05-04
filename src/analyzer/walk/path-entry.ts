import type { AstNode } from "../../ir/primitive/ast-node.js";

export type PathEntry = Readonly<{
  node: AstNode;
  key: string | null;
}>;
