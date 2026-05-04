import type { Scope } from "../../ir/scope/scope.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { handleClassDeclaration } from "./handle-class-declaration.js";
import { handleFunctionDeclaration } from "./handle-function-declaration.js";
import { handleImportDeclaration } from "./handle-import-declaration.js";
import { handleVariableDeclaration } from "./handle-variable-declaration.js";
import { isNodeLike, type NodeLike } from "./node-like.js";

export function visit(
  node: NodeLike,
  scope: Scope,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  switch (node.type) {
    case AST_TYPE.VariableDeclaration:
      handleVariableDeclaration(node, scope, raw, diagnostics);
      return;
    case AST_TYPE.FunctionDeclaration:
      handleFunctionDeclaration(node, scope);
      return;
    case AST_TYPE.ClassDeclaration:
      handleClassDeclaration(node, scope);
      return;
    case AST_TYPE.ImportDeclaration:
      handleImportDeclaration(node, scope);
      return;
    case AST_TYPE.ExportNamedDeclaration: {
      const decl = node["declaration"];
      if (isNodeLike(decl)) {
        visit(decl, scope, raw, diagnostics);
      }
      return;
    }
    case AST_TYPE.ExportDefaultDeclaration: {
      const decl = node["declaration"];
      if (
        isNodeLike(decl) &&
        (decl.type === AST_TYPE.FunctionDeclaration ||
          decl.type === AST_TYPE.ClassDeclaration)
      ) {
        visit(decl, scope, raw, diagnostics);
      }
      return;
    }
    default:
      return;
  }
}
