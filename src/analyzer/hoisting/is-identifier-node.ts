import type { AstIdentifier } from "../../ir/primitive/ast-identifier.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { isNodeLike } from "./node-like.js";

export function isIdentifierNode(value: unknown): value is AstIdentifier {
  return isNodeLike(value) && value.type === AST_TYPE.Identifier;
}
