import type { AstNode, Scope } from "../../ir/model.js";
import { declareVariable } from "../declare/declare-variable.js";
import { isIdentifierNode } from "./is-identifier-node.js";
import type { NodeLike } from "./node-like.js";

export function handleClassDeclaration(node: NodeLike, scope: Scope): void {
  const id = node["id"];
  if (!isIdentifierNode(id)) {
    return;
  }
  declareVariable(scope, id, "ClassName", node as unknown as AstNode, null);
}
