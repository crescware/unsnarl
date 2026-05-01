import type { Scope } from "../../ir/model.js";
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
    case "VariableDeclaration":
      handleVariableDeclaration(node, scope, raw, diagnostics);
      return;
    case "FunctionDeclaration":
      handleFunctionDeclaration(node, scope);
      return;
    case "ClassDeclaration":
      handleClassDeclaration(node, scope);
      return;
    case "ImportDeclaration":
      handleImportDeclaration(node, scope);
      return;
    case "ExportNamedDeclaration": {
      const decl = node["declaration"];
      if (isNodeLike(decl)) {
        visit(decl, scope, raw, diagnostics);
      }
      return;
    }
    case "ExportDefaultDeclaration": {
      const decl = node["declaration"];
      if (
        isNodeLike(decl) &&
        (decl.type === "FunctionDeclaration" ||
          decl.type === "ClassDeclaration")
      ) {
        visit(decl, scope, raw, diagnostics);
      }
      return;
    }
    default:
      return;
  }
}
