import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { declareFunctionParams } from "./declare-function-params.js";
import { declareImplicitArguments } from "./declare/declare-implicit-arguments.js";
import { hoistDeclarations } from "./hoisting/hoist-declarations.js";
import { isNodeLike } from "./is-node-like.js";
import type { ScopeManager } from "./manager.js";
import type { NodeLike } from "./node-like.js";
import type { AnalysisVisitor } from "./visitor.js";
import type { PathEntry } from "./walk/path-entry.js";

export function enterFunction(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: readonly PathEntry[],
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
  visitor: AnalysisVisitor,
): void {
  const scope = manager.push("function", node as unknown as AstNode);
  visitor.onScope?.({ scope, parent, key, path });
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
