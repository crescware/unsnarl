import { DEFINITION_TYPE } from "../../constants.js";
import type { AstNode } from "../../ir/model.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { collectBindingIdentifiers } from "../declare/collect-binding-identifiers.js";
import { declareVariable } from "../declare/declare-variable.js";
import { hoistDeclarations } from "../hoisting/hoist-declarations.js";
import type { ScopeManager } from "../manager.js";
import { blockContextOf } from "./block-context-of.js";
import { isNodeLike } from "./is-node-like.js";
import type { NodeLike } from "./node-like.js";

export function enterCatch(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  const ctx = blockContextOf(parent, key);
  const scope = manager.push("catch", node as unknown as AstNode, ctx);
  const param = node["param"];
  if (isNodeLike(param)) {
    const idents = collectBindingIdentifiers(param as unknown as AstNode);
    for (const ident of idents) {
      declareVariable(
        scope,
        ident,
        DEFINITION_TYPE.CatchClause,
        node as unknown as AstNode,
        null,
      );
    }
  }
  const body = node["body"];
  if (isNodeLike(body) && body.type === "BlockStatement") {
    const stmts = body["body"];
    if (Array.isArray(stmts)) {
      hoistDeclarations(stmts, scope, raw, diagnostics);
    }
  }
}
