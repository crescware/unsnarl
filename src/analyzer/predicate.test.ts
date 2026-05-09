import { describe, expect, test } from "vitest";

import type { PathEntry } from "../boundary/eslint-scope/walk/path-entry.js";
import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { findPredicateContainer } from "./predicate.js";

function ifNode(start: number): AstNode {
  return { type: AST_TYPE.IfStatement, start };
}

function switchNode(start: number): AstNode {
  return { type: AST_TYPE.SwitchStatement, start };
}

function whileNode(start: number): AstNode {
  return { type: AST_TYPE.WhileStatement, start };
}

function doWhileNode(start: number): AstNode {
  return { type: AST_TYPE.DoWhileStatement, start };
}

function forNode(start: number): AstNode {
  return { type: AST_TYPE.ForStatement, start };
}

function forOfNode(start: number): AstNode {
  return { type: AST_TYPE.ForOfStatement, start };
}

function forInNode(start: number): AstNode {
  return { type: AST_TYPE.ForInStatement, start };
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

  test("matches a WhileStatement.test reference and returns the while's start offset", () => {
    const whileStmt = whileNode(20);
    const testExpr = binExprNode(27);
    const path: readonly PathEntry[] = [
      entry(programNode(), null),
      entry(whileStmt, "body"),
      entry(testExpr, "test"),
    ];
    expect(findPredicateContainer(testExpr, "left", path)).toEqual({
      type: AST_TYPE.WhileStatement,
      offset: 20,
    });
  });

  test("WhileStatement only matches when curKey is 'test', not 'body'", () => {
    const whileStmt = whileNode(20);
    const inBody = blockNode(40);
    const path: readonly PathEntry[] = [
      entry(programNode(), null),
      entry(whileStmt, "body"),
      entry(inBody, "body"),
    ];
    expect(findPredicateContainer(inBody, "expression", path)).toBeNull();
  });

  test("matches a DoWhileStatement.test reference and returns the do-while's start offset", () => {
    const doStmt = doWhileNode(30);
    const testExpr = binExprNode(60);
    const path: readonly PathEntry[] = [
      entry(programNode(), null),
      entry(doStmt, "body"),
      entry(testExpr, "test"),
    ];
    expect(findPredicateContainer(testExpr, "left", path)).toEqual({
      type: AST_TYPE.DoWhileStatement,
      offset: 30,
    });
  });

  test.each([
    { key: "init" as const },
    { key: "test" as const },
    { key: "update" as const },
  ])(
    "ForStatement matches when curKey resolves to '$key' and returns the for's start offset",
    ({ key }) => {
      const forStmt = forNode(40);
      const inHeader = binExprNode(45);
      const path: readonly PathEntry[] = [
        entry(programNode(), null),
        entry(forStmt, "body"),
        entry(inHeader, key),
      ];
      expect(findPredicateContainer(inHeader, "left", path)).toEqual({
        type: AST_TYPE.ForStatement,
        offset: 40,
      });
    },
  );

  test("ForStatement does not match when curKey resolves to a non-header slot", () => {
    const forStmt = forNode(40);
    const inBody = blockNode(60);
    const path: readonly PathEntry[] = [
      entry(programNode(), null),
      entry(forStmt, "body"),
      entry(inBody, "body"),
    ];
    expect(findPredicateContainer(inBody, "expression", path)).toBeNull();
  });

  test.each([{ key: "left" as const }, { key: "right" as const }])(
    "ForOfStatement matches when curKey resolves to '$key'",
    ({ key }) => {
      const forOf = forOfNode(50);
      const inHeader = binExprNode(58);
      const path: readonly PathEntry[] = [
        entry(programNode(), null),
        entry(forOf, "body"),
        entry(inHeader, key),
      ];
      expect(findPredicateContainer(inHeader, "name", path)).toEqual({
        type: AST_TYPE.ForOfStatement,
        offset: 50,
      });
    },
  );

  test.each([{ key: "left" as const }, { key: "right" as const }])(
    "ForInStatement matches when curKey resolves to '$key'",
    ({ key }) => {
      const forIn = forInNode(70);
      const inHeader = binExprNode(78);
      const path: readonly PathEntry[] = [
        entry(programNode(), null),
        entry(forIn, "body"),
        entry(inHeader, key),
      ];
      expect(findPredicateContainer(inHeader, "name", path)).toEqual({
        type: AST_TYPE.ForInStatement,
        offset: 70,
      });
    },
  );

  test("ForOfStatement does not match when curKey resolves to 'body'", () => {
    const forOf = forOfNode(50);
    const inBody = blockNode(80);
    const path: readonly PathEntry[] = [
      entry(programNode(), null),
      entry(forOf, "body"),
      entry(inBody, "body"),
    ];
    expect(findPredicateContainer(inBody, "expression", path)).toBeNull();
  });

  test("falls back to parent's start when path is empty for WhileStatement.test", () => {
    const parent = { type: AST_TYPE.WhileStatement, start: 88 };
    expect(findPredicateContainer(parent, "test", [])).toEqual({
      type: AST_TYPE.WhileStatement,
      offset: 88,
    });
  });

  test("falls back to parent's start when path is empty for DoWhileStatement.test", () => {
    const parent = { type: AST_TYPE.DoWhileStatement, start: 77 };
    expect(findPredicateContainer(parent, "test", [])).toEqual({
      type: AST_TYPE.DoWhileStatement,
      offset: 77,
    });
  });

  test.each([
    { key: "init" as const },
    { key: "test" as const },
    { key: "update" as const },
  ])(
    "falls back to parent's start when path is empty for ForStatement.$key",
    ({ key }) => {
      const parent = { type: AST_TYPE.ForStatement, start: 55 };
      expect(findPredicateContainer(parent, key, [])).toEqual({
        type: AST_TYPE.ForStatement,
        offset: 55,
      });
    },
  );

  test.each([{ key: "left" as const }, { key: "right" as const }])(
    "falls back to parent's start when path is empty for ForOfStatement.$key",
    ({ key }) => {
      const parent = { type: AST_TYPE.ForOfStatement, start: 66 };
      expect(findPredicateContainer(parent, key, [])).toEqual({
        type: AST_TYPE.ForOfStatement,
        offset: 66,
      });
    },
  );

  test.each([{ key: "left" as const }, { key: "right" as const }])(
    "falls back to parent's start when path is empty for ForInStatement.$key",
    ({ key }) => {
      const parent = { type: AST_TYPE.ForInStatement, start: 77 };
      expect(findPredicateContainer(parent, key, [])).toEqual({
        type: AST_TYPE.ForInStatement,
        offset: 77,
      });
    },
  );
});
