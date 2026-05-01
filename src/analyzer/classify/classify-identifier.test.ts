import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../constants.js";
import { ReferenceFlags } from "../../ir/model.js";
import type { AstNode } from "../../ir/model.js";
import type { PathEntry } from "../walk/walk.js";
import { classifyIdentifier } from "./classify-identifier.js";

const node = (overrides: Record<string, unknown>): AstNode =>
  ({ type: overrides["type"] as string, ...overrides }) as unknown as AstNode;

describe("classifyIdentifier dispatch", () => {
  test("no parent → plain Read reference", () => {
    expect(classifyIdentifier(null, null, [])).toEqual({
      kind: "reference",
      flags: ReferenceFlags.Read,
      init: false,
      writeExpr: null,
    });
  });

  test("skip context wins (e.g. ImportSpecifier#imported)", () => {
    expect(
      classifyIdentifier(
        node({ type: AST_TYPE.ImportSpecifier }),
        "imported",
        [],
      ),
    ).toEqual({ kind: "skip" });
  });

  test("direct binding context (e.g. VariableDeclarator#id)", () => {
    expect(
      classifyIdentifier(node({ type: AST_TYPE.VariableDeclarator }), "id", []),
    ).toEqual({ kind: "binding" });
  });

  test("pattern step under VariableDeclarator → binding", () => {
    const path: readonly PathEntry[] = [
      { node: node({ type: AST_TYPE.Program }), key: null },
      { node: node({ type: AST_TYPE.VariableDeclaration }), key: "body" },
      {
        node: node({ type: AST_TYPE.VariableDeclarator }),
        key: "declarations",
      },
      { node: node({ type: AST_TYPE.ObjectPattern }), key: "id" },
    ];
    expect(
      classifyIdentifier(node({ type: AST_TYPE.ObjectPattern }), "id", path),
    ).toEqual({ kind: "binding" });
  });

  test("pattern step under AssignmentExpression → Write reference", () => {
    const path: readonly PathEntry[] = [
      { node: node({ type: AST_TYPE.ExpressionStatement }), key: null },
      {
        node: node({ type: AST_TYPE.AssignmentExpression }),
        key: "expression",
      },
      { node: node({ type: AST_TYPE.ArrayPattern }), key: "left" },
    ];
    expect(
      classifyIdentifier(node({ type: AST_TYPE.ArrayPattern }), "left", path),
    ).toEqual({
      kind: "reference",
      flags: ReferenceFlags.Write,
      init: false,
      writeExpr: null,
    });
  });

  test("falls through to ordinary reference (CallExpression#callee)", () => {
    expect(
      classifyIdentifier(node({ type: AST_TYPE.CallExpression }), "callee", []),
    ).toMatchObject({
      kind: "reference",
      flags: ReferenceFlags.Read | ReferenceFlags.Call,
    });
  });
});
