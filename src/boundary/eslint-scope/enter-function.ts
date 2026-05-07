import { declareImplicitArguments } from "../../analyzer/declare/declare-implicit-arguments.js";
import { hoistDeclarations } from "../../analyzer/hoisting/hoist-declarations.js";
import type { ScopeManager } from "../../analyzer/manager.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
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
  if (node.type !== AST_TYPE.ArrowFunctionExpression) {
    declareImplicitArguments(scope);
  }
  declareFunctionParams(node, scope);
  const body = node["body"];
  if (isNodeLike(body) && body.type === AST_TYPE.BlockStatement) {
    const stmts = body["body"];
    if (Array.isArray(stmts)) {
      hoistDeclarations(stmts, scope, raw, diagnostics);
    }
  }
}
