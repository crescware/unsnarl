import { describe, expect, test } from "vitest";

import { ReferenceFlags } from "../../ir/model.js";
import type { AstNode } from "../../ir/model.js";
import type { PathEntry } from "../walk/walk.js";
import { classifyIdentifier } from "./classify-identifier.js";

const node = (overrides: Record<string, unknown>): AstNode =>
  ({ type: overrides.type as string, ...overrides }) as unknown as AstNode;

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
      classifyIdentifier(node({ type: "ImportSpecifier" }), "imported", []),
    ).toEqual({ kind: "skip" });
  });

  test("direct binding context (e.g. VariableDeclarator#id)", () => {
    expect(
      classifyIdentifier(node({ type: "VariableDeclarator" }), "id", []),
    ).toEqual({ kind: "binding" });
  });

  test("pattern step under VariableDeclarator → binding", () => {
    const path: readonly PathEntry[] = [
      { node: node({ type: "Program" }), key: null },
      { node: node({ type: "VariableDeclaration" }), key: "body" },
      { node: node({ type: "VariableDeclarator" }), key: "declarations" },
      { node: node({ type: "ObjectPattern" }), key: "id" },
    ];
    expect(
      classifyIdentifier(node({ type: "ObjectPattern" }), "id", path),
    ).toEqual({ kind: "binding" });
  });

  test("pattern step under AssignmentExpression → Write reference", () => {
    const path: readonly PathEntry[] = [
      { node: node({ type: "ExpressionStatement" }), key: null },
      { node: node({ type: "AssignmentExpression" }), key: "expression" },
      { node: node({ type: "ArrayPattern" }), key: "left" },
    ];
    expect(
      classifyIdentifier(node({ type: "ArrayPattern" }), "left", path),
    ).toEqual({
      kind: "reference",
      flags: ReferenceFlags.Write,
      init: false,
      writeExpr: null,
    });
  });

  test("falls through to ordinary reference (CallExpression#callee)", () => {
    expect(
      classifyIdentifier(node({ type: "CallExpression" }), "callee", []),
    ).toMatchObject({
      kind: "reference",
      flags: ReferenceFlags.Read | ReferenceFlags.Call,
    });
  });
});
