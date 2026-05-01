import type { AstIdentifier, AstNode, Scope } from "../ir/model.js";
import type {
  AnalyzedSource,
  ParsedSource,
  ScopeAnalyzer,
} from "../pipeline/types.js";
import { DiagnosticCollector } from "../util/diagnostic.js";
import { spanFromOffset } from "../util/span.js";
import { classifyIdentifier } from "./classify.js";
import { collectBindingIdentifiers, declareVariable } from "./declare.js";
import { hoistDeclarations } from "./hoisting.js";
import { findJsxElementSpan } from "./jsx-element-span.js";
import { ScopeManager } from "./manager.js";
import { findReferenceOwners } from "./owner.js";
import { findPredicateContainer } from "./predicate.js";
import { bindReference } from "./resolve.js";
import { findReturnContainer } from "./return-container.js";
import { ReferenceImpl } from "./scope.js";
import { isTypeOnlySubtree } from "./skip-types.js";
import type { PathEntry, WalkAction } from "./walk.js";
import { walk } from "./walk.js";

interface NodeLike {
  type: string;
  start?: number;
  end?: number;
  [key: string]: unknown;
}

export class EslintCompatAnalyzer implements ScopeAnalyzer {
  readonly id = "eslint-compat";

  analyze(parsed: ParsedSource): AnalyzedSource {
    const program = parsed.ast as NodeLike;
    const isModule = parsed.language !== "js";
    const manager = new ScopeManager(
      isModule ? "module" : "global",
      program as unknown as AstNode,
    );
    const diagnostics = new DiagnosticCollector();

    hoistInto(program, manager.current(), parsed.raw, diagnostics);

    walk(program as unknown as AstNode, {
      enter(node, parent, key, path) {
        return handleEnter(
          node as unknown as NodeLike,
          parent as unknown as NodeLike | null,
          key,
          path,
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
      raw: parsed.raw,
    };
  }
}

function handleEnter(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: ReadonlyArray<PathEntry>,
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
): WalkAction {
  if (isTypeOnlySubtree(node.type, key)) {
    return "skip";
  }
  if (node.type === "Identifier" || node.type === "JSXIdentifier") {
    handleIdentifierReference(node, parent, key, path, manager);
    return;
  }
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
      const ctx =
        parent && key !== null
          ? {
              parentType: parent.type,
              key,
              parentSpanOffset: parent.start ?? 0,
            }
          : null;
      const scope = manager.push("block", node as unknown as AstNode, ctx);
      const stmts = node["body"];
      if (Array.isArray(stmts)) {
        hoistDeclarations(stmts, scope, raw, diagnostics);
      }
      return;
    }
    case "ForStatement":
    case "ForOfStatement":
    case "ForInStatement": {
      const ctx =
        parent && key !== null
          ? {
              parentType: parent.type,
              key,
              parentSpanOffset: parent.start ?? 0,
            }
          : null;
      const scope = manager.push("for", node as unknown as AstNode, ctx);
      declareForLeft(node, scope, raw, diagnostics);
      return;
    }
    case "SwitchStatement": {
      const ctx =
        parent && key !== null
          ? {
              parentType: parent.type,
              key,
              parentSpanOffset: parent.start ?? 0,
            }
          : null;
      manager.push("switch", node as unknown as AstNode, ctx);
      return;
    }
    case "SwitchCase": {
      const test = node["test"];
      const caseTest = isNodeLike(test) ? formatCaseTest(test, raw) : null;
      const ctx =
        parent && key !== null
          ? {
              parentType: parent.type,
              key,
              parentSpanOffset: parent.start ?? 0,
              caseTest,
            }
          : null;
      const scope = manager.push("block", node as unknown as AstNode, ctx);
      const consequent = node["consequent"];
      if (Array.isArray(consequent)) {
        (
          scope as unknown as {
            unsnarlFallsThrough: boolean;
            unsnarlExitsFunction: boolean;
          }
        ).unsnarlFallsThrough = caseFallsThrough(consequent);
        (
          scope as unknown as {
            unsnarlFallsThrough: boolean;
            unsnarlExitsFunction: boolean;
          }
        ).unsnarlExitsFunction = caseExitsFunction(consequent);
        hoistDeclarations(consequent, scope, raw, diagnostics);
      } else {
        (
          scope as unknown as { unsnarlFallsThrough: boolean }
        ).unsnarlFallsThrough = true;
      }
      return;
    }
    case "CatchClause": {
      const ctx =
        parent && key !== null
          ? {
              parentType: parent.type,
              key,
              parentSpanOffset: parent.start ?? 0,
            }
          : null;
      const scope = manager.push("catch", node as unknown as AstNode, ctx);
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
    case "SwitchCase":
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

function handleIdentifierReference(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: ReadonlyArray<PathEntry>,
  manager: ScopeManager,
): void {
  const result = classifyIdentifier(
    parent as unknown as AstNode | null,
    key,
    path,
  );
  if (result.kind !== "reference") {
    return;
  }
  const ref = new ReferenceImpl({
    identifier: node as unknown as AstIdentifier,
    from: manager.current(),
    flags: result.flags,
    init: result.init,
    writeExpr: result.writeExpr,
  });
  bindReference(manager.current(), ref, manager.globalScope);
  ref.unsnarlOwners = findReferenceOwners(path, manager.current());
  ref.unsnarlPredicateContainer = findPredicateContainer(
    parent as unknown as { type: string; start?: number } | null,
    key,
    path,
  );
  ref.unsnarlReturnContainer = findReturnContainer(path);
  ref.unsnarlJsxElement = findJsxElementSpan(path);
}

const CASE_TEST_MAX_LENGTH = 32;

function formatCaseTest(node: NodeLike, raw: string): string {
  const start = node.start;
  const end = node.end;
  if (
    typeof start === "number" &&
    typeof end === "number" &&
    end > start &&
    end <= raw.length &&
    end - start <= CASE_TEST_MAX_LENGTH
  ) {
    return raw.slice(start, end);
  }
  switch (node.type) {
    case "Identifier": {
      const name = node["name"];
      return typeof name === "string" ? name : "<expr>";
    }
    case "NullLiteral":
      return "null";
    case "BooleanLiteral":
    case "NumericLiteral":
    case "StringLiteral": {
      const value = node["value"];
      if (typeof value === "string") {
        return JSON.stringify(value);
      }
      if (
        typeof value === "number" ||
        typeof value === "boolean" ||
        value === null
      ) {
        return String(value);
      }
      return "<expr>";
    }
    default:
      return "<expr>";
  }
}

function caseFallsThrough(consequent: ReadonlyArray<unknown>): boolean {
  if (consequent.length === 0) {
    return true;
  }
  const last = consequent[consequent.length - 1];
  if (!isNodeLike(last)) {
    return true;
  }
  return !isControlExit(last);
}

function caseExitsFunction(consequent: ReadonlyArray<unknown>): boolean {
  if (consequent.length === 0) {
    return false;
  }
  const last = consequent[consequent.length - 1];
  if (!isNodeLike(last)) {
    return false;
  }
  return isFunctionExit(last);
}

function isFunctionExit(node: NodeLike): boolean {
  switch (node.type) {
    case "ReturnStatement":
    case "ThrowStatement":
      return true;
    case "BlockStatement": {
      const body = node["body"];
      if (Array.isArray(body) && body.length > 0) {
        const last = body[body.length - 1];
        if (isNodeLike(last)) {
          return isFunctionExit(last);
        }
      }
      return false;
    }
    case "IfStatement": {
      const consequent = node["consequent"];
      const alternate = node["alternate"];
      if (
        isNodeLike(consequent) &&
        isNodeLike(alternate) &&
        isFunctionExit(consequent) &&
        isFunctionExit(alternate)
      ) {
        return true;
      }
      return false;
    }
    default:
      return false;
  }
}

function isControlExit(node: NodeLike): boolean {
  switch (node.type) {
    case "BreakStatement":
    case "ContinueStatement":
    case "ReturnStatement":
    case "ThrowStatement":
      return true;
    case "BlockStatement": {
      const body = node["body"];
      if (Array.isArray(body) && body.length > 0) {
        const last = body[body.length - 1];
        if (isNodeLike(last)) {
          return isControlExit(last);
        }
      }
      return false;
    }
    case "IfStatement": {
      const consequent = node["consequent"];
      const alternate = node["alternate"];
      if (
        isNodeLike(consequent) &&
        isNodeLike(alternate) &&
        isControlExit(consequent) &&
        isControlExit(alternate)
      ) {
        return true;
      }
      return false;
    }
    default:
      return false;
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
