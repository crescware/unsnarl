import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../parser/ast-type.js";
import { branchContainerKey } from "./branch-container-key.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

describe("branchContainerKey", () => {
  test("returns null when blockContext is missing", () => {
    expect(branchContainerKey(baseScope())).toBeNull();
  });

  test.each([
    {
      name: "switch cases -> switch:<upper>:<offset>",
      upper: "outer" as string | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: "cases",
        parentSpanOffset: 12,
      },
      expected: "switch:outer:12" as string | null,
    },
    {
      name: "if consequent -> if:<upper>:<offset>",
      upper: "outer" as string | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 3,
      },
      expected: "if:outer:3",
    },
    {
      name: "if alternate -> if:<upper>:<offset>",
      upper: "outer" as string | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "alternate",
        parentSpanOffset: 3,
      },
      expected: "if:outer:3",
    },
    {
      name: "switch with non-cases key -> null",
      upper: "outer" as string | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: "discriminant",
        parentSpanOffset: 7,
      },
      expected: null,
    },
    {
      name: "if with key other than consequent/alternate -> null",
      upper: "outer" as string | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "test",
        parentSpanOffset: 3,
      },
      expected: null,
    },
    {
      name: "unrelated parent type -> null",
      upper: "outer" as string | null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.ForStatement,
        key: "body",
        parentSpanOffset: 5,
      },
      expected: null,
    },
    {
      name: "null upper renders as empty string in the key",
      upper: null,
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 1,
      },
      expected: "if::1",
    },
  ])("$name", ({ upper, ctx, expected }) => {
    const scope = { ...baseScope(), upper, blockContext: ctx };
    expect(branchContainerKey(scope)).toBe(expected);
  });
});
