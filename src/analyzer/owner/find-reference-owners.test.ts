import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import type { AstIdentifier } from "../../ir/primitive/ast-identifier.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { declareVariable } from "../declare/declare-variable.js";
import { DEFINITION_TYPE } from "../definition-type.js";
import { SCOPE_TYPE } from "../scope-type.js";
import { ScopeImpl } from "../scope.js";
import type { PathEntry } from "../walk/walk.js";
import { findReferenceOwners } from "./find-reference-owners.js";

const ident = (name: string): AstIdentifier =>
  ({ type: AST_TYPE.Identifier, name }) as unknown as AstIdentifier;

const scopeWith = (...names: readonly string[]): ScopeImpl => {
  const scope = new ScopeImpl({
    type: SCOPE_TYPE.Module,
    isStrict: true,
    upper: null,
    block: { type: AST_TYPE.Program } as unknown as AstNode,
    blockContext: null,
  });
  for (const n of names) {
    declareVariable(
      scope,
      ident(n),
      DEFINITION_TYPE.Variable,
      { type: AST_TYPE.VariableDeclarator } as unknown as AstNode,
      null,
    );
  }
  return scope;
};

const programOf = (code: string): AstNode =>
  parseSync("input.ts", code, { lang: "ts" }).program as unknown as AstNode;

// Build the path stack (root-first) up to but not including the visited
// reference identifier itself, mirroring the way walk.ts hands `path` to
// the visitor on enter.
const pathTo = (
  root: AstNode,
  predicate: (n: AstNode) => boolean,
): readonly PathEntry[] => {
  const stack: /* mutable */ PathEntry[] = [];
  let found: readonly PathEntry[] | null = null;
  const visit = (node: AstNode, key: string | null): void => {
    if (found !== null) {
      return;
    }
    if (predicate(node)) {
      found = stack.slice();
      return;
    }
    stack.push({ node, key });
    for (const k of Object.keys(node)) {
      const child = (node as Record<string, unknown>)[k];
      if (child === null || child === undefined) {
        continue;
      }
      if (Array.isArray(child)) {
        for (const c of child) {
          if (c !== null && typeof c === "object" && "type" in c) {
            visit(c as AstNode, k);
          }
        }
      } else if (typeof child === "object" && "type" in (child as object)) {
        visit(child as AstNode, k);
      }
    }
    stack.pop();
  };
  visit(root, null);
  if (found === null) {
    throw new Error("predicate matched no node");
  }
  return found;
};

describe("findReferenceOwners", () => {
  test("VariableDeclarator scope: identifier on the right resolves to the LHS variable", () => {
    const program = programOf("const x = y;");
    const scope = scopeWith("x", "y");
    const path = pathTo(
      program,
      (n) =>
        n.type === AST_TYPE.Identifier && (n as { name?: string }).name === "y",
    );
    const owners = findReferenceOwners(path, scope);
    expect(owners.map((v) => v.name)).toEqual(["x"]);
  });

  test("AssignmentExpression with Identifier LHS resolves the LHS variable", () => {
    const program = programOf("x = y;");
    const scope = scopeWith("x", "y");
    const path = pathTo(
      program,
      (n) =>
        n.type === AST_TYPE.Identifier && (n as { name?: string }).name === "y",
    );
    const owners = findReferenceOwners(path, scope);
    expect(owners.map((v) => v.name)).toEqual(["x"]);
  });

  test("AssignmentExpression with destructuring LHS returns each binding", () => {
    const program = programOf("({ a, b } = src);");
    const scope = scopeWith("a", "b", "src");
    const path = pathTo(
      program,
      (n) =>
        n.type === AST_TYPE.Identifier &&
        (n as { name?: string }).name === "src",
    );
    const owners = findReferenceOwners(path, scope);
    expect(owners.map((v) => v.name).sort()).toEqual(["a", "b"]);
  });

  test("hits a function boundary → no owners", () => {
    const program = programOf("function f() { y; }");
    const scope = scopeWith("y");
    const path = pathTo(
      program,
      (n) =>
        n.type === AST_TYPE.Identifier && (n as { name?: string }).name === "y",
    );
    expect(findReferenceOwners(path, scope)).toEqual([]);
  });

  test("hits a class boundary → no owners", () => {
    const program = programOf("class C { m() { y; } }");
    const scope = scopeWith("y");
    const path = pathTo(
      program,
      (n) =>
        n.type === AST_TYPE.Identifier && (n as { name?: string }).name === "y",
    );
    expect(findReferenceOwners(path, scope)).toEqual([]);
  });

  test("identifier outside any declarator/assignment → no owners", () => {
    const program = programOf("foo(y);");
    const scope = scopeWith("y", "foo");
    const path = pathTo(
      program,
      (n) =>
        n.type === AST_TYPE.Identifier && (n as { name?: string }).name === "y",
    );
    expect(findReferenceOwners(path, scope)).toEqual([]);
  });
});
