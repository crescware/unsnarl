import type { AstNode } from "../ir/primitive/ast-node.js";
import type { Scope } from "../ir/scope/scope.js";
import type { Variable } from "../ir/scope/variable.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { isAstNode } from "./is-ast-node.js";

// Source of truth for `Annotations.ofVariable(v).isUnused`. The analysis
// pipeline calls this once per variable to populate the VariableAnnotation
// side-table; downstream consumers should read through
// `Annotations.ofVariable` rather than calling this directly.
//
// A variable is considered unused when no Read reference originates from
// outside the variable's own defining body. Writes (the init Write and
// any later re-assignments) and self-internal Reads (the recursive call
// inside `function foo() { foo(); }` or the body of `const a = () => a;`)
// do not count as usage. See #45 (write-only) and #68 (self-internal Read).
//
// Mutual recursion (`function f() { g(); } function g() { f(); }`) keeps
// both variables not-unused: each Read in `f.references` originates from
// `g`'s body, which is outside `f`'s body, so it counts as external. This
// matches eslint's `no-unused-vars` default.
//
// Class self-references are intentionally not handled here: although
// the eslint-scope-compat layer now pushes a dedicated `ClassScope`
// for ClassDeclaration / ClassExpression (#70), `FUNCTIONLIKE_TYPES`
// below still only enumerates function-shaped definitions. #71 will
// add ClassDeclaration / ClassExpression so the scope-ancestor check
// can recognise a method's function scope as being inside the class
// body.
export function isUnused(variable: Variable): boolean {
  const bodyScopes = collectBodyScopes(variable);
  for (const ref of variable.references) {
    if (!ref.isRead()) {
      continue;
    }
    if (!isFromInside(ref.from, bodyScopes)) {
      return false;
    }
  }
  return true;
}

const FUNCTIONLIKE_TYPES = new Set<string>([
  AST_TYPE.FunctionDeclaration,
  AST_TYPE.FunctionExpression,
  AST_TYPE.ArrowFunctionExpression,
]);

function collectBodyScopes(variable: Variable): ReadonlySet<Scope> {
  const bodyNodes = new Set<AstNode>();
  for (const def of variable.defs) {
    const body = bodyNodeOf(def.node);
    if (body !== null) {
      bodyNodes.add(body);
    }
  }
  if (bodyNodes.size === 0) {
    return new Set();
  }
  const result = new Set<Scope>();
  for (const child of variable.scope.childScopes) {
    if (bodyNodes.has(child.block)) {
      result.add(child);
    }
  }
  return result;
}

function bodyNodeOf(node: AstNode): AstNode | null {
  if (FUNCTIONLIKE_TYPES.has(node.type)) {
    return node;
  }
  if (node.type === AST_TYPE.VariableDeclarator) {
    const init = node["init"];
    if (isAstNode(init) && FUNCTIONLIKE_TYPES.has(init.type)) {
      return init;
    }
  }
  return null;
}

function isFromInside(from: Scope, bodyScopes: ReadonlySet<Scope>): boolean {
  if (bodyScopes.size === 0) {
    return false;
  }
  let scope: Scope | null = from;
  while (scope !== null) {
    if (bodyScopes.has(scope)) {
      return true;
    }
    scope = scope.upper;
  }
  return false;
}
