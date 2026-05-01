import type { AstNode, Scope } from "../../ir/model.js";
import { collectBindingIdentifiers, declareVariable } from "../declare.js";
import { isNodeLike } from "./is-node-like.js";
import type { NodeLike } from "./node-like.js";

export function declareFunctionParams(node: NodeLike, scope: Scope): void {
  const params = node["params"];
  if (!Array.isArray(params)) {
    return;
  }
  for (const p of params) {
    if (!isNodeLike(p)) {
      continue;
    }
    const target = p.type === "RestElement" ? (p["argument"] ?? p) : p;
    const idents = collectBindingIdentifiers(target as unknown as AstNode);
    for (const ident of idents) {
      declareVariable(
        scope,
        ident,
        "Parameter",
        p as unknown as AstNode,
        node as unknown as AstNode,
      );
    }
  }
}
