import { describe, expect, test } from "vitest";

import { ReferenceFlags } from "../../ir/model.js";
import type { AstNode } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { classifyOrdinaryReference } from "./classify-ordinary-reference.js";

const node = (overrides: Record<string, unknown>): AstNode =>
  ({ type: overrides["type"] as string, ...overrides }) as unknown as AstNode;

describe("classifyOrdinaryReference", () => {
  test("AssignmentExpression#left with `=` → Write only, writeExpr from `right`", () => {
    const right = node({ type: AST_TYPE.Literal });
    const r = classifyOrdinaryReference(
      AST_TYPE.AssignmentExpression,
      "left",
      node({ type: AST_TYPE.AssignmentExpression, operator: "=", right }),
    );
    expect(r).toEqual({
      kind: "reference",
      flags: ReferenceFlags.Write,
      init: false,
      writeExpr: right,
    });
  });

  test("AssignmentExpression#left with compound op → Read|Write", () => {
    const right = node({ type: AST_TYPE.Literal });
    const r = classifyOrdinaryReference(
      AST_TYPE.AssignmentExpression,
      "left",
      node({ type: AST_TYPE.AssignmentExpression, operator: "+=", right }),
    );
    expect(r).toMatchObject({
      kind: "reference",
      flags: ReferenceFlags.Read | ReferenceFlags.Write,
      writeExpr: right,
    });
  });

  test("AssignmentExpression with non-AST right yields writeExpr=null", () => {
    const r = classifyOrdinaryReference(
      AST_TYPE.AssignmentExpression,
      "left",
      node({ type: AST_TYPE.AssignmentExpression, operator: "=", right: 1 }),
    );
    expect(r).toMatchObject({ writeExpr: null });
  });

  test("UpdateExpression#argument → Read|Write", () => {
    const r = classifyOrdinaryReference(
      AST_TYPE.UpdateExpression,
      "argument",
      node({ type: AST_TYPE.UpdateExpression }),
    );
    expect(r).toMatchObject({
      flags: ReferenceFlags.Read | ReferenceFlags.Write,
    });
  });

  test("CallExpression#callee → Read|Call", () => {
    const r = classifyOrdinaryReference(
      AST_TYPE.CallExpression,
      "callee",
      node({ type: AST_TYPE.CallExpression }),
    );
    expect(r).toMatchObject({
      flags: ReferenceFlags.Read | ReferenceFlags.Call,
    });
  });

  test("NewExpression#callee → Read|Call", () => {
    const r = classifyOrdinaryReference(
      AST_TYPE.NewExpression,
      "callee",
      node({ type: AST_TYPE.NewExpression }),
    );
    expect(r).toMatchObject({
      flags: ReferenceFlags.Read | ReferenceFlags.Call,
    });
  });

  test("MemberExpression#object → Read|Receiver", () => {
    const r = classifyOrdinaryReference(
      AST_TYPE.MemberExpression,
      "object",
      node({ type: AST_TYPE.MemberExpression }),
    );
    expect(r).toMatchObject({
      flags: ReferenceFlags.Read | ReferenceFlags.Receiver,
    });
  });

  test("VariableDeclarator#init → Read with init=true", () => {
    const r = classifyOrdinaryReference(
      AST_TYPE.VariableDeclarator,
      "init",
      node({ type: AST_TYPE.VariableDeclarator }),
    );
    expect(r).toMatchObject({ flags: ReferenceFlags.Read, init: true });
  });

  test("default fallback → Read with init=false", () => {
    const r = classifyOrdinaryReference(
      AST_TYPE.CallExpression,
      "arguments",
      node({ type: AST_TYPE.CallExpression }),
    );
    expect(r).toMatchObject({
      flags: ReferenceFlags.Read,
      init: false,
      writeExpr: null,
    });
  });
});
