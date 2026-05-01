import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import type { PathEntry } from "../walk/walk.js";
import { findBindingRootContext } from "./find-binding-root-context.js";

const node = (type: string): AstNode => ({ type }) as unknown as AstNode;

describe("findBindingRootContext", () => {
  test("non-pattern parent VariableDeclarator#id → 'var'", () => {
    expect(findBindingRootContext(node("VariableDeclarator"), "id", [])).toBe(
      "var",
    );
  });

  test("non-pattern parent VariableDeclarator#init → null", () => {
    expect(findBindingRootContext(node("VariableDeclarator"), "init", [])).toBe(
      null,
    );
  });

  test("CatchClause#param → 'catch'", () => {
    expect(findBindingRootContext(node("CatchClause"), "param", [])).toBe(
      "catch",
    );
  });

  test("Function/Arrow #params → 'param'", () => {
    expect(
      findBindingRootContext(node("FunctionDeclaration"), "params", []),
    ).toBe("param");
    expect(
      findBindingRootContext(node("ArrowFunctionExpression"), "params", []),
    ).toBe("param");
  });

  test("AssignmentExpression#left → 'assign'", () => {
    expect(
      findBindingRootContext(node("AssignmentExpression"), "left", []),
    ).toBe("assign");
  });

  test("AssignmentExpression#right → null", () => {
    expect(
      findBindingRootContext(node("AssignmentExpression"), "right", []),
    ).toBe(null);
  });

  test("walks up through ObjectPattern to reach VariableDeclarator#id", () => {
    // path stack: [VariableDeclarator (key=id, but recorded with key=parent's key on the way down), ObjectPattern (id), Property (properties)]
    // findBindingRootContext walks `parent` which is the immediate parent of the visited identifier.
    // We start at ObjectPattern parent (pattern step) and walk up to VariableDeclarator.
    const path: PathEntry[] = [
      { node: node("Program"), key: null },
      { node: node("VariableDeclaration"), key: "body" },
      { node: node("VariableDeclarator"), key: "declarations" },
      { node: node("ObjectPattern"), key: "id" },
    ];
    expect(findBindingRootContext(node("ObjectPattern"), "id", path)).toBe(
      "var",
    );
  });

  test("walks up through nested patterns and stops at AssignmentExpression#left", () => {
    const path: PathEntry[] = [
      { node: node("ExpressionStatement"), key: null },
      { node: node("AssignmentExpression"), key: "expression" },
      { node: node("ArrayPattern"), key: "left" },
    ];
    expect(findBindingRootContext(node("ArrayPattern"), "left", path)).toBe(
      "assign",
    );
  });

  test("path exhausted while still inside patterns → null", () => {
    expect(findBindingRootContext(node("ObjectPattern"), "id", [])).toBe(null);
  });

  test("non-binding parent like CallExpression → null", () => {
    expect(findBindingRootContext(node("CallExpression"), "callee", [])).toBe(
      null,
    );
  });
});
