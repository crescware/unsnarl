import { DEFINITION_TYPE } from "../../definition-type.js";
import type { AstNode, Scope } from "../../ir/model.js";
import { declareVariable } from "../declare/declare-variable.js";
import { isIdentifierNode } from "./is-identifier-node.js";
import { isNodeLike, type NodeLike } from "./node-like.js";

export function handleImportDeclaration(node: NodeLike, scope: Scope): void {
  const specifiers = node["specifiers"];
  if (!Array.isArray(specifiers)) {
    return;
  }
  for (const spec of specifiers) {
    if (!isNodeLike(spec)) {
      continue;
    }
    const local = spec["local"];
    if (!isIdentifierNode(local)) {
      continue;
    }
    declareVariable(
      scope,
      local,
      DEFINITION_TYPE.ImportBinding,
      spec as unknown as AstNode,
      node as unknown as AstNode,
    );
  }
}
