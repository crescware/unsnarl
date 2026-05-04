import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { Scope } from "../../ir/scope/scope.js";
import { declareVariable } from "../declare/declare-variable.js";
import { DEFINITION_TYPE } from "../definition-type.js";
import { isIdentifierNode } from "./is-identifier-node.js";
import type { NodeLike } from "./node-like.js";

export function handleFunctionDeclaration(node: NodeLike, scope: Scope): void {
  const id = node["id"];
  if (!isIdentifierNode(id)) {
    return;
  }
  declareVariable(
    scope,
    id,
    DEFINITION_TYPE.FunctionName,
    node as unknown as AstNode,
    null,
  );
}
