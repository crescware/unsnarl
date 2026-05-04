import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { ReferenceFlags } from "../../ir/reference/reference-flags.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { classifyOrdinaryReference } from "./classify-ordinary-reference.js";

const node = (overrides: Record<string, unknown>): AstNode =>
  ({ type: overrides["type"] as string, ...overrides }) as unknown as AstNode;

describe("classifyOrdinaryReference", () => {
  test("AssignmentExpression#left with `=` → Write only", () => {
    const r = classifyOrdinaryReference(
      AST_TYPE.AssignmentExpression,
      "left",
      node({ type: AST_TYPE.AssignmentExpression, operator: "=" }),
    );
    expect(r).toEqual({
      kind: "reference",
      flags: ReferenceFlags.Write,
      init: false,
    });
  });

  test("AssignmentExpression#left with compound op → Read|Write", () => {
    const r = classifyOrdinaryReference(
      AST_TYPE.AssignmentExpression,
      "left",
      node({ type: AST_TYPE.AssignmentExpression, operator: "+=" }),
    );
    expect(r).toMatchObject({
      kind: "reference",
      flags: ReferenceFlags.Read | ReferenceFlags.Write,
    });
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
    expect(r).toMatchObject({ flags: ReferenceFlags.Read, init: false });
  });
});
