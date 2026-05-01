import { AST_TYPE } from "../../constants.js";
import type { AstIdentifier } from "../../ir/model.js";
import { isNodeLike } from "./node-like.js";

export function isIdentifierNode(value: unknown): value is AstIdentifier {
  return isNodeLike(value) && value.type === AST_TYPE.Identifier;
}
