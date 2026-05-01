import type { AstNode } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { hoistDeclarations } from "../hoisting/hoist-declarations.js";
import type { ScopeManager } from "../manager.js";
import { declareFunctionParams } from "./declare-function-params.js";
import { isNodeLike } from "./is-node-like.js";
import type { NodeLike } from "./node-like.js";

export function enterFunction(
  node: NodeLike,
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  const scope = manager.push("function", node as unknown as AstNode, null);
  declareFunctionParams(node, scope);
  const body = node["body"];
  if (isNodeLike(body) && body.type === AST_TYPE.BlockStatement) {
    const stmts = body["body"];
    if (Array.isArray(stmts)) {
      hoistDeclarations(stmts, scope, raw, diagnostics);
    }
  }
}
