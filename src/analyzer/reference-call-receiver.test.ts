import { describe, expect, test } from "vitest";

import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { referenceCallReceiverFlags } from "./reference-call-receiver.js";

const node = (type: string): AstNode => ({ type }) as unknown as AstNode;

describe("referenceCallReceiverFlags", () => {
  test("CallExpression#callee → call only", () => {
    expect(
      referenceCallReceiverFlags(node(AST_TYPE.CallExpression), "callee"),
    ).toEqual({ call: true, receiver: false });
  });

  test("NewExpression#callee → call only", () => {
    expect(
      referenceCallReceiverFlags(node(AST_TYPE.NewExpression), "callee"),
    ).toEqual({ call: true, receiver: false });
  });

  test("MemberExpression#object → receiver only", () => {
    expect(
      referenceCallReceiverFlags(node(AST_TYPE.MemberExpression), "object"),
    ).toEqual({ call: false, receiver: true });
  });

  test("CallExpression at non-callee key → neither", () => {
    expect(
      referenceCallReceiverFlags(node(AST_TYPE.CallExpression), "arguments"),
    ).toEqual({ call: false, receiver: false });
  });

  test("MemberExpression at non-object key → neither", () => {
    expect(
      referenceCallReceiverFlags(node(AST_TYPE.MemberExpression), "property"),
    ).toEqual({ call: false, receiver: false });
  });

  test("null parent → neither", () => {
    expect(referenceCallReceiverFlags(null, null)).toEqual({
      call: false,
      receiver: false,
    });
  });

  test("unrelated parent type → neither", () => {
    expect(
      referenceCallReceiverFlags(node(AST_TYPE.VariableDeclarator), "init"),
    ).toEqual({ call: false, receiver: false });
  });
});
