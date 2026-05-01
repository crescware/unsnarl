import { describe, expect, test } from "vitest";

import { ReferenceFlags } from "../../ir/model.js";
import type { AstNode } from "../../ir/model.js";
import { classifyOrdinaryReference } from "./classify-ordinary-reference.js";

const node = (overrides: Record<string, unknown>): AstNode =>
  ({ type: overrides.type as string, ...overrides }) as unknown as AstNode;

describe("classifyOrdinaryReference", () => {
  test("AssignmentExpression#left with `=` → Write only, writeExpr from `right`", () => {
    const right = node({ type: "Literal" });
    const r = classifyOrdinaryReference(
      "AssignmentExpression",
      "left",
      node({ type: "AssignmentExpression", operator: "=", right }),
    );
    expect(r).toEqual({
      kind: "reference",
      flags: ReferenceFlags.Write,
      init: false,
      writeExpr: right,
    });
  });

  test("AssignmentExpression#left with compound op → Read|Write", () => {
    const right = node({ type: "Literal" });
    const r = classifyOrdinaryReference(
      "AssignmentExpression",
      "left",
      node({ type: "AssignmentExpression", operator: "+=", right }),
    );
    expect(r).toMatchObject({
      kind: "reference",
      flags: ReferenceFlags.Read | ReferenceFlags.Write,
      writeExpr: right,
    });
  });

  test("AssignmentExpression with non-AST right yields writeExpr=null", () => {
    const r = classifyOrdinaryReference(
      "AssignmentExpression",
      "left",
      node({ type: "AssignmentExpression", operator: "=", right: 1 }),
    );
    expect(r).toMatchObject({ writeExpr: null });
  });

  test("UpdateExpression#argument → Read|Write", () => {
    const r = classifyOrdinaryReference("UpdateExpression", "argument", node({ type: "UpdateExpression" }));
    expect(r).toMatchObject({ flags: ReferenceFlags.Read | ReferenceFlags.Write });
  });

  test("CallExpression#callee → Read|Call", () => {
    const r = classifyOrdinaryReference("CallExpression", "callee", node({ type: "CallExpression" }));
    expect(r).toMatchObject({ flags: ReferenceFlags.Read | ReferenceFlags.Call });
  });

  test("NewExpression#callee → Read|Call", () => {
    const r = classifyOrdinaryReference("NewExpression", "callee", node({ type: "NewExpression" }));
    expect(r).toMatchObject({ flags: ReferenceFlags.Read | ReferenceFlags.Call });
  });

  test("MemberExpression#object → Read|Receiver", () => {
    const r = classifyOrdinaryReference("MemberExpression", "object", node({ type: "MemberExpression" }));
    expect(r).toMatchObject({ flags: ReferenceFlags.Read | ReferenceFlags.Receiver });
  });

  test("VariableDeclarator#init → Read with init=true", () => {
    const r = classifyOrdinaryReference("VariableDeclarator", "init", node({ type: "VariableDeclarator" }));
    expect(r).toMatchObject({ flags: ReferenceFlags.Read, init: true });
  });

  test("default fallback → Read with init=false", () => {
    const r = classifyOrdinaryReference("CallExpression", "arguments", node({ type: "CallExpression" }));
    expect(r).toMatchObject({ flags: ReferenceFlags.Read, init: false, writeExpr: null });
  });
});
