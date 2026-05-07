import { collectBindingIdentifiers } from "../../analyzer/declare/collect-binding-identifiers.js";
import { declareVariable } from "../../analyzer/declare/declare-variable.js";
import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import { hoistDeclarations } from "../../analyzer/hoisting/hoist-declarations.js";
import type { ScopeManager } from "../../analyzer/manager.js";
import type { PathEntry } from "../../analyzer/walk/path-entry.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { isNodeLike } from "./is-node-like.js";
import type { NodeLike } from "./node-like.js";
import type { AnalysisVisitor } from "./visitor.js";

export function enterCatch(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: readonly PathEntry[],
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
  visitor: AnalysisVisitor,
): void {
  const scope = manager.push("catch", node as unknown as AstNode);
  visitor.onScope?.({ scope, parent, key, path });
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
  if (isNodeLike(body) && body.type === AST_TYPE.BlockStatement) {
    const stmts = body["body"];
    if (Array.isArray(stmts)) {
      hoistDeclarations(stmts, scope, raw, diagnostics);
    }
  }
}
