import { describe, expect, test } from "vitest";

import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { findPredicateContainer } from "./predicate.js";
import type { PathEntry } from "./walk/path-entry.js";

function ifNode(start: number): AstNode {
  return { type: AST_TYPE.IfStatement, start };
}

function switchNode(start: number): AstNode {
  return { type: AST_TYPE.SwitchStatement, start };
}

function blockNode(start: number): AstNode {
  return { type: AST_TYPE.BlockStatement, start };
}

function programNode(): AstNode {
  return { type: AST_TYPE.Program, start: 0 };
}

function binExprNode(start: number): AstNode {
  return { type: AST_TYPE.BinaryExpression, start };
}

function entry(node: AstNode, key: string | null): PathEntry {
  return { node, key };
}

describe("findPredicateContainer", () => {
  test("returns null for empty path and null parent", () => {
    expect(findPredicateContainer(null, null, [])).toBeNull();
  });

  test("matches a single if's test reference and returns the if's start offset", () => {
    const ifStmt = ifNode(10);
    const testExpr = binExprNode(14);
    const path: readonly PathEntry[] = [
      entry(programNode(), null),
      entry(ifStmt, "body"),
      entry(testExpr, "test"),
    ];
    expect(findPredicateContainer(testExpr, "left", path)).toEqual({
      type: AST_TYPE.IfStatement,
      offset: 10,
    });
  });

  test("returns the inner if's own offset for an else-if test reference", () => {
    const outer = ifNode(10);
    const inner = ifNode(40);
    const innerTest = binExprNode(44);
    const path: readonly PathEntry[] = [
      entry(programNode(), null),
      entry(outer, "body"),
      entry(inner, "alternate"),
      entry(innerTest, "test"),
    ];
    expect(findPredicateContainer(innerTest, "left", path)).toEqual({
      type: AST_TYPE.IfStatement,
      offset: 40,
    });
  });

  test("matches a switch discriminant reference and returns the switch's start offset", () => {
    const switchStmt = switchNode(10);
    const path: readonly PathEntry[] = [
      entry(programNode(), null),
      entry(switchStmt, "body"),
    ];
    expect(findPredicateContainer(switchStmt, "discriminant", path)).toEqual({
      type: AST_TYPE.SwitchStatement,
      offset: 10,
    });
  });

  test("returns null for a reference outside any test or discriminant", () => {
    const ifStmt = ifNode(10);
    const consequent = blockNode(20);
    const path: readonly PathEntry[] = [
      entry(programNode(), null),
      entry(ifStmt, "body"),
      entry(consequent, "consequent"),
    ];
    expect(findPredicateContainer(consequent, "body", path)).toBeNull();
  });

  test("falls back to parent's start when path is empty (immediate IfStatement parent)", () => {
    const parent = { type: AST_TYPE.IfStatement, start: 99 };
    expect(findPredicateContainer(parent, "test", [])).toEqual({
      type: AST_TYPE.IfStatement,
      offset: 99,
    });
  });
});
