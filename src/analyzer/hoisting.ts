import type { AstIdentifier, AstNode, Scope } from "../ir/model.js";
import type { DiagnosticCollector } from "../util/diagnostic.js";
import { spanFromOffset } from "../util/span.js";
import { collectBindingIdentifiers, declareVariable } from "./declare.js";

interface NodeLike {
  type: string;
  start?: number;
  end?: number;
  [key: string]: unknown;
}

export function hoistDeclarations(
  body: ReadonlyArray<unknown>,
  scope: Scope,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  for (const stmt of body) {
    if (!isNodeLike(stmt)) {
      continue;
    }
    visit(stmt, scope, raw, diagnostics);
  }
}

function visit(
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

function handleVariableDeclaration(
  node: NodeLike,
  scope: Scope,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  const kind = node["kind"];
  if (kind === "var") {
    const start = node.start ?? 0;
    diagnostics.add(
      "var-detected",
      "var declaration is not supported and was skipped.",
      spanFromOffset(raw, start),
    );
    return;
  }
  if (kind !== "const" && kind !== "let") {
    return;
  }
  const declarations = node["declarations"];
  if (!Array.isArray(declarations)) {
    return;
  }
  for (const dec of declarations) {
    if (!isNodeLike(dec)) {
      continue;
    }
    const id = dec["id"];
    if (!isNodeLike(id)) {
      continue;
    }
    const idents = collectBindingIdentifiers(id as unknown as AstNode);
    for (const ident of idents) {
      declareVariable(
        scope,
        ident,
        "Variable",
        dec as unknown as AstNode,
        node as unknown as AstNode,
      );
    }
  }
}

function handleFunctionDeclaration(node: NodeLike, scope: Scope): void {
  const id = node["id"];
  if (!isIdentifierNode(id)) {
    return;
  }
  declareVariable(scope, id, "FunctionName", node as unknown as AstNode, null);
}

function handleClassDeclaration(node: NodeLike, scope: Scope): void {
  const id = node["id"];
  if (!isIdentifierNode(id)) {
    return;
  }
  declareVariable(scope, id, "ClassName", node as unknown as AstNode, null);
}

function handleImportDeclaration(node: NodeLike, scope: Scope): void {
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
      "ImportBinding",
      spec as unknown as AstNode,
      node as unknown as AstNode,
    );
  }
}

function isNodeLike(value: unknown): value is NodeLike {
  return (
    value !== null &&
    typeof value === "object" &&
    "type" in value &&
    typeof (value as { type: unknown }).type === "string"
  );
}

function isIdentifierNode(value: unknown): value is AstIdentifier {
  return isNodeLike(value) && value.type === "Identifier";
}
