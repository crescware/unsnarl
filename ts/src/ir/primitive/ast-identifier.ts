import type { AST_TYPE } from "../../parser/ast-type.js";
import type { AstNode } from "./ast-node.js";

export type AstIdentifier = AstNode &
  Readonly<{
    type: typeof AST_TYPE.Identifier | typeof AST_TYPE.JSXIdentifier;
    name: string;
  }>;
