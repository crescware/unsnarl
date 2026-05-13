import { describe, expect, test } from "vitest";

import { asScopeId, type ScopeId } from "../../ir/serialized/scope-id.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { asFilledString } from "../../util/filled-string.js";
import { branchContainerKey } from "./branch-container-key.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

describe("branchContainerKey", () => {
  test("returns null when blockContext is missing", () => {
    expect(branchContainerKey(baseScope())).toEqual(null);
  });

  test.each([
    {
      name: asFilledString("switch cases -> switch:<upper>:<offset>"),
      upper: asScopeId("outer") as ScopeId | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: asFilledString("cases"),
        parentSpanOffset: 12,
      },
      expected: "switch:outer:12" as string | null,
    },
    {
      name: asFilledString("if consequent -> if:<upper>:<offset>"),
      upper: asScopeId("outer") as ScopeId | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 3,
      },
      expected: "if:outer:3",
    },
    {
      name: asFilledString("if alternate -> if:<upper>:<offset>"),
      upper: asScopeId("outer") as ScopeId | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("alternate"),
        parentSpanOffset: 3,
      },
      expected: "if:outer:3",
    },
    {
      name: asFilledString("switch with non-cases key -> null"),
      upper: asScopeId("outer") as ScopeId | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: asFilledString("discriminant"),
        parentSpanOffset: 7,
      },
      expected: null,
    },
    {
      name: asFilledString(
        "if with key other than consequent/alternate -> null",
      ),
      upper: asScopeId("outer") as ScopeId | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("test"),
        parentSpanOffset: 3,
      },
      expected: null,
    },
    {
      name: asFilledString("try block -> try:<upper>:<offset>"),
      upper: asScopeId("outer") as ScopeId | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("block"),
        parentSpanOffset: 9,
      },
      expected: "try:outer:9",
    },
    {
      name: asFilledString("try handler -> try:<upper>:<offset>"),
      upper: asScopeId("outer") as ScopeId | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("handler"),
        parentSpanOffset: 9,
      },
      expected: "try:outer:9",
    },
    {
      name: asFilledString(
        "try finalizer -> null (post-merge node, not a sibling branch)",
      ),
      upper: asScopeId("outer") as ScopeId | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("finalizer"),
        parentSpanOffset: 9,
      },
      expected: null,
    },
    {
      name: asFilledString(
        "if branch with ifChainRootOffset -> uses chain root for the key",
      ),
      upper: asScopeId("outer") as ScopeId | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 40,
        ifChainRootOffset: 5,
      },
      expected: "if:outer:5",
    },
    {
      name: asFilledString(
        "if branch alternate with ifChainRootOffset -> shares the same chain key",
      ),
      upper: asScopeId("outer") as ScopeId | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("alternate"),
        parentSpanOffset: 40,
        ifChainRootOffset: 5,
      },
      expected: "if:outer:5",
    },
    {
      name: asFilledString("unrelated parent type -> null"),
      upper: asScopeId("outer") as ScopeId | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.ForStatement,
        key: asFilledString("body"),
        parentSpanOffset: 5,
      },
      expected: null,
    },
    {
      name: asFilledString("null upper renders as empty string in the key"),
      upper: null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 1,
      },
      expected: "if::1",
    },
  ])("$name", ({ upper, ctx, expected }) => {
    const scope = { ...baseScope(), upper, blockContext: ctx };
    expect(branchContainerKey(scope)).toEqual(expected);
  });
});
