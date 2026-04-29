import type { AstNode, Diagnostic, Scope } from "../ir/model.js";
import type { ParsedSource, ScopeAnalyzer } from "../pipeline/types.js";
import { DiagnosticCollector } from "../util/diagnostic.js";
import { spanFromOffset } from "../util/span.js";
import { collectBindingIdentifiers, declareVariable } from "./declare.js";
import { hoistDeclarations } from "./hoisting.js";
import { ScopeManager } from "./manager.js";
import { walk } from "./walk.js";

export interface AnalysisResult {
  rootScope: Scope;
  diagnostics: readonly Diagnostic[];
}

interface NodeLike {
  type: string;
  start?: number;
  end?: number;
  [key: string]: unknown;
}

export class EslintCompatAnalyzer implements ScopeAnalyzer {
  readonly id = "eslint-compat";

  analyze(parsed: ParsedSource): Scope {
    return this.analyzeWithDiagnostics(parsed).rootScope;
  }

  analyzeWithDiagnostics(parsed: ParsedSource): AnalysisResult {
    const program = parsed.ast as NodeLike;
    const isModule = parsed.language !== "js";
    const manager = new ScopeManager(
      isModule ? "module" : "global",
      program as unknown as AstNode,
    );
    const diagnostics = new DiagnosticCollector();

    hoistInto(program, manager.current(), parsed.raw, diagnostics);

    walk(program as unknown as AstNode, {
      enter(node, parent, key) {
        handleEnter(
          node as unknown as NodeLike,
          parent as unknown as NodeLike | null,
          key,
          manager,
          parsed.raw,
          diagnostics,
        );
      },
      leave(node, parent, key) {
        handleLeave(
          node as unknown as NodeLike,
          parent as unknown as NodeLike | null,
          key,
          manager,
        );
      },
    });

    return {
      rootScope: manager.globalScope,
      diagnostics: diagnostics.list(),
    };
  }
}

function handleEnter(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  switch (node.type) {
    case "FunctionDeclaration":
    case "FunctionExpression":
    case "ArrowFunctionExpression": {
      const scope = manager.push("function", node as unknown as AstNode);
      declareFunctionParams(node, scope);
      const body = node["body"];
      if (isNodeLike(body) && body.type === "BlockStatement") {
        const stmts = body["body"];
        if (Array.isArray(stmts)) {
          hoistDeclarations(stmts, scope, raw, diagnostics);
        }
      }
      return;
    }
    case "BlockStatement": {
      if (parent && key === "body" && skipBlockScope(parent.type)) {
        return;
      }
      const scope = manager.push("block", node as unknown as AstNode);
      const stmts = node["body"];
      if (Array.isArray(stmts)) {
        hoistDeclarations(stmts, scope, raw, diagnostics);
      }
      return;
    }
    case "ForStatement":
    case "ForOfStatement":
    case "ForInStatement": {
      const scope = manager.push("for", node as unknown as AstNode);
      declareForLeft(node, scope, raw, diagnostics);
      return;
    }
    case "SwitchStatement": {
      const scope = manager.push("switch", node as unknown as AstNode);
      const cases = node["cases"];
      if (Array.isArray(cases)) {
        for (const c of cases) {
          if (!isNodeLike(c)) {
            continue;
          }
          const consequent = c["consequent"];
          if (Array.isArray(consequent)) {
            hoistDeclarations(consequent, scope, raw, diagnostics);
          }
        }
      }
      return;
    }
    case "CatchClause": {
      const scope = manager.push("catch", node as unknown as AstNode);
      const param = node["param"];
      if (isNodeLike(param)) {
        const idents = collectBindingIdentifiers(param as unknown as AstNode);
        for (const ident of idents) {
          declareVariable(
            scope,
            ident,
            "CatchClause",
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
      return;
    }
    default:
      return;
  }
}

function handleLeave(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  manager: ScopeManager,
): void {
  switch (node.type) {
    case "FunctionDeclaration":
    case "FunctionExpression":
    case "ArrowFunctionExpression":
    case "ForStatement":
    case "ForOfStatement":
    case "ForInStatement":
    case "SwitchStatement":
    case "CatchClause":
      manager.pop();
      return;
    case "BlockStatement":
      if (parent && key === "body" && skipBlockScope(parent.type)) {
        return;
      }
      manager.pop();
      return;
    default:
      return;
  }
}

function skipBlockScope(parentType: string): boolean {
  return (
    parentType === "FunctionDeclaration" ||
    parentType === "FunctionExpression" ||
    parentType === "ArrowFunctionExpression" ||
    parentType === "CatchClause"
  );
}

function hoistInto(
  program: NodeLike,
  scope: Scope,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  const body = program["body"];
  if (Array.isArray(body)) {
    hoistDeclarations(body, scope, raw, diagnostics);
  }
}

function declareFunctionParams(node: NodeLike, scope: Scope): void {
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

function declareForLeft(
  node: NodeLike,
  scope: Scope,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  const candidates = [node["init"], node["left"]];
  for (const cand of candidates) {
    if (!isNodeLike(cand) || cand.type !== "VariableDeclaration") {
      continue;
    }
    const kind = cand["kind"];
    if (kind === "var") {
      diagnostics.add(
        "var-detected",
        "var declaration is not supported and was skipped.",
        spanFromOffset(raw, cand.start ?? 0),
      );
      continue;
    }
    if (kind !== "const" && kind !== "let") {
      continue;
    }
    const declarations = cand["declarations"];
    if (!Array.isArray(declarations)) {
      continue;
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
          cand as unknown as AstNode,
        );
      }
    }
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
